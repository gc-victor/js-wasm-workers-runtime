export const handleRequest = async function () {
    const response = await fetch("https://my-json-server.typicode.com/typicode/demo/posts");

    return new Response(await response.json(), {
        status: 200,
        headers: {
            "content-type": "application/json;charset=UTF-8",
        },
    });
};
