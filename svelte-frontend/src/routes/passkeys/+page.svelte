<script lang="ts">
	import { PUBLIC_API_URL } from '$env/static/public';
	import { registerPasskey, requireLogin } from '$lib/auth';
	import type { Passkey } from '$lib/chat';
	import { userState } from '$lib/state/user.svelte';
	import PasskeyView from '../../PasskeyView.svelte';

	requireLogin();

	let success = $state(false);
	async function onStartButtonClicked() {
		success = (await registerPasskey()) === null;
	}
	let passkeys = $state<Passkey[] | null>(null);
	async function loadPasskeys() {
		const resp = await fetch(PUBLIC_API_URL + '/passkeys/', {
			headers: {
				Authorization: 'Bearer ' + userState.user?.accessToken
			}
		});

		resp.maybeRedirectToLogin();
		if (!resp.ok) return; // TODO: maybe show an error message here

		const pks = await resp.json();
		console.log(pks);
		pks.forEach((pk: any) => {
			pk.creationDate = new Date(pk.creationDate * 1000);
		});

		passkeys = pks as Passkey[];

		console.log(pks);
	}

	loadPasskeys();
</script>

<div class="container" style="align-items: center;">
	<h1>Passkeys</h1>
	<p>Here, you can add a passkey that can be used for secure passwordless authentication.</p>
	<button onclick={onStartButtonClicked}>Start</button>

	<h2 hidden={success}>Successfully added passkey!</h2>

	<h2>Your passkeys</h2>
	{#if !passkeys}
		<h4>Loading passkeys</h4>
	{:else if passkeys.length === 0}
		<h4>You don't have any passkeys</h4>
	{:else}
		<div>
			{#each passkeys as pk}
				<PasskeyView passkey={pk} />
			{/each}
		</div>
	{/if}
</div>
