<script>
	import Modal from "./Modal.svelte";
	import { jwt } from "./stores.js";
	import { password_reason } from "./login.js";

	export let self_user;

	let show_modal = false;

	let old_password = "";
	let new_password = "";
	let new_password_confirm = "";

	async function changePassword() {
		if (new_password !== new_password_confirm) {
			alert("passwords do not match");
			return;
		}

		const resp = await fetch("/api/auth/change_password", {
			"method": "POST",
			"headers": {
				"jwt": $jwt
			},
			"body": JSON.stringify({
				"old_password": old_password,
				"new_password": new_password
			})
		});

		if (resp.ok) {
			if ("PasswordCredential" in window) {
				const cred = new PasswordCredential({
					id: (await self_user).username,
					password: new_password
				});

				await navigator.credentials.store(cred);
			}

			alert("successfully changed password");
			show_modal = false;
		} else {
			const reason = await resp.json();

			if (reason === "PasswordIncorrect") {
				alert("your old password is incorrect")
			} else if ("InvalidPassword" in reason) {
				alert(password_reason(reason["InvalidPassword"]))
			} else {
				alert("something went wrong??");
			}
		}
	}
</script>

<button on:click={() => show_modal = true}>Change Password</button>

{#if show_modal}
	<Modal on:close={() => show_modal = false}>
		<form on:submit|preventDefault={changePassword}>
			<input placeholder="old password" type="password" bind:value={old_password} required>
			<input placeholder="new password" type="password" bind:value={new_password} required>
			<input placeholder="confirm new password" type="password" bind:value={new_password_confirm} required>
			<input type="submit" value="Change Password" class="dangerous">
		</form>
	</Modal>
{/if}
