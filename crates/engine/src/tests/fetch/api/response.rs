// @see: https://developer.mozilla.org/en-US/docs/Web/API/Response
// @see: https://fetch.spec.whatwg.org/#response-class
// @see: https://github.com/web-platform-tests/wpt/tree/master/fetch/api/response
#[cfg(test)]
mod tests {
    use anyhow::{Ok, Result};
    use serde_json::json;

    use crate::tests::test_utils::context::Context;

    #[test]
    fn test_response_constructor() -> Result<()> {
        let mut ctx = Context::new();

        // Without body and options
        ctx.eval(
            r#"
            var response = new Response();
            var response_default = JSON.stringify(response);
        "#,
        )?;

        assert_eq!(
            json!({"bodyUsed":false,"status":200,"statusText":"OK","ok":true,"headers":{}})
                .to_string(),
            ctx.global.get_property("response_default")?.as_str()?
        );

        // With body as string
        ctx.eval(
            r#"
            var textDecoder = new TextDecoder();
            var response = new Response("Hello World!");
            var response_default = JSON.stringify(response);
            var response_body = textDecoder.decode(response.body);
            "#,
        )?;

        assert_eq!(
            json!({"body":{},"bodyUsed":false,"status":200,"statusText":"OK","ok":true,"headers":{"content-type": "text/plain;charset=UTF-8"}})
                .to_string(),
            ctx.global.get_property("response_default")?.as_str()?
        );

        assert_eq!(
            "Hello World!",
            ctx.global.get_property("response_body")?.as_str()?
        );

        // With body as blob
        ctx.eval(
            r#"
            var blob = new Blob();
            var options = { status: 200, statusText: 'SuperSmashingGreat!' };
            var response = new Response(blob, options);
            var response_default = JSON.stringify(response);
            "#,
        )?;

        assert_eq!(
            json!({"body":{"size": 0, "type": ""},"bodyUsed":false,"status":200,"statusText":"SuperSmashingGreat!","ok":true,"headers":{"content-type": "text/plain;charset=UTF-8"}})
                .to_string(),
            ctx.global.get_property("response_default")?.as_str()?
        );

        Ok(())
    }
}
