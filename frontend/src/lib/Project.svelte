<script>
	import { jwt, jwt_claims } from "./stores.js";
	import { fetchUser } from "./users.js";
	import Modal from "./Modal.svelte";
	import Task from "./Task.svelte";
	import { createEventDispatcher, onMount } from "svelte";

	export let project;

	const dispatch = createEventDispatcher();

	let user = fetchUser(project.owner_id);

	let modal_visible = false;

	async function deleteProject() {
		if ((await tasks).length >= 1) {
			if (!confirm("are you sure you want to delete this project?")) {
				return;
			}
		}

		let resp = await fetch(`/api/projects/delete/${project.id}`, {
			"method": "DELETE",
			"headers": {
				"jwt": $jwt
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

	async function forceRefreshTasks() {
		tasks = getTasks();
	}

	async function maybeRefreshTasks() {
		const expire = 1000 * 15; // 15 seconds

		let new_time = new Date().getTime();
		if (new_time - last_updated > expire) {
			forceRefreshTasks();
			last_updated = new_time;
		}

		return tasks;
	}

	$: if (modal_visible) { maybeRefreshTasks() }

	async function newTask() {
		// TODO: make more visually appealing
		const task_name = prompt("what is the name of your task?");

		if (task_name === null) {
			maybeRefreshTasks();
			return;
		}

		let resp = await fetch(`/api/tasks/new/${project.id}`, {
			"method": "POST",
			"headers": {
				"jwt": $jwt
			},
			"body": task_name
		})

		if (resp.ok) {
			forceRefreshTasks();
		} else {
			alert("failed to create task")
		}
	}

	// fetch tasks if 20% visible
	const observer = new IntersectionObserver((entries) => {
		if (entries[0].isIntersecting) {
			maybeRefreshTasks();
		}
	}, {
		root: null,
		rootMargin: "0px",
		threshold: 0.2
	});

	let projectBox;
	onMount(() => {
		observer.observe(projectBox);
	});

	let projectColor = (project.color === null) ? "#444444" : project.color;

	async function updateColor(event) {
		let resp = await fetch(`/api/projects/set_color/${project.id}`, {
			"method": "POST",
			"headers": {
				"jwt": $jwt
			},
			"body": projectColor
		});

		if (!resp.ok) {
			dispatch("action");
			alert("failed to change project color");
		}
	}

	$: isOwner = $jwt_claims["user_id"] === project.owner_id;
	const notOwnerTip = "you cannot manage this project because you are not the owner";
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-mouse-events-have-key-events -->
<div on:click={() => modal_visible=true} on:mouseover={maybeRefreshTasks} bind:this={projectBox}
	class="box" style={`background-color: ${projectColor}`}>

	<p>{project.name}</p>
	{#await user}
		<p>fetching info...</p>
	{:then user}
		<p>created by:
			<span style={user.is_admin ? "font-style: italic" : null}>{user.username}</span>
		</p>
	{:catch}
		<p>ERROR: created by user with id {project.owner_id}</p>
	{/await}
</div>

{#if modal_visible}
	<Modal on:close={() => modal_visible = false}>
		<p title={isOwner ? undefined : notOwnerTip}>project: {project.name}</p>
		{#if isOwner}
			<input type="color" bind:value={projectColor} on:change={updateColor}>
		{/if}
		{#await tasks}
			<p>fetching tasks...</p>
		{:then tasks}
			<p>tasks:</p>
			<div class="list">
				{#each tasks as task}
					<Task {isOwner} bind:task on:deleted={forceRefreshTasks}/>
				{:else}
					<p>this project has no tasks</p>
				{/each}
			</div>
		{:catch}
			<p>failed to fetch tasks</p>
		{/await}
		{#if $jwt !== ""}
			{#if isOwner}
				<button on:click={newTask}>New Task</button>
				<button class="dangerous" on:click={deleteProject}>Delete Project</button>
			{/if}
		{/if}
	</Modal>
{/if}

<style>
	.box {
		user-select: none;
		margin-top: 1em;
		margin-right: 1em;
		padding-left: 1em;
		padding-right: 1em;
	}

	.list {
		display: flex;
		flex-direction: column;
		max-height: 60vh;
		overflow-y: auto;
		margin-bottom: 1em;
	}
</style>
