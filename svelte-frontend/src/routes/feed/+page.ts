import { PUBLIC_API_URL } from "$env/static/public";

export const load = async () => {
    const resp = await fetch(`${PUBLIC_API_URL}/random_chat/`);

    return await resp.json();
};