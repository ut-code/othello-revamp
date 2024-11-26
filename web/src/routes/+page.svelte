<script lang="ts">
	// @ts-ignore
	import { greet } from 'wasm?init';
	async function onclick() {
		greet();
	}
</script>

<h1>Welcome to SvelteKit</h1>
<button {onclick}>Greet</button>
