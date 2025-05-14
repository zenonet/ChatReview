<script setup lang="ts">
import { computed, ref } from 'vue';
import App from './App.vue';
import LoginView from './components/LoginView.vue';
import ChatEditor from './components/ChatEditor.vue';
import NotFound from './NotFound.vue';



const routes = {
    "/": App,
    "/login": LoginView,
    "/editor": ChatEditor,
}

const currentPath = ref(window.location.hash);
window.addEventListener('hashchange', () => {
    currentPath.value = window.location.hash
})

const currentView = computed(() => {
    return routes[currentPath.value.slice(1) || '/'] || NotFound
});

</script>

<template>
    <component :is="currentView"/>
</template>