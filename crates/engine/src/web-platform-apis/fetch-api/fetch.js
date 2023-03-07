// @see: https://developer.mozilla.org/en-US/docs/Web/API/fetch
// @see: https://developer.mozilla.org/en-US/docs/Web/API/fetch#resource
// @see: https://developer.mozilla.org/en-US/docs/Web/API/fetch#options
async function fetch(resource, options) {
    if (resource instanceof Request) {
        options = resource;
        resource = options.url;
    }

    const strResponse = ___fetcher({
        body: options?.body ? await getBody(options.body) : "",
        credentials: options?.credentials || "same-origin",
        cache: options?.cache,
        headers: JSON.stringify(options?.headers || {}),
        integrity: options?.integrity,
        keepalive: !!options?.keepalive,
        method: options?.method || "GET",
        mode: options?.mode || "cors",
        redirect: options?.redirect || "follow",
        referer: options?.referrer,
        referrerPolicy: options?.referrerPolicy,
        signal: options?.signal,
        url: resource instanceof URL ? resource.href : resource,
    });

    const response = JSON.parse(strResponse);

    return Promise.resolve(
        new Response(new Uint8Array(response.body).buffer, {
            status: response.status,
            url: resource,
            headers: JSON.parse(response.headers),
        }),
    );
}

globalThis.fetch = fetch;

async function getBody(body) {
    if (body instanceof Blob) {
        return new Uint8Array(await body.arrayBuffer());
    } else if (body instanceof FormData) {
        // TODO: Add example
        return new TextEncoder().encode(body.toString());
    } else if (body instanceof URLSearchParams) {
        // TODO: Add example
        return new TextEncoder().encode(body.toString());
    } else if (body instanceof ArrayBuffer) {
        return new Uint8Array(body);
    } else if (body instanceof ReadableStream) {
        // TODO: Add example
        return new Uint8Array(await getBodyStream(body));
    }

    return new TextEncoder().encode(body);
}

async function getBodyStream(body) {
    const reader = body.getReader();
    const chunks = [];

    while (true) {
        const { done, value } = await reader.read();

        if (done) {
            break;
        }

        chunks.push(value);
    }

    return chunks;
}
