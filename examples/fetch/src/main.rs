use std::str;

use anyhow::Result;
use jwwr_run::run;

fn main() -> Result<()> {
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
            "content-type": "application/json",
            "x-test": "test"
        },
        "method": "GET",
        "url": "https://my-json-server.typicode.com/typicode/demo/posts"
    }"#
    .to_string();

    let buffer = run(&handler, &request)?;

    println!("returned: {:?}", String::from_utf8(buffer)?);

    Ok(())
}
