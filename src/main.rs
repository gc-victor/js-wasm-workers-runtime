use quickjs_wasm_rs::{json, Context};
use std::io::{self, stdin, stdout, Read, Write};

mod globals;

use globals::globals;

static POLYFILL: &str = include_str!("../dist/web-platform-apis.js");

fn main() {
    let context = Context::default();

    let mut contents = String::new();
    let mut source = String::new();

    contents.push_str(&POLYFILL);

    stdin().read_to_string(&mut source).unwrap();
    contents.push_str(&source);

    let _ = context.eval_global("handler.js", &contents).unwrap();

    let global = context.global_object().unwrap();

    globals(&context, io::stderr(), io::stderr()).unwrap();

    let handler = global.get_property("handleRequest").unwrap();
    let input_bytes = r#"{"data": "Hola"}"#.as_bytes();
    let input_value = json::transcode_input(&context, &input_bytes).unwrap();

    let output_value = match handler.call(&global, &[input_value]) {
        Ok(result) => result,
        Err(err) => panic!("{:?}", err),
    };

    let output = json::transcode_output(output_value).unwrap();

    stdout()
        .write(&output)
        .expect("Error when returning the response");

    stdout().flush().expect("Error when returning the response");
}
