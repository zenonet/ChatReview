<script setup lang="ts">
import { defineProps, defineEmits, ref } from 'vue'
defineProps({
    show: {
        type: Boolean,
        default: false
    }
})

let ratingVal = ref(0);

const emit = defineEmits<{
    (e: "postRating", value: Number)
}>();


function sendRating(){
    emit('postRating', ratingVal.value);
}

</script>

<template>
    <div class="popup" :class="{ 'show': show }">
        <div class="menu">
            <a>Comment</a>
            <div style="display: flex; flex-direction: column;">
                <span>Rating</span>
                <div class="button-row">
                    <input type="number" placeholder="Your rating..." style="width: 60px;" v-model="ratingVal" v-on:onkeyup.enter="sendRating">
                    <button class="secondary" style="font-size: 0.8em;" v-on:click="sendRating">Submit</button>
                </div>
            </div>
        </div>
    </div>
</template>

<style>
.menu {
    display: flex;
    flex-direction: column;
    background-color: var(--background);
    padding: 10px;
    border-radius: 5px;
    border: var(--primary) solid 2px;

}

.popup {
    display: none;
    text-align: left;
    padding-left: 30px;
    padding-right: 30px;
    color: white;
    z-index: 1000
}

.popup:hover,
div:hover>*>.popup {
    display: block;
    position: absolute;
    width: fit-content;
    right: 0;
    top: 0;

    transform: translateX(100%);
}
</style>