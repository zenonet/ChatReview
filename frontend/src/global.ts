import { reactive } from "vue";
import AsyncChatViewFromApi from "./components/AsyncChatViewFromApi.vue";
import { loadRouteLocation } from "vue-router";
import router from "./routes";

export const API_URL = import.meta.env.PUBLIC_API_URL;
export const WEBSOCKET_SERVER_URL = import.meta.env.PUBLIC_WEBSOCKET_SERVER_URL;

export let appState = reactive({
    redirectAfterLogin: String = null,
    auth: {
        async login(username: String, password: String): Promise<boolean> {
            let res = await fetch(API_URL + "/login/", {
                method: "POST",
                body: JSON.stringify({
                    username,
                    password
                }),
                headers: {
                    "Content-Type": "application/json"
                }
            })

            switch (res.status) {
                case 200:
                    let content: any = await res.json();
                    console.log(content)
                    this._accessToken = content.token;
                    console.log("Logged in successfully!");
                    this._loggedIn = true;
                    this.error = null;
                    this.username = username;
                    this.saveToLocalstorage()
                    return true;
                case 401:
                    this.error = "Wrong username or password";
                    break;
                default:
                    this.error = "An error occured";
            }
            return false;
        },
        async register(username: String, password: String) {
            let res = await fetch(API_URL + "/register/", {
                method: "POST",
                body: JSON.stringify({
                    username,
                    password
                }),
                headers: {
                    "Content-Type": "application/json"
                }
            });

            if (res.status == 201) {
                const content: any = res.json();
                this._accessToken = content.token;
                console.log("Registered successfully!");
                this._loggedIn = true;
                this.error = null;
                this.username = username;
                this.saveToLocalstorage()
            }
        },

        async registerPasskey(){
            const res = await fetch(API_URL + "/registerPasskey/", {
            method: "POST",
            headers: {
                "Authorization": "Bearer " + appState.auth.accessToken(),
            }
            });
            let data:CredentialCreationOptions = await res.json();

            // Modify the data so that browsers accept it
            data.publicKey.challenge = base64ToArrayBuffer(data.publicKey.challenge);
            data.publicKey.user.id = base64ToArrayBuffer(data.publicKey.user.id);

            console.log(data)


            let credential: Credential;
            try {
                credential = await navigator.credentials.create(data)
            }catch(error){
                this.error = "Failed to communicate with authentication hardware";
                return false;
            }
            console.log(credential)

            const resp = await fetch(API_URL + "/registerPasskey/complete/", {
                method: "POST",
                headers: {
                    "Authorization": "Bearer " + appState.auth.accessToken(),
                    "Content-Type": "application/json"
                },
                body: JSON.stringify(credential)
            })
            return resp.ok
        },

        async loginWithPasskey(username: String){
            const resp = await fetch(API_URL + "/loginWithPasskey/", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    username: username
                })
            });
            
            if(!resp.ok){ 
                this.error = await resp.text();
                return false;
            }

            const res = await resp.json();
            let challenge:CredentialRequestOptions = res.challenge;
            const userId = res.userId;

            challenge.publicKey.challenge = base64ToArrayBuffer(challenge.publicKey.challenge);
            challenge.publicKey.allowCredentials.forEach(x =>{
                x.id = base64ToArrayBuffer(x.id);
            });

            let credential: Credential;
            try{
                credential = await navigator.credentials.get(challenge);
            }catch(error){
                this.error = "Failed to communicate with authentication hardware";
                return false;
            }

            const answer = await fetch(API_URL + "/loginWithPasskey/complete/",{
                method: "POST",
                headers: {
                    "userId": userId,
                    "Content-Type": "application/json"
                },
                body: JSON.stringify(credential)
            })

            if(!answer.ok){ 
                this.error = await answer.text();
                return false;
            }

            let content: any = await answer.json();
            this._accessToken = content.token;
            console.log("Logged in successfully using passkey!");
            this._loggedIn = true;
            this.error = null;
            this.username = username;
            this.saveToLocalstorage()

            return true;
        },

        logout() {
            this.username = null;
            this._loggedIn = false;
            this._accessToken = null;
            this.saveToLocalstorage();
            console.log("Logged out successfully!")
        },

        loggedIn() {
            if (!this._loggedIn) this.loadFromLocalstorage();
            return this._loggedIn;
        },
        _loggedIn: false,
        error: String = null,
        _accessToken: String = null,
        username: String = null,

        accessToken() {
            if (this._accessToken == null || this._accessToken == undefined) {
                this.loadFromLocalstorage();

                if (this._accessToken != null) {
                    console.log("Loaded access token from localstorage")
                }
            }
            return this._accessToken;
        },
        loadFromLocalstorage() {
            // Insert all the values from localstorage into this
            let betterThis = JSON.parse(localStorage.getItem("auth"));
            for (const key in betterThis) {
                this[key] = betterThis[key];
            }
        },
        saveToLocalstorage() {
            localStorage.setItem("auth", JSON.stringify(this))
        },

        redirectIfNotLoggedIn() {
            if (!this.loggedIn()) {
                    appState.redirectAfterLogin = router.currentRoute.value.fullPath;
                router.push("/login");
            }
        }
    }
});

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

        appState.auth._loggedIn = false;
        appState.auth.logout();
        appState.redirectAfterLogin = router.currentRoute.value.fullPath;
        router.push("/login");
    }
    return this;
}


export function base64ToArrayBuffer(base64) {
  // Replace URL-safe characters
  base64 = base64.replace(/-/g, '+').replace(/_/g, '/');

  // Re-add base64 padding
  while (base64.length % 4 !== 0) {
    base64 += '=';
  }

  // From here, just normal base64 deserialization
  // (shamelessly stolen from https://stackoverflow.com/a/21797381)
  // Also pretty pathetic that there is no single function for this is js
  const binaryString = atob(base64);
  const bytes = new Uint8Array(binaryString.length);

  for (let i = 0; i < binaryString.length; i++) {
    bytes[i] = binaryString.charCodeAt(i);
  }

  return bytes.buffer;
}