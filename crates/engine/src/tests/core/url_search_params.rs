// TODO: Test URLSearchParams
// @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url
// @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js
#[cfg(test)]
mod test {
    use anyhow::{Ok, Result};

    use crate::tests::test_utils::context::Context;

    #[test]
    fn test_url_search_params_constructor() -> Result<()> {
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
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=b+c');

            var usp_a = params.get("a");

            params = new URLSearchParams('a+b=c');

            var usp_a_b = params.get("a b");
        "#,
        )?;

        assert_eq!("b c", ctx.global.get_property("usp_a")?.as_str()?);
        assert_eq!("c", ctx.global.get_property("usp_a_b")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L122
        ctx.eval(
            r#"
            var testValue = '+15555555555';
            var params = new URLSearchParams();
            params.set('query', testValue);
            var newParams = new URLSearchParams(params.toString());

            var usp_string = params.toString();
            var usp_query = params.get('query');
            var usp_new_query = newParams.get('query');
        "#,
        )?;

        assert_eq!(
            "query=%2B15555555555",
            ctx.global.get_property("usp_string")?.as_str()?
        );
        assert_eq!(
            "+15555555555",
            ctx.global.get_property("usp_query")?.as_str()?
        );
        assert_eq!(
            "+15555555555",
            ctx.global.get_property("usp_new_query")?.as_str()?
        );

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L133
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=b c');

            var usp_a = params.get("a");

            params = new URLSearchParams('a b=c');

            var usp_a_b = params.get("a b");
        "#,
        )?;

        assert_eq!("b c", ctx.global.get_property("usp_a")?.as_str()?);
        assert_eq!("c", ctx.global.get_property("usp_a_b")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L140
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=b%20c');

            var usp_a = params.get("a");

            params = new URLSearchParams('a%20b=c');

            var usp_a_b = params.get("a b");   
        "#,
        )?;

        assert_eq!("b c", ctx.global.get_property("usp_a")?.as_str()?);
        assert_eq!("c", ctx.global.get_property("usp_a_b")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L147
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=b\0c');

            var usp_a = params.get("a");

            params = new URLSearchParams('a\0b=c');

            var usp_a_b = params.get("a\0b");
        "#,
        )?;

        assert_eq!("b\0c", ctx.global.get_property("usp_a")?.as_str()?);
        assert_eq!("c", ctx.global.get_property("usp_a_b")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L154
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=b%00c');

            var usp_a = params.get("a");

            params = new URLSearchParams('a%00b=c');

            var usp_a_b = params.get("a\0b");
        "#,
        )?;

        assert_eq!("b\0c", ctx.global.get_property("usp_a")?.as_str()?);
        assert_eq!("c", ctx.global.get_property("usp_a_b")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L161
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=b\u2384');

            var usp_a = params.get("a");

            params = new URLSearchParams('a\u2384b=c');

            var usp_a_b = params.get("a\u2384b");
        "#,
        )?;

        assert_eq!("b\u{2384}", ctx.global.get_property("usp_a")?.as_str()?);
        assert_eq!("c", ctx.global.get_property("usp_a_b")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L168
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=b%e2%8e%84');

            var usp_a = params.get("a");

            params = new URLSearchParams('a%e2%8e%84b=c');

            var usp_a_b = params.get("a\u2384b");
        "#,
        )?;

        assert_eq!("b\u{2384}", ctx.global.get_property("usp_a")?.as_str()?);
        assert_eq!("c", ctx.global.get_property("usp_a_b")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L175
        // Parse \uD83D\uDCA9
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=b\uD83D\uDCA9c');

            var usp_a = params.get("a");

            params = new URLSearchParams('a\uD83D\uDCA9b=c');

            var usp_a_b = params.get("a\uD83D\uDCA9b");
        "#,
        )?;

        assert_eq!("b\u{1F4A9}c", ctx.global.get_property("usp_a")?.as_str()?);
        assert_eq!("c", ctx.global.get_property("usp_a_b")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L182
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=b%f0%9f%92%a9c');

            var usp_a = params.get("a");

            params = new URLSearchParams('a%f0%9f%92%a9b=c');

            var usp_a_b = params.get('a\uD83D\uDCA9b');
        "#,
        )?;

        assert_eq!("b\u{1F4A9}c", ctx.global.get_property("usp_a")?.as_str()?);
        assert_eq!("c", ctx.global.get_property("usp_a_b")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L189
        ctx.eval(
            r#"
            var params = new URLSearchParams([]);

            var usp_empty = params != null;

            var params = new URLSearchParams([["a", "b"], ["c", "d"]]);

            var usp_a = params.get("a");
            var usp_c = params.get("c");

            var usp_error_1;

            try {
                new URLSearchParams([[1]]);
            } catch (e) {
                usp_error_1 = e instanceof TypeError;
            }

            var usp_error_2;

            try {
                new URLSearchParams([[1,2,3]]);
            } catch (e) {
                usp_error_2 = e instanceof TypeError;
            }

        "#,
        )?;

        assert_eq!("true", ctx.global.get_property("usp_empty")?.as_str()?);
        assert_eq!("b", ctx.global.get_property("usp_a")?.as_str()?);
        assert_eq!("d", ctx.global.get_property("usp_c")?.as_str()?);
        assert_eq!("true", ctx.global.get_property("usp_error_1")?.as_str()?);
        assert_eq!("true", ctx.global.get_property("usp_error_2")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L199
        // TODO: Construct with ...

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-constructor.any.js#L217
        // Custom [Symbol.iterator]
        ctx.eval(
            r#"
            var params = new URLSearchParams()
            params[Symbol.iterator] = function *() {
              yield ["a", "b"]
            }
            let params2 = new URLSearchParams(params)

            var usp_a = params2.get("a");
        "#,
        )?;

        assert_eq!("b", ctx.global.get_property("usp_a")?.as_str()?);

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-append.any.js
    #[test]
    fn test_url_search_params_append() -> Result<()> {
        let mut ctx = Context::new();

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-append.any.js#L1
        ctx.eval(
            r#"
            var params = new URLSearchParams();
            params.append("a", "b");
            
            var usp_a_1 = params + '';

            params.append("a", "b");

            var usp_a_2 = params + '';

            params.append("a", "c");

            var usp_a_3 = params + '';
        "#,
        )?;

        assert_eq!("a=b", ctx.global.get_property("usp_a_1")?.as_str()?);
        assert_eq!("a=b&a=b", ctx.global.get_property("usp_a_2")?.as_str()?);
        assert_eq!("a=b&a=b&a=c", ctx.global.get_property("usp_a_3")?.as_str()?);

        // https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-append.any.js#L11
        ctx.eval(
            r#"
            var params = new URLSearchParams();
            params.append("", "");

            var usp_a_1 = params + '';

            params.append("", "");

            var usp_a_2 = params + '';
        "#,
        )?;

        assert_eq!("=", ctx.global.get_property("usp_a_1")?.as_str()?);
        assert_eq!("=&=", ctx.global.get_property("usp_a_2")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-append.any.js#L19
        ctx.eval(
            r#"
            var params = new URLSearchParams();
            params.append(null, null);

            var usp_a_1 = params + '';

            params.append(null, null);

            var usp_a_2 = params + '';
        "#,
        )?;

        assert_eq!("null=null", ctx.global.get_property("usp_a_1")?.as_str()?);
        assert_eq!(
            "null=null&null=null",
            ctx.global.get_property("usp_a_2")?.as_str()?
        );

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-append.any.js#L27
        ctx.eval(
            r#"
            var params = new URLSearchParams();
            params.append('first', 1);
            params.append('second', 2);
            params.append('third', '');
            params.append('first', 10);

            var usp_has_1 = params.has('first');
            var usp_get_1 = params.get('first');
            var usp_get_2 = params.get('second');
            var usp_get_3 = params.get('third');

            params.append('first', 10);

            var usp_get_1_2 = params.get('first');
        "#,
        )?;

        assert_eq!("true", ctx.global.get_property("usp_has_1")?.as_str()?);
        assert_eq!("1", ctx.global.get_property("usp_get_1")?.as_str()?);
        assert_eq!("2", ctx.global.get_property("usp_get_2")?.as_str()?);
        assert_eq!("", ctx.global.get_property("usp_get_3")?.as_str()?);
        assert_eq!("1", ctx.global.get_property("usp_get_1_2")?.as_str()?);

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-delete.any.js
    #[test]
    fn test_url_search_params_delete() -> Result<()> {
        let mut ctx = Context::new();

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-delete.any.js#L1
        // Delete basics
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=b&c=d');
            params.delete('a');
            var usp_1 = params + '';
            params = new URLSearchParams('a=a&b=b&a=a&c=c');
            params.delete('a');
            var usp_2 = params + '';
            params = new URLSearchParams('a=a&=&b=b&c=c');
            params.delete('');
            var usp_3 = params + '';
            params = new URLSearchParams('a=a&null=null&b=b');
            params.delete(null);
            var usp_4 = params + '';
            params = new URLSearchParams('a=a&undefined=undefined&b=b');
            params.delete(undefined);
            var usp_5 = params + '';
        "#,
        )?;

        assert_eq!("c=d", ctx.global.get_property("usp_1")?.as_str()?);
        assert_eq!("b=b&c=c", ctx.global.get_property("usp_2")?.as_str()?);
        assert_eq!("a=a&b=b&c=c", ctx.global.get_property("usp_3")?.as_str()?);
        assert_eq!("a=a&b=b", ctx.global.get_property("usp_4")?.as_str()?);
        assert_eq!("a=a&b=b", ctx.global.get_property("usp_5")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-delete.any.js#L19
        ctx.eval(
            r#"
            var params = new URLSearchParams();
            params.append('first', 1);
            var usp_has_1 = params.has('first')
            var usp_get_1 = params.get('first');
            params.delete('first');
            var usp_get_1_1 = params.get('first');
            params.append('first', 1);
            params.append('first', 10);
            params.delete('first');
            var usp_get_1_2 = params.get('first');
        "#,
        )?;

        assert_eq!("true", ctx.global.get_property("usp_has_1")?.as_str()?);
        assert_eq!("1", ctx.global.get_property("usp_get_1")?.as_str()?);
        assert_eq!("null", ctx.global.get_property("usp_get_1_1")?.as_str()?);
        assert_eq!("null", ctx.global.get_property("usp_get_1_2")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-delete.any.js#L32
        ctx.eval(
            r#"
            var url = new URL('http://example.com/?param1&param2');
            url.searchParams.delete('param1');
            url.searchParams.delete('param2');
            var usp_href = url.href;
            var usp_search = url.search;
        "#,
        )?;

        assert_eq!(
            "http://example.com/",
            ctx.global.get_property("usp_href")?.as_str()?
        );
        assert_eq!("", ctx.global.get_property("usp_search")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-delete.any.js#L40
        ctx.eval(
            r#"
            var url = new URL('http://example.com/?');
            url.searchParams.delete('param1');
            var usp_href = url.href;
            var usp_search = url.search;
            "#,
        )?;

        assert_eq!(
            "http://example.com/",
            ctx.global.get_property("usp_href")?.as_str()?
        );
        assert_eq!("", ctx.global.get_property("usp_search")?.as_str()?);

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-foreach.any.js
    #[test]
    fn test_url_search_params_foreach() -> Result<()> {
        let mut ctx = Context::new();

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-foreach.any.js#L1
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=1&b=2&c=3');
            var keys = [];
            var values = [];
            params.forEach(function(value, key) {
                keys.push(key);
                values.push(value);
            });
            var usp_keys = JSON.stringify(keys);
            var usp_values = JSON.stringify(values);
        "#,
        )?;

        assert_eq!(
            r#"["a","b","c"]"#,
            ctx.global.get_property("usp_keys")?.as_str()?
        );
        assert_eq!(
            r#"["1","2","3"]"#,
            ctx.global.get_property("usp_values")?.as_str()?
        );

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-foreach.any.js#L13
        // The test doesn't follow the the WPT spec as 'a.search = "x=1&y=2&z=3"' isn't updating the param
        ctx.eval(
            r#"
            var a = new URL("http://a.b/c?a=1&b=2&c=3&d=4");
            var b = a.searchParams;
            var c = [];
            for (const i of b) {
                // Not updates the searchParams
                a.search = "x=1&y=2&z=3"
                c.push(i);
            }
            var usp_c_0 = JSON.stringify(c[0]);
            var usp_c_1 = JSON.stringify(c[1]);
            var usp_c_2 = JSON.stringify(c[2]);
        "#,
        )?;

        assert_eq!(
            r#"["a","1"]"#,
            ctx.global.get_property("usp_c_0")?.as_str()?
        );
        assert_eq!(
            r#"["b","2"]"#,
            ctx.global.get_property("usp_c_1")?.as_str()?
        );
        assert_eq!(
            r#"["c","3"]"#,
            ctx.global.get_property("usp_c_2")?.as_str()?
        );

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-foreach.any.js#L26
        ctx.eval(
            r#"
            var a = new URL("http://a.b/c");
            var b = a.searchParams;
            var usp_unreached = [];
            for (const i of b) {
                usp_unreached.push(i);
            }
        "#,
        )?;

        assert_eq!(r#""#, ctx.global.get_property("usp_unreached")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-foreach.any.js#L34
        // The test doesn't follow the the WPT spec as 'searchParams.delete('param1');' isn't updating the param
        ctx.eval(
            r#"
            var url = new URL("http://localhost/query?param0=0&param1=1&param2=2");
            var searchParams = url.searchParams;
            var seen = [];
            for (const param of searchParams) {
                if (param[0] === 'param0') {
                    // Not updates the searchParams
                    searchParams.delete('param1');
                }
                seen.push(param);
            }
        
            var usp_seen_0 = seen[0];
            var usp_seen_1 = seen[1];
        "#,
        )?;

        assert_eq!(
            r#"param0,0"#,
            ctx.global.get_property("usp_seen_0")?.as_str()?
        );
        assert_eq!(
            r#"param1,1"#,
            ctx.global.get_property("usp_seen_1")?.as_str()?
        );

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-foreach.any.js#L49
        // The test doesn't follow the the WPT spec as 'searchParams.delete('param0');' isn't updating the param
        ctx.eval(
            r#"
            var url = new URL("http://localhost/query?param0=0&param1=1&param2=2");
            var searchParams = url.searchParams;
            var seen = [];
            for (var param of searchParams) {
                if (param[0] === 'param0') {
                    searchParams.delete('param0');
                    // 'param1=1' is now in the first slot, so the next iteration will see 'param2=2'.
                } else {
                    seen.push(param);
                }
            }

            var usp_seen_0 = seen[0];
        "#,
        )?;

        assert_eq!(
            r#"param1,1"#,
            ctx.global.get_property("usp_seen_0")?.as_str()?
        );

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-foreach.any.js#L65
        // The test doesn't follow the the WPT spec as 'searchParams.delete(param[0]);' isn't updating the param
        ctx.eval(
            r#"
            var url = new URL("http://localhost/query?param0=0&param1=1&param2=2");
            var searchParams = url.searchParams;
            var seen = [];
            for (const param of searchParams) {
                seen.push(param[0]);
                searchParams.delete(param[0]);
            }

            var usp_seen = seen;
            var usp_searchParams = searchParams;
        "#,
        )?;

        assert_eq!(
            r#"param0,param1,param2"#,
            ctx.global.get_property("usp_seen")?.as_str()?
        );
        assert_eq!(
            r#""#,
            ctx.global.get_property("usp_searchParams")?.as_str()?
        );

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-get.any.js
    #[test]
    fn test_url_search_params_get() -> Result<()> {
        let mut ctx = Context::new();

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-get.any.js#L1
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=b&c=d');
            var usp_get_a = params.get('a');
            var usp_get_c = params.get('a');
            var usp_get_e = params.get('e');
            params = new URLSearchParams('a=b&c=d&a=e');
            var usp_get_a_2 = params.get('a');
            params = new URLSearchParams('=b&c=d');
            var usp_get_empty = params.get('');
            params = new URLSearchParams('a=&c=d&a=e');
            var usp_get_a_3 = params.get('a');
        "#,
        )?;

        assert_eq!(r#"b"#, ctx.global.get_property("usp_get_a")?.as_str()?);
        assert_eq!(r#"b"#, ctx.global.get_property("usp_get_c")?.as_str()?);
        assert_eq!(r#"null"#, ctx.global.get_property("usp_get_e")?.as_str()?);
        assert_eq!(r#"b"#, ctx.global.get_property("usp_get_a_2")?.as_str()?);
        assert_eq!(r#"b"#, ctx.global.get_property("usp_get_empty")?.as_str()?);
        assert_eq!(r#""#, ctx.global.get_property("usp_get_a_3")?.as_str()?);

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-get.any.js#L14
        ctx.eval(
            r#"
            var params = new URLSearchParams('first=second&third&&');
            var usp_not_null = params != null;
            var usp_has_first = params.has('first');
            var usp_get_first = params.get('first');
            var usp_get_third = params.get('third');
            var usp_get_fourth = params.get('fourth');
        "#,
        )?;

        assert_eq!(
            r#"true"#,
            ctx.global.get_property("usp_not_null")?.as_str()?
        );
        assert_eq!(
            r#"true"#,
            ctx.global.get_property("usp_has_first")?.as_str()?
        );
        assert_eq!(
            r#"second"#,
            ctx.global.get_property("usp_get_first")?.as_str()?
        );
        assert_eq!(r#""#, ctx.global.get_property("usp_get_third")?.as_str()?);
        assert_eq!(
            r#"null"#,
            ctx.global.get_property("usp_get_fourth")?.as_str()?
        );

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-getall.any.js
    #[test]
    fn test_url_search_params_get_all() -> Result<()> {
        let mut ctx = Context::new();

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-getall.any.js#L1
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=b&c=d');
            var usp_get_all_a = JSON.stringify(params.getAll('a'));
            var usp_get_all_c = JSON.stringify(params.getAll('c'));
            var usp_get_all_e = JSON.stringify(params.getAll('e'));
            params = new URLSearchParams('a=b&c=d&a=e');
            var usp_get_all_a_2 = JSON.stringify(params.getAll('a'));
            params = new URLSearchParams('=b&c=d');
            var usp_get_all_empty = JSON.stringify(params.getAll(''));
            params = new URLSearchParams('a=&c=d&a=e');
            var usp_get_all_a_3 = JSON.stringify(params.getAll('a'));
        "#,
        )?;

        assert_eq!(
            r#"["b"]"#,
            ctx.global.get_property("usp_get_all_a")?.as_str()?
        );
        assert_eq!(
            r#"["d"]"#,
            ctx.global.get_property("usp_get_all_c")?.as_str()?
        );
        assert_eq!(r#"[]"#, ctx.global.get_property("usp_get_all_e")?.as_str()?);
        assert_eq!(
            r#"["b","e"]"#,
            ctx.global.get_property("usp_get_all_a_2")?.as_str()?
        );
        assert_eq!(
            r#"["b"]"#,
            ctx.global.get_property("usp_get_all_empty")?.as_str()?
        );
        assert_eq!(
            r#"["","e"]"#,
            ctx.global.get_property("usp_get_all_a_3")?.as_str()?
        );

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-getall.any.js#L14
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=1&a=2&a=3&a');
            var usp_get_a = params.get('a');
            var matches = params.getAll('a');
            var usp_matches_1 = matches && matches.length == 4;
            var usp_matches_2 = JSON.stringify(matches);
            params.set('a', 'one');
            var usp_get_a_2 = params.get('a');
            var matches = params.getAll('a');
            var usp_matches_3 = matches && matches.length == 1;
            var usp_matches_4 = JSON.stringify(matches);
        "#,
        )?;

        assert_eq!(r#"1"#, ctx.global.get_property("usp_get_a")?.as_str()?);
        assert_eq!(
            r#"true"#,
            ctx.global.get_property("usp_matches_1")?.as_str()?
        );
        assert_eq!(
            r#"["1","2","3",""]"#,
            ctx.global.get_property("usp_matches_2")?.as_str()?
        );
        assert_eq!(r#"one"#, ctx.global.get_property("usp_get_a_2")?.as_str()?);
        assert_eq!(
            r#"true"#,
            ctx.global.get_property("usp_matches_3")?.as_str()?
        );
        assert_eq!(
            r#"["one"]"#,
            ctx.global.get_property("usp_matches_4")?.as_str()?
        );

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-has.any.js
    #[test]
    fn test_url_search_params_has() -> Result<()> {
        let mut ctx = Context::new();

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-has.any.js#L1
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=b&c=d');
            var usp_has_a = params.has('a');
            var usp_has_c = params.has('c');
            var usp_has_e = params.has('e');
            params = new URLSearchParams('a=b&c=d&a=e');
            var usp_has_a_1 = params.has('a');
            params = new URLSearchParams('=b&c=d');
            var usp_has_empty = params.has('');
            params = new URLSearchParams('null=a');
            var usp_has_null = params.has('null');
        "#,
        )?;

        assert_eq!(r#"true"#, ctx.global.get_property("usp_has_a")?.as_str()?);
        assert_eq!(r#"true"#, ctx.global.get_property("usp_has_c")?.as_str()?);
        assert_eq!(r#"false"#, ctx.global.get_property("usp_has_e")?.as_str()?);
        assert_eq!(r#"true"#, ctx.global.get_property("usp_has_a_1")?.as_str()?);
        assert_eq!(
            r#"true"#,
            ctx.global.get_property("usp_has_empty")?.as_str()?
        );
        assert_eq!(
            r#"true"#,
            ctx.global.get_property("usp_has_null")?.as_str()?
        );

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-has.any.js#L14
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=b&c=d&&');
            params.append('first', 1);
            params.append('first', 2);
            var usp_has_a = params.has('a');
            var usp_has_c = params.has('c');
            var usp_has_first = params.has('first');
            var usp_has_d = params.has('d');
            params.delete('first');
            var usp_has_first_2 = params.has('first');
        "#,
        )?;

        assert_eq!(r#"true"#, ctx.global.get_property("usp_has_a")?.as_str()?);
        assert_eq!(r#"true"#, ctx.global.get_property("usp_has_c")?.as_str()?);
        assert_eq!(
            r#"true"#,
            ctx.global.get_property("usp_has_first")?.as_str()?
        );
        assert_eq!(r#"false"#, ctx.global.get_property("usp_has_d")?.as_str()?);
        assert_eq!(
            r#"false"#,
            ctx.global.get_property("usp_has_first_2")?.as_str()?
        );

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-set.any.js
    #[test]
    fn test_url_search_params_set() -> Result<()> {
        let mut ctx = Context::new();

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-set.any.js#L1
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=b&c=d');
            params.set('a', 'B');
            var usp_set_a = params + '';
            params = new URLSearchParams('a=b&c=d&a=e');
            params.set('a', 'B');
            var usp_set_a_1 = params + '';
            params.set('e', 'f');
            var usp_set_e = params + '';
        "#,
        )?;

        assert_eq!(
            r#"a=B&c=d"#,
            ctx.global.get_property("usp_set_a")?.as_str()?
        );
        assert_eq!(
            r#"a=B&c=d"#,
            ctx.global.get_property("usp_set_a_1")?.as_str()?
        );
        assert_eq!(
            r#"a=B&c=d&e=f"#,
            ctx.global.get_property("usp_set_e")?.as_str()?
        );

        // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-set.any.js#L12
        ctx.eval(
            r#"
            var params = new URLSearchParams('a=1&a=2&a=3');
            var usp_has_a = params.has('a');
            var usp_get_a = params.get('a');
            params.set('first', 4);
            var usp_has_a_1 = params.has('a');
            var usp_get_a_1 = params.get('a');
            params.set('a', 4);
            var usp_has_a_2 = params.has('a');
            var usp_get_a_2 = params.get('a');
        "#,
        )?;

        assert_eq!(r#"true"#, ctx.global.get_property("usp_has_a")?.as_str()?);
        assert_eq!(r#"1"#, ctx.global.get_property("usp_get_a")?.as_str()?);
        assert_eq!(r#"true"#, ctx.global.get_property("usp_has_a_1")?.as_str()?);
        assert_eq!(r#"1"#, ctx.global.get_property("usp_get_a_1")?.as_str()?);
        assert_eq!(r#"true"#, ctx.global.get_property("usp_has_a_2")?.as_str()?);
        assert_eq!(r#"4"#, ctx.global.get_property("usp_get_a_2")?.as_str()?);

        Ok(())
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-sort.any.js
    // TODO: Test URLSearchParams.sort()

    // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/urlsearchparams-stringifier.any.js
    // TODO: Test URLSearchParams stringifier
}
