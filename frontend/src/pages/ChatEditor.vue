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
let isOwn = ref(false);

let chat = ref(null);

const chatId = props.chatId || route.params.id.toString();
console.log(chatId)
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

async function sendMessageClick() {
    let msg = new Message(message.value);
    msg.isOwn = isOwn.value;
    chat.value.messages.push(msg);
    chatViewKey.value += 1;
    message.value = "";

    // Actually send the message
    const resp = await fetch(API_URL + "/message/", {
        method: "POST",
        headers: {
            "Authorization": "Bearer " + appState.auth.accessToken(),
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            "content": msg.content,
            "isOwn": msg.isOwn,
            "chatId": chat.value.id,
            "index": chat.value.messages.length - 1
        })
    });

    resp.maybeRedirectToLogin()

    // TODO: Show error message here
}

loadChat()

</script>

<template>
    <div style="display:flex; flex-direction: column; min-height: 100vh;">
        <ChatView v-if="chat != null" :key="chatViewKey" :chat="chat" />
        <div style="display: flex; gap: 15px">
            <input class="chat-input" style="flex: 1" placeholder="Type a message here..." v-model="message"
                v-on:keyup.enter="sendMessageClick" />
            <label class="switch">
                <input type="checkbox" v-model="isOwn">
                <span class="slider"></span>
            </label>
            <button v-on:click="sendMessageClick">Send</button>
        </div>
    </div>
</template>

<style scoped>
.switch {
    position: relative;
    display: inline-block;
    width: 60px;
    height: 34px;
}

.switch input {
    opacity: 0;
    width: 0;
    height: 0;
}

.slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: var(--others-message-background);
    transition: 0.4s;
    transition-property: background-color;
}

.slider:before {
    position: absolute;
    content: "";
    height: 26px;
    width: 26px;
    left: 4px;
    bottom: 4px;
    background-color: white;
    -webkit-transition: .4s;
    transition: .1s;
}

input:checked+.slider {
    background-color: var(--own-message-background);
}

input:focus+.slider {
    box-shadow: 0 0 1px #2196F3;
}

input:checked+.slider:before {
    -webkit-transform: translateX(26px);
    -ms-transform: translateX(26px);
    transform: translateX(26px);
}
</style>