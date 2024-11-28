<script lang="ts">
  import { type Board, type Piece, type Point, can_place } from "boardgame-ai";
  type Props = {
    board: Board;
    onclick: (pos: Point) => void;
    clickable: boolean;
    player: Piece;
  };
  const { board, onclick, clickable, player }: Props = $props();
  const data: ("." | "w" | "b")[][] = $derived(board.get_data());
</script>

{#each data as row, y (y)}
  {#each row as cell, x (x)}
    {#if cell === "w"}
      <span class="font-mono">W</span>
    {:else if cell === "b"}
      <span class="font-mono">B</span>
    {:else}
      {@const point = Point.create(x, y)}
      <button
        class="font-mono"
        disabled={!clickable && can_place(board, point, player)}
        onclick={() => {
          onclick(point);
        }}>.</button
      >
    {/if}
  {/each}
  <br />
{/each}
