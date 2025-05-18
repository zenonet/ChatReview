<script setup>
import { RouterLink, RouterView } from 'vue-router';
import TopBar from './components/TopBar.vue';

</script>

<template>
    <TopBar />
    <div id="app">
        <RouterView></RouterView>
    </div>
</template>