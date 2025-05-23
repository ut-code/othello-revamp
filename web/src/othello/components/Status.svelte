<script lang="ts">
  import { type Board, Piece } from "boardgame-ai";
  type Props = {
    board: Board;
    turn: "player" | "ai";
    human: Piece;
    blinking: boolean;
    turnIsPlayable: boolean;
  };
  const { board, turn, human, blinking, turnIsPlayable }: Props = $props();
</script>

{#if turn === "player"}
  <p class="text-lg {blinking && 'pulse'}">あなたの番です {turnIsPlayable ? "" : "(パス)"}</p>
{:else}
  <p class="text-lg">AI の番です {turnIsPlayable ? "" : "(パス)"}</p>
{/if}
<p
  class={[
    "mx-auto w-fit rounded-t-lg px-8 py-1 text-sm",
    human === Piece.Black ? "bg-black text-white" : "bg-white text-black",
  ]}
>
  {human === Piece.Black ? "あなたは黒 (先手) です" : "あなたは白 (後手) です"}
</p>

<style>
  .pulse {
    animation: pulse 1s infinite;
  }

  @keyframes pulse {
    0% {
      opacity: 1;
    }
    50% {
      opacity: 0;
    }
    100% {
      opacity: 1;
    }
  }
</style>
