#![allow(dead_code)]
#![allow(unused_imports)]
use std::rc::Rc;
use std::sync::Once;

use anyhow::Result;
use once_cell::sync::OnceCell;
use quickjs_wasm_rs::{Context as QuickjsContext, Value};
use send_wrapper::SendWrapper;

pub static CONTEXT: OnceCell<SendWrapper<Rc<QuickjsContext>>> = OnceCell::new();

static INIT: Once = Once::new();
static WEB_PLATFORM_APIS: &str = include_str!("../../../dist/web-platform-apis.js");

const SCRIPT_NAME: &str = "context.js";

#[derive(Debug)]
pub struct Context<'a> {
    pub context: &'a SendWrapper<Rc<QuickjsContext>>,
    pub global: Value,
}

impl Context<'_> {
    pub fn new() -> Self {
        init_context();

        let context = CONTEXT.get().unwrap();
        let global = context.global_object().unwrap();

        Self { context, global }
    }

    pub fn eval(&mut self, code: &str) -> Result<()> {
        let mut contents = String::new();

        contents.push_str(WEB_PLATFORM_APIS);
        contents.push_str(code);

        self.context.eval_global(SCRIPT_NAME, &contents)?;

        Ok(())
    }
}

pub fn init_context() -> () {
    INIT.call_once(|| {
        CONTEXT
            .set(SendWrapper::new(Rc::new(QuickjsContext::default())))
            .unwrap()
    });
}
