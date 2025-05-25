<script setup lang="ts">
import ChatViewerWithDetails from '@/components/ChatViewerWithDetails.vue';
import MessageDetails from '@/components/MessageDetails.vue';
import { API_URL } from '@/global';
import { Chat } from '@/model/chat';
import { Ref, ref } from 'vue';


let chat:Ref<Chat> = ref(null);
async function loadChat(){
    let resp = await fetch(API_URL + "/chat/c13b5c10-2401-4565-9c1d-f61a59bd941d");
    chat.value = await resp.json();}


loadChat()
//v-if="chat != null" :chat="chat"
</script>

<template>
    <div>
        <ChatViewerWithDetails :chat="chat"/>
    </div>
</template>