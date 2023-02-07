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

    // static URL_TEST_DATA: &str = include_str!("url-test-data.json");

    // #[derive(Serialize, Deserialize, Debug)]
    // struct URLTest {
    //     base: String,
    //     input: String,
    //     href: Option<String>,
    //     origin: Option<String>,
    //     protocol: Option<String>,
    //     username: Option<String>,
    //     password: Option<String>,
    //     host: Option<String>,
    //     hostname: Option<String>,
    //     port: Option<String>,
    //     pathname: Option<String>,
    //     search: Option<String>,
    //     hash: Option<String>,
    //     failure: Option<bool>,
    // }

    // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/url-constructor.any.js
    // #[test]
    // fn test_url_constructor() -> Result<()> {
    //     let mut ctx = Context::new();

    //     let tests_data: Vec<URLTest> = serde_json::from_str(URL_TEST_DATA)?;

    //     for expected in tests_data.iter() {
    //         if expected.failure.is_some() {
    //             // TODO: Test failure cases
    //             continue;
    //         }

    //         if !expected.input.is_empty() {
    //             ctx.eval(&format!(
    //                 r#"
    //                 var expected = JSON.parse({:?});
    //                 var url = new URL(expected.input, expected.base || "about:blank");
    //                 var href = [url.href === expected.href, url.href, expected.href];
    //                 var protocol = [url.protocol === expected.protocol, url.protocol, expected.protocol];
    //                 var username = [url.username === expected.username, url.username, expected.username];
    //                 var password = [url.password === expected.password, url.password, expected.password];
    //                 var host = [url.host === expected.host, url.host, expected.host];
    //                 var hostname = [url.hostname === expected.hostname, url.hostname, expected.hostname];
    //                 var port = [url.port === expected.port, url.port, expected.port];
    //                 var pathname = [url.pathname === expected.pathname, url.pathname, expected.pathname];
    //                 var search = [url.search === expected.search, url.search, expected.search];
    //                 var searchParams = [!expected.searchParams || url.searchParams === expected.searchParams, url.searchParams, expected.searchParams];

    //                 if (href[0] === false || protocol[0] === false || username[0] === false || password[0] === false || host[0] === false || hostname[0] === false || port[0] === false || pathname[0] === false || search[0] === false || searchParams[0] === false) {{
    //                     ___logger("input", expected.input, "base", expected.base);

    //                     (href[0] === false && ___logger(JSON.stringify({{href}})));
    //                     (protocol[0] === false && ___logger(JSON.stringify({{protocol}})));
    //                     (username[0] === false && ___logger(JSON.stringify({{username}})));
    //                     (password[0] === false && ___logger(JSON.stringify({{password}})));
    //                     (host[0] === false && ___logger(JSON.stringify({{host}})));
    //                     (hostname[0] === false && ___logger(JSON.stringify({{hostname}})));
    //                     (port[0] === false && ___logger(JSON.stringify({{port}})));
    //                     (search[0] === false && ___logger(JSON.stringify({{search}})));
    //                     (searchParams[0] === false && ___logger(JSON.stringify({{searchParams}})));
    //                 }}
    //             "#,
    //                 json!(expected).to_string()
    //             ))?;
    //         }
    //     }

    //     Ok(())
    // }
}
