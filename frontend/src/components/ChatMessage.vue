<script setup lang="ts">
import { ref } from 'vue';
import { API_URL, appState } from '../global';
import { Message } from '../model/chat';
import MessageActionPopup from './MessageActionPopup.vue';

const props = defineProps({
    message: { required: true, type: Message },
    showRatingIndicator: { default: true, type: Boolean },
})


const emit = defineEmits<{
    (e: "clicked")
}>();


async function postRating(value: Number){
    let rating = {
        messageId: props.message.id,
        value
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

</script>



<template>
    <div class="layout-container" :class="{ 'own-message': message.isOwn, 'others-message': !message.isOwn }">
        <div v-show="showRatingIndicator" class="rating-indicator">{{ message.avg_rating.toString() }}</div>
        <div class="message" v-on:mousedown="$emit('clicked')">
            <p style="margin: 0;">
                {{ message.content }}
            </p>
        </div>
    </div>
</template>


<style scoped>
.layout-container {
    margin: 2pt;
    width: fit-content;
    display: flex;
    flex-direction: column;
}

.message {
    background: var(--own-message-background);
    color: black;
    padding: 15pt;

    border-radius: 20px;
    font-size: 1.5em;
    text-align: left;
    /* TOOD: Think about this, this might be a stupid idea */
    user-select: none;

    position: relative;
}

.others-message {
    align-self: flex-start;
    border-bottom-left-radius: 0;
}
.others-message .message{
    background: var(--others-message-background);
    border-bottom-left-radius: 0;
}

.own-message {
    align-self: flex-end;
    border-bottom-right-radius: 0;
}

.own-message .message{
    border-bottom-right-radius: 0;
}


.rating-indicator{
    font-size: 0.75em;
    align-self: flex-end;
}

.others-message .rating-indicator{
    align-self: flex-start;
}
</style>