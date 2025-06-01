<script>
	import { userState } from '$lib/state/user.svelte';

	let { children } = $props();

	let user = userState.user;
</script>

<nav class="bar">
	<a href="/">Home</a>
	<a href="/feed">Feed</a>
	<a href="/mychats">My Chats</a>
	<a href="/stats">Stats</a>
	<div style="flex: 1;"></div>
	<a href={user == null ? '/login' : '/profile'}>
		{user == null ? 'Login' : user.username}
	</a>
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
