export const symbol = Symbol();

export class ___Body {
    constructor(childSymbol) {
        this[symbol] = childSymbol;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/body
    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/body
    get body() {
        const self = this[this[symbol]];

        if (self.body === null) return null;
        if (self.body instanceof ReadableStream)
            return self.body;

        const stream = new TransformStream();
        const writer = stream.writable.getWriter();

        writer.write(self.body);
        writer.close();

        return stream.readable;
    }

    // read-only
    set body(_) {}


    get bodyUsed() {
        return this[this[symbol]].bodyUsed;
    }
    // read-only
    set bodyUsed(_) {}

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/arrayBuffer
    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/arrayBuffer
    async arrayBuffer() {
        const self = this[this[symbol]];

        if (self.bodyUsed) {
            throw new TypeError(
                "Failed to execute 'arrayBuffer': body stream already read",
            );
        }

        let body = self.body;

        if (body instanceof ReadableStream) {
            const read = await body.getReader().read();

            body = read.value;
        }

        if (body instanceof Blob) {
            return body.arrayBuffer();
        }

        if (body instanceof FormData) {
            body = multiPartToString(body, self.headers);
        }

        if (typeof body === "string") {
            body = new TextEncoder().encode(body);
        }

        return new Promise((resolve) => resolve(body));
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/blob
    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/blob
    async blob() {
        const self = this[this[symbol]];

        if (self.bodyUsed) {
            throw new TypeError(
                "Failed to execute 'blob': body stream already read",
            );
        }

        if (self.type === "opaque") {
            return new Promise((resolve) => {
                resolve(new Blob([], { type: "" }));
            });
        }

        return this.arrayBuffer().then((buffer) => {
            let type = "";

            const headers = self.headers;

            if (self.body instanceof Blob) {
                type = self.body.type;
            }

            return new Blob([buffer], {
                type: type || headers?.get("content-type") || "",
            });
        });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/formData
    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/formData
    async formData() {
        const self = this[this[symbol]];

        if (self.bodyUsed) {
            throw new TypeError(
                "Failed to execute 'formData': body stream already read",
            );
        }

        if (self.body instanceof Blob || self.body instanceof ArrayBuffer) {
            throw new TypeError("Failed to fetch");
        }

        if (self.body instanceof FormData) {
            return new Promise((resolve) => {
                resolve(self.body);
            });
        }

        return this.text().then((text) => toFormData(self.headers, text));
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/json
    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/json
    async json() {
        if (this[this[symbol]].bodyUsed) {
            throw new TypeError(
                "Failed to execute 'json': body stream already read",
            );
        }

        return this.text().then((text) => JSON.parse(text));
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Request/text
    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Response/text
    async text() {
        const self = this[this[symbol]];

        if (self.bodyUsed) {
            throw new TypeError(
                "Failed to execute 'text': body stream already read",
            );
        }

        self.bodyUsed = true;

        if (!self.body) {
            return "";
        }

        let body = self.body;

        if (body instanceof ReadableStream) {
            const read = await body.getReader().read();

            body = read.value;
        }

        if (body instanceof Blob) {
            body = await body.arrayBuffer();
        }

        if (body instanceof FormData) {
            return new Promise((resolve) =>
                resolve(multiPartToString(body, self.headers)),
            );
        }

        return new Promise((resolve) => {
            resolve(new TextDecoder().decode(body));
        });
    }
}

function toFormData(headers, body) {
    const formData = new FormData();

    if (!body) formData;

    const contentType = headers.get("content-type");

    if (/multipart\/form-data/.test(contentType)) {
        const boundary = getBoundary(contentType);
        return boundary ? parseMultipart(body, boundary) : formData;
    } else if (/application\/x-www-form-urlencoded/.test(contentType)) {
        body.trim()
            .split("&")
            .forEach(function (bytes) {
                if (bytes) {
                    let split = bytes.split("=");
                    let name = split.shift().replace(/\+/g, " ");
                    let value = split.join("=").replace(/\+/g, " ");
                    formData.append(
                        decodeURIComponent(name),
                        decodeURIComponent(value),
                    );
                }
            });

        return formData;
    } else {
        throw new TypeError("Failed to fetch");
    }
}

function parseMultipart(body, boundary) {
    let name = "";

    const formData = new FormData();
    const chunks = body.split(boundary);

    for (let i = 0, len = chunks.length; i < len; i++) {
        const chunk = chunks[i];
        const lines = chunk.split(/\r?\n/);

        for (let l = 1, lenL = lines.length; l < lenL; l++) {
            const line = lines[l].trim();

            if (!line) continue;
            if (/content-type/i.test(line)) continue;
            if (/content-disposition/i.test(line)) {
                name = line.match(/\sname\=\"(.*?)\"/);
                name = name ? name[1] : "";
                name = name.replace("[]", "");

                continue;
            }

            formData.append(name, line);
        }
    }

    return formData;
}

function getBoundary(contentType) {
    if (!contentType) return "";

    const boundary = contentType
        .split(";")
        .find((item) => item.includes("boundary"));

    return boundary ? boundary.split("=")[1] : "";
}

function multiPartToString(formData, headers) {
    const boundary = `----WebKitFormBoundary${Math.random()
        .toString(36)
        .replace(/0\./, "")}`;
    const body = [];

    for (const [key, value] of formData.entries()) {
        body.push(`--${boundary}`);
        body.push(`Content-Disposition: form-data; name=\"${key}\"`);
        body.push("");
        body.push(value);
    }

    body.push(`--${boundary}--`);

    headers.set("content-type", `multipart/form-data; boundary=${boundary}`);

    return body.join("\n");
}
