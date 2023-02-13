import { ___RequestResponse, statusTextList } from "./request-response.js";

const ___response = Symbol();

/**
 * Response
 *
 * The Response interface of the Fetch API represents the response to a request.
 *
 * @see: https://developer.mozilla.org/en-US/docs/Web/API/Response
 * @see: https://fetch.spec.whatwg.org/#response-class
 * @see: https://github.com/github/fetch/blob/fb5b0cf42b470faf8c5448ab461d561f34380a30/fetch.js#L448
 */
class Response extends ___RequestResponse {
    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/Response
    constructor(body, init = {}) {
        super(___response);

        this[___response] = {};

        const headers = new Headers(init.headers || {});
        const status = init.status !== undefined ? init.status : 200;
        const location = headers.get("location");

        if (body && !headers.has("content-type")) {
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

        // https://fetch.spec.whatwg.org/#null-body-status
        body = [101, 103, 204, 205, 304].includes(init.status) ? null : body;
        body =
            typeof body === "string"
                ? new TextEncoder().encode(body)
                : body || null;

        this[___response].body = body;
        this[___response].bodyUsed = false;
        this[___response].headers = headers;
        this[___response].ok = status >= 200 && status < 300;
        this[___response].redirected = !!location;
        this[___response].status = status;
        this[___response].statusText =
            init.statusText === undefined
                ? statusTextList[this.status]
                : init.statusText;
        this[___response].type = "basic";
        this[___response].url = location || "";
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/error
    static error() {
        const response = new Response(null, { status: 0, statusText: "" });

        response[___response].type = "error";

        return response;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect
    // @see: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Location
    // @see: https://fetch.spec.whatwg.org/#redirect-status
    static redirect(url, statusCode = 307) {
        if ([301, 302, 303, 307, 308].indexOf(statusCode) === -1) {
            throw new TypeError("The status code must be between 301 and 308.");
        }

        return new Response(`Redirecting to ${url}`, {
            status: statusCode,
            statusText: statusTextList[statusCode],
            headers: {
                Location: new URL(url).toString(),
            },
        });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/body
    // The getter body is on the parent class

    get bodyUsed() {
        return this[___response].bodyUsed;
    }
    // read-only
    set bodyUsed(_) {}

    get headers() {
        return this[___response].headers;
    }
    // read-only
    set headers(_) {}

    get ok() {
        return this[___response].ok;
    }
    // read-only
    set ok(_) {}

    get redirected() {
        return this[___response].redirected;
    }
    // read-only
    set redirected(_) {}

    get status() {
        return this[___response].status;
    }
    // read-only
    set status(_) {}

    get statusText() {
        return this[___response].statusText;
    }
    // read-only
    set statusText(_) {}

    get type() {
        return this[___response].type;
    }
    // read-only
    set type(_) {}

    get utl() {
        return this[___response].utl;
    }
    // read-only
    set utl(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/clone
    clone() {
        if (this[___response].bodyUsed) {
            throw new TypeError(
                "Failed to execute 'clone' on 'Response': Response body is already use",
            );
        }

        return new Response(this[___response].body, {
            headers: this[___response].headers,
            status: this[___response].status,
            statusText: this[___response].statusText,
            url: this[___response].url,
        });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/formData
    // async formData() {
    //     return await formData(this, ___response);
    // }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/json
    // async json() {
    //     return await json(this);
    // }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/text
    // async text() {
    //     return await text(this, ___response);
    // }
}

globalThis.Response = Response;
