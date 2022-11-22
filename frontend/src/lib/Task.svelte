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
</script>

<li>
	{task.name}
	<input type="checkbox" bind:checked={task.done} disabled>
	{#if jwt !== ""}
		<button on:click={deleteTask}>Delete Task</button>
	{/if}
</li>