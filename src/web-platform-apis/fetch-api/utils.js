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
    return self.text().then((text) => globalThis.___textEncoder.encode(text));
}

export async function blob(self) {
    const type = self.headers.get("content-type") || undefined;
    return self.arrayBuffer().then((buffer) => new Blob([buffer], { type }));
}

export async function formData(self) {
    const body = await self.text();
    return parseMultipart(self.headers, body);
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
    return new Promise((resolve) => {
        const reader = self.body.getReader();
        let result = "";
        const pull = () => {
            reader.read().then(({ done, value }) => {
                if (done) {
                    self.bodyUsed = true;
                    return resolve(result);
                }
                if (isIterable(value)) {
                    result += globalThis.___textEncoder.decode(value);
                } else {
                    result += value;
                }
                pull();
            });
        };
        pull();
    });
}
