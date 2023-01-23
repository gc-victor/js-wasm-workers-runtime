export const handleRequest = async function (request) {
    ___logger("event", JSON.stringify(request.text()));

    return new Response("", {
        status: 200,
        headers: {
            "content-type": "text/html;charset=UTF-8",
        },
    });
};
