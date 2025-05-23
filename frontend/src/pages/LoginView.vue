<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { API_URL, appState } from '../global';
import router from '@/routes';
import { useRoute } from 'vue-router';

const route = useRoute();

let username = ref("");
let password = ref("");

let errorCounter = ref(0);

async function loginClick() {
    if (!await appState.auth.login(username.value, password.value)) {
        errorCounter.value++;
    }else{
        errorCounter.value = 0;
        if(appState.redirectAfterLogin != null){
            const redirect = appState.redirectAfterLogin;
            console.log("Redirecting to " + redirect + " after login...")
            appState.redirectAfterLogin = null;
            router.replace({
                path: redirect
            })
        }else{
            router.replace("/")
        }
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
    <div class="page">
        <div class="login-container">
            <h1>Login</h1>
            <div class="field-container">
                <label>Username</label>
                <input v-model="username" maxlength="128" />
            </div>

            <div class="field-container">
                <label>Password</label>
                <input v-model="password" v-on:keyup.enter="loginClick" type="password" />
            </div>

            <span style="color: red; display: none" :style="{ 'display': errorDisplayState }">
                {{ errorMsg }}
            </span>

            <div class="button-row">
                <button v-on:click="loginClick">
                    Login
                </button>
                <button class="secondary" v-on:click="registerClick">
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

input {
    padding: 10pt;
    font-size: 1.5em;
}

</style>