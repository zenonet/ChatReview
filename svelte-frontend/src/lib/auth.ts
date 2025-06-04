import { goto } from "$app/navigation";
import { page } from "$app/state";
import { PUBLIC_API_URL } from "$env/static/public";
import { User, userState } from "./state/user.svelte";
import { base64ToArrayBuffer } from "./utils";

export async function login(username: string, password: string): Promise<string | null> {
    const res = await fetch(`${PUBLIC_API_URL}/login/`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            username,
            password
        })
    })

    switch (res.status) {
        case 200:
            let content: any = await res.json();

            userState.set({
                username: username,
                accessToken: content.token
            });
            return null;
        case 401:
            return "Wrong username or password";
        default:
            return "An error occured";
    }
}

export async function register(username: string, password: string): Promise<string | null> {
    const res = await fetch(`${PUBLIC_API_URL}/register/`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            username,
            password
        })
    });

    if (res.ok) {
        const content: any = await res.json();

        userState.set({
            username: username,
            accessToken: content.token
        });

        return null;
    }

    return `Error: ${await res.text()}`
}

export async function registerPasskey(): Promise<string | null> {
    const res = await fetch(PUBLIC_API_URL + "/registerPasskey/", {
        method: "POST",
        headers: {
            "Authorization": "Bearer " + userState.user?.accessToken,
        }
    });
    let data:any = await res.json();

    // Modify the data so that browsers accept it
    data.publicKey.challenge = base64ToArrayBuffer(data.publicKey.challenge);
    data.publicKey.user.id = base64ToArrayBuffer(data.publicKey.user.id);

    let credential: Credential | null;
    try {
        credential = await navigator.credentials.create(data);
    } catch (error) {
        return "Failed to communicate with authentication hardware";
    }
    console.log(credential)

    const resp = await fetch(PUBLIC_API_URL + "/registerPasskey/complete/", {
        method: "POST",
        headers: {
            "Authorization": "Bearer " + userState.user?.accessToken,
            "Content-Type": "application/json"
        },
        body: JSON.stringify(credential)
    })

    if (resp.ok)
        return null;
    else
        return await resp.text();
}

export async function loginWithPasskey(username: string):Promise<string | null> {
    const resp = await fetch(PUBLIC_API_URL + "/loginWithPasskey/", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            username: username
        })
    });

    if (!resp.ok) {
        return await resp.text();
    }

    const res = await resp.json();
    let challenge: any = res.challenge;
    const userId = res.userId;

    challenge.publicKey.challenge = base64ToArrayBuffer(challenge.publicKey.challenge);
    challenge.publicKey.allowCredentials.forEach((x:any) => {
        x.id = base64ToArrayBuffer(x.id);
    });

    let credential: Credential | null;
    try {
        credential = await navigator.credentials.get(challenge);
    } catch (error) {
        return "Failed to communicate with authentication hardware";
    }

    const answer = await fetch(PUBLIC_API_URL + "/loginWithPasskey/complete/", {
        method: "POST",
        headers: {
            "userId": userId,
            "Content-Type": "application/json"
        },
        body: JSON.stringify(credential)
    })

    if (!answer.ok) {
        return await answer.text();
    }

    let content: any = await answer.json();
    userState.set({
        username,
        accessToken: content.token
    });
    console.log("Logged in successfully using passkey!");

    return null;
}


export function requireLogin() {
    if (userState.user === null) {
        goto(`login?redirect=${encodeURI(window.location.pathname)}`)
    }
}

export function logout() {
    userState.reset();
}

export function redirectToLogin() {
    goto(`login?redirect=${encodeURI(page.url.pathname)}`)
}

declare global {
    interface Response {
        /**
        Redirect to the login page if the request is unauthorized
        */
        maybeRedirectToLogin(): Response;
    }
}

Response.prototype.maybeRedirectToLogin = function () {
    if (this.status == 401) {
        userState.reset();
        goto(`login?redirect=${encodeURI(page.url.pathname)}`)
    }
    return this;
}