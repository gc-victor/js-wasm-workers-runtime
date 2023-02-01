// @see: https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder
// @see: https://github.com/Shopify/javy/blob/ecbe8cdf302acbff28287370960a0c3e7481a505/crates/core/prelude/text-encoding.js
(function () {
    const ___encodeStringToUtf8Buffer = globalThis.___encodeStringToUtf8Buffer;

    class TextEncoder {
        constructor() {
            Object.defineProperties(this, {
                encoding: { value: "utf-8", enumerable: true, writable: false },
            });
        }

        encode(input = "") {
            input = input.toString(); // non-string inputs are converted to strings
            return new Uint8Array(___encodeStringToUtf8Buffer(input));
        }

        encodeInto(source, destination) {
            throw new Error("encodeInto is not supported");
        }
    }

    globalThis.TextEncoder = TextEncoder;

    Reflect.deleteProperty(globalThis, "___encodeStringToUtf8Buffer");
})();
