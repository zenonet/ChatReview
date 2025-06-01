<script lang="ts">
	import ChatMessage from './ChatMessage.svelte';

	const { chat, messageClicked } = $props();
    let showRatingIndicators = false;
</script>

<div class="container" style="align-items: center">
	<h2 style="text-align: center;">{chat?.name}</h2>
	{#if chat.description}
		<p>{chat.description}</p>
	{/if}
	<div class="msg-list">
		{#each chat.messages as msg}
			<ChatMessage
				message={msg}
				showRatingIndicator="showRatingIndicators"
				onclick={ () => messageClicked(msg) }
			/>
		{/each}
	</div>
</div>

<style>
	.msg-list {
		display: flex;
		flex-direction: column;
		max-width: 80vw;
		margin-left: auto;
		margin-right: auto;
		min-width: 50vw;
		gap: 10px;
	}
</style>
