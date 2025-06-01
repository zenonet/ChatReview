<script lang="ts">
	import { goto } from '$app/navigation';
	import { PUBLIC_API_URL } from '$env/static/public';
	import { logout, requireLogin } from '$lib/auth';
	import { userState } from '$lib/state/user.svelte';


    requireLogin();

	function onLogoutClicked() {
		logout();
		goto('/login');
	}

	async function onDeleteAccountClicked() {
		let resp = prompt(
			'Are you sure that you want to delete your account?\nIf you really want to delete your account type in DELETE into the field:'
		);
		if (resp === 'DELETE') {
			const res = await fetch(PUBLIC_API_URL + '/profile/', {
				method: 'DELETE',
				headers: {
					Authorization: 'Bearer ' + userState.user?.accessToken
				}
			});

			if (res.ok) {
				alert('Account deleted successfully. Are you happy now?');
				logout();
				goto('/login');
			} else {
				alert(
					'ooops, failed to delete account. Too bad.\nfr though: Contact me pls so I can fix this'
				);
			}
		} else {
			alert("Nope, ain't gonna happen as long as you can't spell");
		}
	}
</script>

<div class="page" style="justify-content: flex-start">
	<div style="margin-left: auto; margin-right: auto;">
		<h2>Welcome {userState.user?.username}!</h2>
		<p>Here you can adjust your profile</p>
		<div style="height: 30px;"></div>

		<div
			id="links"
			style="display: flex; flex-direction: column; align-items: flex-start; gap: 15pt">
			<a href="/mychats" >My chats</a>
			<button class="secondary" onclick={() => goto('/passkeys')}>Add Passkey for authentication</button>
			<button class="secondary" onclick={onLogoutClicked}>Log out</button>
			<button class="secondary" onclick={onDeleteAccountClicked}>Delete account</button>
		</div>
	</div>
</div>
