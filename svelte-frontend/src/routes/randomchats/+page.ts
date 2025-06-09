import { PUBLIC_API_URL } from "$env/static/public";
import type { Chat } from "$lib/chat";
import { userState } from "$lib/state/user.svelte";
import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
    if(!userState.user) return {
        loggedIn: false,
        status: -1,
        error: null,
        chats: null,
    };


    const resp = await fetch(`${PUBLIC_API_URL}/mychats/random`, {
        headers: {
            "Authorization": `Bearer ${userState.user.accessToken}`
        }
    });

    if(resp.status !== 200){
        return {
            loggedIn: true,
            status: resp.status,
            error: await resp.text(),
            chats: null,
        }
    }

    const chats: Chat[] = await resp.json();
    return {
        loggedIn: true,
        status: 200,
        error: null,
        chats: chats,
    }
};