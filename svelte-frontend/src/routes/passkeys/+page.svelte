<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { PUBLIC_API_URL } from '$env/static/public';
	import { redirectToLogin, registerPasskey, requireLogin } from '$lib/auth';
	import type { Passkey } from '$lib/chat';
	import { userState } from '$lib/state/user.svelte';
	import PasskeyView from '../../PasskeyView.svelte';

	let { data }: { data: { passkeys: Passkey[]; statusCode: number } } = $props();

	let success = $state(false);
	let error = $state<string | null>(null);

	$effect(() => {
		console.log("Effect running!")
		if (data.statusCode == 401 || data.passkeys == null) {
			redirectToLogin();
		}
	});

	async function onStartButtonClicked() {
		let res = await registerPasskey();
		success = res == null;
		error = res;
		if (success) {
			// Reload passkeys from API
			invalidateAll();
		}
	}
</script>

<div class="container" style="align-items: center;">
	<h1>Passkeys</h1>
	<p>Here, you can add a passkey that can be used for secure passwordless authentication.</p>
	<button onclick={onStartButtonClicked}>Start</button>

	<h2 hidden={!success}>Successfully added passkey!</h2>
	<h2 hidden={error === null} style="color: red">Error: {error}</h2>

	<h2>Your passkeys</h2>
	{#if !data.passkeys}
		<h4>Loading passkeys</h4>
	{:else if data.passkeys.length === 0}
		<h4>You don't have any passkeys</h4>
	{:else}
		<div>
			{#each data.passkeys as pk}
				<PasskeyView passkey={pk} />
			{/each}
		</div>
	{/if}
</div>
