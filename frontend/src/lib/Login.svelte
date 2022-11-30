<script>
	import Modal from "./Modal.svelte";
	import { createEventDispatcher } from "svelte";
	import { login } from "./login.js";

	const dispatch = createEventDispatcher();

	let username = "";
	let password = "";
	let failed = false;

	async function handleLogin() {
		try {
			await login(username, password);
			password = ""; // just to be safe
			dispatch("close");
		} catch {
			failed = true;
		}
	}
</script>

<Modal on:close={close}>
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

<style>
	.error {
		background-color: red;
		text-align: center;
	}
</style>
