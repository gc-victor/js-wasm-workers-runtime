const ___data = Symbol();

// @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData
class FormData {
    constructor(form) {
        this[___data] = [];

        if (form !== undefined) {
            throw new TypeError(
                "Failed to construct 'FormData': parameters are not allowed.",
            );
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/append
    append(key, value) {
        if (this.has(key)) {
            this[___data] = this[___data].map((pair) => {
                if (pair[0] === key) {
                    const oldValue = pair[1];
                    const newValue = stringifyValue(value);

                    if (Array.isArray(oldValue)) {
                        oldValue.push(newValue);
                        return [key, oldValue];
                    } else {
                        return [key, [oldValue, newValue]];
                    }
                } else {
                    return pair;
                }
            });
        } else {
            this[___data].push([key, stringifyValue(value)]);
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/get
    get(key) {
        const result = this[___data].find((pair) => pair[0] === key);

        if (result === undefined) {
            return null;
        }

        const value = result[1];

        if (value === undefined) {
            return null;
        }

        return Array.isArray(value) ? value[0] : value;
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/getAll
    getAll(key) {
        const result = this[___data].find((pair) => pair[0] === key);

        if (result === undefined) {
            return [];
        }

        return result[1];
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/has
    has(key) {
        return this[___data].some((pair) => pair[0] === key);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/set
    set(key, value) {
        if (this.has(key)) {
            this[___data] = this[___data].map((pair) =>
                pair[0] === key ? [key, stringifyValue(value)] : pair,
            );
        } else {
            this[___data].push([key, stringifyValue(value)]);
        }
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/delete
    delete(key) {
        this[___data] = this[___data].filter((pair) => pair[0] !== key);
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/forEach
    entries() {
        return this[___data][Symbol.iterator]();
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/forEach
    keys() {
        return this[___data].map((pair) => pair[0])[Symbol.iterator]();
    }

    // @see: https://developer.mozilla.org/en-US/docs/Web/API/FormData/forEach
    values() {
        return this[___data].map((pair) => pair[1])[Symbol.iterator]();
    }
}

globalThis.FormData = FormData;

function stringifyValue(value) {
    return typeof value === "string" || value instanceof Blob
        ? value
        : String(value);
}
