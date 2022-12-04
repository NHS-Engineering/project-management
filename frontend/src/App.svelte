<script>
	import Login from "./lib/Login.svelte";
	import NewProject from "./lib/NewProject.svelte";
	import Project from "./lib/Project.svelte";
	import Invite from "./lib/Invite.svelte";
	import CreateAccount from "./lib/CreateAccount.svelte";
	import { fetchUser } from "./lib/users.js";
	import { jwt, jwt_claims, invite_jwt } from "./lib/stores.js";
	import { login, manualLogout } from "./lib/login.js";

	let showLogin = false;

	async function getProjects(no_cache) {
		let projects = await fetch("/api/projects/list", no_cache ? {
			"headers": {
				"Cache-Control": "no-cache"
			}
		} : {});
		projects = await projects.json();
		return projects["projects"];
	}

	let projects = getProjects(false);

	function refreshProjects() {
		projects = getProjects(true);
	}

	let showNewProject = false;

	invite_jwt.set(new URL(window.location.href).searchParams.get("invite"));

	$: self_user = fetchUser($jwt_claims["user_id"]);

	if ("PasswordCredential" in window) {
		navigator.credentials.get({
			"password": true,
			"mediation": "optional"
		}).then(creds => {
			if (creds !== null) {
				login(creds.id, creds.password).catch(() => {
					alert("failed to log you in automatically, manually log in with correct password to fix");
				});
			}
		});
	}
</script>

<main>
	<nav>
		<h1>Projects List</h1>
		{#if $jwt === ""}
			<button on:click={() => showLogin = true}>Login</button>
		{:else}
			<div>
				{#await self_user}
					<p>Logged in as ...</p>
				{:then self_user}
					<p>Logged in as: {self_user.username}</p>
				{:catch}
					<p>Logged in as ???</p>
				{/await}

				{#if $jwt_claims["user_id"] === 1}
					<Invite/>
				{/if}
				<button on:click={manualLogout}>Logout</button>
			</div>
		{/if}
	</nav>

	{#if showLogin}
		<Login on:close={() => showLogin = false}/>
	{/if}

	{#if $invite_jwt !== null}
		<CreateAccount/>
	{/if}

	<button on:click={refreshProjects}>Refresh</button>
	{#if $jwt !== ""}
		<button on:click={() => showNewProject = true}>New Project</button>

		{#if showNewProject}
			<NewProject on:close={() => showNewProject = false} on:success={refreshProjects}/>
		{/if}
	{/if}

	{#await projects}
		<p>fetching projects...</p>
	{:then projects}
		<div class="list">
			{#each projects as project}
				<Project {project} on:action={refreshProjects}/>
			{:else}
				<p>no projects exist</p>
			{/each}
		</div>
	{:catch}
		<p>could not fetch projects</p>
	{/await}
</main>

<div class="copyright">
	Copyright 2022 NHS Engineering Club.  Licensed under the <a href="https://www.gnu.org/licenses/agpl-3.0.en.html">GNU Affero General Public License version 3</a>.  Source code may be found <a href="https://github.com/NHS-Engineering/project-management">here</a>.
</div>

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
		left: 0;
		position: fixed;
		bottom: 0;
		width: 100%;
		text-align: center;
		font-size: .8em;
	}
</style>
