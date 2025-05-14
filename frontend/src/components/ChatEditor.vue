<script setup lang="ts">
import { ref } from 'vue';
import { Chat, Message } from '../model/chat';
import ChatView from './ChatView.vue';

let chat = new Chat();
let chatViewKey = ref(0);
let message = ref("");

function sendMessageClick(){
    let msg = new Message(message.value);
    msg.isOwn = true;
    chat.messages.push(msg);
    chatViewKey.value += 1;
    message.value = "";
}

</script>

<template>
    <div style="display:flex; flex-direction: column;">
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