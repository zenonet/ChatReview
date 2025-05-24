<script setup lang="ts">
import { API_URL, appState } from '@/global';
import { Ref, ref } from 'vue';
import StatGauge from './StatGauge.vue'
import { Stat } from '@/model/chat';


const stats: Ref<Stat[]> = ref(null)
async function fetchStats() {
    const resp = await fetch(API_URL + "/stats/");
    stats.value = await resp.json()
    console.log(stats.value)
}

fetchStats();

</script>

<template>
    <div style="display: flex; flex-direction: column; align-items: center;">
        <h1>Platform Statistics</h1>
        <template v-if="stats">
            <div class="row">
                <StatGauge v-for="stat in stats" :stat="stat" />
                <StatGauge v-if="appState.auth.loggedIn()" :stat="{ name: appState.auth.username + 's', value: 1 }" />
                <template v-else />
            </div>
            <p class="secondary-text" style="margin-top: 40px;">This would probably look more impressive if people actually used this</p>
        </template>
        <div v-else>
            Loading stats...
        </div>
    </div>
</template>

<style scoped>
.row {
    display: flex;
    flex-direction: row;
    justify-items: center;
    margin-top: 80pt;
    padding-left: 20pt;
    padding-right: 20pt;
    justify-content: space-evenly;
    gap: 25pt;
    flex-wrap: wrap;
}
</style>