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
                    console.log(this._accessToken)
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