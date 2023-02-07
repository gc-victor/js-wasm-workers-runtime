const entries = Symbol("entries");

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
        this[entries] = {};

        if (!init) {
            return;
        }

        if (typeof init === "string") {
            let index;
            let value;
            let i = 0;

            const pairs = init.split("&");
            const length = pairs.length;

            if (init.charAt(0) === "?") {
                init = init.slice(1);
            }

            for (i; i < length; i++) {
                value = pairs[i];
                index = value.indexOf("=");
                if (-1 < index) {
                    appendTo(
                        this[entries],
                        decode(value.slice(0, index)),
                        decode(value.slice(index + 1)),
                    );
                } else if (value.length) {
                    appendTo(this[entries], decode(value), "");
                }
            }
        } else if (Array.isArray(init)) {
            let value;

            for (i; i < length; i++) {
                value = init[i];
                appendTo(this[entries], value[0], value[1]);
            }
        } else if ("forEach" in init) {
            init.forEach(addEach, dict);
        } else {
            for (let key in init) appendTo(this[entries], key, init[key]);
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/append
    append(name, value) {
        appendTo(this[entries], name, value);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/delete
    delete(name) {
        this[entries][name] = null;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/entries
    entries() {
        return iterator(this, function (value, key) {
            this.push([key, value]);
        });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/forEach
    forEach(callback, thisArg) {
        var self = this;
        var names = Object.create(null);
        this.toString()
            .replace(/=[\s\S]*?(?:&|$)/g, "=")
            .split("=")
            .forEach(function (name) {
                if (!name.length || name in names) return;
                (names[name] = self.getAll(name)).forEach(function (value) {
                    callback.call(thisArg, value, name, self);
                });
            });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/get
    get(name) {
        return this.getAll(name)[0] || null;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/getAll
    getAll(name) {
        return this[entries][name] || [];
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/has
    has(name) {
        return !!this.getAll(name).length;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/set
    set(name, value) {
        this[entries][name] = [value];
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/keys
    keys() {
        return iterator(this, function (_, name) {
            this.push(name);
        });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/values
    values() {
        return iterator(this, function (value) {
            this.push(value);
        });
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/sort
    sort() {
        const values = Object.create(null);
        const entries = this.entries();
        const names = [];

        let i;
        let entry = entries.next();
        let done = entry.done;
        let name;
        let value;

        while (!done) {
            value = entry.value;
            name = value[0];
            names.push(name);
            if (!(name in values)) {
                values[name] = [];
            }
            values[name].push(value[1]);
            entry = entries.next();
            done = entry.done;
        }

        names.sort();

        for (i = 0; i < names.length; i++) {
            this.delete(names[i]);
        }

        for (i = 0; i < names.length; i++) {
            name = names[i];
            this.append(name, values[name].shift());
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/toString
    toString() {
        const query = [];

        for (let key in this[entries]) {
            let encoded = encode(key);

            for (let i = 0, value = this[entries][key]; i < value.length; i++) {
                query.push(`${encoded}=${encode(value[i])}`);
            }
        }

        return query.join("&");
    }
}

globalThis.URLSearchParams = URLSearchParams;

function appendTo(entries, key, value) {
    var res = Array.isArray(value) ? value.join(",") : value;
    if (key in entries) entries[key].push(res);
    else entries[key] = [res];
}

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

function iterator(self, callback) {
    var items = [];

    self.forEach(callback, items);

    return items[Symbol.iterator]();
}
