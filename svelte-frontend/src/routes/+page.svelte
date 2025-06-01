<script lang="ts">
	import { goto } from '$app/navigation';
	import type { Chat } from '$lib/chat';
	import ChatView from '../ChatView.svelte';

	let demoChat: Chat = {
		id: '',
		name: '',
		description: '',
		isFromPerspectiveA: true,
		isPendingRequest: false,
		messages: [
			{
				id: '',
				content: 'I would not have thought that of you',
				isOwn: false,
				avg_rating: -2
			},
			{
				id: '',
				content: 'wdym?',
				isOwn: true,
				avg_rating: 1
			},
			{
				id: '',
				content: "Well, you don't seem like somebody who would think about how others see them",
				isOwn: false,
				avg_rating: 1
			},
			{
				id: '',
				content: 'What makes you think like that?',
				isOwn: true,
				avg_rating: 1
			},
			{
				id: '',
				content: '...',
				isOwn: false,
				avg_rating: 1
			}
		]
	};
</script>

<div class="main-container">
	<h1>A Platform for Peer-Reviewing Conversations</h1>
	<p style="font-size: 1.3em; text-align: center;">
		You feel insecure when talking to people?<br />
		<strong>Let your conversations be peer-reviewed!</strong>
	</p>

	<div class="button-row" style="font-size: 1.3em; margin-top: 40px;">
		<button class="strong" onclick={async () => await goto('/feed')}>Start reviewing</button>
		<button class="strong" onclick={async () => await goto('/newchat')}>Publish a chat</button>
	</div>

	<div id="demoChat" class="fading-msg-chat">
		<ChatView chat={demoChat} showRatingIndicators={false}></ChatView>
	</div>
</div>

<style lang="css">
	.main-container {
		position: absolute;
		display: flex;
		margin-top: 200px;
		width: 100%;
		flex-direction: column;
		align-items: center;

		height: 200vh;
	}

	#demoChat {
		margin-top: 200px;
	}

	h1 {
		text-align: center;
		font-size: 3em;
	}

	@keyframes demoChatAppear {
		0% {
			opacity: 0;
		}

		50% {
			opacity: 0;
		}

		70% {
			opacity: 100%;
		}

		/*     100%{
        opacity: 0;
    } */
	}
</style>
