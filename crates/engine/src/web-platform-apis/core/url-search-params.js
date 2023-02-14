const ___entries = Symbol();
const ___fnIterator = Symbol();
const ___fnAppendTo = Symbol();

/**
 * URLSearchParams
 *
 * The URLSearchParams interface defines utility methods to work with the query string of a URL.
 *
 * @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams
 * @see: https://url.spec.whatwg.org/#interface-urlsearchparams
 * @see: https://github.com/ungap/url-search-params
 */
class URLSearchParams {
    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams
    constructor(init) {
        this[___entries] = {};

        if (!init) {
            return;
        }

        if (typeof init === "string") {
            let index;
            let value;

            const pairs = init.split("&");
            const length = pairs.length || 0;

            if (init.charAt(0) === "?") {
                init = init.slice(1);
            }

            for (let i = 0; i < length; i++) {
                value = pairs[i];
                index = value.indexOf("=");

                if (-1 < index) {
                    this[___fnAppendTo](
                        decode(value.slice(0, index)),
                        decode(value.slice(index + 1)),
                    );
                } else if (value.length) {
                    this[___fnAppendTo](decode(value), "");
                }
            }
        } else if (Array.isArray(init)) {
            let value;
            let length = init.length;

            for (let i = 0; i < length; i++) {
                value = init[i];
                this[___fnAppendTo](value[0], value[1]);
            }
        } else if (init.hasOwnProperty(Symbol.iterator)) {
            for (const [key, value] of init) {
                this[___fnAppendTo](key, value);
            }
        } else if (init?.forEach) {
            init.forEach((value, key) => this[___fnAppendTo](key, value), init);
        } else if (init instanceof FormData) {
            for (let [key, value] of init.entries())
                this[___fnAppendTo](key, value);
        } else {
            for (let key in init) this[___fnAppendTo](key, init[key]);
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/append
    append(key, value) {
        this[___fnAppendTo](key, value);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/delete
    delete(key) {
        this[___entries][key] = null;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/entries
    entries() {
        return this[___fnIterator](function (value, key) {
            this.push([key, value]);
        });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/forEach
    forEach(callback, thisArg) {
        var self = this;
        var keys = Object.create(null);
        this.toString()
            .replace(/=[\s\S]*?(?:&|$)/g, "=")
            .split("=")
            .forEach(function (key) {
                if (!key.length || key in keys) return;
                (keys[key] = self.getAll(key)).forEach(function (value) {
                    callback.call(thisArg, value, key, self);
                });
            });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/get
    get(key) {
        const value = this.getAll(key)[0];
        return value !== undefined ? value : null;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/getAll
    getAll(key) {
        return this[___entries][key] || [];
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/has
    has(key) {
        return !!this.getAll(key).length;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/set
    set(key, value) {
        this[___entries][key] = [value];
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/keys
    keys() {
        return this[___fnIterator](function (_, key) {
            this.push(key);
        });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/values
    values() {
        return this[___fnIterator](function (value) {
            this.push(value);
        });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/sort
    sort() {
        const values = Object.create(null);
        const entries = this.entries();
        const keys = [];

        let i;
        let entry = entries.next();
        let done = entry.done;
        let key;
        let value;

        while (!done) {
            value = entry.value;
            key = value[0];
            keys.push(key);
            if (!(key in values)) {
                values[key] = [];
            }
            values[key].push(value[1]);
            entry = entries.next();
            done = entry.done;
        }

        keys.sort();

        for (i = 0; i < keys.length; i++) {
            this.delete(keys[i]);
        }

        for (i = 0; i < keys.length; i++) {
            key = keys[i];
            this.append(key, values[key].shift());
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/toString
    toString() {
        const query = [];

        for (let key in this[___entries]) {
            let encoded = encode(key);

            const value = this[___entries][key];
            const length = value ? value.length : 0;

            for (let i = 0; i < length; i++) {
                query.push(`${encoded}=${encode(value[i])}`);
            }
        }

        return query.join("&");
    }

    [Symbol.iterator]() {
        return this.entries();
    }

    [___fnAppendTo](key, value) {
        if (key === null) key = "null";

        key = key.replace(/^\?/, "");
        var res = Array.isArray(value) ? value.join(",") : value;
        if (key in this[___entries]) {
            this[___entries][key] = Array.isArray(this[___entries][key])
                ? this[___entries][key]
                : [];
            this[___entries][key].push(res);
        } else {
            this[___entries][key] = [res];
        }
    }

    [___fnIterator](callback) {
        const items = [];

        this.forEach(callback, items);

        return items[Symbol.iterator]();
    }
}

globalThis.URLSearchParams = URLSearchParams;

function decode(str) {
    return decodeURIComponent(
        str.replace(/%(?![0-9a-fA-F]{2})/g, "%25").replace(/\+/g, " "),
    );
}

function encode(str) {
    const find = /[!'\(\)~]|%20|%00/g;

    return encodeURIComponent(str).replace(find, replacer);
}

function replacer(match) {
    const replace = {
        "!": "%21",
        "'": "%27",
        "(": "%28",
        ")": "%29",
        "~": "%7E",
        "%20": "+",
        "%00": "\x00",
    };

    return replace[match];
}
