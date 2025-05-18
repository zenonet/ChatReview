<script setup lang="ts">
import MyChats from './MyChats.vue';


</script>

<template>
    <Suspense>
        <MyChats></MyChats>
        <template #fallback>
            <h3>Loading chats</h3>
        </template>
    </Suspense>
</template>