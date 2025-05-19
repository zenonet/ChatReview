<script setup lang="ts">
import MyChats from './MyChats.vue';


</script>

<template>
    <Suspense>
        <MyChats></MyChats>
        <template #fallback>
            <div style="min-height: 100vh;"></div>
            <h3>Loading chats</h3>
        </template>
    </Suspense>
</template>