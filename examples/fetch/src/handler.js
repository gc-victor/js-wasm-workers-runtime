export const handleRequest = async function (request) {
    const response = await fetch("https://my-json-server.typicode.com/typicode/demo/posts");

    return await response.json();
};
