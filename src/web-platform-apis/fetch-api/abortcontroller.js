import AbortController, {
    AbortSignal,
} from "abortcontroller-polyfill/src/abortcontroller.js";

globalThis.AbortController = AbortController;
globalThis.AbortSignal = AbortSignal;
