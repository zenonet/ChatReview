<script setup lang="ts">
import { appState } from '@/global';
import router from '@/routes';
import { RouterLink } from 'vue-router';

appState.auth.redirectIfNotLoggedIn();


function onLogoutClicked(){
    appState.auth.logout();
    router.replace("/login")
}

function onDeleteAccountClicked(){
    let resp = prompt("Are you sure that you want to delete your account?\nIf you really want to delete your account type in DELETE into the field:");
    if(resp === "DELETE"){
        console.log("Cannot delete profile! (Not implemented yet)")
    }
}

</script>

<template>
    <div class="page" style="justify-content: flex-start">
        <div style="margin-left: auto; margin-right: auto;">
            <h2>Welcome {{ appState.auth.username }}!</h2>
            <p>
                Here you can adjust your profile
            </p>
            <div style="height: 30px;"/>

            <div id="links" style="display: flex; flex-direction: column; align-items: flex-start; gap: 15pt">
                <RouterLink to="/mychats" v-text="'My Chats'"/>
                <button class="secondary" v-on:click="onLogoutClicked">Log out</button>
                <button class="secondary" v-on:click="onDeleteAccountClicked">Delete account</button>
            </div>
        </div>
    </div>
</template>
