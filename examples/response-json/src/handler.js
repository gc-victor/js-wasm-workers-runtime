export const handleRequest = async function (_) {
    const response = new Response(
        { hello: "world" },
        {
            status: 200,
            headers: {
                "content-type": "text/html;charset=UTF-8",
            },
        },
    );

    try {
        ___logger("response-json", JSON.stringify(await response.json()));
    } catch (error) {
        ___logger("response-json error", error);
    }

    return response;
};
