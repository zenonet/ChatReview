<script setup lang="ts">
import { API_URL, appState } from '@/global';
import { Passkey } from '@/model/chat';
import { ref } from 'vue';


const props = defineProps({
    passkey: { required: true, type: Passkey }
})


const isRenaming = ref(false);
const isDeleted = ref(false);


async function onRenameClicked(){
    if(isRenaming.value){
        await onRenameFinished();
        return;
    }
    
    isRenaming.value = true;
    setTimeout(() => {
        document.getElementById("renameInput").focus();
    })
}

async function onRenameFinished(){
    isRenaming.value = false;
    
    const resp = await fetch(API_URL + "/passkey/rename/" + props.passkey.id, {
        method: "PUT",
        headers: {
            "Authorization": "Bearer " + appState.auth.accessToken(),
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            name: props.passkey.name
        })
    });
    
    if(resp.ok) return;
    
    console.log(await resp.text())
}


async function onDeleteClicked(){
    if (!confirm("Are you sure that you want to remove this passkey from your account?")) return;
    
    const resp = await fetch(API_URL + "/passkey/" + props.passkey.id, {
        method: "DELETE",
        headers: {
            "Authorization": "Bearer " + appState.auth.accessToken()
        }
    });
    
    if(resp.ok){
        isDeleted.value = true;
    }else{
        console.log(`Failed to delete passkey: ${await resp.text()}`);
    }
}

</script>

<template>
    <template v-if="isDeleted"></template>
    <div v-else style="display: flex; gap: 20pt; align-items: center;">
        <span v-if="!isRenaming" class="name">{{ passkey.name }}</span>
        <input v-else id="renameInput" class="name" v-model="passkey.name" v-on:keyup.enter="onRenameFinished">
        <span class="secondary-text">Registered {{ passkey.creationDate.toLocaleDateString() }}</span>
        <button class="secondary" v-on:click="onDeleteClicked">Delete</button>
        <button class="secondary" v-on:click="onRenameClicked">{{ isRenaming ? "Done" : "Rename"}}</button>
    </div>
</template>

<style scoped>
.name{
    width: 150px;
}
</style>