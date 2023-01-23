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

    // ___logger("fetch strResponse:", strResponse);
    // ___logger("fetch isStrResponseString:", typeof strResponse === 'string');
    // ___logger("fetch JSON.parse:", JSON.parse(strResponse));
    
    const response = JSON.parse(strResponse);
    
    // ___logger("fetch url:", url);
    // ___logger("fetch options:", options); 
    // ___logger("fetch response:", response);
    // ___logger("fetch headers:", response.headers);
    // ___logger("fetch body:", response.body);

    return Promise.resolve(
        new Response(response.body, {
            status: response.status,
            url,
            headers: JSON.parse(response.headers),
        }),
    );
}

globalThis.fetch = fetch;
