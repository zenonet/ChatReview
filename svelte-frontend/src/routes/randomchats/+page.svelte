<script lang="ts">
	import { goto, invalidate, invalidateAll } from '$app/navigation';
	import { PUBLIC_API_URL } from '$env/static/public';
	import { redirectToLogin } from '$lib/auth';
	import { userState } from '$lib/state/user.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	if (data.status === 401) {
		userState.reset();
	}

	async function newRandomCheckedClicked() {
		const resp = await fetch(`${PUBLIC_API_URL}/chat/random/`, {
			method: 'POST',
			headers: {
				Authorization: `Bearer ${userState.user?.accessToken}`
			}
		});

		if (!resp.ok) {
			// TODO: display error
			return;
		}

		// Request was created but not fulfilled
		if (resp.status == 201) {
			// Reload the list of chats
			invalidateAll();
		}
		// Request was fulfilled
		else if (resp.status == 200) {
			const id = await resp.text();
			goto(`edit/${id}`);
			// open chat editor for random chat
		}
	}
</script>

<div>
	<div class="page">
		<h1>Random chats</h1>
		<p>
			Random chats allow you to anonymously chat with people and improve your social skills! Of
			course you can also just have fun and meet new people.
		</p>
		<p>Please be respectful :)</p>

		<div style="width: 30vw; max-width: 20cm">
			{#if !userState.user}
				<strong>Please log-in / register to continue.</strong>
				<p>Don't worry, I don't want your data, just a username and password</p>
				<button onclick={() => redirectToLogin()}>Login</button>
			{:else if data.chats}
				<div class="button-row" style="width: 100%;">
					<div style="flex: 1"></div>
					<button onclick={newRandomCheckedClicked}>Start random chat</button>
				</div>
				{#if data.chats.length > 0}
					<h3>Your random chats</h3>
					<div class="list">
						{#each data.chats as chat}
							<a href="edit/{chat.id}">
								{chat.name}
								<span class="secondary-text">
									{chat.isPendingRequest ? '  (pending)' : ''}
								</span>
							</a>
						{/each}
					</div>
				{:else}
					<h3>You don't have any random chats yet</h3>
				{/if}
			{:else}
				<h2>Failed to load random chats: {data.error}</h2>
			{/if}
		</div>
	</div>
</div>
