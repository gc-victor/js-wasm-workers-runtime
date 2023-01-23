use anyhow::Result;
use js_wasm_workers_runtime::runtime;

fn main() -> Result<()> {
    let handler: &str = include_str!("./handler.js");

    let request = r#"{
        "body": null,
        "headers": {
            "content-type": "application/json"
        },
        "method": "GET",
        "url": "https://test.test"
    }"#
    .to_string();

    let buffer = runtime(&handler, &request)?;

    println!("returned: {:?}", String::from_utf8(buffer)?);

    Ok(())
}
