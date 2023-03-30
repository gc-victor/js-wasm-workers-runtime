export const handleRequest = async function (request) {

    ___logger("body:", request.body);
    ___logger("method:", request.method);
    ___logger("url:", request.url);
    ___logger("headers:", JSON.stringify(request.headers));


    return new Response("", {
        status: 200,
        headers: {
            "content-type": "text/html;charset=UTF-8",
        },
    });
};
