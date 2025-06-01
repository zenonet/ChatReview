import { page } from "$app/state";
import { PUBLIC_API_URL } from "$env/static/public";
import type { Chat } from "$lib/chat";
import { userState } from "$lib/state/user.svelte";


import type { PageLoad } from '../$types';
export const load: PageLoad = async ({params}) => {

    console.log(params);
    const resp = await fetch(`${PUBLIC_API_URL}/chat/fromMyPerspective/${params.chatId}`, {
        headers: {
            "Authorization": `Bearer ${userState.user?.accessToken}`
        }
    });

    return { 
        chats: await resp.json() as Chat[]
    };
};