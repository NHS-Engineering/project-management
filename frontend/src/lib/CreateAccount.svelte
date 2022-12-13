<script>
	import Modal from "./Modal.svelte";
	import { login, password_reason } from "./login.js";
	import { invite_jwt } from "./stores.js";
	import { get_jwt_claims } from "./jwt.js";

	let invite_claims = get_jwt_claims($invite_jwt);

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
				"jwt": $invite_jwt
			},
			"body": password
		});

		if (!resp.ok) {
			if (resp.status === 400) {
				const reason = password_reason(await resp.json());
				alert("ERROR: password does not meet requirements:\n" + reason);
			} else {
				alert("failed to redeem invite (maybe it's expired idk)");
			}

			return;
		}

		alert("successfully created account");

		await login(invite_claims["username"], password);

		// just to be safe...
		password = "";
		password_confirm = "";

		history.replaceState(null, "", "/");
		invite_jwt.set(null);
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
