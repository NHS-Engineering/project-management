<script>
	import Login from "./lib/Login.svelte";
	import NewProject from "./lib/NewProject.svelte";
	import Project from "./lib/Project.svelte";

	let showLogin = false;
	let jwt = "";

	function setJwt(event) {
		jwt = event.detail;

		const parts = jwt.split(".");
		const claims = JSON.parse(atob(parts[1]));

		const now = new Date().getTime();
		const delta_expires = (claims["exp"] * 1000) - now;

		setTimeout(() => {
			jwt = "";
			console.debug("session expired");
		}, delta_expires);
	}

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
		<Login on:close={() => showLogin = false} on:success={setJwt}/>
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
			{:else}
				<p>no projects exist</p>
			{/each}
		</div>
	{:catch}
		<p>could not fetch projects</p>
	{/await}

	<div class="copyright">
		Copyright 2022 NHS Engineering Club.  Licensed under the <a href="https://www.gnu.org/licenses/agpl-3.0.en.html">GNU Affero General Public License version 3</a>.  Source code may be found <a href="https://github.com/NHS-Engineering/project-management">here</a>.
	</div>
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

	.copyright {
		position: fixed;
		bottom: 0;
		width: 100%;
		text-align: center;
	}
</style>
