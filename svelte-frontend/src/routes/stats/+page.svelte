<script lang="ts">
	import { PUBLIC_API_URL } from '$env/static/public';
	import type { Stat } from '$lib/chat';
	import { userState } from '$lib/state/user.svelte';
	import StatGauge from '../../StatGauge.svelte';

	let stats: Stat[] = $state([]);
	async function fetchStats() {
		const resp = await fetch(PUBLIC_API_URL + '/stats/');
		stats = await resp.json();
	}

	fetchStats();
</script>

<div style="display: flex; flex-direction: column; align-items: center;">
	<h1>Platform Statistics</h1>
	{#if stats}
		<div class="row">
			{#each stats as stat}
				<StatGauge {stat} />
			{/each}
			{#if userState.user}
				<StatGauge stat={{ name: userState.user.username + 's', value: 1 }} />
			{/if}
		</div>
		<p class="secondary-text" style="margin-top: 40px;">
			This would probably look more impressive if people actually used this
		</p>
	{:else}
		<div>Loading stats...</div>
	{/if}
</div>

<style scoped>
	.row {
		display: flex;
		flex-direction: row;
		justify-items: center;
		margin-top: 80pt;
		padding-left: 20pt;
		padding-right: 20pt;
		justify-content: space-evenly;
		gap: 25pt;
		flex-wrap: wrap;
	}
</style>
