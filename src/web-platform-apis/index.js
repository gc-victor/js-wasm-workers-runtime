import "./core/web-streams.js";

import "./core/blob.js";
import "./core/form-data.js";
import "./core/text-encoder.js";
import "./core/text-decoder.js";
import "./core/url-search-params";

import "./fetch-api/abortcontroller.js";
import "./fetch-api/headers.js";
import "./fetch-api/request.js";
import "./fetch-api/response.js";
import "./fetch-api/fetch.js";

globalThis.___textEncoder = new TextEncoder();
globalThis.___textDecoder = new TextDecoder();
