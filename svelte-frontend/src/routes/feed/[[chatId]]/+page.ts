import { replaceState } from "$app/navigation";
import { page } from "$app/state";
import { PUBLIC_API_URL } from "$env/static/public";
import type { Chat } from "$lib/chat";
import type { RouteParams } from "./$types";

export const load = async ({ depends, params }) => {
    let headers: any = {};
    let resp: Response;

    console.log(params)
    if(params.chatId){
        console.log("Got chat id in load function!")
        resp = await fetch(`${PUBLIC_API_URL}/chat/${params.chatId}`);
    }else{

        let lastId = localStorage.getItem("lastFeedUuid");
        if(lastId) headers["Exclude-Chat-Id"] = lastId;
        resp = await fetch(`${PUBLIC_API_URL}/random_chat/`, {
            headers
        });
    }

    if(!resp.ok){
        return {
            success: false,
            error: await resp.text(),
            chat: null
        };
    }

    const chat:Chat = await resp.json();

    for (const m of chat.messages){
        m.chatId = chat.id ?? undefined;
    }

    if(chat.id !== null) localStorage.setItem("lastFeedUuid", chat.id as string);

    // Allows invalidating the chat data with this key
    depends(`data:chat/${chat.id}`);

    return {
        success: true,
        error: null,
        chat: chat,
    };
};