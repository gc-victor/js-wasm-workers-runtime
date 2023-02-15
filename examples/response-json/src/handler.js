export const handleRequest = async function (_) {
    return new Response(
        JSON.stringify({ hello: "world" }),
        {
            status: 200,
            headers: {
                "content-type": "text/html;charset=UTF-8",
            },
        },
    );
};
