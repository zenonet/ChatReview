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

let avgRating = computed(() => (props.message.avg_rating || 0).toString())
</script>

<template>
    <div v-if="message" style="width: fit-content;">
        <h2>Message details</h2>
        <ChatMessage :message="message" />
        <div id="rating">
            <h3>Rating</h3>
            <span>Average rating: {{ avgRating }}</span>

            <div style="padding: 5pt">
                <h4>Your rating:</h4>
                <input type="number" v-model="ratingInputVal"></input>
                <button v-on:click="submitRating">Submit</button>
            </div>
        </div>
        <div id="comments">
            <h3>Comments</h3>
        </div>
    </div>
    <div v-else>No message selected</div>
</template>