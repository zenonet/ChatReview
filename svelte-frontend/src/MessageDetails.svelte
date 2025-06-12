<script lang="ts">
	import { PUBLIC_API_URL } from '$env/static/public';
	import type { Message, Comment } from '$lib/chat';
	import { userState } from '$lib/state/user.svelte';
	import ChatMessage from './ChatMessage.svelte';
	import '$lib/auth';

	let { message } = $props<{ message: Message }>();

	let ratingInputVal = $state(0);

	async function submitRating() {
		//appState.auth.redirectIfNotLoggedIn();

		let rating = {
			messageId: message.id,
			value: ratingInputVal
		};
		let res = await fetch(PUBLIC_API_URL + '/rating/', {
			method: 'POST',
			headers: {
				Authorization: 'Bearer ' + userState.user?.accessToken,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(rating)
		});

		res.maybeRedirectToLogin();

		if (res.ok) {
			console.log('Rating posted successfully!');
		}
	}

	let commentVal = $state('');
	async function postComment() {
		// appState.auth.redirectIfNotLoggedIn();

		const comment = {
			messageId: message.id,
			content: commentVal
		};

		const res = await fetch(PUBLIC_API_URL + '/comment/', {
			method: 'POST',
			headers: {
				Authorization: 'Bearer ' + userState.user?.accessToken,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(comment)
		});
		res.maybeRedirectToLogin();

		if (res.ok) {
			commentVal = '';
			console.log('Comment posted successfully!');
			await loadComments();
		}
	}

	let comments: Comment[] | null = $state(null);
	async function loadComments() {
		const resp = await fetch(PUBLIC_API_URL + '/comment/forMessage/' + message.id);

		comments = await resp.json();
		console.log('Comments for message loaded!');
		console.log(comments);
	}

	let avgRating = $derived(() => (message.avg_rating || 0).toString());

	$effect(() => {
		if (message) loadComments();
	});
	loadComments();

	function formatTimestamp(timestamp: number) {
		let date = new Date(timestamp * 1000);
		date.setSeconds(0);
		const timeStr = `${date.getHours()}:${date.getMinutes()}`;
		/*return `${timeStr} ${date.toDateString()}`
     return date.toLocaleString(null,  {
        seco
    }) */
		return date.toLocaleString('DE-de');
	}
</script>

{#if message}
	<div style="width: fit-content;">
		<h2>Message details</h2>
		<ChatMessage {message} showRatingIndicator={false} />
		<div id="rating">
			<h3>Rating</h3>
			<span>Average rating: { avgRating() }</span>

			<div class="container">
				<h4>Your rating:</h4>
				<input type="number" bind:value={ratingInputVal} />
				<button onclick={submitRating}>Submit</button>
			</div>
		</div>
		<div id="comments">
			<h3>Comments</h3>
			<div class="container">
				<h4>Your comment:</h4>
				<textarea placeholder="What do you think of this?" bind:value={commentVal}></textarea>
				<button onclick={postComment}>Submit</button>
				<h4>Other comments</h4>
				{#if comments}
					<div id="commentList">
						{#each comments as comment}
							<div class="comment">
								<div style="display: flex; flex-direction: row; gap: 5pt">
									<span>{comment.ownerName}</span>
									<span class="secondary-text">{formatTimestamp(comment.timestamp)}</span>
								</div>
								<p>{comment.content}</p>
							</div>
						{/each}
					</div>
				{:else}
					<div>Loading comments...</div>
				{/if}
			</div>
		</div>
	</div>
{:else}
	<div>No message selected</div>
{/if}

<style>
	#commentList {
		display: flex;
		flex-direction: column;
		gap: 15px;
	}

	.comment p {
		margin-top: 5px;
		margin-bottom: 5px;
	}
</style>
