<script setup lang="ts">
import ChatViewFromApi from './ChatViewFromApi.vue';

let id = "looll";
</script>

<template>
    <Suspense>
        <template #default>
            <ChatViewFromApi :id="id"/>
        </template>
        <template #fallback>
            <div>
                Loading chat...
            </div>
        </template>
    </Suspense>
</template>