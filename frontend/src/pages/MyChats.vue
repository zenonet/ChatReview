<script setup lang="js">
import { API_URL, appState } from '../global';


let chats = await loadChats();
console.log(chats)
async function loadChats() {
    const res = await fetch(API_URL + "/mychats/", {
        headers: {
            "Authorization": "Bearer " + appState.auth.accessToken(),
        }
    });

    return await res.json();
}

</script>

<template>

    <div class="outer">
        <div class="list">
            <div v-for="chat in chats">
                <RouterLink :to="'/edit/' + chat.id" v-text="chat.name" />
            </div>
        </div>

    </div>
</template>

<style scoped>
.outer {
    text-align: center;
    flex-direction: column;
    min-height: 100vh;
    line-height: 1.1;
    display: flex;
}

.list{
    display: flex;
    flex-direction: column;
    gap: 25px;
}

a{
    color: white;
    font-size: 2em;
    text-decoration: none;
}
</style>