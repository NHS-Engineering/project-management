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
		position: absolute;
		top: -50vh;
		left: -50vw;
		height: 150vh;
		width: 150vw;
		background-color: rgba(0, 0, 0, .6);
	}

	.modal {
		position: fixed;
		top: 50%;
		left: 50%;
		transform: translate(-50%,-50%);
	}
</style>
