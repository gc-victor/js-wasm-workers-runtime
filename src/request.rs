use std::{collections::HashMap, str};

use anyhow::Result;
use quickjs_wasm_rs::{Context, Serializer, Value};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HttpRequest {
    pub body: Option<ByteBuf>,
    pub cache: Option<String>,
    pub credentials: Option<String>,
    pub headers: HashMap<String, String>,
    pub integrity: Option<String>,
    pub method: String,
    pub mode: Option<String>,
    pub redirect: Option<String>,
    pub referrer: Option<String>,
    pub referrer_policy: Option<String>,
    pub url: String,
}

pub fn request(args: Vec<String>, context: &Context) -> Result<Value> {
    let mut serializer = Serializer::from_context(context)?;
    let request: HttpRequest = serde_json::from_str(&args[0])?;
    request.serialize(&mut serializer)?;

    let body = request.body;
    let str_body = match &body {
        Some(body) => str::from_utf8(body)?,
        _ => "",
    };

    let mut headers: Vec<String> = vec![];
    for (key, value) in request.headers {
        headers.push(format!(r#""{}": "{}""#, key, value));
    }

    let content = format!(
        "globalThis.___request = new Request('{url}', {{
                body: '{body}',
                cache: '{cache}',
                credentials: '{credentials}',
                headers: {{ {headers} }},
                integrity: '{integrity}',
                method: '{method}',
                mode: '{mode}',
                redirect: '{redirect}',
                referrer: '{referrer}',
                referrerPolicy: '{referrerPolicy}',
                url: '{url}',
        }});",
        body = str_body,
        cache = request.cache.unwrap_or_default(),
        credentials = request.credentials.unwrap_or_default(),
        headers = headers.join(", "),
        integrity = request.integrity.unwrap_or_default(),
        method = request.method,
        mode = request.mode.unwrap_or_default(),
        redirect = request.redirect.unwrap_or_default(),
        referrer = request.referrer.unwrap_or_default(),
        referrerPolicy = request.referrer_policy.unwrap_or_default(),
        url = request.url,
    );

    let _ = context.eval_global("request", &content)?;
    let global = context.global_object()?;

    global.get_property("___request")
}
