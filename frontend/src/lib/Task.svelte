<script>
	import { createEventDispatcher } from "svelte";
	import { fetchUser } from "./users.js";
	import { jwt, jwt_claims } from "./stores.js";
	import Modal from "./Modal.svelte";
	import SelectUser from "./SelectUser.svelte";

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

	let selected_user_id = task.assignee_id;

	async function selectUser() {
		let resp = await fetch(`/api/tasks/assign/${task.id}/${selected_user_id}`, {
			"method": "POST",
			"headers": {
				"jwt": $jwt
			}
		});

		if (resp.ok) {
			task.assignee_id = selected_user_id;
		} else {
			alert("failed to assign user to task");
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
	const notAssignedTip = "you cannot mark this task as done because you are not assigned to it";

	let show_details = false;
</script>

<div class={show_details ? "seperate" : undefined}>
	<input type="checkbox" id={task.id} bind:checked={task.done} on:input={setDone} disabled={!isAssigned} title={isAssigned ? undefined : notAssignedTip}>
	<label for={task.id}>{task.name}</label>

	{#if show_details}
		{@const assignee = fetchUser(task.assignee_id)}
		{#await assignee}
			<p>fetching info...</p>
		{:then user}
			{#if isOwner}
				<SelectUser bind:selected_user_id/>
				<button on:click={selectUser}>change assignee</button>
			{:else}
				<p>assigned to: {user.username}</p>
			{/if}
		{:catch}
			<p>ERROR: assigned to user with id {task.assignee_id}</p>
		{/await}
		<button on:click={() => show_details = false}>hide details</button>
	{:else}
		<button on:click={() => show_details = true}>details</button>
	{/if}

	{#if isOwner}
		<button class="dangerous" on:click={deleteTask}>X</button>
	{/if}
</div>

<style>
	.dangerous {
		color: red;
	}

	.seperate:not(:first-child) {
		border-top: 1px solid white;
		padding-top: 0.5em;
		margin-top: 0.5em;
	}

	.seperate:not(:last-child) {
		border-bottom: 1px solid white;
		padding-bottom: 0.5em;
		margin-bottom: 0.5em;
	}
</style>
