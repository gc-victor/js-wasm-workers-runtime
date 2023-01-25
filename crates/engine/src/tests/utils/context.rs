use crate::globals::globals;
use anyhow::Result;
use quickjs_wasm_rs::Context as QuickjsContext;

use super::shared_stream::SharedStream;

static WEB_PLATFORM_APIS: &str = include_str!("../../../dist/web-platform-apis.js");

const SCRIPT_NAME: &str = "context.js";

pub struct Context {
    pub code: &'static str,
    pub stream: SharedStream,
    pub context: QuickjsContext,
}

impl Context {
    pub fn new() -> Self {
        let context = QuickjsContext::default();
        let stream = SharedStream::default();

        Self {
            code: "",
            context,
            stream,
        }
    }

    pub fn globals(&mut self) {
        globals(&self.context, self.stream.clone(), self.stream.clone()).unwrap();
    }

    pub fn eval(&mut self, code: &str) -> Result<()> {
        let mut contents = String::new();

        contents.push_str(WEB_PLATFORM_APIS);
        contents.push_str(code);

        let _ = self.context.eval_global(SCRIPT_NAME, &contents)?;

        Ok(())
    }
}
