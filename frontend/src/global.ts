import { reactive } from "vue";
import AsyncChatViewFromApi from "./components/AsyncChatViewFromApi.vue";

export const API_URL = "http://127.0.0.1:2555"; //TODO: make https

export let appState = reactive({
    auth: {
        async login(username: String, password: String) {
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
                    let content:any = res.json();
                    this.accessToken = content.token;
                    console.log("Logged in successfully!");
                    this.loggedIn = true;
                    break;
                case 401:
                    this.error = "Wrong username or password";
                    break;
                default:
                    this.error = "An error occured";
            }
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
                this.accessToken = content.token;
                console.log("Registered successfully!");
                this.loggedIn = true;
            }
        },
        loggedIn: false,
        error: String = null,
        accessToken: String = null,
    }
});