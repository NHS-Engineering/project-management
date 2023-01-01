<script>
	import { createEventDispatcher } from "svelte";

	const dispatch = createEventDispatcher();

	function close() {
		dispatch("close");
	}

	function handle_keydown(e) {
		if (e.key == 'Escape') {
			close();
		}
	}
</script>

<svelte:window on:keydown={handle_keydown}/>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class="modal-background" on:click={close}>
	<div class="modal" on:click|stopPropagation>
		<slot/>
	</div>
</div>

<style>
	.modal-background {
		position: fixed;
		top: 0;
		left: 0;
		height: 100%;
		width: 100%;
		background-color: rgba(0, 0, 0, .6);
	}

	.modal {
		position: fixed;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
	}
</style>
