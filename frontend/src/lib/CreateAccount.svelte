<script>
	import Modal from "./Modal.svelte";

	export let invite_jwt;

	let invite_claims = JSON.parse(atob(invite_jwt.split(".")[1]));

	let password = "";
	let password_confirm = "";

	async function redeem_invite() {
		if (password !== password_confirm) {
			alert("passwords do not match");
			return;
		}

		let resp = await fetch("/api/auth/redeem_invite", {
			"method": "POST",
			"headers": {
				"jwt": invite_jwt
			},
			"body": password
		});

		if (!resp.ok) {
			alert("failed to redeem invite (maybe it's expired idk)");
			return;
		}

		alert("successfully created account");
		document.location.href = "/"; // TODO: don't reload page
	}
</script>

<Modal>
	<form on:submit|preventDefault={redeem_invite}>
		<input type="text" value={invite_claims["username"]} required disabled>
		<input placeholder="password" type="password" bind:value={password} required>
		<input placeholder="confirm password" type="password" bind:value={password_confirm} required>
		<input type="submit" value="Create Account">
	</form>
</Modal>
