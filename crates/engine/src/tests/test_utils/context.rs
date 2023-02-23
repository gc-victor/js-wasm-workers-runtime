#![allow(dead_code)]
#![allow(unused_imports)]
use std::{
    ops::Deref,
    rc::Rc,
    str,
    sync::{Mutex, Once},
};

use anyhow::{anyhow, Result};
use once_cell::sync::{Lazy, OnceCell};
use quickjs_wasm_rs::{json, Context as QuickjsContext, Exception, Value};
use send_wrapper::SendWrapper;

use crate::globals::utils::set_global_utils;

pub static CONTEXT: OnceCell<SendWrapper<Rc<QuickjsContext>>> = OnceCell::new();

static ON_RESOLVE: OnceCell<SendWrapper<Value>> = OnceCell::new();
static ON_REJECT: OnceCell<SendWrapper<Value>> = OnceCell::new();
static RESPONSE: Lazy<Mutex<Option<Result<SendWrapper<Value>>>>> = Lazy::new(|| Mutex::new(None));

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

        set_global_utils(self.context).unwrap();

        self.context.eval_global(SCRIPT_NAME, &contents)?;

        Ok(())
    }

    pub fn get_handler_value(&mut self) -> Result<String> {
        let on_resolve = ON_RESOLVE.get().unwrap();
        let on_reject = ON_REJECT.get().unwrap();

        let handler = self.global.get_property("handler")?;
        let output = handler.call(&self.global, &[])?;
        let then = output.get_property("then")?;
        let response = if then.is_function() {
            then.call(
                &output,
                &[on_resolve.deref().clone(), on_reject.deref().clone()],
            )?;

            self.context.execute_pending()?;

            json::transcode_output(RESPONSE.lock().unwrap().take().unwrap()?.take())?
        } else {
            json::transcode_output(output)?
        };

        Ok(String::from_utf8(response)?)
    }
}

pub fn init_context() {
    INIT.call_once(|| {
        let context = QuickjsContext::default();
        let on_resolve_wrap = context.wrap_callback(on_resolve).unwrap();
        let on_reject_wrap = context.wrap_callback(on_reject).unwrap();

        ON_RESOLVE.set(SendWrapper::new(on_resolve_wrap)).unwrap();
        ON_REJECT.set(SendWrapper::new(on_reject_wrap)).unwrap();

        CONTEXT
            .set(SendWrapper::new(Rc::new(QuickjsContext::default())))
            .unwrap()
    });
}

fn on_resolve(context: &QuickjsContext, _this: &Value, args: &[Value]) -> Result<Value> {
    match args {
        [response] => {
            *RESPONSE.lock().unwrap() = Some(Ok(SendWrapper::new(response.clone())));

            context.undefined_value()
        }
        _ => Err(anyhow!("expected 1 argument, got {}", args.len())),
    }
}

fn on_reject(context: &QuickjsContext, _this: &Value, args: &[Value]) -> Result<Value> {
    match args {
        [error] => {
            *RESPONSE.lock().unwrap() = Some(Err(Exception::from(error.clone())?.into_error()));

            context.undefined_value()
        }
        _ => Err(anyhow!("expected 1 argument, got {}", args.len())),
    }
}
