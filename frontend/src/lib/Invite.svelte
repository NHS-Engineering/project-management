<script>
	import Modal from "./Modal.svelte";

	export let jwt;

	let show = false;

	let username = "";
	let qr = "";

	async function generateInvite() {
		qr = "";

		let resp = await fetch("/api/auth/invite", {
			"method": "POST",
			"headers": {
				"jwt": jwt
			},
			"body": username
		});

		if (!resp.ok) {
			alert("failed to generate invite");
			return;
		}

		qr = "data:image/svg+xml," + encodeURIComponent(await resp.text());
	}
</script>

<button on:click={() => show = true}>Invite</button>

{#if show}
	<Modal on:close={() => show = false}>
	<form on:submit|preventDefault={generateInvite}>
		<input placeholder="username" type="text" bind:value={username} required autocapitalize="off">
		<input type="submit" value="Create Invite">
		{#if qr !== ""}
			<img src={qr}/>
		{/if}
	</form>
	</Modal>
{/if}
