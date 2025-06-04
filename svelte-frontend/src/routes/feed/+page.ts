import { PUBLIC_API_URL } from "$env/static/public";
import type { Chat } from "$lib/chat";

export const load = async () => {
    let headers: any = {};

    let lastId = localStorage.getItem("lastFeedUuid");
    if(lastId) headers["Exclude-Chat-Id"] = lastId;

    const resp = await fetch(`${PUBLIC_API_URL}/random_chat/`, {
        headers
    });

    const chat:Chat = await resp.json();
    if(chat.id !== null) localStorage.setItem("lastFeedUuid", chat.id as string);

    return chat;
};