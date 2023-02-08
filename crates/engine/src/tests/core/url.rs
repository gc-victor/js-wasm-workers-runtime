// @see: https://github.com/web-platform-tests/wpt/tree/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url
#[cfg(test)]
mod tests {
    use anyhow::{Ok, Result};

    use crate::tests::test_utils::context::Context;

    // NOTE: This test don't cover most of the cases.
    #[test]
    fn test_url() -> Result<()> {
        let mut ctx = Context::new();

        ctx.eval(
            r#"
            var url = new URL('http:@www.example.com?key_1=value_1');
            
            url.host = '127.0.0.1:8080';

            var host_init = url.host;
            var port_init = url.port;
            var search_init = url.search;
            var searchParams_init = url.searchParams;
            
            url.hash = 'is_a_hash';
            url.hostname = 'test.com';
            url.password = 'is_a_password';
            url.pathname = '/en-US/docs/Web/API/URL/pathname';
            url.port = '2023';
            url.protocol = 'https';
            url.search = 'key=value';
            url.username = 'is_a_username';
            
            var hash = url.hash;
            var host = url.host;
            var hostname = url.hostname;
            var password = url.password;
            var pathname = url.pathname;
            var port = url.port;
            var protocol = url.protocol;
            var search = url.search;
            var searchParams = url.searchParams;
            var username = url.username;

            "#,
        )?;

        assert_eq!(
            "127.0.0.1:8080",
            ctx.global.get_property("host_init")?.as_str()?,
        );
        assert_eq!("8080", ctx.global.get_property("port_init")?.as_str()?,);

        assert_eq!("is_a_hash", ctx.global.get_property("hash")?.as_str()?,);
        assert_eq!("test.com", ctx.global.get_property("hostname")?.as_str()?,);
        assert_eq!("2023", ctx.global.get_property("port")?.as_str()?,);
        assert_eq!("test.com:2023", ctx.global.get_property("host")?.as_str()?,);
        assert_eq!(
            "is_a_password",
            ctx.global.get_property("password")?.as_str()?,
        );
        assert_eq!(
            "/en-US/docs/Web/API/URL/pathname",
            ctx.global.get_property("pathname")?.as_str()?,
        );
        assert_eq!("https:", ctx.global.get_property("protocol")?.as_str()?,);
        assert_eq!("?key=value", ctx.global.get_property("search")?.as_str()?,);
        assert_eq!(
            "key=value",
            ctx.global.get_property("searchParams")?.as_str()?,
        );
        assert_eq!(
            "?key_1=value_1",
            ctx.global.get_property("search_init")?.as_str()?,
        );
        assert_eq!(
            "key_1=value_1",
            ctx.global.get_property("searchParams_init")?.as_str()?,
        );
        assert_eq!(
            "is_a_username",
            ctx.global.get_property("username")?.as_str()?,
        );

        Ok(())
    }
}
