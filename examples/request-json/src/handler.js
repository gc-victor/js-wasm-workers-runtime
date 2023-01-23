export const handleRequest = async function (request) {
    const json = await request.json();

    ___logger("method:", request.method);
    ___logger("headers:", JSON.stringify(request.headers));
    ___logger("body:", JSON.stringify(json));

    return "";
};
