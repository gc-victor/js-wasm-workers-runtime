use anyhow::Result;
use quickjs_wasm_rs::{Context, Value};
use std::io::Write;

pub(crate) fn set_global_console<T1, T2>(
    context: &Context,
    log_stream: T1,
    error_stream: T2,
) -> Result<()>
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

    Ok(())
}

fn console_log_to<T>(mut stream: T) -> impl FnMut(&Context, &Value, &[Value]) -> Result<Value>
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
