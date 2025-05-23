<script setup lang="ts">
import ChatViewerWithDetails from '@/components/ChatViewerWithDetails.vue';
import MessageDetails from '@/components/MessageDetails.vue';
import { API_URL } from '@/global';
import { Chat } from '@/model/chat';
import { Ref, ref } from 'vue';


let chat:Ref<Chat> = ref(null);
async function loadChat(){
    let resp = await fetch(API_URL + "/random_chat/");
    chat.value = await resp.json();
    console.log("chat loaded successfully")
}


loadChat()
//v-if="chat != null" :chat="chat"
</script>

<template>
    <div>
        <ChatViewerWithDetails :chat="chat"/>
    </div>
</template>