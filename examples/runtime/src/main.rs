use std::collections::HashMap;

use anyhow::Result;
use js_wasm_workers_runtime::runtime;
use serde::Deserialize;
use serde_bytes::ByteBuf;
use serde_json;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Response {
    pub status: usize,
    pub ok: bool,
    pub status_text: String,
    pub body: Option<ByteBuf>,
    pub body_used: bool,
    pub headers: Option<HashMap<String, String>>,
}

fn main() -> Result<()> {
    let handler: &str = include_str!("./handler.js");

    let body = "Hello World!".to_string();
    let request = r#"{
        "body": __BODY__,
        "headers": {
            "content-type": "application/json",
            "x-test": "test"
        },
        "method": "POST",
        "url": "https://test.test"
    }"#
    .to_string()
    .replace("__BODY__", &format!("{:?}", body.as_bytes()));

    let buffer = runtime(&handler, &request)?;
    let response: Response = serde_json::from_str(&String::from_utf8(buffer)?)?;
    let body = response.body.unwrap();

    println!("response: {:?}", String::from_utf8(body.into_vec())?);

    Ok(())
}
