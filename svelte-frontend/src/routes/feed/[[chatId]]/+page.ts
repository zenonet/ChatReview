import { replaceState } from "$app/navigation";
import { page } from "$app/state";
import { PUBLIC_API_URL } from "$env/static/public";
import type { Chat } from "$lib/chat";
import type { RouteParams } from "./$types";

export const load = async ({ params }) => {
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

    const chat:Chat = await resp.json();
    if(chat.id !== null) localStorage.setItem("lastFeedUuid", chat.id as string);

    return chat;
};