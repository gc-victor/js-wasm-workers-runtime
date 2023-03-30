use std::{
    str,
    sync::{Arc, RwLock},
};

use wasmtime_environment::WasmtimeEnvironment;

use lazy_static::lazy_static;
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmtime::*;
use wasmtime_wasi::tokio::WasiCtxBuilder;

mod http;
mod import_send_request;
mod wasmtime_environment;

lazy_static! {
    static ref WASMTIME_ENVIRONMENT: WasmtimeEnvironment = WasmtimeEnvironment::default();
}

pub async fn runtime(handler: &str, request: &str) -> anyhow::Result<Vec<u8>> {
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

    let environment = WASMTIME_ENVIRONMENT.clone();
    let engine = environment.engine;
    let module = environment.module;
    let linker = environment.linker;

    let mut store = Store::new(&engine, wasi);
    let instance = linker.instantiate_async(&mut store, &module).await?;
    instance
        .get_typed_func::<(), ()>(&mut store, "_start")?
        .call_async(&mut store, ())
        .await?;

    let mut buffer = Vec::new();

    stdout_mutex
        .read()
        .map_err(|e| anyhow::Error::msg(format!("{e:?}")))?
        .iter()
        .for_each(|i| buffer.push(*i));

    Ok(buffer)
}
