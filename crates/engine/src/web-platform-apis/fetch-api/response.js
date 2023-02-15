import { ___Body } from "./body.js";

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
class Response extends ___Body {
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

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/bodyUsed
    // The getter bodyUsed is on the parent class

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/headers
    get headers() {
        return this[___response].headers;
    }
    // read-only
    set headers(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/ok
    get ok() {
        return this[___response].ok;
    }
    // read-only
    set ok(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/redirected
    get redirected() {
        return this[___response].redirected;
    }
    // read-only
    set redirected(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/status
    get status() {
        return this[___response].status;
    }
    // read-only
    set status(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/statusText
    get statusText() {
        return this[___response].statusText;
    }
    // read-only
    set statusText(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/type
    get type() {
        return this[___response].type;
    }
    // read-only
    set type(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/url
    get url() {
        return this[___response].url;
    }
    // read-only
    set url(_) {}

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
}

globalThis.Response = Response;

// @see: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
const statusTextList = {
    100: "Continue",
    101: "Switching Protocols",
    102: "Processing",
    103: "Early Hints",
    200: "OK",
    201: "Created",
    202: "Accepted",
    203: "Non-Authoritative Information",
    205: "Reset Content",
    206: "Partial Content",
    207: "Multi-Status",
    208: "Already reported",
    226: "IM Used",
    300: "Multiple Choices",
    301: "Moved Permanently",
    302: "Found",
    303: "See Other",
    304: "Not Modified",
    305: "Use Proxy",
    306: "unused",
    307: "Temporary Redirect",
    308: "Permanent Redirect",
    400: "Bad Request",
    401: "Unauthorized",
    402: "Payment Required",
    403: "Forbidden",
    404: "Not Found",
    405: "Method Not Allowed",
    406: "Not Acceptable",
    407: "Proxy Authentication Required",
    408: "Request Timeout",
    409: "Conflict",
    410: "Gone",
    411: "Length Required",
    412: "Precondition Failed",
    413: "Payload Too Large",
    414: "URI Too Long",
    415: "Unsupported Media Type",
    416: "Range Not Satisfiable",
    417: "Expectation Failed",
    418: "I'm a teapot",
    421: "Misdirected Request",
    422: "Unprocessable Entity",
    423: "Locked",
    424: "Failed Dependency",
    425: "Too Early",
    426: "Upgrade Required",
    428: "Precondition Required",
    429: "Too Many Requests",
    431: "Request Header Fields Too Large",
    451: "Unavailable For Legal Reasons",
    500: "Internal Server Error",
    501: "Not Implemented",
    502: "Bad Gateway",
    503: "Service Unavailable",
    504: "Gateway Timeout",
    505: "Http Version Not Supported",
    506: "Variant Also Negotiates",
    507: "Insufficient Storage",
    508: "Loop Detected",
    510: "Not Extended",
    511: "Network Authentication Required",
};
