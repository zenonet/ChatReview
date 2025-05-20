
<script setup>
import { Chat } from '../model/chat';
import ChatMessage from './ChatMessage.vue'

const props = defineProps({
    chat: { required: true, type: Chat}
})

</script>

<template>
    <h2 style="text-align: center;">{{ chat.name }}</h2>
    <div class="msg-list">
        <ChatMessage v-for="msg in chat.messages" :message="msg">            
        </ChatMessage>
    </div>    
</template>

<style scoped>

.msg-list{
    display: flex;
    flex-direction: column;
    max-width: 80vw;
    margin-left: auto;
    margin-right: auto;
    min-width: 50vw;
    gap: 10px
}

</style>