const ___state = Symbol();
const ___update = Symbol();

/**
 * URL
 *
 * The URL interface is used to parse, construct, normalize, and encode URLs. It works by providing properties which allow you to easily read and modify the components of a URL.
 *
 * @see: https://developer.mozilla.org/en-US/docs/Web/API/URL
 * @see: https://url.spec.whatwg.org/
 */
class URL {
    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/URL
    constructor(url, base) {
        if (!url) {
            throw new TypeError(
                "Failed to construct 'URL': 1 argument required, but only 0 present",
            );
        }

        if (typeof url !== "string") url = String(url);
        if (base && typeof base !== "string") base = String(base);

        url = url.replace(/[\t\n\r]/g, "").trim();

        let properties = ___parseUrl(url, base || "about:blank");

        this[___state] = {};

        this[___update](properties);

        this[___state].setHash = properties.setHash;
        this[___state].setHost = properties.setHost;
        this[___state].setPassword = properties.setPassword;
        this[___state].setPathname = properties.setPathname;
        this[___state].setPort = properties.setPort;
        this[___state].setProtocol = properties.setProtocol;
        this[___state].setSearch = properties.setSearch;
        this[___state].setUsername = properties.setUsername;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/toString
    toString() {
        return this[___state].href;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/toJSON
    toJSON() {
        return this[___state].href;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/hash
    get hash() {
        return this[___state].hash;
    }

    set hash(value) {
        this[___update](
            this[___state].setHash(this[___state].href, String(value)),
        );
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/host
    get host() {
        const port = this[___state].port;
        return this[___state].host + (port ? `:${port}` : "");
    }

    set host(value) {
        value = String(value);

        this[___update](this[___state].setHost(this[___state].href, value));

        if (/:/.test(value)) {
            const parts = value.split(":");
            this[___state].port = parts.length === 2 ? parts[1] : "";
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/hostname
    get hostname() {
        return this[___state].hostname || this[___state].host;
    }

    set hostname(value) {
        this[___update](
            this[___state].setHost(this[___state].href, String(value)),
        );
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/href
    get href() {
        return this[___state].href;
    }

    set href(value) {
        this[___update](___parseUrl(String(value), "about:blank"));
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/origin
    get origin() {
        return this[___state].origin;
    }

    // It is read only
    set origin(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/password
    get password() {
        return this[___state].password;
    }

    set password(value) {
        this[___update](
            this[___state].setPassword(this[___state].href, String(value)),
        );
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname
    get pathname() {
        return this[___state].pathname;
    }

    set pathname(value) {
        this[___update](
            this[___state].setPathname(this[___state].href, String(value)),
        );
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/port
    get port() {
        return this[___state].port;
    }

    set port(value) {
        this[___update](
            this[___state].setPort(this[___state].href, String(value)),
        );
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/search
    get search() {
        let search = this[___state].search;
        return search && !/^\?/.test(search) ? `?${search}` : search;
    }

    set search(value) {
        const params = this[___state].setSearch(
            this[___state].href,
            String(value),
        );

        this[___update]({ ...params, href: params.href.replace(/\?$/, "") });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/protocol
    get protocol() {
        const protocol = this[___state].protocol;
        return protocol ? `${protocol}:` : "";
    }

    // @see: https://github.com/lifaon74/url-polyfill/blob/341221207263c9214e794fc3eaf221a71c596d29/do-not-use/url.js#L250
    set protocol(value) {
        this[___update](
            this[___state].setProtocol(this[___state].href, String(value)),
        );
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/searchParams
    get searchParams() {
        const self = this;
        const searchParams = self[___state].searchParams;

        return {
            append(name, value) {
                searchParams.append(name, value);
                self.search = searchParams.toString();
            },
            delete(name) {
                searchParams.delete(name);
                self.search = searchParams.toString();
            },
            entries() {
                return searchParams.entries();
            },
            forEach() {
                return searchParams.forEach();
            },
            get(name) {
                return searchParams.get(name);
            },
            getAll(name) {
                return searchParams.getAll(name);
            },
            has(name) {
                return searchParams.has(name);
            },
            keys() {
                return searchParams.keys();
            },
            set(name, value) {
                searchParams.set(name, value);
                self.search = searchParams.toString();
            },
            sort() {
                searchParams.sort();
                self.search = searchParams.toString();
            },
            toString() {
                return searchParams.toString();
            },
            values() {
                return searchParams.values();
            },
            [Symbol.iterator]() {
                return searchParams.entries();
            },
        };
    }

    // It is read only
    set searchParams(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/username
    get username() {
        return this[___state].username;
    }

    set username(value) {
        this[___update](
            this[___state].setUsername(this[___state].href, String(value)),
        );
    }

    [___update](properties) {
        this[___state].hash = properties.hash;
        this[___state].host = properties.host;
        this[___state].hostname = properties.hostname;
        this[___state].href = properties.href;
        this[___state].origin = properties.origin;
        this[___state].password = properties.password;
        this[___state].pathname = properties.pathname;
        this[___state].port = properties.port;
        this[___state].protocol = properties.protocol;
        this[___state].search = properties.search;
        this[___state].searchParams = new URLSearchParams(properties.search);
        this[___state].username = properties.username;
    }
}

globalThis.URL = URL;
