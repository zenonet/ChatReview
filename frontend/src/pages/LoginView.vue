<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { API_URL, appState } from '../global';


let username = ref("");
let password = ref("");

let errorCounter = ref(0);

async function loginClick() {
    if (!await appState.auth.login(username.value, password.value)) {
        errorCounter.value++;
    }else{
        errorCounter.value = 0;
    }
}

async function registerClick() {
    errorCounter.value = 0;
    await appState.auth.register(username.value, password.value);
}

let errorDisplayState = computed(() => appState.auth.error === null ? "none" : "block");

let errorMsg = computed(() =>
    appState.auth.error + (errorCounter.value > 1 ? " (" + errorCounter.value + "x)" : ""));

</script>

<template>
    <div class="outer">
        <div class="login-container">
            <div class="field-container">
                <label>Username</label>
                <input v-model="username" maxlength="128" />
            </div>

            <div class="field-container">
                <label>Password</label>
                <input v-model="password" type="password" />
            </div>

            <span style="color: red; display: none" :style="{ 'display': errorDisplayState }">
                {{ errorMsg }}
            </span>

            <div style="display: flex">
                <button v-on:click="loginClick">
                    Login
                </button>
                <button v-on:click="registerClick">
                    Register
                </button>
            </div>
        </div>
    </div>
</template>

<style scoped>
.field-container {
    display: flex;
    flex-direction: column;
}

.login-container {
    width: fit-content;
    display: flex;
    flex-direction: column;
    margin: auto;
    margin-top: 50pt;
    gap: 15pt;
}

.outer {
    font-size: large;
    min-width: 95vw;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    padding-top: 5vw;
}


input {
    padding: 10pt;
    font-size: 1.5em;
}

button {
    padding: 10pt;
    background: white;
    border: black 2px solid;
    border-radius: 10%;
}

button:active {
    transform: translateZ(5pt)
}
</style>