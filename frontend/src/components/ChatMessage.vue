<script setup lang="ts">
import { API_URL, appState } from '../global';
import { Message } from '../model/chat';

const props = defineProps({
    message: { required: true, type: Message }
})


async function clicked() {
    // TODO: Replace all popup stuff with nice UI
    console.log(appState.auth)
    if (!appState.auth.loggedIn()) {
        alert("Login to rate messages");
        return;
    }

    let resp = window.prompt("How would you rate this message?");
    let value: Number = +resp;

    if (value == null || value == undefined) {
        return;
    }

    let rating = {
        messageId: props.message.id,
        value
    };

    // Post rating
    let res = await fetch(API_URL + "/rating/", {
        method: "POST",
        headers: {
            "Authorization": "Bearer " + appState.auth.accessToken(),
            "Content-Type": "application/json"
        },
        body: JSON.stringify(rating)
    });

    if (res.ok) {
        alert("Rating accepted")
    } else {
        alert("An error occured")
    }
}

</script>



<template>
    <div class="layout-container" :class="{ 'own-message': message.isOwn, 'others-message': !message.isOwn }">
        <div class="rating-indicator">{{ message.avg_rating.toString() }}</div>
        <div class="message" v-on:mousedown="clicked">
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
    background: lime;
    color: black;
    padding: 15pt;

    border-radius: 20px;
    font-size: 1.5em;
    user-select: none;
    text-align: left;
    /* TOOD: Think about this, this might be a stupid idea */
}

.others-message {
    align-self: flex-start;
    border-bottom-left-radius: 0;
}
.others-message .message{
    background: cyan;
}

.own-message {
    align-self: flex-end;
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