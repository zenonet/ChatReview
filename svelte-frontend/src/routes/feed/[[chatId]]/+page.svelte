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
		if (data.id && !page.params.chatId) {
			goto(page.url.href + '/' + data.id);
		}
	}
</script>

<div style="display: flex; flex-direction: column; align-items: center; min-height: 80vh">
	<ChatViewerWithDetails chat={data}></ChatViewerWithDetails>
	<button style="margin-top: auto" onclick={onNextChatClicked}>Next Chat</button>
</div>
