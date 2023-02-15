use std::{
    env,
    io::{stderr, stdin, stdout, Read, Write},
    ops::Deref,
    str,
    sync::Mutex,
};

use anyhow::{anyhow, Result};
use once_cell::sync::{Lazy, OnceCell};
use quickjs_wasm_rs::{json, Context, Exception, Value};
use send_wrapper::SendWrapper;

mod fetch;
mod globals;
mod request;
mod tests;

use fetch::fetch::fetch;
use globals::{console::set_global_console, utils::set_global_utils};

static WEB_PLATFORM_APIS: &str = include_str!("../dist/web-platform-apis.js");

static ON_RESOLVE: OnceCell<SendWrapper<Value>> = OnceCell::new();
static ON_REJECT: OnceCell<SendWrapper<Value>> = OnceCell::new();
static RESPONSE: Lazy<Mutex<Option<Result<SendWrapper<Value>>>>> = Lazy::new(|| Mutex::new(None));

fn main() -> Result<()> {
    let context = Context::default();

    fetch(&context)?;
    set_global_utils(&context)?;
    set_global_console(&context, stderr(), stderr())?;

    let mut contents = String::new();
    let mut source = String::new();

    contents.push_str(WEB_PLATFORM_APIS);

    stdin().read_to_string(&mut source)?;

    contents.push_str(&source);

    let _ = context.eval_global("handler.js", &contents)?;

    let global = context.global_object()?;

    let env = context.object_value()?;
    for (key, value) in env::vars() {
        env.set_property(key, context.value_from_str(&value)?)?;
    }

    let process = context.object_value()?;
    process.set_property("env", env)?;

    global.set_property("process", process)?;

    let on_resolve_wrap = context.wrap_callback(on_resolve)?;
    let on_reject_wrap = context.wrap_callback(on_reject)?;

    ON_RESOLVE.set(SendWrapper::new(on_resolve_wrap)).unwrap();
    ON_REJECT.set(SendWrapper::new(on_reject_wrap)).unwrap();

    let on_resolve = ON_RESOLVE.get().unwrap();
    let on_reject = ON_REJECT.get().unwrap();

    let handler_request = global.get_property("handleRequest")?;

    if !handler_request.is_function() {
        panic!(r#"Expected "handleRequest" function"#);
    }

    let handler = global.get_property("___handleResponse")?;

    let args = env::args().collect::<Vec<String>>();

    // @see: https://github.com/fermyon/spin-js-sdk/blob/569b76d32c06d44d9b6c928e526c82594782c4cb/crates/spin-js-engine/src/lib.rs#L552
    let output = handler.call(&global, &[request::request(args, &context)?])?;
    let then = output.get_property("then")?;
    let response = if then.is_function() {
        then.call(
            &output,
            &[on_resolve.deref().clone(), on_reject.deref().clone()],
        )?;

        context.execute_pending()?;

        json::transcode_output(RESPONSE.lock().unwrap().take().unwrap()?.take())?
    } else {
        json::transcode_output(output)?
    };

    stdout()
        .write_all(&response)
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
