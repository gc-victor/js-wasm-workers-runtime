use std::{
    slice,
    str::{self, FromStr},
};

use reqwest;
use serde_bytes::ByteBuf;
use serde_json;
use wasmtime::*;
use wasmtime_wasi::WasiCtx;

use super::http::{Request, RequestError, RequestErrorKind, Response};

pub(crate) fn import_send_request(mut caller: Caller<'_, WasiCtx>, ptr: i32) -> i32 {
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

    let request = read_string(&mut caller, &memory, ptr);
    let request = serde_json::from_str::<Request>(request).unwrap();
    let client = reqwest::blocking::Client::new();

    let response = client.execute(reqwest::blocking::Request::new(
        reqwest::Method::from_str(&request.method).unwrap(),
        reqwest::Url::from_str(&request.url).unwrap(),
    ));

    let response = parse_response(response).unwrap();
    let json = serde_json::to_string(&response).unwrap();

    write_string(&mut caller, &memory, json.as_str())
}

fn read_string<'c, 'm>(
    caller: &'c mut Caller<'_, WasiCtx>,
    memory: &'m Memory,
    ptr: i32,
) -> &'m str {
    let len = stack_pop(caller) as usize;

    unsafe {
        let ptr = memory.data_ptr(&caller).offset(ptr as isize);
        let bytes = slice::from_raw_parts(ptr, len);
        std::str::from_utf8(bytes).unwrap()
    }
}

fn write_string<'c, 'm>(
    caller: &'c mut Caller<'_, WasiCtx>,
    memory: &'m Memory,
    value: &str,
) -> i32 {
    let alloc_func = caller.get_export("alloc").unwrap().into_func().unwrap();

    let ptr = alloc_func
        .typed::<i32, i32>(caller.as_context())
        .unwrap()
        .call(caller.as_context_mut(), value.len() as i32)
        .unwrap();

    stack_push(caller, value.len() as i32);

    memory
        .write(caller.as_context_mut(), ptr as usize, value.as_bytes())
        .unwrap();

    ptr
}

fn stack_push<'c, 'm>(caller: &'c mut Caller<'_, WasiCtx>, value: i32) {
    let push_fn = caller
        .get_export("stack_push")
        .unwrap()
        .into_func()
        .unwrap();

    push_fn
        .typed::<i32, ()>(&caller)
        .unwrap()
        .call(caller, value)
        .unwrap();
}

fn stack_pop<'c, 'm>(caller: &'c mut Caller<'_, WasiCtx>) -> i32 {
    let pop_fn = caller.get_export("stack_pop").unwrap().into_func().unwrap();

    let value = pop_fn
        .typed::<(), i32>(&caller)
        .unwrap()
        .call(caller, ())
        .unwrap();

    value
}

fn parse_response(
    response: reqwest::Result<reqwest::blocking::Response>,
) -> Result<Response, RequestError> {
    let response = response.map_err(|_| RequestError {
        kind: RequestErrorKind::Serial,
        url: Some("".to_string()),
        message: String::from("request serialization failed"),
    })?;
    let header_map = response
        .headers()
        .into_iter()
        .map(|(n, v)| (n.to_string(), v.to_str().unwrap_or_default().to_string()))
        .collect::<std::collections::HashMap<_, _>>();

    let headers = serde_json::to_string(&header_map).map_err(|_| RequestError {
        kind: RequestErrorKind::Serial,
        url: Some("".to_string()),
        message: String::from("request serialization failed"),
    })?;

    Ok(Response {
        status: response.status().as_u16() as usize,
        headers: Some(headers),
        body: Some(ByteBuf::from(response.bytes().unwrap().to_vec())),
    })
}
