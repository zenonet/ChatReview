<script setup lang="ts">
import { API_URL, appState, base64ToArrayBuffer } from '@/global';
import { ref } from 'vue';


let success = ref(false);
async function onStartButtonClicked() {

    success.value = await appState.auth.registerPasskey();
}

</script>


<template>
<div class="container" style="align-items: center;">
    <h1>Add Passkey</h1>
    <p>Here, you can add a passkey that can be used for secure passwordless authentication.</p>
    <button v-on:click="onStartButtonClicked">Start</button>

    <h2 v-show="success">Successfully added passkey!</h2>
</div>
</template>