<script>
	import Modal from "./Modal.svelte";
	import { createEventDispatcher } from "svelte";
	import { jwt } from "./stores.js";

	const dispatch = createEventDispatcher();

	function close() {
	    dispatch("close");
	}

	function success() {
	    dispatch("success");
	    dispatch("close");
	}

	let name = "";

	async function newProject() {
		let resp = await fetch(`/api/projects/new`, {
			"method": "POST",
			"headers": {
				"jwt": $jwt
			},
			"body": name
		});

		if (resp.ok) {
			success();
		} else {
			alert("failed to create project");
		}
	}
</script>

<Modal on:close={close}>
	<form on:submit|preventDefault={newProject}>
		<!-- svelte-ignore a11y-autofocus -->
		<input autofocus placeholder="name" type="text" bind:value={name} required>
		<input type="submit" value="Create">
	</form>
</Modal>