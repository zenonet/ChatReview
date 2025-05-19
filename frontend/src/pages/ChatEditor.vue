<script setup lang="ts">
import { reactive, Ref, ref } from 'vue';
import { Chat, Message } from '../model/chat';
import ChatView from './../components/ChatView.vue';
import { API_URL, appState } from '../global';
import { useRoute } from 'vue-router';


const props = defineProps({
    chatId: String || null
})

const route = useRoute();


let chatViewKey = ref(0);
let message = ref("");
let isOwn = ref(true);

let chat = ref(null);

const chatId = props.chatId || route.params.id.toString();
async function loadChat() {
    const response = await fetch(API_URL + "/chat/" + chatId,
         {
            headers: {
                "Authorization": "Bearer " + appState.auth.accessToken()
            }
         }
    );
    chat.value = await response.json();
}

async function sendMessageClick(){
    let msg = new Message(message.value);
    msg.isOwn = isOwn.value;
    chat.value.messages.push(msg);
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
            "chatId": chat.value.id,
            "index": chat.value.messages.length-1
        })
    })
}

loadChat()

</script>

<template>
    <div style="display:flex; flex-direction: column; min-height: 100vh;">
        <ChatView :key="chatViewKey" :chat="chat" />
        <div style="display: flex; gap: 15px">
            <input class="chat-input" style="flex: 1" placeholder="Type a message here..." v-model="message" v-on:keyup.enter="sendMessageClick"/>
            <label>yours?</label>
            <input type="checkbox" v-model="isOwn">
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