<script lang="ts">
	import { goto } from '$app/navigation';
	import { PUBLIC_API_URL } from '$env/static/public';
	import { redirectToLogin, requireLogin } from '$lib/auth';
	import type { Chat } from '$lib/chat';
	import { userState } from '$lib/state/user.svelte';

	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	if (data.status === 401) {
		redirectToLogin();
	}

	if (data.chats == null) {
		redirectToLogin();
	}

	let chats = data.chats as Chat[];

	console.log(data);

	function createChatClicked() {
		goto('/newchat');
	}

	async function createChatWithRandomClicked() {
		requireLogin();
		const resp = await fetch(PUBLIC_API_URL + '/chat/random/', {
			method: 'POST',
			headers: {
				Authorization: 'Bearer ' + userState.user?.accessToken
			}
		});

		if (!resp.ok) return;

		let chatId = await resp.text();

		if (resp.status == 200) {
			console.log('Found a chat partner!');
			goto(`/edit/${chatId}`);
		} else if (resp.status == 201) {
			console.log('Created chat request');

			// TODO: update chat list
		}
	}
</script>

<div>
	<div class="page" style="justify-content: flex-start;">
		<div class="button-row" style="max-width: 80vw;">
			<div style="flex: 1"></div>
			<button onclick={createChatClicked}>New Chat</button>
			<button onclick={createChatWithRandomClicked}>Chat with random</button>
		</div>
		<div class="list">
			<h2>Your chats:</h2>
			{#each chats as chat}
				<a href="/edit/{chat.id}"
					>{chat.name}<span class="secondary-text"
						>{chat.isPendingRequest ? '  (pending)' : ''}</span
					></a
				>
			{/each}
		</div>
	</div>
</div>

<style>
	.list {
		margin-left: auto;
		margin-right: auto;
		display: flex;
		flex-direction: column;
		gap: 25px;
		text-align: left;
	}
</style>
