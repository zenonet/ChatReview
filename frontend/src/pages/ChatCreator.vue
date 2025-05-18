<script setup lang="ts">
import { ref } from 'vue';
import { API_URL, appState } from '../global';
import router from '../routes';


let chatName = ref("")


async function createChat(){

    console.log("Using token:" + appState.auth._accessToken)

    let res = await fetch(API_URL + "/chat/",{
        method: "POST",
        headers: {
            "Authorization": "Bearer " +  appState.auth.accessToken(),
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            name: chatName.value,
        })
    })

    if(res.ok){
        const chatId = await res.text();
        router.replace("/edit/" + chatId);
        return;
    }else{
        console.log("Creating chat failed")
        //TODO: Show error message
    }
}

</script>

<template>
    <div class="outer">
        <div class="form-container">
            <div class="field-container">
                <label>Chat Name</label>
                <input v-model="chatName" maxlength="128" />
            </div>
            <button v-on:click="createChat">Create</button>

        </div>
    </div>
</template>


<style scoped>
.outer {
    font-size: large;
    min-width: 95vw;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    padding-top: 5vw;
}

.form-container {
    width: fit-content;
    display: flex;
    flex-direction: column;
    margin: auto;
    margin-top: 50pt;
    gap: 15pt;
}

.field-container{
    display: flex;
    flex-direction: column;
}
</style>