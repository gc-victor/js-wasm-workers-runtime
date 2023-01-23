class Blob {
    constructor(blobParts, options) {
        this.textEncoder = new TextEncoder();
        this.textDecoder = new TextDecoder();

        if (blobParts) {
            const chunks = blobParts.map((blobPart) => {
                if (typeof blobPart === "string") {
                    return this.textEncoder.encode(blobPart);
                } else if (
                    blobPart instanceof ArrayBuffer ||
                    blobPart instanceof Uint8Array
                ) {
                    return new Uint8Array(blobPart);
                } else if (blobPart instanceof Blob) {
                    return blobPart.buffer;
                } else {
                    return new Uint8Array(0);
                }
            });
            const totalSize = chunks.reduce(
                (acc, chunk) => acc + chunk.byteLength,
                0,
            );
            const buffer = new Uint8Array(totalSize);
            let offset = 0;
            for (const chunk of chunks) {
                buffer.set(chunk, offset);
                offset += chunk.byteLength;
            }
            this.size = buffer.byteLength;
            this.buffer = buffer;
        } else {
            this.size = 0;
            this.buffer = new Uint8Array(0);
        }
        this.type =
            (options === null || options === void 0 ? void 0 : options.type) ||
            "";
    }
    arrayBuffer() {
        return Promise.resolve(this.buffer.buffer);
    }
    slice(start, end, contentType) {
        let type = contentType;
        if (type === undefined) {
            type = this.type;
        } else if (type === null) {
            type = "null";
        }
        return new Blob([this.buffer.slice(start, end)], { type });
    }
    stream() {
        return new ReadableStream({
            pull: async (controller) => {
                controller.enqueue(this.buffer);
                controller.close();
            },
        });
    }
    text() {
        return Promise.resolve(this.textDecoder.decode(this.buffer));
    }
}

globalThis.Blob = Blob;
