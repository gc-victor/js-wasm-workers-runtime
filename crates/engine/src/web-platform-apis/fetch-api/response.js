import {
    arrayBuffer,
    blob,
    formData,
    json,
    statusTextList,
    text,
} from "./request-response-methods.js";

// @see: https://developer.mozilla.org/en-US/docs/Web/API/Response
// @see: https://fetch.spec.whatwg.org/#response-class
// @see: https://github.com/github/fetch/blob/fb5b0cf42b470faf8c5448ab461d561f34380a30/fetch.js#L448
class Response {
    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/Response
    constructor(body, options = {}) {
        const textEncoder = new TextEncoder();
        const headers = new Headers(options.headers || {});
        const status = options.status || 200;

        if (body) {
            const types = {
                "[object Blob]": () => body.type,
                "[object FormData]": () => "multipart/form-data",
                "[object URLSearchParams]": () =>
                    "application/x-www-form-urlencoded;charset=UTF-8",
                "[object Undefined]": () => null,
                "[object Null]": () => null,
            };
            const type = types[Object.prototype.toString.call(body)];
        
            headers.set("content-type", type ? type() : "text/plain;charset=UTF-8");
        }

        // https://fetch.spec.whatwg.org/#null-body-status
        this.body = [101, 103, 204, 205, 304].includes(options.status)
            ? null
            : body;
        this.body =
            typeof this.body === "string"
                ? textEncoder.encode(this.body).buffer
                : this.body;
        this.bodyUsed = false;
        this.headers = headers.getAll();
        this.ok = status >= 200 && status < 300;
        this.status = status;
        this.statusText = options.statusText || statusTextList[this.status];
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect
    static redirect(url, statusCode = 307) {
        if ([301, 302, 303, 307, 308].indexOf(statusCode) === -1) {
            throw new TypeError("The status code must be between 301 and 308.");
        }

        return new Response(`Redirecting to ${url}`, {
            status: statusCode,
            statusText: statusTextList[statusCode],
            headers: {
                Location: url,
            },
        });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/clone
    clone() {
        const headers = new Headers(this.headers);

        return new Response(this._bodyInit, {
            status: this.status,
            statusText: this.statusText,
            headers: headers.getAll(),
            url: this.url,
        });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/error
    error() {
        const response = new Response(null, { status: 0, statusText: "" });
        response.type = "error";
        return response;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/arrayBuffer
    async arrayBuffer() {
        return await arrayBuffer(this);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/blob
    async blob() {
        return await blob(this);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/formData
    async formData() {
        return await formData(this);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/json
    async json() {
        return await json(this);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/text
    async text() {
        return await text(this);
    }
}

globalThis.Response = Response;
