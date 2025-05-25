<script setup lang="ts">
import ChatViewerWithDetails from '@/components/ChatViewerWithDetails.vue';
import { API_URL } from '@/global';
import { Chat } from '@/model/chat';
import { ref, Ref } from 'vue';




const chat: Ref<Chat> = ref(null);

async function loadRandomChat() {
    const response = await fetch(API_URL + "/random_chat/");
    chat.value = await response.json();
}

loadRandomChat()

</script>

<template>
    <div class="container" style="align-items: center">
        <ChatViewerWithDetails v-if="chat" :chat="chat"/>
        <div v-else>
            <h2 style="text-align: center;">Loading chat...</h2>
        </div>
    </div>
</template>