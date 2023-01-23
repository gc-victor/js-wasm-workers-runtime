import {
    arrayBuffer,
    blob,
    formData,
    hasNullBody,
    json,
    statusTextList,
    text,
} from "./utils.js";

// TODO: add documentation
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
        this.statusText = options.statusText || statusTextList[this.status];
        this.ok = this.status >= 200 && this.status < 300;

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
            statusText: statusTextList[statusCode],
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
        return await arrayBuffer(this);
    }

    async blob() {
        return await blob(this);
    }

    async formData() {
        return await formData(this);
    }

    async json() {
        return await json(this);
    }

    async text() {
        return await text(this);
    }
}

globalThis.Response = Response;
