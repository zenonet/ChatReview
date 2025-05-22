<script setup lang="ts">
import { API_URL, appState } from '@/global';
import router from '@/routes';
import { RouterLink } from 'vue-router';

appState.auth.redirectIfNotLoggedIn();


function onLogoutClicked(){
    appState.auth.logout();
    router.replace("/login")
}

async function onDeleteAccountClicked(){
    let resp = prompt("Are you sure that you want to delete your account?\nIf you really want to delete your account type in DELETE into the field:");
    if(resp === "DELETE"){
        const res = await fetch(API_URL + "/profile/", {
            method: "DELETE",
            headers: {
                "Authorization": "Bearer " + appState.auth.accessToken()
            }
        });

        if(res.ok){
            alert("Account deleted successfully. Are you happy now?")
            appState.auth.logout();
            router.replace("/login")
        }else{
            alert("ooops, failed to delete account. Too bad.\nfr though: Contact me pls so I can fix this")
        }
    }else{
        alert("Nope, ain't gonna happen as long as you can't spell")
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
