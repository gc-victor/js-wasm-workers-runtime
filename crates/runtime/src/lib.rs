use std::{
    str,
    sync::{Arc, RwLock},
};

use anyhow;
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

mod http;
mod import_send_request;

use import_send_request::import_send_request;

static WASM: &[u8] =
    include_bytes!("../../../target/wasm32-wasi/release/js-wasm-workers-runtime.wasm");

pub fn runtime(handler: &str, request: &str) -> anyhow::Result<Vec<u8>> {
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
    let stdout_buf: Vec<u8> = vec![];
    let stdout_mutex = Arc::new(RwLock::new(stdout_buf));
    let stdout = WritePipe::from_shared(stdout_mutex.clone());
    let stdin = ReadPipe::from(handler);

    let wasi = WasiCtxBuilder::new()
        .stdin(Box::new(stdin))
        .stdout(Box::new(stdout))
        .args(&[request.to_string()])?
        .build();

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    linker.func_wrap("env", "import_send_request", import_send_request)?;

    let mut store = Store::new(&engine, wasi);
    let module = Module::from_binary(&engine, WASM)?;

    linker.module(&mut store, "", &module)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;

    let mut buffer: Vec<u8> = Vec::new();

    stdout_mutex
        .read()
        .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?
        .iter()
        .for_each(|i| buffer.push(*i));

    Ok(buffer)
}
