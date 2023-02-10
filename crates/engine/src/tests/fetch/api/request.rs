#[cfg(test)]
mod test {
    use anyhow::Result;
    use serde_json::json;

    use crate::tests::test_utils::context::Context;

    #[test]
    fn test_request_constructor() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            var request = new Request("https://example.com",{
                headers: {
                    "Content-Type": "text/plain",
                }
            });

            var request_url = request.url;
            var request_method = request.method;
            var request_headers = request.headers.getAll();
            var request_body = request.body;
        "#,
        )?;

        assert_eq!(
            "https://example.com".to_string(),
            ctx.global.get_property("request_url")?.as_str()?
        );

        assert_eq!(
            "GET".to_string(),
            ctx.global.get_property("request_method")?.as_str()?
        );

        assert_eq!(
            "text/plain".to_string(),
            ctx.global
                .get_property("request_headers")?
                .get_property("content-type")?
                .as_str()?
        );

        assert_eq!(
            "null".to_string(),
            ctx.global.get_property("request_body")?.as_str()?
        );

        ctx.eval(
            r#"
            async function handler() {
                const request = new Request("https://example.com",{
                    method: "POST",
                    headers: {
                        "Content-Type": "text/plain",
                    },
                    body: "Hello World"
                });

                const request_body = await request.text();
                const request_bodyUsed = request.bodyUsed;
                const request_cache = request.cache;
                const request_credentials = request.credentials;
                const request_destination = request.destination;
                const request_headers = request.headers.getAll();
                const request_integrity = request.integrity;
                const request_keepalive = request.keepalive;
                const request_method = request.method;
                const request_mode = request.mode;
                const request_redirect = request.redirect;
                const request_referrer = request.referrer;
                const request_referrerPolicy = request.referrerPolicy;
                const request_signal = String(request.signal);
                const request_url = request.url;

                return {
                    request_body,
                    request_bodyUsed,
                    request_cache,
                    request_credentials,
                    request_destination,
                    request_headers,
                    request_integrity,
                    request_keepalive,
                    request_method,
                    request_mode,
                    request_redirect,
                    request_referrer,
                    request_referrerPolicy,
                    request_signal,
                    request_url
                };
            }
        "#,
        )?;

        assert_eq!(
            json!({
                "request_body": "Hello World",
                "request_bodyUsed": true,
                "request_cache": "default",
                "request_credentials": "same-origin",
                "request_destination": "worker",
                "request_headers": {
                    "content-type": "text/plain",
                },
                "request_integrity": "",
                "request_keepalive": false,
                "request_method": "POST",
                "request_mode": "cors",
                "request_redirect": "follow",
                "request_referrer": "",
                "request_referrerPolicy": "",
                "request_signal": "[object AbortSignal]",
                "request_url": "https://example.com"
            })
            .to_string(),
            ctx.get_handler_value()?
        );

        ctx.eval(
            r#"
            async function handler() {
                const request_initial = new Request("https://example.com",{
                    method: "POST",
                    headers: {
                        "Content-Type": "text/plain",
                    },
                    body: "Hello World"
                });
                const request = new Request(request_initial);

                const request_body = await request.text();
                const request_bodyUsed = request.bodyUsed;
                const request_cache = request.cache;
                const request_credentials = request.credentials;
                const request_destination = request.destination;
                const request_headers = request.headers.getAll();
                const request_integrity = request.integrity;
                const request_keepalive = request.keepalive;
                const request_method = request.method;
                const request_mode = request.mode;
                const request_redirect = request.redirect;
                const request_referrer = request.referrer;
                const request_referrerPolicy = request.referrerPolicy;
                const request_signal = String(request.signal);
                const request_url = request.url;

                return {
                    request_body,
                    request_bodyUsed,
                    request_cache,
                    request_credentials,
                    request_destination,
                    request_headers,
                    request_integrity,
                    request_keepalive,
                    request_method,
                    request_mode,
                    request_redirect,
                    request_referrer,
                    request_referrerPolicy,
                    request_signal,
                    request_url
                };
            }            
        "#,
        )?;

        assert_eq!(
            json!({
                "request_body": "Hello World",
                "request_bodyUsed": true,
                "request_cache": "default",
                "request_credentials": "same-origin",
                "request_destination": "worker",
                "request_headers": {"content-type": "text/plain",},
                "request_integrity": "",
                "request_keepalive": false,
                "request_method": "POST",
                "request_mode": "cors",
                "request_redirect": "follow",
                "request_referrer": "",
                "request_referrerPolicy": "",
                "request_signal": "[object AbortSignal]",
                "request_url": "https://example.com"
            })
            .to_string(),
            ctx.get_handler_value()?
        );

        ctx.eval(
            r#"
            async function handler() {
                const request_initial = new Request("https://example.com",{
                    method: "POST",
                    headers: {
                        "Content-Type": "text/plain",
                    },
                    body: "Hello World"
                });
                const request = new Request(request_initial, {
                    headers: {
                        "Content-Type": "application/potato",
                    }
                });

                const request_headers = request.headers.getAll();

                return request_headers;
            }            
        "#,
        )?;

        assert_eq!(
            json!({"content-type": "application/potato"}).to_string(),
            ctx.get_handler_value()?
        );

        // https://github.com/web-platform-tests/wpt/blob/master/fetch/api/request/request-error.any.js#L39
        ctx.eval(
            r#"
            async function handler() {
                const initialHeaders = new Headers([["Content-Type-Extra", "application/potato"]]);
                const request_initial = new Request("https://example.com",{
                    method: "POST",
                    body: "Hello World",
                    headers: initialHeaders
                });
                const request = new Request(request_initial);
                const request_headers = request.headers?.getAll();

                return request_headers;
            }            
        "#,
        )?;

        assert_eq!(
            "{\"content-type-extra\":\"application/potato\",\"content-type\":\"text/plain;charset=UTF-8\"}"
            .to_string(),
            ctx.get_handler_value()?
        );

        // @see: https://github.com/web-platform-tests/wpt/blob/master/fetch/api/request/request-error.any.js#L46
        ctx.eval(
            r#"
            async function handler() {
                const initialHeaders = new Headers([["Content-Type", "application/potato"]]);
                const request_initial = new Request("https://example.com",{
                    method: "POST",
                    body: "Hello World",
                    headers: initialHeaders
                });
                const request = new Request(request_initial);
                const request_headers = request.headers?.getAll();

                return request_headers;
            }
        "#,
        )?;

        assert_eq!(
            "{\"content-type\":\"application/potato\"}".to_string(),
            ctx.get_handler_value()?
        );

        // @see: https://github.com/web-platform-tests/wpt/blob/master/fetch/api/request/request-error.any.js#L53
        ctx.eval(
            r#"
            async function handler() {
                const request = new Request("https://example.com",{
                    cache: "only-if-cached",
                    mode: "same-origin"
                });
                const request_cache = request.cache;
                const request_mode = request.mode;

                return {
                    request_cache,
                    request_mode
                };
            }
        "#,
        )?;

        assert_eq!(
            json!({
                "request_cache": "only-if-cached",
                "request_mode": "same-origin"
            })
            .to_string(),
            ctx.get_handler_value()?
        );

        ctx.eval(
            r#"
            var request_error = null;
            try {
                new Request("https://example.com",{
                    body: "Hello World"
                });
            } catch (e) {
                request_error = e.message;
            }
        "#,
        )?;

        assert_eq!(
            "Body not allowed for GET or HEAD requests".to_string(),
            ctx.global.get_property("request_error")?.as_str()?
        );

        ctx.eval(
            r#"
            var request_error = null;
            try {
                new Request("https://example.com",{
                    method: "GET",
                    body: "Hello World"
                });
            } catch (e) {
                request_error = e.message;
            }
        "#,
        )?;

        assert_eq!(
            "Body not allowed for GET or HEAD requests".to_string(),
            ctx.global.get_property("request_error")?.as_str()?
        );

        ctx.eval(
            r#"
            var request_error = null;
            try {
                new Request("https://example.com",{
                    method: "HEAD",
                    body: "Hello World"
                });
            } catch (e) {
                request_error = e.message;
            }
        "#,
        )?;

        assert_eq!(
            "Body not allowed for GET or HEAD requests".to_string(),
            ctx.global.get_property("request_error")?.as_str()?
        );

        Ok(())
    }

    #[test]
    // @see: https://github.com/web-platform-tests/wpt/blob/master/fetch/api/request/forbidden-method.any.js
    fn test_request_forbidden_method() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            var request_error = null;
            try {
                new Request("https://example.com",{
                    method: "CONNECT"
                });
            } catch (e) {
                request_error = e.message;
            }
        "#,
        )?;

        assert_eq!(
            "Failed to construct 'Request': 'CONNECT' HTTP method is unsupported".to_string(),
            ctx.global.get_property("request_error")?.as_str()?
        );

        ctx.eval(
            r#"
            var request_error = null;
            try {
                new Request("https://example.com",{
                    method: "connect"
                });
            } catch (e) {
                request_error = e.message;
            }
        "#,
        )?;

        assert_eq!(
            "Failed to construct 'Request': 'connect' HTTP method is unsupported".to_string(),
            ctx.global.get_property("request_error")?.as_str()?
        );

        ctx.eval(
            r#"
            var request_error = null;
            try {
                new Request("https://example.com",{
                    method: "TRACE"
                });
            } catch (e) {
                request_error = e.message;
            }
        "#,
        )?;

        assert_eq!(
            "Failed to construct 'Request': 'TRACE' HTTP method is unsupported".to_string(),
            ctx.global.get_property("request_error")?.as_str()?
        );

        ctx.eval(
            r#"
            var request_error = null;
            try {
                new Request("https://example.com",{
                    method: "trace"
                });
            } catch (e) {
                request_error = e.message;
            }
        "#,
        )?;

        assert_eq!(
            "Failed to construct 'Request': 'trace' HTTP method is unsupported".to_string(),
            ctx.global.get_property("request_error")?.as_str()?
        );

        ctx.eval(
            r#"
            var request_error = null;
            try {
                new Request("https://example.com",{
                    method: "TRACK"
                });
            } catch (e) {
                request_error = e.message;
            }
        "#,
        )?;

        assert_eq!(
            "Failed to construct 'Request': 'TRACK' HTTP method is unsupported".to_string(),
            ctx.global.get_property("request_error")?.as_str()?
        );

        ctx.eval(
            r#"
            var request_error = null;
            try {
                new Request("https://example.com",{
                    method: "track"
                });
            } catch (e) {
                request_error = e.message;
            }
        "#,
        )?;

        assert_eq!(
            "Failed to construct 'Request': 'track' HTTP method is unsupported".to_string(),
            ctx.global.get_property("request_error")?.as_str()?
        );

        Ok(())
    }

    #[test]
    fn test_request_body_readable_stream() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            async function handler() {
                const textDecoder = new TextDecoder();

                const request = new Request('/test', {
                    method: 'POST',
                    body: "Hello World!"
                });;

                const request_body_readable = request.body;
                const request_body_readable_read = await request_body_readable.getReader().read();
                const request_body_readable_read_value = textDecoder.decode(request_body_readable_read.value);

                return request_body_readable_read_value;
            }
            "#,
        )?;

        assert_eq!(r#""Hello World!""#.to_string(), ctx.get_handler_value()?);

        Ok(())
    }

    #[test]
    fn test_request_array_buffer() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            async function handler() {
                const request = new Request('/test', {
                    method: 'POST',
                    body: new ArrayBuffer(8)
                });
                const request_body = await request.arrayBuffer();

                return request_body;
            }
            "#,
        )?;

        assert_eq!(r#"[0,0,0,0,0,0,0,0]"#, ctx.get_handler_value()?);

        Ok(())
    }

    #[test]
    fn test_request_blob() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            async function handler() {
                const request = new Request('/test', {
                    method: 'POST',
                    body: new Blob(["Hello World!"], {type: "text/plain"})
                });
                const request_body = await request.arrayBuffer();

                return {size: request_body.size, type: request_body.type};
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
    fn test_request_clone() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            async function handler() {
                const request = new Request('/test', {
                    method: 'POST',
                    body: 'Hello World!'
                });
                const request_clone = request.clone();
                const request_body = await request.text();
                const request_clone_body = await request_clone.text();

                return {
                    request_body: request_body === request_clone_body,
                    request_bodyUsed: request.bodyUsed === request_clone.bodyUsed,
                    request_cache: request.cache === request_clone.cache,
                    request_credentials: request.credentials === request_clone.credentials,
                    request_headers: request.headers === request_clone.headers,
                    request_integrity: request.integrity === request_clone.integrity,
                    request_keepalive: request.keepalive === request_clone.keepalive,
                    request_method: request.method === request_clone.method,
                    request_mode: request.mode === request_clone.mode,
                    request_redirect: request.redirect === request_clone.redirect,
                    request_referrer: request.referrer === request_clone.referrer,
                    request_signal: (request.signal instanceof AbortSignal) && (request_clone.signal instanceof AbortSignal),
                    request_url: request.url === request_clone.url,
                };
            }
            "#,
        )?;

        assert_eq!(
            json!(
                {
                    "request_body": true,
                    "request_bodyUsed": true,
                    "request_cache": true,
                    "request_credentials": true,
                    "request_headers": true,
                    "request_integrity": true,
                    "request_keepalive": true,
                    "request_method": true,
                    "request_mode": true,
                    "request_redirect": true,
                    "request_referrer": true,
                    "request_signal": true,
                    "request_url": true,
                }
            )
            .to_string(),
            ctx.get_handler_value()?
        );

        ctx.eval(
            r#"
            async function handler() {
                const request = new Request('/test', {
                    method: 'POST',
                    body: 'Hello World!'
                });

                try {
                    const request_body = await request.text();
                    const request_clone = request.clone();
                } catch (e) {
                    return e.message;
                }
            }
            "#,
        )?;

        assert_eq!(
            r#""Failed to execute 'clone' on 'Request': Request body is already used""#.to_string(),
            ctx.get_handler_value()?
        );

        Ok(())
    }

    #[test]
    fn test_request_text() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            async function handler() {
                const request = new Request('/test', {
                    method: 'POST',
                    body: "Hello World!"
                });
                const request_body = await request.text();

                return request_body;
            }
            "#,
        )?;

        assert_eq!(r#""Hello World!""#, ctx.get_handler_value()?);

        Ok(())
    }

    #[test]
    fn test_request_form_data() -> Result<()> {
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
                const request = new Request('/test', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'multipart/form-data; boundary=------cb6762ec1a35a74a'
                    },
                    body
                });
                const formData = await request.formData();

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
                var body = `------abcd
                    Content-Disposition: form-data; name="textFile1"; filename="text.txt"
                    Content-Type: text/plain
                    
                    Plain Text 1
                    ------abcd
                    Content-Disposition: form-data; name="textFile2"; filename="text.txt"
                    Content-Type: text/plain
                    
                    Plain Text 2
                    ------abcd--`;
                const request = new Request('/test', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'multipart/form-data; boundary=------abcd'
                    },
                    body
                });
                const formData = await request.formData();
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
