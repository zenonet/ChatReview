<script setup lang="ts">
import { Message } from '@/model/chat';
import ChatMessage from './ChatMessage.vue';
import { computed, ref, Ref } from 'vue';
import { API_URL, appState } from '@/global';


const props = defineProps({
    message: { type: Message, default: null }
})

let ratingInputVal: Ref<Number> = ref(0);

async function submitRating(){
    appState.auth.redirectIfNotLoggedIn();

    let rating = {
        messageId: props.message.id,
        value: ratingInputVal.value
    };
    let res = await fetch(API_URL + "/rating/", {
        method: "POST",
        headers: {
            "Authorization": "Bearer " + appState.auth.accessToken(),
            "Content-Type": "application/json"
        },
        body: JSON.stringify(rating)
    });

    res.maybeRedirectToLogin()

    if(res.ok){
        console.log("Rating posted successfully!")
    }
}

const commentVal: Ref<string> = ref("");
async function postComment(){
    appState.auth.redirectIfNotLoggedIn();

    const comment = {
        messageId: props.message.id,
        content: commentVal.value
    };

    const res = await fetch(API_URL + "/comment/", {
        method: "POST",
        headers: {
            "Authorization": "Bearer " + appState.auth.accessToken(),
            "Content-Type": "application/json"
        },
        body: JSON.stringify(comment),
    })
    res.maybeRedirectToLogin()

    if(res.ok){
        console.log("Comment posted successfully!")
    }
}

let avgRating = computed(() => (props.message.avg_rating || 0).toString())
</script>

<template>
    <div v-if="message" style="width: fit-content;">
        <h2>Message details</h2>
        <ChatMessage :message="message" />
        <div id="rating">
            <h3>Rating</h3>
            <span>Average rating: {{ avgRating }}</span>

            <div class="container">
                <h4>Your rating:</h4>
                <input type="number" v-model="ratingInputVal">
                <button v-on:click="submitRating">Submit</button>
            </div>
        </div>
        <div id="comments">
            <h3>Comments</h3>
            <div class="container">
                <h4>Your comment:</h4>
                <textarea placeholder="What do you think of this?" v-model="commentVal"/>
                <button v-on:click="postComment">Submit</button>
            </div>
        </div>
    </div>
    <div v-else>No message selected</div>
</template>

<style scoped>
textarea{
    background-color: var(--background);
    color: var(--foreground);
    border: none;
    font-size: 1.2em;
    height: 2lh;
    padding: 5px;
    margin: 5px;
}
</style>