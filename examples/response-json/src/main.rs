use anyhow::Result;
use js_wasm_workers_runtime::runtime;

#[tokio::main]
async fn main() -> Result<()> {
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

    let buffer = runtime(&handler, &request).await?;

    println!("response: {:?}", &String::from_utf8(buffer)?);

    Ok(())
}
