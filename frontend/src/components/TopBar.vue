<script setup lang="ts">
import { compile, computed } from 'vue';
import { appState } from '../global';


let loginText = computed(() => {
    appState.auth.loadFromLocalstorage();
    if (appState.auth.loggedIn()) {
        return appState.auth.username;
    }
    else {
        return "Login";
    }
});

let loginClickPath = computed(() => {
    if(appState.auth.loggedIn()){
        return "/profile";
    }else {
        return "/login";
    }
});

</script>

<template>
    <nav class="bar">
        <RouterLink to="/" v-text="'Home'" />
        <RouterLink to="/mychats" v-text="'My Chats'" />
        <div style="flex: 1;"></div>
        <RouterLink :to="loginClickPath" v-text="loginText" />
    </nav>
</template>

<style scoped>
.bar {
    display: flex;
    flex-direction: row;
    gap: 20px;
    padding: 5px
}

a {
    padding: 20px;
    color: white;
    text-decoration: none;
}

a:hover {
    text-decoration: underline;
}
</style>