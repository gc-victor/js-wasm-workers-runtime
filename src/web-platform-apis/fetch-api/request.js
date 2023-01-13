import { arrayBuffer, blob, formData, json, text } from "./utils.js";

/*
 * Request
 *
 * The Request interface of the Fetch API represents a resource request.
 *
 * @see: https://developer.mozilla.org/en-US/docs/Web/API/Request
 * @see: https://developers.cloudflare.com/workers/runtime-apis/request/
 * @see: https://fetch.spec.whatwg.org/#request-class
 * @see: https://github.com/github/fetch/blob/fb5b0cf42b470faf8c5448ab461d561f34380a30/fetch.js#L339
 */
class Request {
    constructor(input, options) {
        if (!(this instanceof Request)) {
            throw new TypeError(
                'Please use the "new" operator, this DOM object constructor cannot be called as a function.',
            );
        }

        options = options || {};

        let body = options.body;

        this.cache = options?.cache || "default";
        this.destination = "worker";
        this.integrity = options?.integrity || "";
        this.keepalive = options?.keepalive;
        this.mode = options?.mode || "cors";
        this.redirect = options?.redirect || "follow";
        this.referrer = options?.referrer || "";
        this.referrerPolicy = options?.referrerPolicy || "";

        if (input instanceof Request) {
            if (input.bodyUsed) {
                throw new TypeError("Already read");
            }

            this.url = input.url;
            this.credentials = input.credentials;

            if (!options.headers) {
                const headers = new Headers(input.headers);
                this.headers = headers.getAll();
            }

            this.method = input.method;
            this.mode = input.mode;
            this.signal = input.signal;

            if (!body && input._bodyInit != null) {
                body = input._bodyInit;
                input.bodyUsed = true;
            }
        } else {
            this.url = String(input);
        }

        this.credentials =
            options.credentials || this.credentials || "same-origin";

        if (options.headers || !this.headers) {
            const headers = new Headers(options.headers);
            this.headers = headers.getAll();
        }

        this.method = normalizeMethod(options.method || this.method || "GET");
        this.signal =
            options.signal ||
            this.signal ||
            (() => {
                // TODO: implement AbortController
                if ("AbortController" in globalThis) {
                    const ctrl = new AbortController();
                    return ctrl.signal;
                }
            })();

        if ((this.method === "GET" || this.method === "HEAD") && body) {
            throw new TypeError("Body not allowed for GET or HEAD requests");
        }

        if (this.method === "GET" || this.method === "HEAD") {
            if (options.cache === "no-store" || options.cache === "no-cache") {
                // Search for a '_' parameter in the query string
                const reParamSearch = /([?&])_=[^&]*/;
                if (reParamSearch.test(this.url)) {
                    // If it already exists then set the value with the current time
                    this.url = this.url.replace(
                        reParamSearch,
                        `$1_=${new Date().getTime()}`,
                    );
                } else {
                    // Otherwise add a new '_' parameter to the end with the current time
                    const reQueryString = /\?/;
                    this.url += `${
                        reQueryString.test(this.url) ? "&" : "?"
                    }_=${new Date().getTime()}`;
                }
            }
        }
    }

    clone() {
        return new Request(this, { body: this._bodyInit });
    }

    async arrayBuffer() {
        return arrayBuffer(this);
    }

    async blob() {
        return blob(this);
    }

    async formData() {
        return formData(this);
    }

    async json() {
        return json(this);
    }

    async text() {
        return text(this);
    }
}

globalThis.Request = Request;

// HTTP methods whose capitalization should be normalized
var methods = ["DELETE", "GET", "HEAD", "OPTIONS", "POST", "PUT"];

function normalizeMethod(method) {
    var upcased = method.toUpperCase();

    return methods.indexOf(upcased) > -1 ? upcased : method;
}
