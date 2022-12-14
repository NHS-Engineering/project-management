<script>
	import Modal from "./Modal.svelte";
	import { jwt } from "./stores.js";

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
			alert("successfully changed password");
			show_modal = false;
		} else {
			alert("failed to change password"); // TODO: reason
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
