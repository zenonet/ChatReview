<script setup lang="ts">
import { Message, Comment } from '@/model/chat';
import ChatMessage from './ChatMessage.vue';
import { computed, onMounted, ref, Ref, watch } from 'vue';
import { API_URL, appState } from '@/global';


const props = defineProps({
    message: { type: Message, default: null }
})

let ratingInputVal: Ref<Number> = ref(0);

async function submitRating() {
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

    if (res.ok) {
        console.log("Rating posted successfully!")
    }
}

const commentVal: Ref<string> = ref("");
async function postComment() {
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

    if (res.ok) {
        console.log("Comment posted successfully!")
    }
}

const comments: Ref<Comment[]> = ref(null);
async function loadComments() {
    const resp = await fetch(API_URL + "/comment/forMessage/" + props.message.id);

    comments.value = await resp.json();
    console.log("Comments for message loaded!")
    console.log(comments.value)
}

let avgRating = computed(() => (props.message.avg_rating || 0).toString())


// Watch for changes of the selected message
// If the message changed, load the comments for the new message
watch(
    () => props.message,
    (_) => {
        if (props.message) loadComments();
    },
    { immediate: true }
);


function formatTimestamp(timestamp: number){
    let date = new Date(timestamp * 1000);
    date.setSeconds(0)
    const timeStr = `${date.getHours()}:${date.getMinutes()}`
    /*return `${timeStr} ${date.toDateString()}`
     return date.toLocaleString(null,  {
        seco
    }) */
   return date.toLocaleString("DE-de")
}

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
                <textarea placeholder="What do you think of this?" v-model="commentVal" />
                <button v-on:click="postComment">Submit</button>
                <h4>Other comments</h4>
                <div id="commentList" v-if="comments != null">
                    <div class="comment" v-for="comment in comments">
                        <div style="display: flex; flex-direction: row; gap: 5pt">
                            <span>{{ comment.ownerName }}</span>
                            <span class="secondary-text">{{ formatTimestamp(comment.timestamp) }}</span>
                        </div>
                        <p>{{ comment.content }}</p>
                    </div>
                </div>
                <div v-else>
                    Loading comments...
                </div>
            </div>
        </div>
    </div>
    <div v-else>No message selected</div>
</template>

<style scoped>
textarea {
    background-color: var(--background);
    color: var(--foreground);
    border: none;
    font-size: 1.2em;
    height: 2lh;
    padding: 5px;
    margin: 5px;
}

#commentList{
    display: flex;
    flex-direction: column;
    gap: 15px;
}

.comment p {
    margin-top: 5px;
    margin-bottom: 5px;
}
</style>