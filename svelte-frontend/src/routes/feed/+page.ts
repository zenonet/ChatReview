export const load = async () => {
    const resp = await fetch("http://localhost:2555/random_chat/");

    return await resp.json();
};