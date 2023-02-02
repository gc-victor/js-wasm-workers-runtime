const ___headers = Symbol();

/*
 * Headers
 *
 * The Headers interface of the Fetch API allows you to perform various actions on HTTP request and response headers.
 * These actions include retrieving, setting, adding to, and removing headers from the list of the request's headers.
 *
 * @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers
 * @see: https://developers.cloudflare.com/workers/runtime-apis/headers/
 * @see: https://github.com/github/fetch/blob/fb5b0cf42b470faf8c5448ab461d561f34380a30/fetch.js#L84
 * @see: https://fetch.spec.whatwg.org/#headers-class
 */
class Headers {
    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/Headers
    constructor(init) {
        this[___headers] = [];

        if (init instanceof Headers) {
            init.forEach(function (value, name) {
                this.append(name, value);
            }, this);
        } else if (Array.isArray(init)) {
            init.forEach(function (header) {
                if (header.length !== 2) {
                    throw new TypeError(
                        "Failed to construct 'Headers': Invalid value",
                    );
                }

                this.append(header[0], header[1]);
            }, this);
        } else if (init) {
            Object.getOwnPropertyNames(init).forEach(function (name) {
                this.append(name, init[name]);
            }, this);
        } else if (init === null) {
            throw new TypeError(
                "Failed to construct 'Headers': The provided value is null",
            );
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/append
    append(name, value) {
        name = normalizeName(name);
        value = normalizeValue(value);

        if (this.has(name)) {
            this[___headers] = this[___headers].map((pair) => {
                if (pair[0] === name) {
                    return [name, `${pair[1]}, ${value}`];
                } else {
                    return pair;
                }
            });
        } else {
            this[___headers].push([name, value]);
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/get
    get(name) {
        const result = this[___headers].find(
            (pair) => pair[0] === normalizeName(name),
        );

        if (result === undefined) {
            return null;
        }

        return result[1];
    }

    getAll() {
        return this[___headers];
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/get
    has(name) {
        return this[___headers].some((pair) => pair[0] === normalizeName(name));
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/set
    set(name, value) {
        name = normalizeName(name);

        if (this.has(name)) {
            this[___headers] = this[___headers].map((pair) =>
                pair[0] === name ? [name, normalizeValue(value)] : pair,
            );
        } else {
            this[___headers].push([name, normalizeValue(value)]);
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/delete
    delete(name) {
        this[___headers] = this[___headers].filter(
            (pair) => pair[0] !== normalizeName(name),
        );
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/forEach
    forEach(callback, thisArg) {
        for (let pair of headers.entries()) {
            callback.call(thisArg, pair[1], pair[0], this);
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/keys
    keys() {
        return this[___headers].map((pair) => pair[0])[Symbol.iterator]();
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/values
    values() {
        return this[___headers].map((pair) => pair[1])[Symbol.iterator]();
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/entries
    entries() {
        return this[___headers][Symbol.iterator]();
    }
}

globalThis.Headers = Headers;

function normalizeName(name) {
    if (typeof name !== "string") {
        name = String(name);
    }

    name = name.replace(/^[\n\t\r\x20]+|[\n\t\r\x20]+$/g, "");

    // TODO: implement a test for ["\t\f\tnewLine4\n","\f\tnewLine"], result \f\tnewLine
    // TODO: implement a test for ["newLine5\xa0","newLine5\xa0"], result newLine5\xa0

    if (/[^a-z0-9\-#$%&'*+.^_`|~!]/i.test(name) || name === "") {
        throw new TypeError(
            `Invalid character in header field name: "${name}"`,
        );
    }

    return name.toLowerCase();
}

function normalizeValue(value) {
    if (typeof value !== "string") {
        value = String(value);
    }

    return value;
}
