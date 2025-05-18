<script setup lang="ts">
import { ref } from 'vue';
import { Chat, Message } from '../model/chat';
import ChatView from './../components/ChatView.vue';
import { API_URL, appState } from '../global';
import { useRoute } from 'vue-router';


const props = defineProps({
    chatId: String
})

const route = useRoute();

let chat = new Chat();
chat.id = props.chatId || route.params.id.toString();
let chatViewKey = ref(0);
let message = ref("");

async function sendMessageClick(){
    let msg = new Message(message.value);
    msg.isOwn = true;
    chat.messages.push(msg);
    chatViewKey.value += 1;
    message.value = "";

    // Actually send the message
    fetch(API_URL + "/message/", {
        method: "POST",
        headers: {
            "Authorization": "Bearer " + appState.auth.accessToken(),
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            "content": msg.content,
            "isOwn": msg.isOwn,
            "chatId": chat.id,
            "index": chat.messages.length-1
        })
    })
}

</script>

<template>
    <div style="display:flex; flex-direction: column; min-height: 100vh;">
        <ChatView :key="chatViewKey" :chat="chat" />
        <div style="display: flex; gap: 15px">
            <input class="chat-input" style="flex: 1" placeholder="Type a message here..." v-model="message" v-on:keyup.enter="sendMessageClick"/>
            <button v-on:click="sendMessageClick">Send</button>
        </div>
    </div>
</template>

<style scoped>
.chat-input{
    padding: 5px;
    border: none
}

button{
    background: white;
}
</style>