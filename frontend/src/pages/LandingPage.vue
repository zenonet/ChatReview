<script setup lang="ts">
import ChatView from '@/components/ChatView.vue';
import { Chat } from '@/model/chat';
import router from '@/routes';



let demoChat: Chat = {
    id: "",
    name: "",
    description: "",
    messages: [
        {
            id: "",
            content: "I would not have thought that of you",
            isOwn: false,
            avg_rating: -2
        },
        {
            id: "",
            content: "wdym?",
            isOwn: true,
            avg_rating: 1
        },
        {
            id: "",
            content: "Well, you don't seem like somebody who would think about how others see them",
            isOwn: false,
            avg_rating: 1
        },
        {
            id: "",
            content: "What makes you think like that?",
            isOwn: true,
            avg_rating: 1
        },
        {
            id: "",
            content: "Uh, idk, just a feeling...",
            isOwn: false,
            avg_rating: 1
        },

    ]
}

function startReviewingClicked() {
    router.push("/feed")
}

function publishAChatClicked() {
    router.push("/newchat")
}

</script>


<template>

    <div class="main-container">
        <h1>A Platform for Peer-Reviewing Conversations</h1>
        <p style="font-size: 1.3em; text-align: center;">
            You feel insecure when talking to people?<br>
            <strong>Let your conversations be peer-reviewed!</strong>
        </p>

        <div class="button-row" style="font-size: 1.3em; margin-top: 40px;">
            <button class="strong" v-on:click="startReviewingClicked">Start reviewing</button>
            <button class="strong" v-on:click="publishAChatClicked">Publish a chat</button>
        </div>

        <div id="demoChat">
            <ChatView :chat="demoChat" :show-rating-indicators="false" />
        </div>
    </div>
</template>

<style lang="css" scoped>
.main-container {
    position: absolute;
    display: flex;
    margin-top: 200px;
    width: 100%;
    flex-direction: column;
    align-items: center;

    height: 200vh
}


#demoChat {
    margin-top: 200px;
}

h1 {
    text-align: center;
    font-size: 3em;
}

</style>

<style lang="css">

@keyframes demoChatAppear {
    0% {
        opacity: 0;
    }

    50% {
        opacity: 0;
    }

    70% {
        opacity: 100%;
    }

    /*     100%{
        opacity: 0;
    } */
}

/* this can't be scoped because .message is not in scope */
#demoChat .message {
    animation-name: demoChatAppear;
    animation-fill-mode: forwards;
    animation-timeline: view(y);
}

</style>