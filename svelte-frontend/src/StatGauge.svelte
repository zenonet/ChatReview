<script lang="ts">
	import { Stat } from "$lib/chat";


	let {stat} = $props<{stat: Stat}>();

	let currentVal = $state(0);
	function change() {
		if (currentVal === stat.value) return;

		if (stat.value - currentVal > 1) {
			currentVal += 1;
		} else {
			currentVal = stat.value;
		}

		setTimeout(change, 20);
	}

    $effect(() => {
        stat; // I hope this makes stat a dependency
        change();
    })
</script>

{#if stat}
	<div class="gauge">
		<span class="title">{stat.name}</span>
		<span class="value">{(Math.round(currentVal * 100) / 100).toString()}</span>
	</div>
{/if}

<style lang="css" scoped>
	.gauge {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.gauge > .value {
		font-size: 2em;
		font-weight: bold;
	}

	.gauge > .title {
		text-align: center;
	}
</style>
