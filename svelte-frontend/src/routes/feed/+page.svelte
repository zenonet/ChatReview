

<script lang="ts">
	import ChatView from '../../ChatView.svelte';

let { data } = $props();
console.log(data);
</script>

<h2>Feed</h2>

<ChatView chat={data} messageClicked={ (msg) => {console.log(msg.content)}}></ChatView>