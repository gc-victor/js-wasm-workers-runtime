use anyhow::{anyhow, Result};
use once_cell::sync::{Lazy, OnceCell};
use quickjs_wasm_rs::{json, Context, Exception, Value};
use send_wrapper::SendWrapper;
use std::{
    env,
    io::{stderr, stdin, stdout, Read, Write},
    ops::Deref,
    sync::Mutex,
};

mod globals;

use globals::globals;

mod request;

static POLYFILL: &str = include_str!("../dist/web-platform-apis.js");

static ON_RESOLVE: OnceCell<SendWrapper<Value>> = OnceCell::new();
static ON_REJECT: OnceCell<SendWrapper<Value>> = OnceCell::new();
static RESPONSE: Lazy<Mutex<Option<Result<SendWrapper<Value>>>>> = Lazy::new(|| Mutex::new(None));

fn main() -> Result<()> {
    let context = Context::default();

    let mut contents = String::new();
    let mut source = String::new();

    contents.push_str(&POLYFILL);

    stdin().read_to_string(&mut source)?;

    contents.push_str(&source);

    let _ = context.eval_global("handler.js", &contents)?;

    let global = context.global_object()?;

    globals(&context, stderr(), stderr())?;

    let on_resolve_wrap = context.wrap_callback(on_resolve)?;
    let on_reject_wrap = context.wrap_callback(on_reject)?;

    ON_RESOLVE.set(SendWrapper::new(on_resolve_wrap)).unwrap();
    ON_REJECT.set(SendWrapper::new(on_reject_wrap)).unwrap();

    let on_resolve = ON_RESOLVE.get().unwrap();
    let on_reject = ON_REJECT.get().unwrap();

    let handler = global.get_property("handleRequest")?;

    if !handler.is_function() {
        panic!(r#"expected function named "handleRequest""#);
    }

    let args = env::args().collect::<Vec<String>>();

    // TODO: set env as globals

    // TODO: set request as input value
    let input_bytes = args[0].as_bytes();
    let request = json::transcode_input(&context, &input_bytes)?;
    // @see: https://github.com/fermyon/spin-js-sdk/blob/569b76d32c06d44d9b6c928e526c82594782c4cb/crates/spin-js-engine/src/lib.rs#L552
    let output = handler.call(&global, &[request])?;
    let then = output.get_property("then")?;
    let response: Vec<u8>;

    if then.is_function() {
        then.call(
            &output,
            &[on_resolve.deref().clone(), on_reject.deref().clone()],
        )?;

        context.execute_pending()?;

        response = json::transcode_output(RESPONSE.lock().unwrap().take().unwrap()?.take())?;
    } else {
        response = json::transcode_output(output)?;
    }

    stdout()
        .write(&response)
        .expect("Error when returning the response");

    stdout().flush().expect("Error when returning the response");

    Ok(())
}

fn on_resolve(context: &Context, _this: &Value, args: &[Value]) -> Result<Value> {
    match args {
        [response] => {
            *RESPONSE.lock().unwrap() = Some(Ok(SendWrapper::new(response.clone())));

            context.undefined_value()
        }

        _ => Err(anyhow!("expected 1 argument, got {}", args.len())),
    }
}

fn on_reject(context: &Context, _this: &Value, args: &[Value]) -> Result<Value> {
    match args {
        [error] => {
            *RESPONSE.lock().unwrap() = Some(Err(Exception::from(error.clone())?.into_error()));

            context.undefined_value()
        }

        _ => Err(anyhow!("expected 1 argument, got {}", args.len())),
    }
}
