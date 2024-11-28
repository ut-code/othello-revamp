<script lang="ts">
  import { Piece, greet, init_othello, place_at } from "boardgame-ai";
  import Board from "./Board.svelte";
  let board_size = $state(6);
  // svelte-ignore state_referenced_locally
  let board = $state(init_othello(board_size));
  let player = $state(Piece.Black);
  $effect(() => {
    board = init_othello(board_size);
  });
  function reset() {
    board = init_othello(board_size);
  }
</script>

<h1 class="text-5xl font-bold">Welcome to Othello AI!</h1>
<center>
  <Board
    {board}
    onclick={(point) => {
      board = place_at(board, player, point);
    }}
    clickable={true}
  />
</center>
<button class="btn btn-error align-right" onclick={reset}>Reset</button>
