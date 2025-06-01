

<script lang="ts">
	import type { Chat } from '$lib/chat';
	import ChatView from '../../ChatView.svelte';
	import ChatViewerWithDetails from '../../ChatViewerWithDetails.svelte';

    // imported from load function
    export let data: Chat;
</script>

<ChatViewerWithDetails chat={data}></ChatViewerWithDetails>