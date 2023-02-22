export const handleRequest = async function () {
    const response = await fetch("https://my-json-server.typicode.com/typicode/demo/posts");

    const json = await response.json();

    return new Response(JSON.stringify(json), {
        status: 200,
        headers: {
            "content-type": "application/json;charset=UTF-8",
        },
    });
};
