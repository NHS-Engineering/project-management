<script>
	import { createEventDispatcher } from "svelte";
	import { jwt, jwt_claims } from "./stores.js";
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
</script>

<div>
	<input type="checkbox" id={task.id} bind:checked={task.done} on:input={setDone} disabled={!isAssigned}>
	<label for={task.id}>{task.name}</label>
	{#if isOwner}
		<button class="dangerous" on:click={deleteTask}>X</button>
	{/if}
</div>

<style>
	.dangerous {
		color: red;
	}
</style>