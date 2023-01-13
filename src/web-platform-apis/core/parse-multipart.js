const parseMultipart = (headers, body) => {
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
