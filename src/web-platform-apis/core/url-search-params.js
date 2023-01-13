/**
 * URLSearchParams
 *
 * The URLSearchParams interface defines utility methods to work with the query string of a URL.
 *
 * @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams
 * @see: https://url.spec.whatwg.org/#interface-urlsearchparams
 */
class URLSearchParams {
    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams
    constructor(init) {
        this.params = new Map();
        if (init) {
            if (typeof init === "string") {
                init.replace("?", "")
                    .split("&")
                    .forEach((entry) => {
                        const [key, value] = entry.split("=");
                        this.addValue(key, decodeURIComponent(value));
                    });
            } else if (typeof init === "object") {
                if (Array.isArray(init)) {
                    init.forEach(([key, value]) => {
                        this.addValue(key, value);
                    });
                } else {
                    Object.entries(init).forEach(([key, value]) => {
                        this.addValue(key, value);
                    });
                }
            }
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/addValue
    addValue(name, value) {
        const values = this.params.get(name);
        if (values) {
            values.push(value);
        } else {
            this.params.set(name, [value]);
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/append
    append(name, value) {
        this.addValue(name, value);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/delete
    delete(name) {
        this.params.delete(name);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/entries
    *entries() {
        for (const [key, values] of this.params) {
            for (const value of values) {
                yield [key, value];
            }
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/forEach
    forEach(callbackfn, thisArg) {
        this.params.forEach((values, key) => {
            values.forEach((value) => {
                callbackfn.call(thisArg, value, key, this);
            });
        });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/get
    get(name) {
        var _a;
        return (
            ((_a = this.params.get(name)) === null || _a === void 0
                ? void 0
                : _a[0]) || null
        );
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/getAll
    getAll(name) {
        return this.params.get(name) || [];
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/has
    has(name) {
        return this.params.has(name);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/keys
    keys() {
        return this.params.keys();
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/set
    set(name, value) {
        this.params.set(name, [value]);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/sort
    sort() {
        this.params = new Map([...this.params].sort());
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/toString
    toString() {
        return Array.from(this.params.entries())
            .map(([key, value]) => `${key}=${value}`)
            .join("&");
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/values
    *values() {
        for (const [, values] of this.params) {
            for (const value of values) {
                yield value;
            }
        }
    }

    [Symbol.iterator]() {
        return this.entries();
    }
}

globalThis.URLSearchParams = URLSearchParams;
