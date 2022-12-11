<script>
	import { createEventDispatcher } from "svelte";
	import { fetchUser } from "./users.js";
	import { jwt, jwt_claims } from "./stores.js";
	import Modal from "./Modal.svelte";

	export let task;
	export let isOwner;

	const dispatch = createEventDispatcher();

	async function deleteTask() {
		let resp = await fetch(`/api/tasks/delete/${task.id}`, {
			"method": "DELETE",
			"headers": {
				"jwt": $jwt
			}
		});

		if (resp.ok) {
			dispatch("deleted")
		} else {
			alert("failed to delete task");
		}
	}

	async function setDone() {
		let resp = await fetch(`/api/tasks/set_done/${task.id}/${!task.done}`, {
			"method": "POST",
			"headers": {
				"jwt": $jwt
			}
		});

		if (!resp.ok) {
			task.done = !task.done;
			alert("failed to mark task as done");
		}
	}

	$: isAssigned = $jwt_claims["user_id"] === task.assignee_id;

	let show_details = false;
</script>

<div>
	<input type="checkbox" id={task.id} bind:checked={task.done} on:input={setDone} disabled={!isAssigned}>
	<label for={task.id}>{task.name}</label>
	<button on:click={() => show_details = true}>details</button>
	{#if isOwner}
		<button class="dangerous" on:click={deleteTask}>X</button>
	{/if}
</div>

{#if show_details}
	<Modal on:close={() => show_details = false}>
		{@const assignee = fetchUser(task.assignee_id)}
		<p>task name: {task.name}</p>
		{#await assignee}
			<p>fetching info...</p>
		{:then user}
			<p>assigned to: {user.username}</p>
		{:catch}
			<p>ERROR: assigned to user with id {task.assignee_id}</p>
		{/await}

		<p>assinging tasks to users is coming very soon...</p>
	</Modal>
{/if}

<style>
	.dangerous {
		color: red;
	}
</style>
