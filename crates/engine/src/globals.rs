use anyhow::{anyhow, Result};
use quickjs_wasm_rs::{Context, JSError, Value};
use std::{borrow::Cow, io::Write, str};

pub(crate) fn globals<T1, T2>(context: &Context, log_stream: T1, error_stream: T2) -> Result<()>
where
    T1: Write + 'static,
    T2: Write + 'static,
{
    let global = context.global_object()?;

    let console_object = context.object_value()?;
    console_object.set_property("log", context.wrap_callback(console_log_to(log_stream))?)?;
    console_object.set_property(
        "error",
        context.wrap_callback(console_log_to(error_stream))?,
    )?;

    global.set_property("console", console_object)?;
    global.set_property("___logger", context.wrap_callback(logger)?)?;
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

fn console_log_to<T>(
    mut stream: T,
) -> impl FnMut(&Context, &Value, &[Value]) -> anyhow::Result<Value>
where
    T: Write + 'static,
{
    move |ctx: &Context, _this: &Value, args: &[Value]| {
        for (i, arg) in args.iter().enumerate() {
            if i != 0 {
                write!(stream, " ")?;
            }

            stream.write_all(arg.as_str()?.as_bytes())?;
        }

        writeln!(stream)?;
        ctx.undefined_value()
    }
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
