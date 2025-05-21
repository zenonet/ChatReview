<script setup lang="ts">
import MyChats from './MyChats.vue';


</script>

<template>
    <Suspense>
        <MyChats />
        <template #fallback>
            <div>
                <div style="min-height: 100vh;"></div>
                <h3>Loading chats</h3>
            </div>
        </template>
    </Suspense>
</template>