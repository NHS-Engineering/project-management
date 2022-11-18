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

	let tasks = new Promise((resolve) => resolve([]));
	let last_updated = 0;

	async function refreshTasks() {
		const expire = 1000 * 15; // 15 seconds

		let new_time = new Date().getTime();
		if (new_time - last_updated > expire) {
			tasks = getTasks();
			last_updated = new_time;
		}

		return tasks;
	}

	$: if (modal_visible) { refreshTasks() }

	async function newTask() {
		let resp = await fetch(`/api/tasks/new/${project.id}/name`, {
			"method": "POST",
			"headers": {
				"jwt": jwt
			}
		})

		if (resp.ok) {
			refreshTasks();
		} else {
			alert("failed to create task")
		}
	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-mouse-events-have-key-events -->
<div on:click={() => modal_visible=true} on:mouseover={refreshTasks} class="box">
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
		{#await tasks}
			<p>fetching tasks...</p>
		{:then tasks}
			<p>tasks:</p>
			<ul>
				{#each tasks as task}
					<Task {jwt} {task} on:deleted={refreshTasks}/>
				{:else}
					<p>this project has no tasks</p>
				{/each}
			</ul>
		{:catch}
			<p>failed to fetch tasks</p>
		{/await}
		{#if jwt !== ""}
			<button on:click={newTask}>New Task</button>
			<button on:click={deleteProject}>Delete Project</button>
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
