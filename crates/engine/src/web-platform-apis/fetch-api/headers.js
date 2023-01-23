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
    constructor(headers) {
        this.map = {};

        if (headers instanceof Headers) {
            headers.forEach(function (value, name) {
                this.append(name, value);
            }, this);
        } else if (Array.isArray(headers)) {
            headers.forEach(function (header) {
                this.append(header[0], header[1]);
            }, this);
        } else if (headers) {
            Object.getOwnPropertyNames(headers).forEach(function (name) {
                this.append(name, headers[name]);
            }, this);
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/append
    append(name, value) {
        name = normalizeName(name);
        value = normalizeValue(value);

        const oldValue = this.map[name];

        this.map[name] = oldValue ? `${oldValue}, ${value}` : value;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/get
    get(name) {
        name = normalizeName(name);

        return this.has(name) ? this.map[name] : null;
    }

    getAll() {
        return this.map;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/get
    has(name) {
        return this.map.hasOwnProperty(normalizeName(name));
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/set
    set(name, value) {
        this.map[normalizeName(name)] = normalizeValue(value);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/delete
    delete(name) {
        this.map[normalizeName(name)] = undefined;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/forEach
    forEach(callback, thisArg) {
        for (const name in this.map) {
            if (this.map.hasOwnProperty(name)) {
                callback.call(thisArg, this.map[name], name, this);
            }
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/keys
    keys() {
        const items = [];

        this.forEach((_, name) => {
            items.push(name);
        });

        return iteratorFor(items);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/values
    values() {
        const items = [];

        this.forEach((value) => {
            items.push(value);
        });

        return iteratorFor(items);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/Headers/entries
    entries() {
        const items = [];

        this.forEach((value, name) => {
            items.push([name, value]);
        });

        return iteratorFor(items);
    }
}

globalThis.Headers = Headers;

function normalizeName(name) {
    if (typeof name !== "string") {
        name = String(name);
    }

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

function iteratorFor(items) {
    var iterator = {
        next: function () {
            var value = items.shift();

            return { done: value === undefined, value: value };
        },
    };

    if (support.iterable) {
        iterator[Symbol.iterator] = function () {
            return iterator;
        };
    }

    return iterator;
}
