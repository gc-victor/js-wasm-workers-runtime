use std::sync::Arc;

use anyhow::Error;
use wasi_common::WasiCtx;
use wasmtime::{Config, Engine, Linker, Module};

use crate::import_send_request::import_send_request;

static WASM: &[u8] =
    include_bytes!("../../../target/wasm32-wasi/release/js-wasm-workers-engine.wasm");

#[derive(Clone)]
pub struct WasmtimeEnvironment {
    pub engine: Engine,
    pub module: Module,
    pub linker: Arc<Linker<WasiCtx>>,
}

impl WasmtimeEnvironment {
    pub fn new() -> Result<Self, Error> {
        let mut config = Config::new();
        let engine = Engine::new(config.async_support(true))?;
        let module = Module::from_binary(&engine, WASM)?;

        let mut linker = Linker::new(&engine);

        wasmtime_wasi::tokio::add_to_linker(&mut linker, |cx| cx)?;

        linker.func_wrap1_async("env", "import_send_request", import_send_request)?;

        Ok(Self {
            engine,
            module,
            linker: Arc::new(linker),
        })
    }
}

impl Default for WasmtimeEnvironment {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
