<script lang="ts">
	import { goto, invalidate, invalidateAll } from '$app/navigation';
	import type { Chat } from '$lib/chat';
	import ChatView from '../../ChatView.svelte';
	import ChatViewerWithDetails from '../../ChatViewerWithDetails.svelte';

	// imported from load function
	export let data: Chat;
</script>

<div style="display: flex; flex-direction: column; align-items: center; min-height: 80vh">
	<ChatViewerWithDetails chat={data}></ChatViewerWithDetails>
	<button style="margin-top: auto" onclick={async () => await invalidateAll()}>Next Chat</button>
</div>
