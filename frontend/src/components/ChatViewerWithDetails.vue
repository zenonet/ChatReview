<script setup lang="ts">

import { Chat, Message } from "@/model/chat";
import ChatView from "./ChatView.vue";
import MessageDetails from "./MessageDetails.vue";
import { Ref, ref } from "vue";

const props = defineProps({
    chat: { type: Chat, default: null }
})
let e_chat = props.chat;
if (e_chat == null) {
    /*     e_chat = new Chat();
        e_chat.id = "deez";
        e_chat.name = "Ch-At"
        e_chat.messages = [
            {
                id: "",
                avg_rating: 0,
                content: "Deez",
                isOwn: false
            }
        ]; */
}


console.log("Details viewer is there")

let selectedMessage: Ref<Message> = ref(null);
function onMessageClicked(msg: Message) {
    selectedMessage.value = msg
}
console.log("Details viewer is there")
</script>


<template>
    <div class="split-layout">
        <ChatView id="chatView" :chat="chat" @message-clicked="onMessageClicked" />
        <div id="detailsView" :class="{ 'hidden': selectedMessage == null }">
            <MessageDetails :message="selectedMessage" />
        </div>
    </div>
</template>

<style scoped>
.split-layout {
    display: flex;
    flex-direction: row;
}

#chatView {
    flex: 40;
}

#detailsView {
    flex: 40;
    min-width: 40vw;
    overflow: clip;
    margin-left: 40px;
/*     max-width: 20px;
 */    
    justify-self: flex-end;
    flex-grow: 1;
    transition: all 400ms;
    transition-duration: 1s;
    transition-timing-function: cubic-bezier();
}

#detailsView.hidden {
    margin-left: 0;
    flex-basis: 0;
    flex-grow: 0;
    width: 0;
    min-width: 0;
}
</style>