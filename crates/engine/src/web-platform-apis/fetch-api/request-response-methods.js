export async function arrayBuffer(self) {
    return new Promise((resolve) => resolve(self.body));
}

export async function blob(self) {
    const type = self.headers.get("content-type") || undefined;
    return self.arrayBuffer().then((buffer) => new Blob([buffer], { type }));
}

export async function formData(self) {
    return self.text().then((text) => parseMultipart(self.headers, text));
}

export async function json(self) {
    return self.text().then((text) => JSON.parse(text));
}

export async function text(self) {
    if (self.bodyUsed) {
        throw new TypeError("Body is already used");
    }

    self.bodyUsed = true;

    if (!self.body) {
        return "";
    }

    if (typeof self.body === "string") {
        return self.body;
    }

    if (!(self.body instanceof ArrayBuffer)) {
        return JSON.stringify(self.body);
    }

    return new Promise((resolve) => {
        const textDecoder = new TextDecoder();

        resolve(textDecoder.decode(self.body));
    });
}

// @see: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
export const statusTextList = {
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
