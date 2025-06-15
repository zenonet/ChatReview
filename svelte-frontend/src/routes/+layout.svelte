<script>
	import { base } from '$app/paths';
	import { userState } from '$lib/state/user.svelte';

	let { children } = $props();

	// Try to load login from localstorage
	userState.user;
</script>

<nav class="bar">
	<a href="{base}/">Home</a>
	<a href="{base}/feed">Feed</a>
	<a href="{base}/mychats">My Chats</a>
	<a href="{base}/stats">Stats</a>
	<div style="flex: 1;"></div>
	{#if userState.user}
		<a href="{base}/profile">{userState.user?.username}</a>
	{:else}
		<a href="{base}/login">Login</a>
	{/if}
</nav>
<div>
	{@render children()}
</div>

<style>
	.bar {
		display: flex;
		flex-direction: row;
		gap: 20px;
		padding: 5px;
	}

	a {
		padding: 20px;
		color: white;
		text-decoration: none;
	}

	a:hover {
		text-decoration: underline;
	}
</style>
