<script>
	import Modal from "./Modal.svelte";
	import { createEventDispatcher } from "svelte";
	import { jwt } from "./stores";

	const dispatch = createEventDispatcher();

	function close() {
	    dispatch("close");
	}

	function success(new_jwt) {
		const parts = new_jwt.split(".");
		const claims = JSON.parse(atob(parts[1]));

		const now = new Date().getTime();
		const delta_expires = (claims["exp"] * 1000) - now;

		setTimeout(() => {
			jwt.set("");
			console.debug("session expired");
		}, delta_expires);

		jwt.set(new_jwt);

	    dispatch("close");
	}

	let username = "";
	let password = "";
	let failed = false;

	async function login() {
		let resp = await fetch("/api/auth/login", {
			"method": "POST",
			"body": JSON.stringify({
				"username": username,
				"password": password
			})
		});

		if (resp.ok) {
			let jwt = await resp.text();
			password = ""; // just to be safe...
			success(jwt);
		} else {
			failed = true;
		}
	}
</script>

<Modal on:close={close}>
	<form on:submit|preventDefault={login}>
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
