import { goto } from "$app/navigation";
import { PUBLIC_API_URL } from "$env/static/public";
import { User, userState } from "./state/user.svelte";

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



// TODO: add functions for passkey auth

export function requireLogin(){
    if(userState.user === null){
        goto(`/login?redirect=${encodeURI(window.location.pathname)}`)
    }
}

export function logout(){
    userState.reset();
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
        goto(`/login?redirect=${encodeURI(window.location.pathname)}`)
    }
    return this;
}