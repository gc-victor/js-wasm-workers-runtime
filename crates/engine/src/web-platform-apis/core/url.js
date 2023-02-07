const ___state = Symbol();

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

        update(this, properties);

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
        update(
            this,
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

        update(this, this[___state].setHost(this[___state].href, value));

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
        update(
            this,
            this[___state].setHost(this[___state].href, String(value)),
        );
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/href
    get href() {
        return this[___state].href;
    }

    set href(value) {
        update(this, ___parseUrl(String(value), "about:blank"));
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
        update(
            this,
            this[___state].setPassword(this[___state].href, String(value)),
        );
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname
    get pathname() {
        return this[___state].pathname;
    }

    set pathname(value) {
        update(
            this,
            this[___state].setPathname(this[___state].href, String(value)),
        );
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/port
    get port() {
        return this[___state].port;
    }

    set port(value) {
        update(
            this,
            this[___state].setPort(this[___state].href, String(value)),
        );
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/search
    get search() {
        let search = this[___state].search;
        return search && !/^\?/.test(search) ? `?${search}` : search;
    }

    set search(value) {
        update(
            this,
            this[___state].setSearch(this[___state].href, String(value)),
        );

        this[___state].searchParams = new URLSearchParams(this[___state].search);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/protocol
    get protocol() {
        const protocol = this[___state].protocol;
        return protocol ? `${protocol}:` : "";
    }

    // @see: https://github.com/lifaon74/url-polyfill/blob/341221207263c9214e794fc3eaf221a71c596d29/do-not-use/url.js#L250
    set protocol(value) {
        update(
            this,
            this[___state].setProtocol(this[___state].href, String(value)),
        );
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/searchParams
    get searchParams() {
        // TODO: implement searchParams
        return this[___state].searchParams;
    }

    // It is read only
    set searchParams(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URL/username
    get username() {
        return this[___state].username;
    }

    set username(value) {
        update(
            this,
            this[___state].setUsername(this[___state].href, String(value)),
        );
    }
}

globalThis.URL = URL;

function update(self, properties) {
    self[___state].hash = properties.hash;
    self[___state].host = properties.host;
    self[___state].hostname = properties.hostname;
    self[___state].href = properties.href;
    self[___state].origin = properties.origin;
    self[___state].password = properties.password;
    self[___state].pathname = properties.pathname;
    self[___state].port = properties.port;
    self[___state].protocol = properties.protocol;
    self[___state].search = properties.search;
    self[___state].searchParams = new URLSearchParams(properties.search);
    self[___state].username = properties.username;
}
