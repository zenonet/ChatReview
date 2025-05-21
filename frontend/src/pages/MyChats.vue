<script setup lang="js">
import { API_URL, appState } from '../global';
import router from '../routes';

let chats = await loadChats();
console.log(chats)
async function loadChats() {
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

</script>

<template>

    <div class="page" style="justify-content: flex-start;">
        <div style="display: flex; max-width: 80vw;">
            <div style="flex: 1"></div>
            <button v-on:click="createChatClicked">New Chat</button>
        </div>
        <div class="list">
            <h2>Your chats:</h2>
            <div v-for="chat in chats">
                <RouterLink :to="'/edit/' + chat.id" v-text="chat.name" />
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