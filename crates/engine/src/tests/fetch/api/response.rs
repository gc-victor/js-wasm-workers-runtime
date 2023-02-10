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
            var response_status = response.status;
            var response_statusText = response.statusText;
            var response_headers = response.headers.get('content-type');
        "#,
        )?;

        assert_eq!(
            200.to_string(),
            ctx.global.get_property("response_status")?.as_str()?
        );

        assert_eq!(
            "OK".to_string(),
            ctx.global.get_property("response_statusText")?.as_str()?
        );

        assert_eq!(
            "null",
            ctx.global.get_property("response_headers")?.as_str()?
        );

        // With body as ArrayBuffer
        ctx.eval(
            r#"
            var response = new Response(new ArrayBuffer(8));
            var response_status = response.status;
            var response_statusText = response.statusText;
            var response_content_type = response.headers.get('content-type');
            "#,
        )?;

        assert_eq!(
            200.to_string(),
            ctx.global.get_property("response_status")?.as_str()?
        );

        assert_eq!(
            "OK".to_string(),
            ctx.global.get_property("response_statusText")?.as_str()?
        );

        assert_eq!(
            "null",
            ctx.global.get_property("response_content_type")?.as_str()?
        );

        // With body as TypedArray
        ctx.eval(
            r#"
            var response = new Response(new Int8Array(8));
            var response_status = response.status;
            var response_statusText = response.statusText;
            var response_content_type = response.headers.get('content-type');
            "#,
        )?;

        assert_eq!(
            200.to_string(),
            ctx.global.get_property("response_status")?.as_str()?
        );

        assert_eq!(
            "OK".to_string(),
            ctx.global.get_property("response_statusText")?.as_str()?
        );

        assert_eq!(
            "null",
            ctx.global.get_property("response_content_type")?.as_str()?
        );

        // With body as Blob
        ctx.eval(
            r#"
            var blob = new Blob([], { type: "blob" });
            var response = new Response(blob);
            var response_status = response.status;
            var response_statusText = response.statusText;
            var response_content_type = response.headers.get('content-type');
            "#,
        )?;

        assert_eq!(
            200.to_string(),
            ctx.global.get_property("response_status")?.as_str()?
        );

        assert_eq!(
            "OK".to_string(),
            ctx.global.get_property("response_statusText")?.as_str()?
        );

        assert_eq!(
            "blob",
            ctx.global.get_property("response_content_type")?.as_str()?
        );

        // With body as String
        ctx.eval(
            r#"
            var response = new Response("Hello World!");
            var response_status = response.status;
            var response_statusText = response.statusText;
            var response_content_type = response.headers.get('content-type');
            "#,
        )?;

        assert_eq!(
            200.to_string(),
            ctx.global.get_property("response_status")?.as_str()?
        );

        assert_eq!(
            "OK".to_string(),
            ctx.global.get_property("response_statusText")?.as_str()?
        );

        assert_eq!(
            "text/plain;charset=UTF-8",
            ctx.global.get_property("response_content_type")?.as_str()?
        );

        // With body as URLSearchParams
        ctx.eval(
            r#"
            var urlParams = new URLSearchParams("a=1&b=2");
            var response = new Response(urlParams);
            var response_status = response.status;
            var response_statusText = response.statusText;
            var response_content_type = response.headers.get('content-type');
            "#,
        )?;

        assert_eq!(
            200.to_string(),
            ctx.global.get_property("response_status")?.as_str()?
        );

        assert_eq!(
            "OK".to_string(),
            ctx.global.get_property("response_statusText")?.as_str()?
        );

        assert_eq!(
            "application/x-www-form-urlencoded;charset=UTF-8",
            ctx.global.get_property("response_content_type")?.as_str()?
        );

        // TODO: Response body using FormData
        // TODO: Response body using ReadableStream

        // With body as Undefined
        ctx.eval(
            r#"
            var response = new Response(undefined);
            var response_status = response.status;
            var response_statusText = response.statusText;
            var response_body = response.body;
            var response_content_type = response.headers.get('content-type');
            "#,
        )?;

        assert_eq!(
            200.to_string(),
            ctx.global.get_property("response_status")?.as_str()?
        );

        assert_eq!(
            "OK".to_string(),
            ctx.global.get_property("response_statusText")?.as_str()?
        );

        assert_eq!("null", ctx.global.get_property("response_body")?.as_str()?);

        assert_eq!(
            "null",
            ctx.global.get_property("response_content_type")?.as_str()?
        );

        // With body as Null
        ctx.eval(
            r#"
            var response = new Response(null);
            var response_status = response.status;
            var response_statusText = response.statusText;
            var response_body = response.body;
            var response_content_type = response.headers.get('content-type');
            "#,
        )?;

        assert_eq!(
            200.to_string(),
            ctx.global.get_property("response_status")?.as_str()?
        );

        assert_eq!(
            "OK".to_string(),
            ctx.global.get_property("response_statusText")?.as_str()?
        );

        assert_eq!("null", ctx.global.get_property("response_body")?.as_str()?);

        assert_eq!(
            "null",
            ctx.global.get_property("response_content_type")?.as_str()?
        );

        Ok(())
    }

    #[test]
    fn test_response_body_readable_stream() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            async function handler() {
                const textDecoder = new TextDecoder();

                const response = new Response("Hello World!");

                const response_body_readable = response.body;
                const response_body_readable_read = await response_body_readable.getReader().read();
                const response_body_readable_read_value = textDecoder.decode(response_body_readable_read.value);

                return response_body_readable_read_value;
            }
            "#,
        )?;

        assert_eq!(r#""Hello World!""#.to_string(), ctx.get_handler_value()?);

        Ok(())
    }

    #[test]
    fn test_response_error() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
                async function handler() {
                    var response = Response.error();

                    var response_body = response.body;
                    var response_content_type = response.headers.get('content-type');
                    var response_ok = response.ok;
                    var response_status = response.status;
                    var response_statusText = response.statusText;
                    var response_type = response.type;

                    return {
                        response_body: String(response_body),
                        response_content_type: String(response_content_type),
                        response_ok,
                        response_status,
                        response_statusText,
                        response_type,
                    };
                }
        "#,
        )?;

        assert_eq!(
            json!({
                "response_body": "null",
                "response_content_type": "null",
                "response_ok": false,
                "response_status": 0,
                "response_statusText": "",
                "response_type":"error",
            })
            .to_string(),
            ctx.get_handler_value()?
        );

        Ok(())
    }

    #[test]
    fn test_response_redirect() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
                async function handler() {
                    var response = Response.redirect("https://example.com");
                    var response_status = response.status;
                    var response_statusText = response.statusText;
                    var response_content_type = response.headers.get('content-type');
                    var response_location = response.headers.get('location');

                    return {
                        response_content_type,
                        response_location,
                        response_status,
                        response_statusText,
                    };
                }
        "#,
        )?;

        assert_eq!(
            json!({
                "response_content_type": "text/plain;charset=UTF-8",
                "response_location": "https://example.com/",
                "response_status":307,
                "response_statusText":"Temporary Redirect",
            })
            .to_string(),
            ctx.get_handler_value()?
        );

        ctx.eval(
            r#"
                async function handler() {
                    var response1 = Response.redirect("https://example.com", 301).statusText;
                    var response2 = Response.redirect("https://example.com", 302).statusText;
                    var response3 = Response.redirect("https://example.com", 303).statusText;
                    var response4 = Response.redirect("https://example.com", 307).statusText;
                    var response5 = Response.redirect("https://example.com", 308).statusText;

                    return {
                        response1,
                        response2,
                        response3,
                        response4,
                        response5,
                    };
                }
            "#,
        )?;

        assert_eq!(
            json!({
                "response1": "Moved Permanently",
                "response2": "Found",
                "response3": "See Other",
                "response4": "Temporary Redirect",
                "response5": "Permanent Redirect",
            })
            .to_string(),
            ctx.get_handler_value()?
        );

        ctx.eval(
            r#"
                var response_error;
                try {
                    Response.redirect("https://example.com", 200);
                } catch (e) {
                    response_error = e.message;
                }
            "#,
        )?;

        assert_eq!(
            "The status code must be between 301 and 308.",
            ctx.global.get_property("response_error")?.as_str()?
        );

        Ok(())
    }

    #[test]
    fn test_response_array_buffer() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            async function handler() {
                const response = new Response(new ArrayBuffer(8));
                const response_body = await response.arrayBuffer();

                return response_body;
            }
            "#,
        )?;

        assert_eq!(r#"[0,0,0,0,0,0,0,0]"#, ctx.get_handler_value()?);

        Ok(())
    }

    #[test]
    fn test_response_blob() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            async function handler() {
                const response = new Response(new Blob(["Hello World!"], {type: "text/plain"}));
                const response_body = await response.blob();

                return {size: response_body.size, type: response_body.type};
            }
            "#,
        )?;

        assert_eq!(
            json!({"size":12,"type":"text/plain"}).to_string(),
            ctx.get_handler_value()?
        );

        Ok(())
    }

    #[test]
    fn test_response_clone() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            var response = new Response(undefined);
            var response_clone = response.clone();
            var response_status = response_clone.status;
            var response_statusText = response_clone.statusText;
            var response_body = response_clone.body;
            var response_content_type = response.headers.get('content-type');
            "#,
        )?;

        assert_eq!(
            200.to_string(),
            ctx.global.get_property("response_status")?.as_str()?
        );

        assert_eq!(
            "OK".to_string(),
            ctx.global.get_property("response_statusText")?.as_str()?
        );

        assert_eq!("null", ctx.global.get_property("response_body")?.as_str()?);

        assert_eq!(
            "null",
            ctx.global.get_property("response_content_type")?.as_str()?
        );

        Ok(())
    }

    #[test]
    fn test_response_text() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            async function handler() {
                const response = new Response("Hello World!");
                const response_body = await response.text();

                return response_body;
            }
            "#,
        )?;

        assert_eq!(r#""Hello World!""#, ctx.get_handler_value()?);

        Ok(())
    }

    #[test]
    fn test_response_form_data() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            async function handler() {
                const body = `------cb6762ec1a35a74a
                    Content-Disposition: form-data; name="textFile[]"; filename="text.txt"
                    Content-Type: text/plain
                    
                    Plain Text
                    ------cb6762ec1a35a74a
                    Content-Disposition: form-data; name="textFile[]"; filename="text.txt"
                    Content-Type: text/plain
                    
                    Plain Text 1
                    ------cb6762ec1a35a74a--`;
                const response = new Response(body, {
                    headers: {
                        'Content-Type': 'multipart/form-data; boundary=------cb6762ec1a35a74a'
                    }
                });
                const formData = await response.formData();

                return formData.getAll('textFile');
            }
            "#,
        )?;

        assert_eq!(
            json!(["Plain Text", "Plain Text 1"]).to_string(),
            ctx.get_handler_value()?
        );

        ctx.eval(
            r#"
            async function handler() {
                const body = `------abcd
                    Content-Disposition: form-data; name="textFile1"; filename="text.txt"
                    Content-Type: text/plain
                    
                    Plain Text 1
                    ------abcd
                    Content-Disposition: form-data; name="textFile2"; filename="text.txt"
                    Content-Type: text/plain
                    
                    Plain Text 2
                    ------abcd--`;
                const response = new Response(body, {
                    headers: {
                        'Content-Type': 'multipart/form-data; boundary=------abcd'
                    }
                });
                const formData = await response.formData();
                const textFile1 = formData.get('textFile1');
                const textFile2 = formData.get('textFile2');

                return {textFile1, textFile2};
            }
            "#,
        )?;

        assert_eq!(
            json!({"textFile1": "Plain Text 1", "textFile2": "Plain Text 2"}).to_string(),
            ctx.get_handler_value()?
        );

        Ok(())
    }
}
