import { reactive } from "vue";
import AsyncChatViewFromApi from "./components/AsyncChatViewFromApi.vue";

export const API_URL = "http://127.0.0.1:2555"; //TODO: make https

export let appState = reactive({
    auth: {
        async login(username: String, password: String):Promise<boolean> {
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

            switch(res.status){
                case 200:
                    let content:any = await res.json();
                    console.log(content)
                    this._accessToken = content.token;
                    console.log("Logged in successfully!");
                    console.log(this._accessToken)
                    this.loggedIn = true;
                    this.error = null;
                    localStorage.setItem("accessToken", this._accessToken)
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

            if(res.status == 201){
                const content:any = res.json();
                this._accessToken = content.token;
                console.log("Registered successfully!");
                localStorage.setItem("accessToken", this._accessToken)
                this.loggedIn = true;
                this.error = null;
            }
        },
        loggedIn: false,
        error: String = null,
        _accessToken: String = null,

        accessToken() {
            if(this._accessToken == null || this._accessToken == undefined){
                this._accessToken = localStorage.getItem("accessToken");
                if(this._accessToken != null){
                    console.log("Loaded access token from localstorage")
                }
            }
            return this._accessToken;
        }
    }
});