export const handleRequest = async function (request) {
    ___logger("text:", await request.text());

    return new Response("", {
        status: 200,
        headers: {
            "content-type": "text/html;charset=UTF-8",
        },
    });
};
