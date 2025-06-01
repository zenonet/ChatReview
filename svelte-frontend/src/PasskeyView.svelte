<script lang="ts">
	import { PUBLIC_API_URL } from '$env/static/public';
	import type { Passkey } from '$lib/chat';
	import { userState } from '$lib/state/user.svelte';

	let { passkey } = $props<{ passkey: Passkey }>();

	let isRenaming = $state(false);
	let isDeleted = $state(false);

	async function onRenameClicked() {
		if (isRenaming) {
			await onRenameFinished();
			return;
		}

		isRenaming = true;
		setTimeout(() => {
			document.getElementById('renameInput')?.focus();
		});
	}

	async function onRenameFinished() {
		isRenaming = false;

		const resp = await fetch(PUBLIC_API_URL + '/passkey/rename/' + passkey.id, {
			method: 'PUT',
			headers: {
				Authorization: 'Bearer ' + userState.user?.accessToken,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				name: passkey.name
			})
		});

		if (resp.ok) return;

		console.log(await resp.text());
	}

	async function onDeleteClicked() {
		if (!confirm('Are you sure that you want to remove this passkey from your account?')) return;

		const resp = await fetch(PUBLIC_API_URL + '/passkey/' + passkey.id, {
			method: 'DELETE',
			headers: {
				Authorization: 'Bearer ' + userState.user?.accessToken
			}
		});

		if (resp.ok) {
			isDeleted = true;
		} else {
			console.log(`Failed to delete passkey: ${await resp.text()}`);
		}
	}
</script>

{#if !isDeleted}
	<div style="display: flex; gap: 20pt; align-items: center;">
		{#if isRenaming}
			<input
				id="renameInput"
				class="name"
				bind:value={passkey.name}
				onkeyup={(ev) => (ev.key === 'Enter' ? onRenameFinished() : 0)}
			/>
		{:else}
			<span class="name">{passkey.name}</span>
		{/if}
		<span class="secondary-text">Registered {passkey.creationDate.toLocaleDateString()}</span>
		<button class="secondary" onclick={onDeleteClicked}>Delete</button>
		<button class="secondary" onclick={onRenameClicked}>{isRenaming ? 'Done' : 'Rename'}</button>
	</div>
{/if}

<style scoped>
	.name {
		width: 150px;
	}
</style>
