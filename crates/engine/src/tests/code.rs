#![allow(dead_code)]

static WEB_PLATFORM_APIS: &str = include_str!("../../dist/web-platform-apis.js");

const SCRIPT_NAME: &str = "context.js";

pub fn code(code: &str) -> String {
    let mut contents = String::new();

    contents.push_str(WEB_PLATFORM_APIS);
    contents.push_str(code);

    contents
}
