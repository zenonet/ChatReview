<script setup lang="ts">
import ChatViewerWithDetails from '@/components/ChatViewerWithDetails.vue';
import { API_URL } from '@/global';
import { Chat } from '@/model/chat';
import router from '@/routes';
import { ref, Ref } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();

const chat: Ref<Chat> = ref(null);
const error: Ref<String> = ref(null);

async function loadChat(){
    const resp = await fetch(API_URL + "/chat/" + route.params.id)

    if(resp.ok){
        error.value = null;
        chat.value = await resp.json();
    }else{
        error.value = "Chat could not be loaded"
    }

}

loadChat()

</script>

<template>
    <ChatViewerWithDetails v-if="chat" :chat="chat"/>
    <h2 v-else-if="!error" style="text-align: center;">Loading chat...</h2>
    <h2 v-else style="text-align: center">An error occured while loading the chat. It probably does not exist</h2>
</template>