<script setup lang="ts">
import { API_URL, appState } from '@/global';
import { ref } from 'vue';



function base64ToArrayBuffer(base64) {
  // Replace URL-safe characters
  base64 = base64.replace(/-/g, '+').replace(/_/g, '/');

  // Re-add base64 padding
  while (base64.length % 4 !== 0) {
    base64 += '=';
  }

  // From here, just normal base64 deserialization
  // (shamelessly stolen from https://stackoverflow.com/a/21797381)
  // Also pretty pathetic that there is no single function for this is js
  const binaryString = atob(base64);
  const bytes = new Uint8Array(binaryString.length);

  for (let i = 0; i < binaryString.length; i++) {
    bytes[i] = binaryString.charCodeAt(i);
  }

  return bytes.buffer;
}


let success = ref(false);
async function onStartButtonClicked() {

    const res = await fetch(API_URL + "/registerPasskey/", {
        method: "POST",
        headers: {
            "Authorization": "Bearer " + appState.auth.accessToken(),
        }
    });
    let data:CredentialCreationOptions = await res.json();

    // Modify the data so that browsers accept it
    data.publicKey.challenge = base64ToArrayBuffer(data.publicKey.challenge);
    data.publicKey.user.id = base64ToArrayBuffer(data.publicKey.user.id);

    console.log(data)
    const credential = await navigator.credentials.create(data)
    console.log(credential)

    const resp = await fetch(API_URL + "/registerPasskey/complete/", {
        method: "POST",
        headers: {
            "Authorization": "Bearer " + appState.auth.accessToken(),
            "Content-Type": "application/json"
        },
        body: JSON.stringify(credential)
    })
    success.value = resp.ok
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