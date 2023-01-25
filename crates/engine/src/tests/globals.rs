#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::tests::utils::context::Context;

    #[test]
    fn test_console_log() -> Result<()> {
        let mut ctx = Context::new();

        ctx.globals();

        ctx.eval("console.log(\"hello world\");")?;
        assert_eq!(b"hello world\n", ctx.stream.0.borrow().as_slice());
        ctx.stream.clear();

        ctx.eval("console.log(\"bonjour\", \"le\", \"monde\")")?;
        assert_eq!(b"bonjour le monde\n", ctx.stream.0.borrow().as_slice());
        ctx.stream.clear();

        ctx.eval("console.log(2.3, true, { foo: 'bar' }, null, undefined)")?;
        assert_eq!(
            b"2.3 true [object Object] null undefined\n",
            ctx.stream.0.borrow().as_slice()
        );
        ctx.stream.clear();

        Ok(())
    }
}
