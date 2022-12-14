<script>
	import { messages } from "./stores.js";
</script>

<div>
	{#each $messages as message}
		<p>{message}</p>
	{/each}
</div>

<style>
	div {
		text-align: center;
	}

	p {
		border-color: darkred;
		border-style: solid;
	}
</style>