

<script lang="ts">
	import type { Chat } from '$lib/chat';
	import ChatView from '../../ChatView.svelte';

    // imported from load function
    export let data: Chat;
</script>

<h2>Feed</h2>

<ChatView chat={data} messageClicked={ (msg) => {console.log(msg.content)}}></ChatView>