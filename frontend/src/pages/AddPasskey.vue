<script setup lang="ts">
import PasskeyListEntry from '@/components/PasskeyListEntry.vue';
import { API_URL, appState, base64ToArrayBuffer } from '@/global';
import { Passkey } from '@/model/chat';
import { Ref, ref } from 'vue';



appState.auth.redirectIfNotLoggedIn();

let success = ref(false);
async function onStartButtonClicked() {

    success.value = await appState.auth.registerPasskey();
}
const passkeys: Ref<Passkey[]> = ref(null);
async function loadPasskeys(){
    const resp = await fetch(API_URL + "/passkeys/", {
        headers: {
            "Authorization": "Bearer " + appState.auth.accessToken()
        }
    });

    resp.maybeRedirectToLogin();
    if(!resp.ok) return; // TODO: maybe show an error message here

    const pks = await resp.json();
    pks.forEach(pk => {
        pk.creationDate = new Date(pk.creationDate * 1000)
    });

    passkeys.value = pks;

    console.log(passkeys)
}


loadPasskeys()

</script>

<template>
<div class="container" style="align-items: center;">
    <h1>Passkeys</h1>
    <p>Here, you can add a passkey that can be used for secure passwordless authentication.</p>
    <button v-on:click="onStartButtonClicked">Start</button>

    <h2 v-show="success">Successfully added passkey!</h2>


    <h2>Your passkeys</h2>
    <div v-if="passkeys && passkeys.length > 0">
        <PasskeyListEntry v-for="pk in passkeys" :passkey="pk"/>
    </div>
    <h4 v-else>You don't have any passkeys</h4>
</div>
</template>