// https://fetch.spec.whatwg.org/#null-body-status
const NULL_BODY_STATUS = [101, 103, 204, 205, 304];

export const hasNullBody = (status) => NULL_BODY_STATUS.includes(status);

export const isIterable = (value) =>
    typeof value !== "string" && Symbol.iterator in Object(value);

export const parseMultipart = (headers, body) => {
    var _a;
    const formData = new FormData();
    if (!body) {
        return formData;
    }
    const contentTypeHeader = headers.get("content-type");
    if (contentTypeHeader === APPLICATION_X_WWW_FORM_URLENCODED) {
        const params = new URLSearchParams(body);
        for (const [key, value] of params) {
            formData.append(key, value);
        }
        return formData;
    } else if (
        contentTypeHeader === null || contentTypeHeader === void 0
            ? void 0
            : contentTypeHeader.startsWith(MULTIPART_FORMDATA_CONTENTYPE)
    ) {
        let boundary;
        const getBoundary = (header) => {
            var _a;
            var _b;
            var _c;
            return (_c =
                (_b =
                    (_a =
                        header === null || header === void 0
                            ? void 0
                            : header.split(";")) === null || _a === void 0
                        ? void 0
                        : _a[1]) === null || _b === void 0
                    ? void 0
                    : _b.split("=")) === null || _c === void 0
                ? void 0
                : _c[1];
        };
        if (Array.isArray(contentTypeHeader)) {
            contentTypeHeader.forEach((header) => {
                if (!boundary) {
                    boundary = getBoundary(header);
                }
            });
        } else {
            boundary = getBoundary(contentTypeHeader);
        }
        if (!boundary) {
            return formData;
        }
        for (const part of body.split(boundary)) {
            if (
                part === null || part === void 0
                    ? void 0
                    : part.includes(CONTENT_DISPOSITION)
            ) {
                const content =
                    (_a = part.split('; name="')) === null || _a === void 0
                        ? void 0
                        : _a[1].split("\n\n");
                if (content) {
                    const [name, value] = content;
                    formData.append(
                        name.split('"')[0],
                        value.replace("\n--", ""),
                    );
                }
            }
        }
        return formData;
    } else {
        throw new Error(`Unsupported content type: ${contentTypeHeader}`);
    }
};

export async function arrayBuffer(self) {
    return new Promise((resolve) => resolve(self.body));
}

export async function blob(self) {
    const type = self.headers.get("content-type") || undefined;
    return self.arrayBuffer().then((buffer) => new Blob([buffer], { type }));
}

export async function formData(self) {
    const body = await self.text();
    return new Promise((resolve) => resolve(parseMultipart(self.headers, body)));
}

export async function json(self) {
    return self.text().then((text) => JSON.parse(text));
}

export async function text(self) {
    if (self.bodyUsed) {
        throw new TypeError("Body is already used");
    }

    if (!self.body) {
        self.bodyUsed = true;
        return "";
    }

    if (typeof self.body === "string") {
        self.bodyUsed = true;
        return self.body;
    }

    return new Promise((resolve) =>
        resolve(globalThis.___textDecoder.decode(self.body)),
    );
}

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
