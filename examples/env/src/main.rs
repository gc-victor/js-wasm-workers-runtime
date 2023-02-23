use std::{
    str,
    sync::{Arc, RwLock},
};

use anyhow::Result;
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

fn main() -> Result<()> {
    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let handler: &str = include_str!("./handler.js");
    let handler = handler
        .trim()
        .replace(
            "export const handleRequest = ",
            "globalThis.handleRequest = ",
        )
        .replace(
            "export async function handleRequest",
            "async function handleRequest",
        );
    let mut contents = String::new();

    contents.push_str(&handler);

    // create a buffer to store the response
    let stdout_buf: Vec<u8> = vec![];
    let stdout_mutex = Arc::new(RwLock::new(stdout_buf));
    let stdout = WritePipe::from_shared(stdout_mutex.clone());
    let stdin = ReadPipe::from(contents);

    let body = serde_json::json!({"hello": "world"}).to_string();

    let arg = r#"{
        "body": __BODY__,
        "headers": {
            "content-type": "application/json",
            "x-test": "test"
        },
        "method": "POST",
        "url": "https://test.test"
    }"#
    .to_string()
    .replace("__BODY__", &format!("{:?}", body.as_bytes()));

    let wasi = WasiCtxBuilder::new()
        .stdin(Box::new(stdin))
        .stdout(Box::new(stdout))
        .arg(&arg)?
        .envs(&[("FOO".to_string(), "bar".to_string())])?
        .build();
    let mut store = Store::new(&engine, wasi);

    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(
        &engine,
        "target/wasm32-wasi/release/js-wasm-workers-runtime.wasm",
    )?;

    linker.module(&mut store, "", &module)?;

    linker
        .get_default(&mut store, "")
        .unwrap()
        .typed::<(), ()>(&store)
        .unwrap()
        .call(&mut store, ())
        .unwrap();

    // read the response into a string
    let mut buffer: Vec<u8> = Vec::new();

    stdout_mutex
        .read()
        .map_err(|e| anyhow::Error::msg(format!("{e:?}")))?
        .iter()
        .for_each(|i| buffer.push(*i));

    println!("returned: {:?}", String::from_utf8(buffer)?);

    Ok(())
}
