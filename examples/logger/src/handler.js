export const handleRequest = async function (request) {
    ___logger("request", JSON.stringify(request));

    return new Response("", {
        status: 200,
        headers: {
            "content-type": "text/html;charset=UTF-8",
        },
    });
};
