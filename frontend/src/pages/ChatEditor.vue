<script setup lang="ts">
import { reactive, Ref, ref } from 'vue';
import { Chat, Message } from '../model/chat';
import ChatView from './../components/ChatView.vue';
import { API_URL, appState, WEBSOCKET_SERVER_URL } from '../global';
import { useRoute } from 'vue-router';


/* const props = defineProps({
    chatId: String || null
}) */

const route = useRoute();


let chatViewKey = ref(0);
let message = ref("");
let isOwn = ref(true);

let chat = ref(null);

const chatId = route.params.id.toString();
console.log(chatId)
async function loadChat() {
    const response = await fetch(API_URL + "/chat/fromMyPerspective/" + chatId,
        {
            headers: {
                "Authorization": "Bearer " + appState.auth.accessToken()
            }
        }
    );
    chat.value = await response.json();
}

async function getLiveUpdates() {
    const sock = new WebSocket(WEBSOCKET_SERVER_URL + "/ws/" + chatId);

    sock.onopen = (e) => {
        // The token allows the api to automatically invert the chat if we're side b
        sock.send(JSON.stringify({
            token: "Bearer " + appState.auth.accessToken()
        }))
    }


    sock.onmessage = (e) => {
        const msg = JSON.parse(e.data);
        chat.value.messages.push(msg);
        chatViewKey.value = chatViewKey.value + 1;
    };
}


async function sendMessage(msg: Message) {
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
}

function sendMessageClick() {
    let msg = new Message(message.value);
    msg.avg_rating = 0;
    msg.isOwn = isOwn.value;
    //chat.value.messages.push(msg);
    //chatViewKey.value = chatViewKey.value + 1;


    // Actually send the message

    // TODO: Show error message here

    message.value = "";

    console.log("Added message to chat view!")
    console.log(message.value)

    sendMessage(msg)
}
loadChat()
getLiveUpdates()
</script>

<template>
    <div style="display:flex; flex-direction: column; min-height: 100vh;">
        <template v-if="chat != null">
            <ChatView :key="chatViewKey" :chat="chat" />
            <div style="display: flex; gap: 15px">
                <input class="chat-input" style="flex: 1" placeholder="Type a message here..." v-model="message"
                    v-on:keyup.enter="sendMessageClick" />
                <label class="switch">
                    <input type="checkbox" v-model="isOwn">
                    <span class="slider"></span>
                </label>
                <button v-on:click="sendMessageClick">Send</button>
            </div>
        </template>
        <h2 style="text-align: center;" v-else>Loading chat...</h2>
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