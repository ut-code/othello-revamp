<script lang="ts">
  import { type Board, type Piece, Point, can_place } from "boardgame-ai";
  type Props = {
    board: Board;
    onclick: (pos: Point) => void;
    clickable: boolean;
    player: Piece;
  };
  const { board, onclick, clickable, player }: Props = $props();
  const data: ("." | "w" | "b")[][] = $derived(board.get_data());

  Point.create(0, 0); // prevent biome from turning this into a type import
</script>

{#each data as row, y (y)}
  {#each row as cell, x (x)}
    {#if cell === "w"}
      <span class="font-mono">W</span>
    {:else if cell === "b"}
      <span class="font-mono">B</span>
    {:else}
      {@const point = Point.create(x, y)}
      {@const can_click = clickable && can_place(board, point, player)}
      <button
        class="font-mono"
        disabled={!can_click}
        onclick={() => {
          if (!can_click) return;
          onclick(point);
        }}>{can_click ? "_" : "."}</button
      >
    {/if}
  {/each}
  <br />
{/each}
