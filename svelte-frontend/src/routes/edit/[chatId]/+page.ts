import { page } from "$app/state";
import { PUBLIC_API_URL } from "$env/static/public";
import type { Chat } from "$lib/chat";
import { userState } from "$lib/state/user.svelte";


import type { PageLoad } from '../$types';
export const load: PageLoad = async ({params}) => {

    const resp = await fetch(`${PUBLIC_API_URL}/chat/fromMyPerspective/${params.chatId}`, {
        headers: {
            "Authorization": `Bearer ${userState.user?.accessToken}`
        }
    });

    if(!resp.ok){
        return {
            chat: null,
            status: resp.status,
            error: await resp.text()
        }
    }

    return { 
        chat: await resp.json() as Chat,
        status: resp.status,
        error: null,
    };
};