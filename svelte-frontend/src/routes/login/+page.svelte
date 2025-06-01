<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { login, register } from '$lib/auth';
    

    let username = $state('');
	let password = $state('');

	let errorCounter = $state(0);

	let error = $state<string | null>(null);

	// Move this value to this component so that it doesn't affect later login tries
	let redirectAfterLogin: string | null = page.url.searchParams.get("redirect"); //appState.redirectAfterLogin;
	//appState.redirectAfterLogin = null;
	let lastError = '';
	async function handleLoginResult(result: string | null) {
		if (result != null) {
            error = result;
			if (lastError !== result) {
				errorCounter = 0;
			} else {
				errorCounter++;
			}
			lastError = result;
		} else {
			errorCounter = 0;
			error = '';
			if (redirectAfterLogin != null) {
				console.log('Redirecting to ' + redirectAfterLogin + ' after login...');
				goto(redirectAfterLogin);
			} else {
				goto('/');
			}
		}
	}

	async function loginClick() {
		await handleLoginResult(await login(username, password));
	}

	async function loginWithPasskeyClick() {
		//await handleLoginResult(await loginWithPasskey(username));
	}

	async function registerClick() {
		errorCounter = 0;
		await register(username, password);
	}

	let errorDisplayState = $derived(() => (error === null ? 'none' : 'block'));
	let errorMsg = $derived(() => error + (errorCounter > 1 ? ' (' + errorCounter + 'x)' : ''));
</script>

<div class="page">
	<div class="login-container">
		<h1>Login</h1>

		<!--         <div v-if="redirectAfterLogin">
            <span style="color: red">This action requires logging in</span>
        </div> -->

		<div class="field-container">
			<label for="usernameInput">Username</label>
			<input id="usernameInput" bind:value={username} maxlength="128" />
		</div>

		<div class="field-container">
			<label for="passwordInput">Password</label>
			<input id="passwordInput" bind:value={password} onkeydown={(ev) => ev.key === 'Enter' ? loginClick() : 0} type="password" />
		</div>
		<a href="javascript:void(0)" onclick={loginWithPasskeyClick}>Login with passkey instead</a>

		<span style="color: red; display: {errorDisplayState()}">
			{ errorMsg() }
		</span>

		<div class="button-row">
			<button onclick={loginClick}> Login </button>
			<button class="secondary" onclick={registerClick}> Register </button>
		</div>
	</div>
</div>

<style>
	.field-container {
		display: flex;
		flex-direction: column;
	}

	.login-container {
		width: fit-content;
		display: flex;
		flex-direction: column;
		margin: auto;
		margin-top: 50pt;
		gap: 15pt;
	}

	input {
		padding: 10pt;
		font-size: 1.5em;
	}
</style>
