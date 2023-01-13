import {
    arrayBuffer,
    blob,
    formData,
    hasNullBody,
    json,
    text,
} from "./utils.js";

// TODO: add documentation
// TODO: add statusText for each status code
// @see: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
class Response {
    constructor(body, options = {}) {
        this.body = hasNullBody(options.status) ? null : body;
        this.body =
            typeof this.body === "string"
                ? globalThis.___textEncoder.encode(this.body).buffer
                : this.body;
        this.bodyUsed = false;
        this.status = options.status || 200;
        this.statusText = options.statusText || codes[this.status];

        const headers = new Headers(options.headers || {});
        this.headers = headers.getAll();

        if (body instanceof URLSearchParams) {
            this.headers.set(
                "content-type",
                "application/x-www-form-urlencoded;charset=UTF-8",
            );
        }
    }

    static redirect(url, statusCode = 307) {
        const redirectStatuses = [301, 302, 303, 307, 308];

        if (redirectStatuses.indexOf(statusCode) === -1) {
            throw new TypeError("The status code must be between 301 and 308.");
        }

        return new Response(`Redirecting to ${url}`, {
            status: statusCode,
            statusText: codes[statusCode],
            headers: {
                Location: url,
            },
        });
    }

    clone() {
        const headers = new Headers(this.headers);

        return new Response(this._bodyInit, {
            status: this.status,
            statusText: this.statusText,
            headers: headers.getAll(),
            url: this.url,
        });
    }

    error() {
        const response = new Response(null, { status: 0, statusText: "" });
        response.type = "error";
        return response;
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

const codes = {
    100: "Continue",
    101: "Switching Protocols",
    102: "Processing",
    103: "Early Hints",
    110: "Response is Stale",
    111: "Revalidation Failed",
    112: "Disconnected Operation",
    113: "Heuristic Expiration",
    199: "Miscellaneous Warning",
    200: "OK",
    201: "Created",
    202: "Accepted",
    203: "Non-Authoritative Information",
    204: "No Content",
    205: "Reset Content",
    206: "Partial Content",
    207: "Multi-Status",
    208: "Already Reported",
    214: "Transformation Applied",
    226: "IM Used",
    299: "Miscellaneous Persistent Warning",
    300: "Multiple Choices",
    301: "Moved Permanently",
    302: "Found",
    303: "See Other",
    304: "Not Modified",
    305: "Use Proxy",
    306: "Switch Proxy",
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
    505: "HTTP Version Not Supported",
    506: "Variant Also Negotiates",
    507: "Insufficient Storage",
    508: "Loop Detected",
    509: "Bandwidth Limit Exceeded",
    510: "Not Extended",
    511: "Network Authentication Required",
};

globalThis.Response = Response;
