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
    res.maybeRedirectToLogin();

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
    <div class="page">
        <div class="form-container">
            <h2>Create a new chat</h2>
            <p>
                Here you can create a new chat. After creating it,
                you can recreate the dialogue you've had and others can peer-review it.
            </p>
            <div class="field-container">
                <label>Chat Name</label>
                <input v-model="chatName" maxlength="128" />
            </div>
            <div>
                <button v-on:click="createChat">Create</button>
            </div>

        </div>
    </div>
</template>


<style scoped>

.form-container {
    width: fit-content;
    display: flex;
    flex-direction: column;
    margin: auto;
    margin-top: 50pt;
    gap: 5pt;
}

h2{
    margin: 0;
}

.field-container{
    display: flex;
    flex-direction: column;
}

button{
    font-size: large;
}
</style>