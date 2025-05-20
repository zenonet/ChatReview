<script async setup lang="ts">
import { API_URL } from '../global';
import { Chat } from '../model/chat';
import ChatView from './ChatView.vue';


defineProps({
    id: { required: true, type: String }
})

console.log("Starting to load chat from API...");

const response = await fetch(API_URL + "/random_chat/");
const chat:Chat = await response.json();
console.log("Successfully fetched chat from api")
</script>


<template>
    <ChatView :chat="chat"/>
</template>