// TODO: Test URLSearchParams
// @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url
// @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js
#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::tests::test_utils::context::Context;

    #[test]
    fn test_urlsearchparams_constructor() -> Result<()> {
        let mut ctx = Context::new();

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L1
        ctx.eval(
            r#"
            var params = new URLSearchParams();

            var usp_string = params.toString();
        "#,
        )?;

        assert_eq!("", ctx.global.get_property("usp_string")?.as_str()?);

        ctx.eval(
            r#"
            var params = new URLSearchParams("");

            var usp_string = params.toString();
        "#,
        )?;

        assert_eq!("", ctx.global.get_property("usp_string")?.as_str()?);

        ctx.eval(
            r#"
            var params = new URLSearchParams("a=b");

            var usp_string = params.toString();
        "#,
        )?;

        assert_eq!("a=b", ctx.global.get_property("usp_string")?.as_str()?);

        ctx.eval(
            r#"
            var params = new URLSearchParams("a=b");
            var params = new URLSearchParams(params);

            var usp_string = params.toString();
        "#,
        )?;

        assert_eq!("a=b", ctx.global.get_property("usp_string")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L17
        ctx.eval(
            r#"
            var params = new URLSearchParams("?a=b");

            var usp_string = params.toString();
        "#,
        )?;

        assert_eq!("a=b", ctx.global.get_property("usp_string")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L35
        ctx.eval(
            r#"
            var params = new URLSearchParams({});

            var usp_string = params.toString();
        "#,
        )?;

        assert_eq!("", ctx.global.get_property("usp_string")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L41
        ctx.eval(
            r#"
            var params = new URLSearchParams("a=b");
            
            var usp_has_a = params.has("a");
            var usp_has_b = params.has("b");
        "#,
        )?;

        assert_eq!("true", ctx.global.get_property("usp_has_a")?.as_str()?);
        assert_eq!("false", ctx.global.get_property("usp_has_b")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L46
        ctx.eval(
            r#"
            var params = new URLSearchParams("a=b&c");

            var usp_has_a = params.has("a");
            var usp_has_c = params.has("c");
        "#,
        )?;

        assert_eq!("true", ctx.global.get_property("usp_has_a")?.as_str()?);
        assert_eq!("true", ctx.global.get_property("usp_has_c")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L51
        ctx.eval(
            r#"
            var params = new URLSearchParams("&a&&& &&&&&a+b=& c&m%c3%b8%c3%b8");

            var usp_has_a = params.has("a");
            var usp_has_a_b = params.has("a b");
            var usp_has_ = params.has(" ");
            var usp_has_c = params.has("c");
            var usp_has__c = params.has(" c");
            var usp_has_m = params.has("møø");

        "#,
        )?;

        assert_eq!("true", ctx.global.get_property("usp_has_a")?.as_str()?);
        assert_eq!("true", ctx.global.get_property("usp_has_a_b")?.as_str()?);
        assert_eq!("true", ctx.global.get_property("usp_has_")?.as_str()?);
        assert_eq!("false", ctx.global.get_property("usp_has_c")?.as_str()?);
        assert_eq!("true", ctx.global.get_property("usp_has__c")?.as_str()?);
        assert_eq!("true", ctx.global.get_property("usp_has_m")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L60
        ctx.eval(
            r#"
            var params = new URLSearchParams("id=0&value=%");

            var usp_has_id = params.has("id");
            var usp_has_value = params.has("value");
            var usp_id = params.get("id");
            var usp_value = params.get("value");
        "#,
        )?;

        assert_eq!("true", ctx.global.get_property("usp_has_id")?.as_str()?);
        assert_eq!("true", ctx.global.get_property("usp_has_value")?.as_str()?);
        assert_eq!("0", ctx.global.get_property("usp_id")?.as_str()?);
        assert_eq!("%", ctx.global.get_property("usp_value")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L67
        ctx.eval(
            r#"
            var params = new URLSearchParams("b=%2sf%2a");

            var usp_has_b = params.has("b");
            var usp_b = params.get("b");
        "#,
        )?;

        assert_eq!("true", ctx.global.get_property("usp_has_b")?.as_str()?);
        assert_eq!("%2sf*", ctx.global.get_property("usp_b")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L72
        ctx.eval(
            r#"
            var params = new URLSearchParams("b=%2%2af%2a");

            var usp_has_b = params.has("b");
            var usp_b = params.get("b");
        "#,
        )?;

        assert_eq!("true", ctx.global.get_property("usp_has_b")?.as_str()?);
        assert_eq!("%2*f*", ctx.global.get_property("usp_b")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L77
        ctx.eval(
            r#"
            var params = new URLSearchParams("b=%%2a");

            var usp_has_b = params.has("b");
            var usp_b = params.get("b");
        "#,
        )?;

        assert_eq!("true", ctx.global.get_property("usp_has_b")?.as_str()?);
        assert_eq!("%*", ctx.global.get_property("usp_b")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L83
        ctx.eval(
            r#"
            var seed = new URLSearchParams('a=b&c=d');
            var params = new URLSearchParams(seed);

            var usp_a = params.get("a");
            var usp_c = params.get("c");
            var usp_has_d = params.has("d");

            // The name-value pairs are copied when created; later updates
            // should not be observable.
            seed.append('e', 'f');
            var usp_has_e = params.has("e");

            params.append('g', 'h');
            var usp_has_g = seed.has("g");
        "#,
        )?;

        assert_eq!("b", ctx.global.get_property("usp_a")?.as_str()?);
        assert_eq!("d", ctx.global.get_property("usp_c")?.as_str()?);
        assert_eq!("false", ctx.global.get_property("usp_has_d")?.as_str()?);
        assert_eq!("false", ctx.global.get_property("usp_has_e")?.as_str()?);
        assert_eq!("false", ctx.global.get_property("usp_has_g")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L98
        ctx.eval(
            r#"
            var formData = new FormData()
            formData.append('a', 'b')
            formData.append('c', 'd')
            var params = new URLSearchParams(formData);

            var usp_a = params.get("a");
            var usp_c = params.get("c");
            var usp_has_d = params.has("d");

            // The name-value pairs are copied when created; later updates
            // should not be observable.
            formData.append('e', 'f');
            var usp_has_e = params.has("e");

            params.append('g', 'h')
            var usp_has_g = formData.has("g");
        "#,
        )?;

        assert_eq!("b", ctx.global.get_property("usp_a")?.as_str()?);
        assert_eq!("d", ctx.global.get_property("usp_c")?.as_str()?);
        assert_eq!("false", ctx.global.get_property("usp_has_d")?.as_str()?);
        assert_eq!("false", ctx.global.get_property("usp_has_e")?.as_str()?);
        assert_eq!("false", ctx.global.get_property("usp_has_g")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L115
        // TODO: Parse +

        Ok(())
    }
}
