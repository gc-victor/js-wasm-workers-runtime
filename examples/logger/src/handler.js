export const handleRequest = function (event) {
    ___logger("event", JSON.stringify(event));

    return new Response("", {
      status: 200,
      headers: {
          "content-type": "text/html;charset=UTF-8",
      },
  });
};
