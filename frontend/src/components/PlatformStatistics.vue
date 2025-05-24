<script setup lang="ts">
import { API_URL } from '@/global';
import { Ref, ref } from 'vue';


class Stat {
    name: string
    value: number
}

const stats: Ref<Stat[]> = ref(null)
async function fetchStats() {
    const resp = await fetch(API_URL + "/stats/");
    stats.value = await resp.json()
    console.log(stats.value)
}

fetchStats();

</script>

<template>
    <div style="display: flex; flex-direction: column;">
        <h1 style="align-self: center;">Platform Statistics</h1>
        <div class="row" v-if="stats">
            <div class="gauge" v-for="stat in stats">
                <span class="title">{{ stat.name }}</span>
                <span class="value">{{ (Math.round(stat.value * 100) / 100).toString() }}</span>
            </div>
        </div>
        <div v-else>
            Loading stats...
        </div>
    </div>
</template>

<style scoped>
.gauge {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.gauge > .value {
    font-size: 2em;
    font-weight: bold;
}

.gauge > .title {
    text-align: center;
}

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