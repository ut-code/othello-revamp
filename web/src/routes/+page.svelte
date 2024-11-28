<script lang="ts">
  import * as wasm from "wasm";
  function onclick() {
    const board = wasm.init_othello(6);
    console.log(
      board
        .get_data()
        .map((row: Array<string>) => row.join(""))
        .join(""),
    );
  }
</script>

<h1>Welcome to SvelteKit</h1>
<button {onclick}>Greet</button>
