export const handleRequest = async function () {
    const token =
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOnsiaXYiOiIxZTQ4ZDczOGIxNjI2NjllNDk2OGU4NDExNmY3NTZlZiIsImNvbnRlbnQiOiJjYTc4M2FhNDE5NjNmMDJlNjc5NTUzZjU2NzBiOGFjYzg3Yjk5YjZlM2U2MjZjY2UxMWU4YjMxNDUwMmIwNjQzNTVkMmMzOGFhMmVhMjgwZjIzY2M3OWI0Y2NmZTFlZDZmYWFmOTRmMTllYjZjYjdiZjA1NzZkNTVkMjdkNzY3MjhhYjI5NTdkMGUxMjZhMTIzZjVjYmM2MTMwNjgxN2ZkM2UyMWVjMTg4ODVlMDhjYzdkMTlmZDc1MDlhOTAzYWIzMTQ5OWIzY2UzNTVlYzg1NzQ3M2VkMDAzYzY2NDMzNjIwY2VlZmZiMjYyMTA1MWJiNSJ9LCJjcmVhdGVkIjoxNjUyNjE0MTEyNTQwLCJpYXQiOjE2NTI2MTQxMTJ9.x_o_TFdO9OWlZGpFxh3GTkZ1Cvd7B6eLwgGKyJa_0Bk";

    const response = await fetch("https://odb.dev/api/friend", {
        headers: {
            Authorization: `Bearer ${token}`,
        },
    });

    const json = await response.json();

    return new Response(JSON.stringify(json), {
        status: 200,
        headers: {
            "content-type": "application/json;charset=UTF-8",
        },
    });
};
