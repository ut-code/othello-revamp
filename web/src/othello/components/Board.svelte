<script lang="ts">
  import { type Board, type Piece, Point, can_place } from "boardgame-ai";
  type Props = {
    board: Board;
    onclick: (pos: Point) => void;
    clickable: boolean;
    player: Piece;
    highlightClickable: boolean;
  };

  const { board, onclick, clickable, player, highlightClickable }: Props = $props();
  const data: ("." | "w" | "b")[][] = $derived(board.get_data());
</script>

<table class="bg-[var(--color-board-green)]">
  <tbody>
    {#each data as row, y (y)}
      <tr>
        {#each row as cell, x (x)}
          <td class="cell h-[46px] w-[46px] border border-black">
            {@render CellButton(x, y, cell)}
          </td>
        {/each}
      </tr>
    {/each}
  </tbody>
</table>

{#snippet CellButton(x: number, y: number, cell: "b" | "w" | ".")}
  {@const point = Point.create(x, y)}
  {@const can_click = clickable && can_place(board, point, player)}
  <button
    class={["inline-block h-full w-full", can_click && "cursor-pointer"]}
    aria-label={cell === "b" ? "black disc" : cell === "w" ? "white disc" : "empty cell"}
    disabled={!can_click}
    onclick={() => {
      if (!can_click) return;
      onclick(point);
    }}
  >
    <div
      class={[
        "mx-auto my-auto h-[35px] w-[35px] rounded-full",
        cell === "b" ? "bg-black" : cell === "w" ? "bg-white" : "",
        can_click && highlightClickable ? "h-5 w-5 animate-ping bg-red-300 opacity-100" : "",
      ]}
    ></div>
  </button>
{/snippet}

<style>
  :global(body) {
    text-align: center;
  }
</style>
