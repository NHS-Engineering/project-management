<script>
	import Modal from "./Modal.svelte";
	import { createEventDispatcher } from "svelte";
	import { login } from "./login.js";

	let show = false;

	let username = "";
	let password = "";
	let failed = false;

	async function handleLogin() {
		try {
			await login(username, password);
			password = ""; // just to be safe
		} catch {
			failed = true;
		}
	}
</script>

<button on:click={() => show = true}>Login</button>

{#if show}
	<Modal on:close={() => show = false}>
		<form on:submit|preventDefault={handleLogin}>
			<!-- svelte-ignore a11y-autofocus -->
			<input autofocus placeholder="username" type="text" bind:value={username} required autocapitalize="off">
			<input placeholder="password" type="password" bind:value={password} required>
			<input type="submit" value="Login">
		</form>
		{#if failed}
			<p class="error">login failed</p>
		{/if}
	</Modal>
{/if}

<style>
	.error {
		background-color: red;
		text-align: center;
	}
</style>
