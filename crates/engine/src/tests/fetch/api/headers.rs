// @see: https://fetch.spec.whatwg.org/#headers-class
// @see: https://github.com/web-platform-tests/wpt/tree/master/fetch/api/headers
#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::tests::test_utils::context::Context;

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/Headers
    #[test]
    fn test_headers_constructor() -> Result<()> {
        let mut ctx = Context::new();

        // Without headers
        ctx.eval(
            r#"
                var headers = new Headers();
                var header_value = JSON.stringify(headers.getAll());
            "#,
        )?;

        assert_eq!("{}", ctx.global.get_property("header_value")?.as_str()?);

        // Undefined
        ctx.eval(
            r#"
                var headers = new Headers(undefined);
                var header_value = JSON.stringify(headers.getAll());
            "#,
        )?;

        assert_eq!("{}", ctx.global.get_property("header_value")?.as_str()?);

        // Object
        ctx.eval(
            r#"
                var headers = new Headers({ "Content-Type": "application/json", "Authorization": "Bearer 12345" });
                var header_value = JSON.stringify(headers.getAll());
            "#,
        )?;

        assert_eq!(
            r#"{"content-type":"application/json","authorization":"Bearer 12345"}"#,
            ctx.global.get_property("header_value")?.as_str()?
        );

        // Array
        ctx.eval(
            r#"
                var headers = new Headers([["Content-Type", "application/json"], ["Authorization", "Bearer 12345"]]);
                var header_value = JSON.stringify(headers.getAll());
            "#,
        )?;

        assert_eq!(
            r#"{"content-type":"application/json","authorization":"Bearer 12345"}"#,
            ctx.global.get_property("header_value")?.as_str()?
        );

        // TODO: test case where the array contains an array with more than 2 elements

        // Headers
        ctx.eval(
            r#"
                var headers1 = new Headers({ "Content-Type": "application/json", "Authorization": "Bearer 12345" });
                var headers2 = new Headers(headers1);
                var header_value = JSON.stringify(headers.getAll());
            "#,
        )?;

        assert_eq!(
            r#"{"content-type":"application/json","authorization":"Bearer 12345"}"#,
            ctx.global.get_property("header_value")?.as_str()?
        );

        // // Null
        // // @see: https://github.com/web-platform-tests/wpt/blob/master/fetch/api/headers/headers-record.any.js#L35
        assert_eq!(
            "Uncaught TypeError: Failed to construct 'Headers': The provided value is null\n    at Ve (context.js)\n    at <eval> (context.js:2)\n",
            ctx.eval(r#"var headers = new Headers(null);"#)
                .unwrap_err()
                .to_string()
        );

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/master/fetch/api/headers/headers-structure.any.js
    #[test]
    fn test_headers_structure() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(r#"var headers = new Headers();"#)?;

        let headers = ctx.context.global_object()?.get_property("headers")?;

        assert!(headers.get_property("append")?.is_function());
        assert!(headers.get_property("get")?.is_function());
        // getAll is implemented for our use case, but not part of the spec
        assert!(headers.get_property("getAll")?.is_function());
        assert!(headers.get_property("has")?.is_function());
        assert!(headers.get_property("set")?.is_function());
        assert!(headers.get_property("delete")?.is_function());
        assert!(headers.get_property("forEach")?.is_function());
        assert!(headers.get_property("keys")?.is_function());
        assert!(headers.get_property("values")?.is_function());
        assert!(headers.get_property("entries")?.is_function());

        Ok(())
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/append
    #[test]
    fn test_headers_append() -> Result<()> {
        let mut ctx = Context::new();

        // Create header
        ctx.eval(
            r#"
            var headers = new Headers();
            headers.append("Content-Type", "application/json");
            var header_value = headers.get("Content-Type");
            "#,
        )?;

        assert_eq!(
            r#"application/json"#,
            ctx.global.get_property("header_value")?.as_str()?
        );

        // Append new value
        ctx.eval(
            r#"
            var headers = new Headers();
            
            headers.append('Accept-Encoding', 'deflate');
            headers.append('Accept-Encoding', 'gzip');

            var header_value = headers.get('Accept-Encoding');
            "#,
        )?;

        assert_eq!(
            r#"deflate, gzip"#,
            ctx.global.get_property("header_value")?.as_str()?
        );

        Ok(())
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/get
    #[test]
    fn test_headers_get() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
                var headers = new Headers();
                headers.append("Content-Type", "application/json");
                var header_value = headers.get("Content-Type");
            "#,
        )?;

        assert_eq!(
            r#"application/json"#,
            ctx.global.get_property("header_value")?.as_str()?
        );

        Ok(())
    }

    #[test]
    fn test_headers_get_all() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
                var headers = new Headers({ "Content-Type": "application/json", "Authorization": "Bearer 12345" });
                var header_value = JSON.stringify(headers.getAll());
            "#,
        )?;

        assert_eq!(
            r#"{"content-type":"application/json","authorization":"Bearer 12345"}"#,
            ctx.global.get_property("header_value")?.as_str()?
        );

        Ok(())
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/has
    #[test]
    fn test_headers_has() -> Result<()> {
        let mut ctx = Context::new();

        // True
        ctx.eval(
            r#"
            var headers = new Headers();
            headers.append("Content-Type", "application/json");
            var header_value = headers.has("Content-Type");
        "#,
        )?;

        assert_eq!("true", ctx.global.get_property("header_value")?.as_str()?);

        // False
        ctx.eval(
            r#"
            var headers = new Headers();
            headers.append("Content-Type", "application/json");
            var header_value = headers.has("Authorization");
        "#,
        )?;

        assert_eq!("false", ctx.global.get_property("header_value")?.as_str()?);

        Ok(())
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/set
    #[test]
    fn test_headers_set() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            var headers = new Headers();
            headers.set("Content-Type", "text/plain");
            var header_value = headers.get("Content-Type");
            "#,
        )?;

        assert_eq!(
            "text/plain",
            ctx.global.get_property("header_value")?.as_str()?
        );

        Ok(())
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/delete
    #[test]
    fn test_headers_delete() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            var headers = new Headers();
            headers.append("Content-Type", "application/json");
            headers.delete("Content-Type");
            var header_value = headers.get("Content-Type");
            "#,
        )?;

        assert_eq!("null", ctx.global.get_property("header_value")?.as_str()?);

        Ok(())
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/forEach
    #[test]
    fn test_headers_for_each() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            var header_values = [], header_names = [];
            var headers = new Headers();
            headers.append("Content-Type", "application/json");
            headers.append("Authorization", "Bearer 12345");
            headers.forEach((value, name) => {
                header_values.push(value);
                header_names.push(name);
            });
        "#,
        )?;

        assert_eq!(
            "application/json,Bearer 12345",
            ctx.global.get_property("header_values")?.as_str()?
        );

        assert_eq!(
            "content-type,authorization",
            ctx.global.get_property("header_names")?.as_str()?
        );

        Ok(())
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/keys
    #[test]
    fn test_headers_keys() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            var headers = new Headers();
            headers.append("Content-Type", "application/json");
            headers.append("Authorization", "Bearer 12345");
            var header_keys = [];

            for (let key of headers.keys()) {
                header_keys.push(key);
             }
            "#,
        )?;

        assert_eq!(
            "content-type,authorization",
            ctx.global.get_property("header_keys")?.as_str()?
        );

        Ok(())
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/values
    #[test]
    fn test_headers_values() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            var headers = new Headers();
            headers.append("Content-Type", "application/json");
            headers.append("Authorization", "Bearer 12345");
            var header_values = [];

            for (let value of headers.values()) {
                header_values.push(value);
             }
            "#,
        )?;

        assert_eq!(
            "application/json,Bearer 12345",
            ctx.global.get_property("header_values")?.as_str()?
        );

        Ok(())
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/entries
    #[test]
    fn test_headers_entries() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            var headers = new Headers();
            headers.append("Content-Type", "application/json");
            headers.append("Authorization", "Bearer 12345");
            var header_entries = [];

            for (let pair of headers.entries()) {
                header_entries.push(pair[0] + ": " + pair[1]);
             }
            "#,
        )?;

        assert_eq!(
            "content-type: application/json,authorization: Bearer 12345",
            ctx.global.get_property("header_entries")?.as_str()?
        );

        Ok(())
    }

    // @see: https://fetch.spec.whatwg.org/#header-name
    // @see: https://github.com/web-platform-tests/wpt/blob/master/fetch/api/headers/headers-normalize.any.js
    #[test]
    fn test_headers_normalize_name() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
                var headers = new Headers([
                    ["Content-Type", ""],
                    ["Authorization", ""],
                    [" space ",""],
                    ["\ttab\t",""],
                    [" spaceAndTab\t",""],
                    ["\r\n newLine1",""],
                    ["newLine2\r\n ",""],
                    ["\r\n\tnewLine3",""],
                    // TODO: cover these cases
                    // ["\t\f\tnewLine4\n","\f\tnewLine"],
                    // ["newLine5\xa0",""],
                    
                ]);
                var header_keys = [];

                for (let key of headers.keys()) {
                    header_keys.push(key);
                }
            "#,
        )?;

        assert_eq!(
            r#"content-type,authorization,space,tab,spaceandtab,newline1,newline2,newline3"#,
            ctx.global.get_property("header_keys")?.as_str()?
        );

        Ok(())
    }

    #[test]
    fn test_headers_normalize_value() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            var headers = new Headers();
            headers.append("X-Null", null);
            headers.append("X-Undefined", undefined);
            headers.append("X-Number", 12345);
            headers.append("X-Array", ["a", "b", "c"]);
            headers.append("X-Object", {});
            var header_values = [];

            for (let value of headers.values()) {
                header_values.push(`${value}:type(${typeof value})`);
             }
            "#,
        )?;

        assert_eq!(
            "null:type(string),undefined:type(string),12345:type(string),a,b,c:type(string),[object Object]:type(string)",
            ctx.global.get_property("header_values")?.as_str()?
        );

        Ok(())
    }
}
