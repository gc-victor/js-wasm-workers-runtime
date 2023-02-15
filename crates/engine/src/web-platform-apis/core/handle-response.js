globalThis.___handleResponse = async function () {
    const response = await handleRequest();
    const body = await response.arrayBuffer();

    return {
        body: body.buffer,
        bodyUsed: response.bodyUsed,
        headers: response.headers.getAll(),
        ok: response.ok,
        redirected: response.redirected,
        status: response.status,
        statusText: response.statusText,
        type: response.type,
        url: response.url,
    };
}