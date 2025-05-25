<script setup lang="ts">
import { Chat, Message } from '../model/chat';
import ChatMessage from './ChatMessage.vue';

const props = defineProps({
    chat: { default: null, type: Chat },
    showRatingIndicators: { default: true, type: Boolean }
})

const emit = defineEmits<{
    (e: "messageClicked", message: Message)
}>();

</script>

<template>
    <div v-if="chat">
        <h2 style="text-align: center;">{{ chat?.name }}</h2>
        <div class="msg-list">
            <ChatMessage 
            v-for="msg of chat?.messages" 
            :message="msg"
            :showRatingIndicator="showRatingIndicators"
            @clicked="emit('messageClicked', msg)"/>
        </div>
    </div>
    <div v-else>
        <h1>Chat hasn't loaded yet</h1>
    </div>
</template>

<style scoped>
.msg-list {
    display: flex;
    flex-direction: column;
    max-width: 80vw;
    margin-left: auto;
    margin-right: auto;
    min-width: 50vw;
    gap: 10px
}
</style>