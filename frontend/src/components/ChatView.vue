
<script setup>
import { Chat } from '../model/chat';
import ChatMessage from './ChatMessage.vue'

const props = defineProps({
    chat: { required: true, type: Chat}
})

</script>

<template>
<div>
    <div class="msg-list">
        <ChatMessage v-for="msg in chat.messages" :message="msg">            
        </ChatMessage>
    </div>    
</div>
</template>

<style scoped>


.msg-list{
    display: flex;
    flex-direction: column;
}

</style>

