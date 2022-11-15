<script>
	import { users } from "./stores.js";
	import { fetchUser } from "./users.js";
	import Modal from "./Modal.svelte";
	import Task from "./Task.svelte";
	import { createEventDispatcher } from "svelte";

	export let project;
	export let jwt;

	const dispatch = createEventDispatcher();

	users.update(users => {
		if (!users.hasOwnProperty(project.owner_id)) {
			users[project.owner_id] = fetchUser(project.owner_id);
		}

		return users;
	});

	let user;
	users.subscribe(users => {
		user = users[project.owner_id]
	});

	let modal_visible = false;

	async function deleteProject() {
		let resp = await fetch(`/api/projects/delete/${project.id}`, {
			"method": "DELETE",
			"headers": {
				"jwt": jwt
			}
		});

		if (resp.ok) {
			dispatch("action");
		} else {
			alert("failed to delete project");
		}
	}

	async function getTasks() {
		let resp = await fetch(`/api/tasks/list/${project.id}`);
		let tasks = await resp.json();
		return tasks["tasks"];
	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div on:click={() => modal_visible=true} class="box">
	<p>{project.name}</p>
	{#await user}
		<p>fetching info...</p>
	{:then user}
		<p>created by: {user.username}</p>
	{:catch}
		<p>ERROR: created by user with id {project.owner_id}</p>
	{/await}
</div>

{#if modal_visible}
	<Modal on:close={() => modal_visible = false}>
		<p>project: {project.name}</p>
		{#await getTasks()}
			<p>fetching tasks...</p>
		{:then tasks}
			{#if tasks.length >= 1}
				<p>tasks:</p>
				<ul>
					{#each tasks as task}
						<Task {task}/>
					{/each}
				</ul>
			{:else}
				<p>this project has no tasks</p>
			{/if}
		{:catch}
			<p>failed to fetch tasks</p>
		{/await}
		{#if jwt !== ""}
			<button on:click={deleteProject}>Delete</button>
		{/if}
	</Modal>
{/if}

<style>
	.box {
		user-select: none;
		background-color: #444444;
		margin-top: 1em;
		margin-right: 1em;
		padding-left: 1em;
		padding-right: 1em;
	}
</style>
