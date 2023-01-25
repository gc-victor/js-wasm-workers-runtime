function fetch(url, options) {
    const strResponse = ___fetcher({
        // rome-ignore lint/complexity/useOptionalChain: <explanation>
        method: (options && options.method) || "GET",
        url,
        // rome-ignore lint/complexity/useOptionalChain: <explanation>
        headers: (options && options.headers) || {},
        // rome-ignore lint/complexity/useOptionalChain: <explanation>
        body: options && options.body,
    });

    const response = JSON.parse(strResponse);

    return Promise.resolve(
        new Response(response.body, {
            status: response.status,
            url,
            headers: JSON.parse(response.headers),
        }),
    );
}

globalThis.fetch = fetch;
