<script lang="ts">
  export const ssr = false;
  import { greet, init_othello } from "wasm";
  function onclick() {
    const board = init_othello(6);
    console.log(
      board
        .get_data()
        .map((row: Array<string>) => row.join(""))
        .join(""),
    );
  }
</script>

<h1>Welcome to SvelteKit</h1>
<button onclick={() => greet()}>Greet</button>
<button {onclick}>Create othello board</button>
