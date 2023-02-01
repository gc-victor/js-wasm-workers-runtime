// @see: https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder
// @see: https://github.com/Shopify/javy/blob/ecbe8cdf302acbff28287370960a0c3e7481a505/crates/core/prelude/text-encoding.js
(function () {
    const ___decodeUtf8BufferToString = globalThis.___decodeUtf8BufferToString;

    class TextDecoder {
        constructor(label = "utf-8", options = {}) {
            label = label.trim().toLowerCase();
            const acceptedLabels = [
                "utf-8",
                "utf8",
                "unicode-1-1-utf-8",
                "unicode11utf8",
                "unicode20utf8",
                "x-unicode20utf8",
            ];
            if (!acceptedLabels.includes(label)) {
                // Not spec-compliant behaviour
                throw new RangeError(
                    "The encoding label provided must be utf-8",
                );
            }
            Object.defineProperties(this, {
                encoding: { value: "utf-8", enumerable: true, writable: false },
                fatal: {
                    value: !!options.fatal,
                    enumerable: true,
                    writable: false,
                },
                ignoreBOM: {
                    value: !!options.ignoreBOM,
                    enumerable: true,
                    writable: false,
                },
            });
        }

        decode(input, options = {}) {
            if (input === undefined) {
                return "";
            }

            if (options.stream) {
                throw new Error("Streaming decode is not supported");
            }

            // backing buffer would not have byteOffset and may have different byteLength
            let byteOffset = input.byteOffset || 0;
            let byteLength = input.byteLength;

            if (ArrayBuffer.isView(input)) {
                input = input.buffer;
            }

            if (!(input instanceof ArrayBuffer)) {
                throw new TypeError(
                    "The provided value is not of type '(ArrayBuffer or ArrayBufferView)'",
                );
            }

            return ___decodeUtf8BufferToString(
                input,
                byteOffset,
                byteLength,
                this.fatal,
                this.ignoreBOM,
            );
        }
    }

    globalThis.TextDecoder = TextDecoder;

    Reflect.deleteProperty(globalThis, "___decodeUtf8BufferToString");
})();
