<script lang="ts">
	import { PUBLIC_API_URL, PUBLIC_WEBSOCKET_SERVER_URL } from '$env/static/public';
	import { Chat, Message } from '$lib/chat';
	import { userState } from '$lib/state/user.svelte';
	import ChatView from '../../../ChatView.svelte';

	let message = $state('');
	let isOwn = $state(true);

	let { data } = $props();

	let chat: Chat | null = $state(data.chat);

	async function getLiveUpdates() {
		if(!chat) return;

		const sock = new WebSocket(PUBLIC_WEBSOCKET_SERVER_URL + '/ws/' + chat.id);

		sock.onopen = (e) => {
			// The token allows the api to automatically invert the chat if we're side b
			sock.send(
				JSON.stringify({
					token: 'Bearer ' + userState.user?.accessToken
				})
			);
		};

		sock.onmessage = (e) => {
			const msg = JSON.parse(e.data);
			chat?.messages.push(msg);
		};
	}

	async function sendMessage(msg: Message) {
		if(!chat) return;

		const resp = await fetch(PUBLIC_API_URL + '/message/', {
			method: 'POST',
			headers: {
				Authorization: 'Bearer ' + userState.user?.accessToken,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				content: msg.content,
				isOwn: msg.isOwn,
				chatId: chat.id,
				index: chat.messages.length - 1
			})
		});

		resp.maybeRedirectToLogin();
	}

	function sendMessageClick() {
		if (message.length === 0) return;

		let msg = new Message(message);
		msg.avg_rating = 0;
		msg.isOwn = isOwn;
		//chat.value.messages.push(msg);
		//chatViewKey.value = chatViewKey.value + 1;

		// Actually send the message

		// TODO: Show error message here

		message = '';

		console.log('Added message to chat view!');
		console.log(message);

		sendMessage(msg);
	}
	getLiveUpdates();
</script>

<div style="display:flex; flex-direction: column; min-height: 100vh;">
	{#if chat != null}
		<ChatView {chat} />
		<div style="display: flex; gap: 15px">
			<input
				class="chat-input"
				style="flex: 1"
				placeholder="Type a message here..."
				bind:value={message}
				onkeydown={(ev) => (ev.key === 'Enter' ? sendMessageClick() : 0)}
			/>
			<label class="switch">
				<input type="checkbox" bind:checked={isOwn} />
				<span class="slider"></span>
			</label>
			<button onclick={sendMessageClick}>Send</button>
		</div>
	{:else if data.status !== 200}
		<h2>Failed to load chat: {data.error}</h2>
	{:else}
		<h2 style="text-align: center;">Loading chat...</h2>
	{/if}
</div>

<style>
	.switch {
		position: relative;
		display: inline-block;
		width: 60px;
		height: 34px;
	}

	.switch input {
		opacity: 0;
		width: 0;
		height: 0;
	}

	.slider {
		position: absolute;
		cursor: pointer;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: var(--others-message-background);
		transition: 0.4s;
		transition-property: background-color;
	}

	.slider:before {
		position: absolute;
		content: '';
		height: 26px;
		width: 26px;
		left: 4px;
		bottom: 4px;
		background-color: white;
		-webkit-transition: 0.4s;
		transition: 0.1s;
	}

	input:checked + .slider {
		background-color: var(--own-message-background);
	}

	input:focus + .slider {
		box-shadow: 0 0 1px #2196f3;
	}

	input:checked + .slider:before {
		-webkit-transform: translateX(26px);
		-ms-transform: translateX(26px);
		transform: translateX(26px);
	}
</style>
