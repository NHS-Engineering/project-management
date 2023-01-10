<script>
	import { fetchUser } from "./users.js";
	import { jwt } from "./stores.js";

	export let selected_user_id;

	async function fetchUsers() {
		let resp = await fetch("/api/users/preview", {
			"headers": {
				"jwt": $jwt
			}
		});

		if (resp.ok) {
			return await resp.json();
		} else {
			alert("failed to fetch all users");
		}
	}

	let users = fetchUsers();
</script>

{#await users}
	<select disabled>
		<option>loading...</option>
	</select>
{:then users}
	<select bind:value={selected_user_id}>
		{#each users as user}
			<option value={user.id}>{user.username}</option>
		{:else}
			<option>something went wrong???</option>
		{/each}
	</select>
{:catch}
	<p>could not fetch users</p>
{/await}
