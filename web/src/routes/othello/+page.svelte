<script lang="ts">
  import Navbar from "~/othello/components/Navbar.svelte";
  import OthelloGame from "~/othello/components/OthelloGame.svelte";
</script>

<Navbar currentPage="/othello" />

<h1 class="mb-20 mt-10 text-5xl font-bold">Othello AI</h1>

<OthelloGame />
