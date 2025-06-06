import { PUBLIC_API_URL } from "$env/static/public";
import type { Chat, Passkey } from "$lib/chat";
import { userState } from "$lib/state/user.svelte";


export const load = async () => {
    console.log("Loading passkeys from API...")
    const resp = await fetch(`${PUBLIC_API_URL}/passkeys/`, {
        headers: {
            "Authorization": `Bearer ${userState.user?.accessToken}`
        }
    });

    if (!resp.ok) {
        return {
            passkeys: null,
            status: resp.status
        }
    }

    const pks = await resp.json();
    pks.forEach((pk: any) => {
        pk.creationDate = new Date(pk.creationDate * 1000);
    });

    return {
        passkeys: pks as Passkey[]
    };
};