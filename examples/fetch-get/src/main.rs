use std::{collections::HashMap, str};

use anyhow::Result;
use js_wasm_workers_runtime::runtime;
use serde::Deserialize;
use serde_bytes::ByteBuf;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Response {
    pub body_used: bool,
    pub body: Option<ByteBuf>,
    pub headers: Option<HashMap<String, String>>,
    pub ok: bool,
    pub redirected: bool,
    pub status_text: String,
    pub status: usize,
    pub r#type: String,
    pub url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let handler: &str = include_str!("./handler.js");
    // let body = serde_json::json!({"hello": "world"}).to_string();
    // let request = r#"{
    //     "body": __BODY__,
    //     "headers": {
    //         "content-type": "application/json",
    //         "x-test": "test"
    //     },
    //     "method": "POST",
    //     "url": "https://test.test"
    // }"#
    // .to_string()
    // .replace("__BODY__", &format!("{:?}", body.as_bytes()));
    let request = r#"{
        "body": null,
        "headers": {
            "content-type": "application/json"
        },
        "method": "GET",
        "url": "https://test.test"
    }"#
    .to_string();

    let buffer = runtime(handler, &request).await?;
    let value = String::from_utf8(buffer)?;
    let response: Response = serde_json::from_str(&value)?;

    println!("response: {response:?}");

    let body = response.body.unwrap();

    println!("body: {:?}", String::from_utf8(body.into_vec())?);

    Ok(())
}
