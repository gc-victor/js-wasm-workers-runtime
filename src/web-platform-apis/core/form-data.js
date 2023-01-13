class FormData {
    constructor() {
        this.fields = new Map();
    }
    addValue(name, value) {
        const values = this.fields.get(name);
        if (values) {
            values.push(value);
        } else {
            this.fields.set(name, [value]);
        }
    }
    append(name, value) {
        this.addValue(name, value);
    }
    delete(name) {
        this.fields.delete(name);
    }
    *entries() {
        for (const [key, values] of this.fields) {
            for (const value of values) {
                yield [key, value];
            }
        }
    }
    forEach(callbackfn, thisArg) {
        this.fields.forEach((values, key) => {
            values.forEach((value) => {
                callbackfn.call(thisArg, value, key, this);
            });
        });
    }
    get(name) {
        var _a;
        return (
            ((_a = this.fields.get(name)) === null || _a === void 0
                ? void 0
                : _a[0]) || null
        );
    }
    getAll(name) {
        return this.fields.get(name) || [];
    }
    has(name) {
        return this.fields.has(name);
    }
    keys() {
        return this.fields.keys();
    }
    set(name, value) {
        this.fields.set(name, [value]);
    }
    *values() {
        for (const [, values] of this.fields) {
            for (const value of values) {
                yield value;
            }
        }
    }
    [Symbol.iterator]() {
        return this.entries();
    }
}

globalThis.FormData = FormData;
