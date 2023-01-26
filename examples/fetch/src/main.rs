use std::str;

use anyhow::Result;
use js_wasm_workers_runtime::runtime;

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

    let buffer = runtime(&handler, &request).await?;

    println!("returned: {:?}", String::from_utf8(buffer)?);

    Ok(())
}
