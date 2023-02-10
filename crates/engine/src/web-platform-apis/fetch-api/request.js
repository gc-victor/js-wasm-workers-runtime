import {
    arrayBuffer,
    blob,
    formData,
    json,
    text,
} from "./request-response-methods.js";

const ___request = Symbol();

/*
 * Request
 *
 * The Request interface of the Fetch API represents a resource request.
 *
 * @see: https://developer.mozilla.org/en-US/docs/Web/API/Request
 * @see: https://fetch.spec.whatwg.org/#request-class
 * @see: https://github.com/github/fetch/blob/fb5b0cf42b470faf8c5448ab461d561f34380a30/fetch.js#L339
 */
class Request {
    constructor(input, init) {
        const textEncoder = new TextEncoder();

        const self = (this[___request] = {});
        const isRequest = input instanceof Request;
        const options = isRequest ? input : init || {};
        const body = options.body;

        if (/CONNECT|TRACE|TRACK/i.test(options.method)) {
            throw new TypeError(
                `Failed to construct 'Request': '${options.method}' HTTP method is unsupported`,
            );
        }

        if ((!options.method || /GET|HEAD/i.test(options.method)) && body) {
            throw new TypeError("Body not allowed for GET or HEAD requests");
        }

        self.body =
            typeof body === "string" ? textEncoder.encode(body) : body || null;

        self.cache = options?.cache || "default";
        self.credentials = options?.credentials || "same-origin";
        self.destination = "worker";
        self.integrity = options?.integrity || "";
        self.keepalive = !!options?.keepalive;
        self.method = normalizeMethod(options.method || "GET");
        self.mode = options?.mode || "cors";
        self.redirect = options?.redirect || "follow";
        self.referrer = options?.referrer || "";
        self.referrerPolicy = options?.referrerPolicy || "";
        self.signal = options?.signal || null;
        self.url = isRequest ? input.url.toString() : input.toString();

        if (options.headers) {
            const headers = new Headers(
                isRequest && !init?.headers
                    ? options.headers.getAll()
                    : init.headers,
            );

            if (!headers.get("content-type") && self.body) {
                const types = {
                    "[object Object]": () => {
                        // TODO: add FormData support with boundary creation
                        if (body instanceof Blob) {
                            return body.type;
                        } else if (body instanceof URLSearchParams) {
                            return "application/x-www-form-urlencoded;charset=UTF-8";
                        } else {
                            return null;
                        }
                    },
                    "[object String]": () => "text/plain;charset=UTF-8",
                };
                const type = types[Object.prototype.toString.call(body)];

                headers.set("content-type", type ? type() : null);
            }

            self.headers = headers;
        }

        if (self.method === "GET" || self.method === "HEAD") {
            if (options.cache === "no-store" || options.cache === "no-cache") {
                // Search for a '_' parameter in the query string
                const reParamSearch = /([?&])_=[^&]*/;
                if (reParamSearch.test(self.url)) {
                    // If it already exists then set the value with the current time
                    self.url = self.url.replace(
                        reParamSearch,
                        `$1_=${new Date().getTime()}`,
                    );
                } else {
                    // Otherwise add a new '_' parameter to the end with the current time
                    const reQueryString = /\?/;
                    self.url += `${
                        reQueryString.test(self.url) ? "&" : "?"
                    }_=${new Date().getTime()}`;
                }
            }
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/body
    get body() {
        if (this[___request].body === null) return null;
        if (this[___request].body instanceof ReadableStream)
            return this[___request].body;

        const stream = new TransformStream();
        const writer = stream.writable.getWriter();

        writer.write(this[___request].body);
        writer.close();

        return stream.readable;
    }
    // readonly
    set body(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/bodyUsed
    get bodyUsed() {
        return this[___request].bodyUsed;
    }
    // readonly
    set bodyUsed(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/cache
    get cache() {
        return this[___request].cache;
    }
    // readonly
    set cache(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/credentials
    get credentials() {
        return this[___request].credentials;
    }
    // readonly
    set credentials(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/destination
    get destination() {
        return this[___request].destination;
    }
    // readonly
    set destination(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/headers
    get headers() {
        return this[___request].headers;
    }
    // readonly
    set headers(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/integrity
    get integrity() {
        return this[___request].integrity;
    }
    // readonly
    set integrity(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#keepalive
    get keepalive() {
        return this[___request].keepalive;
    }
    // readonly
    set keepalive(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/method
    get method() {
        return this[___request].method;
    }
    // readonly
    set method(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/mode
    get mode() {
        return this[___request].mode;
    }
    // readonly
    set mode(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/redirect
    get redirect() {
        return this[___request].redirect;
    }
    // readonly
    set redirect(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/referrer
    get referrer() {
        return this[___request].referrer;
    }
    // readonly
    set referrer(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/referrerPolicy
    get referrerPolicy() {
        return this[___request].referrerPolicy;
    }
    // readonly
    set referrerPolicy(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#signal
    get signal() {
        return this[___request].signal || new AbortSignal();
    }
    // readonly
    set signal(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/url
    get url() {
        return this[___request].url;
    }
    // readonly
    set url(_) {}

    clone() {
        if (this[___request].bodyUsed) {
            throw new TypeError(
                "Failed to execute 'clone' on 'Request': Request body is already used",
            );
        }

        return new Request(this[___request].url, {
            body: this[___request].body,
            cache: this[___request].cache,
            credentials: this[___request].credentials,
            headers: this[___request].headers,
            integrity: this[___request].integrity,
            keepalive: this[___request].keepalive,
            method: this[___request].method,
            mode: this[___request].mode,
            redirect: this[___request].redirect,
            referrer: this[___request].referrer,
            referrerPolicy: this[___request].referrerPolicy,
            signal: this[___request].signal,
        });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/arrayBuffer
    async arrayBuffer() {
        return await arrayBuffer(this, ___request);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/blob
    async blob() {
        return await blob(this, ___request);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/formData
    async formData() {
        return await formData(this, ___request);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/json
    async json() {
        return await json(this);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/text
    async text() {
        return await text(this, ___request);
    }
}

globalThis.Request = Request;

function normalizeMethod(method) {
    var methods = ["DELETE", "GET", "HEAD", "OPTIONS", "POST", "PUT"];
    var upcased = method.toUpperCase();

    return methods.indexOf(upcased) > -1 ? upcased : method;
}
