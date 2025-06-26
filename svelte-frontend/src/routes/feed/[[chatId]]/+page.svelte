<script lang="ts">
	import { goto, invalidate, invalidateAll, replaceState } from '$app/navigation';
	import { page } from '$app/state';
	import { onMount, tick, untrack } from 'svelte';
	import ChatViewerWithDetails from '../../../ChatViewerWithDetails.svelte';
	import type { PageProps } from './$types';
	import { base } from '$app/paths';

	// imported from load function
	let { data }: PageProps = $props();

	tick().then(() => {
		updateUrl()
	});

	$effect(() => {
		data;
		tick().then(() => updateUrl());
	});

	async function onNextChatClicked() {
		await goto(`${base}/feed/`, {
			replaceState: false
		});
	}

	function updateUrl() {
		if(!data.chat) return;
		if (data.chat.id && !page.params.chatId) {
			goto(page.url.href + '/' + data.chat.id);
		}
	}
</script>

<div style="display: flex; flex-direction: column; align-items: center; min-height: 80vh">
	{#if data.chat}
		<ChatViewerWithDetails chat={data.chat}></ChatViewerWithDetails>
	{:else}
		<h2>Failed to load chat</h2>
	{/if}
	<button style="margin-top: auto" onclick={onNextChatClicked}>Next Chat</button>
</div>
