use std::sync::{Arc, RwLock};

use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasi_common::{pipe::{ReadPipe, WritePipe}};

fn main() -> Result<()> {
    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let handler: &str = include_str!("./handler.js");
    let handler = handler.trim()
        .replace("export const handleRequest = ", "handleRequest = ")
        .replace("export function handleRequest", "function handleRequest")
        .replace("export async function handleRequest", "async function handleRequest");
    let mut contents = String::new();

    contents.push_str(&handler);

    // create a buffer to store the response
    let stdout_buf: Vec<u8> = vec![];
    let stdout_mutex = Arc::new(RwLock::new(stdout_buf));
    let stdout = WritePipe::from_shared(stdout_mutex.clone());
    let stdin = ReadPipe::from(contents);
    
    // TODO: use stderr to get the errors
    let wasi = WasiCtxBuilder::new()
        .stdin(Box::new(stdin))
        .stdout(Box::new(stdout))
        .build();
    let mut store = Store::new(&engine, wasi);
            
    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(&engine, "target/wasm32-wasi/release/js-wasm-workers-runtime.wasm")?;

    linker.module(&mut store, "", &module)?;

    linker
        .get_default(&mut store, "").unwrap()
        .typed::<(), ()>(&store).unwrap()
        .call(&mut store, ()).unwrap();

    // read the response into a string
    let mut buffer: Vec<u8> = Vec::new();

    stdout_mutex.read()
        .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?
        .iter().for_each(|i| {
            buffer.push(*i)
        });
    
    println!("returned: {:?}", String::from_utf8(buffer)?);

    Ok(())
}
