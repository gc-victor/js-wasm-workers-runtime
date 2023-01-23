use anyhow::{anyhow, Result};
use quickjs_wasm_rs::{Context, Value};
use serde_bytes::ByteBuf;

use super::http::*;
use super::mem::{FromMem, ToMem};

extern "C" {
    fn import_send_request(ptr: *mut u8) -> *mut u8;
}

pub fn fetch(context: &Context) -> Result<()> {
    let global = context.global_object()?;

    global.set_property("___fetcher", context.wrap_callback(fetcher)?)?;

    Ok(())
}

fn fetcher(context: &Context, _this: &Value, args: &[Value]) -> Result<Value> {
    match args {
        [request] => {
            let method = request.get_property("method")?;
            let method = method.as_str()?.to_string();
            let url = request.get_property("url")?;
            let url = url.as_str()?.to_string();
            let body = request.get_property("body")?;
            let body = body.as_str()?.to_string();

            let response = send_request(Request {
                method,
                url,
                headers: None,
                body: Some(ByteBuf::from(body).to_owned()),
            })?;

            context.value_from_str(&response)
        }
        _ => Err(anyhow!("expected 1 argument, got {}", args.len())),
    }
}

fn send_request(request: Request) -> Result<String> {
    let req = serde_json::to_string(&request)?;

    let resp = unsafe {
        let ptr = import_send_request(req.to_mem());
        String::from_mem(ptr)
    };

    Ok(resp)
}
