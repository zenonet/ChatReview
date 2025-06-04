<script lang="ts">
	import { goto } from '$app/navigation';
	import { PUBLIC_API_URL } from '$env/static/public';
	import { requireLogin } from '$lib/auth';
	import { userState } from '$lib/state/user.svelte';

	let chatName = $state('');
	let description = $state('');

	requireLogin();

	async function createChat() {
		console.log('Using token:' + userState.user?.accessToken);

		let res = await fetch(PUBLIC_API_URL + '/chat/', {
			method: 'POST',
			headers: {
				Authorization: 'Bearer ' + userState.user?.accessToken,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				name: chatName,
				description: description
			})
		});
		res.maybeRedirectToLogin();

		if (res.ok) {
			const chatId = await res.text();
			goto(`edit/${chatId}`);
			return;
		} else {
			console.log('Creating chat failed');
			//TODO: Show error message
		}
	}
</script>

<div class="page">
	<div class="form-container">
		<h2>Create a new chat</h2>
		<p>
			Here you can create a new chat. After creating it, you can recreate the dialogue you've had
			and others can peer-review it.
		</p>
		<div class="field-container">
			<label for="nameInput">Chat Name</label>
			<input id="nameInput" bind:value={chatName} maxlength="128" />
		</div>
		<div class="field-container">
			<label for="descriptionInput">Context: </label>
			<textarea
				id="descriptionInput"
				bind:value={description}
				placeholder="Write something about the situation or your relation to the other person"
			></textarea>
		</div>
		<div>
			<button onclick={createChat}>Create</button>
		</div>
	</div>
</div>

<style>
	.form-container {
		width: fit-content;
		display: flex;
		flex-direction: column;
		margin: auto;
		margin-top: 50pt;
		gap: 10pt;
	}

	h2 {
		margin: 0;
	}

	.field-container {
		display: flex;
		flex-direction: column;
	}

	button {
		font-size: large;
	}
</style>
