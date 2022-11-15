<script>
	import Login from "./lib/Login.svelte";
	import NewProject from "./lib/NewProject.svelte";
	import Project from "./lib/Project.svelte";

	let showLogin = false;
	let jwt = "";

	async function getProjects() {
		let projects = await fetch("/api/projects/list");
		projects = await projects.json();
		return projects["projects"];
	}

	let projects = getProjects();

	function refreshProjects() {
		projects = getProjects();
	}

	let showNewProject = false;
</script>

<main>
	<nav>
		<h1>Projects List</h1>
		{#if jwt === ""}
			<button on:click={() => showLogin = true}>Login</button>
		{/if}
	</nav>

	{#if showLogin}
		<Login on:close={() => showLogin = false} on:success={(e) => jwt = e.detail}/>
	{/if}

	<button on:click={refreshProjects}>Refresh</button>
	{#if jwt !== ""}
		<button on:click={() => showNewProject = true}>New Project</button>
	{/if}

	{#if showNewProject}
		<NewProject {jwt} on:close={() => showNewProject = false} on:success={refreshProjects}/>
	{/if}

	{#await projects}
		<p>fetching projects...</p>
	{:then projects}
		<div class="list">
			{#each projects as project}
				<Project {project} {jwt} on:action={refreshProjects}/>
			{/each}
		</div>
	{:catch}
		<p>could not fetch projects</p>
	{/await}
</main>

<style>
	nav {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.list {
		display: grid;
		justify-content: center;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
	}
</style>
