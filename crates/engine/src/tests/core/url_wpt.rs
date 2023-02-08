// @see: https://github.com/web-platform-tests/wpt/tree/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url
#[cfg(test)]
mod tests {
    use anyhow::{Ok, Result};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::tests::test_utils::context::Context;

    static URL_TEST_DATA: &str = include_str!("url-test-data.json");

    #[derive(Serialize, Deserialize, Debug)]
    struct URLTest {
        base: String,
        input: String,
        href: Option<String>,
        origin: Option<String>,
        protocol: Option<String>,
        username: Option<String>,
        password: Option<String>,
        host: Option<String>,
        hostname: Option<String>,
        port: Option<String>,
        pathname: Option<String>,
        search: Option<String>,
        hash: Option<String>,
        failure: Option<bool>,
    }

    // @see: https://github.com/web-platform-tests/wpt/blob/7b0ebaccc62b566a1965396e5be7bb2bc06f841f/url/url-constructor.any.js
    #[test]
    fn test_url_wpt_constructor() -> Result<()> {
        let mut ctx = Context::new();

        // NOTE: the file has been edited
        let tests_data: Vec<URLTest> = serde_json::from_str(URL_TEST_DATA)?;

        for expected in tests_data.iter() {
            if expected.failure.is_some() {
                // TODO: Test failure cases
                continue;
            }

            if !expected.input.is_empty() {
                ctx.eval(&format!(
                    r#"
                    var expected = JSON.parse({:?});

                    var url = new URL(expected.input, expected.base || "about:blank");
                    var href = [url.href === expected.href, url.href, expected.href];
                    var protocol = [url.protocol === expected.protocol, url.protocol, expected.protocol];
                    var username = [url.username === expected.username, url.username, expected.username];
                    var password = [url.password === expected.password, url.password, expected.password];
                    var host = [url.host === expected.host, url.host, expected.host];
                    var hostname = [url.hostname === expected.hostname, url.hostname, expected.hostname];
                    var port = [url.port === expected.port, url.port, expected.port];
                    var pathname = [url.pathname === expected.pathname, url.pathname, expected.pathname];
                    var search = [url.search === expected.search, url.search, expected.search];
                    var searchParams = [!expected.searchParams || url.searchParams === expected.searchParams, url.searchParams, expected.searchParams];

                    var has_error = false;

                    if (href[0] === false || protocol[0] === false || username[0] === false || password[0] === false || host[0] === false || hostname[0] === false || port[0] === false || pathname[0] === false || search[0] === false || searchParams[0] === false) {{
                        ___logger("input", expected.input, "base", expected.base);

                        has_error = true;

                        (href[0] === false && ___logger(JSON.stringify({{href}})));
                        (protocol[0] === false && ___logger(JSON.stringify({{protocol}})));
                        (username[0] === false && ___logger(JSON.stringify({{username}})));
                        (password[0] === false && ___logger(JSON.stringify({{password}})));
                        (host[0] === false && ___logger(JSON.stringify({{host}})));
                        (hostname[0] === false && ___logger(JSON.stringify({{hostname}})));
                        (port[0] === false && ___logger(JSON.stringify({{port}})));
                        (search[0] === false && ___logger(JSON.stringify({{search}})));
                        (searchParams[0] === false && ___logger(JSON.stringify({{searchParams}})));
                    }}
                "#,
                    json!(expected).to_string()
                ))?;

                assert_eq!("false", ctx.global.get_property("has_error")?.as_str()?,);
            }
        }

        Ok(())
    }
}
