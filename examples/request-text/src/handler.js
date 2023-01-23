export const handleRequest = async function (request) {
    ___logger("method:", request.method);
    ___logger("headers:", JSON.stringify(request.headers));
    ___logger("body:", await request.text());

    return "";
};
