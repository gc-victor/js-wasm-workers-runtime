#[cfg(test)]
mod tests {
    use anyhow::Result;
    use quickjs_wasm_rs::Context;
    use std::cell::RefCell;
    use std::io;
    use std::rc::Rc;

    use crate::globals::console::set_global_console;

    #[test]
    fn test_console_log() -> Result<()> {
        let mut stream = SharedStream::default();

        let ctx = Context::default();
        set_global_console(&ctx, stream.clone(), stream.clone())?;

        ctx.eval_global("main", "console.log(\"hello world\");")?;
        assert_eq!(b"hello world\n", stream.0.borrow().as_slice());

        stream.clear();

        ctx.eval_global("main", "console.log(\"bonjour\", \"le\", \"monde\")")?;
        assert_eq!(b"bonjour le monde\n", stream.0.borrow().as_slice());

        stream.clear();

        ctx.eval_global(
            "main",
            "console.log(2.3, true, { foo: 'bar' }, null, undefined)",
        )?;
        assert_eq!(
            b"2.3 true [object Object] null undefined\n",
            stream.0.borrow().as_slice()
        );
        Ok(())
    }

    #[test]
    fn test_console_error() -> Result<()> {
        let mut stream = SharedStream::default();

        let ctx = Context::default();
        set_global_console(&ctx, stream.clone(), stream.clone())?;

        ctx.eval_global("main", "console.error(\"hello world\");")?;
        assert_eq!(b"hello world\n", stream.0.borrow().as_slice());

        stream.clear();

        ctx.eval_global("main", "console.error(\"bonjour\", \"le\", \"monde\")")?;
        assert_eq!(b"bonjour le monde\n", stream.0.borrow().as_slice());

        stream.clear();

        ctx.eval_global(
            "main",
            "console.error(2.3, true, { foo: 'bar' }, null, undefined)",
        )?;
        assert_eq!(
            b"2.3 true [object Object] null undefined\n",
            stream.0.borrow().as_slice()
        );
        Ok(())
    }

    #[derive(Default, Clone)]
    struct SharedStream(Rc<RefCell<Vec<u8>>>);

    impl SharedStream {
        fn clear(&mut self) {
            (*self.0).borrow_mut().clear();
        }
    }

    impl io::Write for SharedStream {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            (*self.0).borrow_mut().write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            (*self.0).borrow_mut().flush()
        }
    }
}
