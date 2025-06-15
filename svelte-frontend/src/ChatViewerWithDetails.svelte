<script lang="ts">
	import type { Chat, Message } from '$lib/chat';
	import { untrack } from 'svelte';
	import ChatView from './ChatView.svelte';
	import MessageDetails from './MessageDetails.svelte';

	let { chat }: { chat: Chat } = $props<{ chat: Chat }>();
	
	let selectedMessage = $state<Message | null>(null);
	function onMessageClicked(msg: Message) {
		selectedMessage = msg;
	}

	$effect(() => {
		chat;

		untrack(() => {
			const id = selectedMessage?.id;
			let newMessage = chat.messages.find(m => m.id === id);
			selectedMessage = newMessage ?? null;
		})
	})
</script>

<div class="split-layout">
	<div class="chat-view">
		<ChatView {chat} messageClicked={onMessageClicked} />
	</div>
	<div id="detailsView" class={selectedMessage == null ? 'hidden' : ''}>
		{#if selectedMessage}
			<MessageDetails message={selectedMessage} />
		{/if}
	</div>
</div>

<style scoped>
	.split-layout {
		display: flex;
		flex-direction: row;
	}

	.chat-view {
		flex: 40;
	}

	#detailsView {
		flex: 40;
		min-width: 40vw;
		overflow: clip;
		margin-left: 40px;
		/*     max-width: 20px;
 */
		justify-self: flex-end;
		flex-grow: 1;
		transition: all 400ms;
		transition-duration: 1s;
		transition-timing-function: cubic-bezier();
	}

	#detailsView.hidden {
		margin-left: 0;
		flex-basis: 0;
		flex-grow: 0;
		width: 0;
		min-width: 0;
	}
</style>
