<script setup lang="ts">
import { Chat } from '@/model/chat';
import { API_URL, appState } from '../global';
import router from '../routes';

let chats = await loadChats();
console.log(chats)
async function loadChats(): Promise<Chat[]> {
    const res = await fetch(API_URL + "/mychats/", {
        headers: {
            "Authorization": "Bearer " + appState.auth.accessToken(),
        }
    });

    res.maybeRedirectToLogin()

    return await res.json();
}

async function createChatClicked() {
    router.push("/newchat");
}

async function createChatWithRandomClicked() {
    appState.auth.redirectIfNotLoggedIn();
    const resp = await fetch(API_URL + "/chat/random/", {
        method: "POST",
        headers: {
            "Authorization": "Bearer " + appState.auth.accessToken(),
        }
    })

    if (!resp.ok) return;

    let chatId = await resp.text();

    if (resp.status == 200) {
        console.log("Found a chat partner!");
        router.push("/edit/" + chatId)
    } else if (resp.status == 201) {
        console.log("Created chat request");
        // TODO
    }
}

</script>

<template>

    <div class="page" style="justify-content: flex-start;">
        <div class="button-row" style="max-width: 80vw;">
            <div style="flex: 1"></div>
            <button v-on:click="createChatClicked">New Chat</button>
            <button v-on:click="createChatWithRandomClicked">Chat with random</button>
        </div>
        <div class="list">
            <h2>Your chats:</h2>
            <div v-for="chat in chats">
                <RouterLink :to="'/edit/' + chat.id" v-text="chat.name + (chat.isPendingRequest ? '  (pending)' : '')" />
            </div>
        </div>

    </div>
</template>

<style scoped>
.list {
    margin-left: auto;
    margin-right: auto;
    display: flex;
    flex-direction: column;
    gap: 25px;
    text-align: left;
}
</style>