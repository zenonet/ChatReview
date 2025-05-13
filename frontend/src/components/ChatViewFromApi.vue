<script async setup>
import ChatView from './ChatView.vue';


defineProps({
    id: { required: true, type: String }
})

console.log("Starting to load chat from API...");

const response = await fetch("http://127.0.0.1:2555/random_chat/");
const chat = await response.json();
console.log("Successfully fetched chat from api")
</script>


<template>
    <ChatView :chat="chat"/>
</template>