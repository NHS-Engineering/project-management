<script>
	import { createEventDispatcher } from "svelte";
	export let task;
	export let jwt;

	const dispatch = createEventDispatcher();

	async function deleteTask() {
		let resp = await fetch(`/api/tasks/delete/${task.id}`, {
			"method": "DELETE",
			"headers": {
				"jwt": jwt
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
				"jwt": jwt
			}
		});

		if (!resp.ok) {
			task.done = !task.done;
			alert("failed to mark task as done");
		}
	}

	$: isLoggedIn = jwt !== "";
</script>

<li>
	{task.name}
	<input type="checkbox" bind:checked={task.done} on:input={setDone} disabled={!isLoggedIn}>
	{#if isLoggedIn}
		<button on:click={deleteTask}>Delete Task</button>
	{/if}
</li>