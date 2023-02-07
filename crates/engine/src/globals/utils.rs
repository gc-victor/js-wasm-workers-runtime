use std::{borrow::Cow, str};

use anyhow::{anyhow, Result};
use quickjs_wasm_rs::{Context, JSError, Value};
use url::Url;

pub(crate) fn set_global_utils(context: &Context) -> Result<()> {
    let global = context.global_object()?;

    global.set_property("___logger", context.wrap_callback(logger)?)?;
    global.set_property("___parseUrl", context.wrap_callback(parse_url)?)?;
    global.set_property(
        "___decodeUtf8BufferToString",
        context.wrap_callback(decode_utf8_buffer_to_js_string())?,
    )?;
    global.set_property(
        "___encodeStringToUtf8Buffer",
        context.wrap_callback(encode_js_string_to_utf8_buffer())?,
    )?;

    Ok(())
}

fn logger(context: &Context, _this: &Value, args: &[Value]) -> Result<Value> {
    let mut spaced = false;
    print!("___logger(");
    for arg in args {
        if spaced {
            print!(", ");
        } else {
            spaced = true;
        }
        print!("{}", arg.as_str()?);
    }
    println!(")");

    context.undefined_value()
}

fn parse_url(context: &Context, _this: &Value, args: &[Value]) -> Result<Value> {
    let url = args[0].as_str()?;
    let base = args[1].as_str()?;

    let options = Url::options();
    let parse_base_url = Url::parse(base)?;

    let list_url = options.base_url(Some(&parse_base_url)).parse(url)?;
    let obj = context.object_value()?;
    obj.set_property("href", context.value_from_str(list_url.as_str())?)?;
    obj.set_property(
        "origin",
        context.value_from_str(list_url.origin().unicode_serialization().as_str())?,
    )?;
    obj.set_property("protocol", context.value_from_str(list_url.scheme())?)?;
    obj.set_property(
        "setProtocol",
        context.wrap_callback(
            |ctx: &Context, _this: &Value, args: &[Value]| -> Result<Value> {
                let href = args[0].as_str()?;
                let protocol = args[1].as_str()?;
                let mut url = Url::parse(href)?;

                url.set_scheme(protocol).unwrap_or_else(|e| e);

                let obj = parse_url(
                    ctx,
                    _this,
                    &[
                        ctx.value_from_str(url.as_str())?,
                        ctx.value_from_str("about:blank")?,
                    ],
                );

                obj
            },
        )?,
    )?;
    obj.set_property(
        "host",
        context.value_from_str(list_url.host_str().unwrap_or(""))?,
    )?;
    obj.set_property(
        "setHost",
        context.wrap_callback(
            |ctx: &Context, _this: &Value, args: &[Value]| -> Result<Value> {
                let href = args[0].as_str()?;
                let host = args[1].as_str()?;
                let mut url = Url::parse(href)?;

                url.set_host(Some(host))?;

                let obj = parse_url(
                    ctx,
                    _this,
                    &[
                        ctx.value_from_str(url.as_str())?,
                        ctx.value_from_str("about:blank")?,
                    ],
                );

                obj
            },
        )?,
    )?;
    obj.set_property(
        "hostname",
        context.value_from_str(list_url.domain().unwrap_or(""))?,
    )?;

    let port = match list_url.port() {
        Some(p) => p.to_string(),
        None => String::new(),
    };

    obj.set_property("port", context.value_from_str(&port)?)?;
    obj.set_property(
        "setPort",
        context.wrap_callback(
            |ctx: &Context, _this: &Value, args: &[Value]| -> Result<Value> {
                let href = args[0].as_str()?;
                let port = args[1].as_str()?;

                let mut url = Url::parse(href)?;

                url.set_port(Some(port.parse()?)).unwrap_or_else(|e| e);

                let obj = parse_url(
                    ctx,
                    _this,
                    &[
                        ctx.value_from_str(url.as_str())?,
                        ctx.value_from_str("about:blank")?,
                    ],
                );

                obj
            },
        )?,
    )?;
    obj.set_property("pathname", context.value_from_str(list_url.path())?)?;
    obj.set_property(
        "setPathname",
        context.wrap_callback(
            |ctx: &Context, _this: &Value, args: &[Value]| -> Result<Value> {
                let href = args[0].as_str()?;
                let path = args[1].as_str()?;
                let mut url = Url::parse(href)?;

                url.set_path(path);

                let obj = parse_url(
                    ctx,
                    _this,
                    &[
                        ctx.value_from_str(url.as_str())?,
                        ctx.value_from_str("about:blank")?,
                    ],
                );

                obj
            },
        )?,
    )?;
    obj.set_property(
        "search",
        context.value_from_str(list_url.query().unwrap_or(""))?,
    )?;
    obj.set_property(
        "setSearch",
        context.wrap_callback(
            |ctx: &Context, _this: &Value, args: &[Value]| -> Result<Value> {
                let href = args[0].as_str()?;
                let query = args[1].as_str()?;
                let mut url = Url::parse(href)?;

                url.set_query(Some(query));

                let obj = parse_url(
                    ctx,
                    _this,
                    &[
                        ctx.value_from_str(url.as_str())?,
                        ctx.value_from_str("about:blank")?,
                    ],
                );

                obj
            },
        )?,
    )?;
    obj.set_property(
        "hash",
        context.value_from_str(list_url.fragment().unwrap_or(""))?,
    )?;
    obj.set_property(
        "setHash",
        context.wrap_callback(
            |ctx: &Context, _this: &Value, args: &[Value]| -> Result<Value> {
                let href = args[0].as_str()?;
                let hash = args[1].as_str()?;
                let mut url = Url::parse(href)?;

                url.set_fragment(Some(hash));

                let obj = parse_url(
                    ctx,
                    _this,
                    &[
                        ctx.value_from_str(url.as_str())?,
                        ctx.value_from_str("about:blank")?,
                    ],
                );

                obj
            },
        )?,
    )?;
    obj.set_property("username", context.value_from_str(list_url.username())?)?;
    obj.set_property(
        "setUsername",
        context.wrap_callback(
            |ctx: &Context, _this: &Value, args: &[Value]| -> Result<Value> {
                let href = args[0].as_str()?;
                let username = args[1].as_str()?;
                let mut url = Url::parse(href)?;

                url.set_username(username).unwrap_or_else(|e| e);

                let obj = parse_url(
                    ctx,
                    _this,
                    &[
                        ctx.value_from_str(url.as_str())?,
                        ctx.value_from_str("about:blank")?,
                    ],
                );

                obj
            },
        )?,
    )?;
    obj.set_property(
        "password",
        context.value_from_str(list_url.password().unwrap_or(""))?,
    )?;
    obj.set_property(
        "setPassword",
        context.wrap_callback(
            |ctx: &Context, _this: &Value, args: &[Value]| -> Result<Value> {
                let href = args[0].as_str()?;
                let password = args[1].as_str()?;
                let mut url = Url::parse(href)?;

                url.set_password(Some(password)).unwrap_or_else(|e| e);

                let obj = parse_url(
                    ctx,
                    _this,
                    &[
                        ctx.value_from_str(url.as_str())?,
                        ctx.value_from_str("about:blank")?,
                    ],
                );

                obj
            },
        )?,
    )?;

    Ok(obj)
}

fn decode_utf8_buffer_to_js_string(
) -> impl FnMut(&Context, &Value, &[Value]) -> anyhow::Result<Value> {
    move |ctx: &Context, _this: &Value, args: &[Value]| {
        if args.len() != 5 {
            return Err(anyhow!("Expecting 5 arguments, received {}", args.len()));
        }

        let buffer = args[0].as_bytes()?;
        let byte_offset = {
            let byte_offset_val = &args[1];
            if !byte_offset_val.is_repr_as_i32() {
                return Err(anyhow!("byte_offset must be an u32"));
            }
            byte_offset_val.as_u32_unchecked()
        }
        .try_into()?;
        let byte_length: usize = {
            let byte_length_val = &args[2];
            if !byte_length_val.is_repr_as_i32() {
                return Err(anyhow!("byte_length must be an u32"));
            }
            byte_length_val.as_u32_unchecked()
        }
        .try_into()?;
        let fatal = args[3].as_bool()?;
        let ignore_bom = args[4].as_bool()?;

        let mut view = buffer
            .get(byte_offset..(byte_offset + byte_length))
            .ok_or_else(|| {
                anyhow!("Provided offset and length is not valid for provided buffer")
            })?;

        if !ignore_bom {
            view = match view {
                // [0xEF, 0xBB, 0xBF] is the UTF-8 BOM which we want to strip
                [0xEF, 0xBB, 0xBF, rest @ ..] => rest,
                _ => view,
            };
        }

        let str =
            if fatal {
                Cow::from(str::from_utf8(view).map_err(|_| {
                    JSError::Type("The encoded data was not valid utf-8".to_string())
                })?)
            } else {
                String::from_utf8_lossy(view)
            };
        ctx.value_from_str(&str)
    }
}

fn encode_js_string_to_utf8_buffer(
) -> impl FnMut(&Context, &Value, &[Value]) -> anyhow::Result<Value> {
    move |ctx: &Context, _this: &Value, args: &[Value]| {
        if args.len() != 1 {
            return Err(anyhow!("Expecting 1 argument, got {}", args.len()));
        }

        let js_string = args[0].as_str_lossy();
        ctx.array_buffer_value(js_string.as_bytes())
    }
}
