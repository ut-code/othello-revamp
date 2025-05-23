<script lang="ts">
  import { type Board, Piece, score } from "boardgame-ai";

  type Props = {
    board: Board;
  };

  const { board }: Props = $props();

  let whiteScore = $derived(score(board, Piece.White));
  let blackScore = $derived(score(board, Piece.Black));
</script>

<div class="my-5 flex flex-1 justify-around text-center text-3xl font-bold">
  <span class="rounded bg-black px-10 py-1 text-white">
    {blackScore}
  </span>
  <span class="rounded border border-black bg-white px-10 py-1 text-black">
    {whiteScore}
  </span>
</div>
