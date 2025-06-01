
<script lang="ts">
	import type { Message } from "$lib/chat";

    export let message: Message;
    export let showRatingIndicator: boolean;
    export let onclick: (() => void) | null = null;

</script>


<div class="layout-container { message.isOwn ? 'own-message' : "others-message" }">
    <div hidden="{!showRatingIndicator}" class="rating-indicator">{ message.avg_rating }</div>
    <button class="message" onclick={ onclick }>
        <p style="margin: 0;">
            { message.content }
        </p>
    </button>
</div>

<style>
	.layout-container {
		margin: 2pt;
		width: fit-content;
		display: flex;
		flex-direction: column;
	}

	.message {
		background: var(--own-message-background);
		color: black;
		padding: 15pt;

		border-radius: 20px;
		font-size: 1.5em;
		text-align: left;
		/* TOOD: Think about this, this might be a stupid idea */
		user-select: none;

		position: relative;
	}

	.others-message {
		align-self: flex-start;
		border-bottom-left-radius: 0;
	}
	.others-message .message {
		background: var(--others-message-background);
		border-bottom-left-radius: 0;
	}

	.own-message {
		align-self: flex-end;
		border-bottom-right-radius: 0;
	}

	.own-message .message {
		border-bottom-right-radius: 0;
	}

	.rating-indicator {
		font-size: 0.75em;
		align-self: flex-end;
	}

	.others-message .rating-indicator {
		align-self: flex-start;
	}
</style>
