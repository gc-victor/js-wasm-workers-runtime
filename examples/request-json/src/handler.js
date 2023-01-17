export const handleRequest = async function (request) {
    const json = await request.json();

    ___logger("content-type:", request.headers['content-type']);
    ___logger("headers:", JSON.stringify(request.headers));
    ___logger("json:", JSON.stringify(json));

    return new Response("", { 
        status: 200,
        headers: {
            "content-type": "text/html;charset=UTF-8",
        },
    });
};
