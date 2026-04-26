<script lang="ts">
  import {
    count_score,
    generate_ai_play,
    init_othello,
    Piece,
    Point,
    place_at,
    placeable,
  } from "boardgame-ai";
  import Board from "./Board.svelte";
  import ControlPanel from "./ControlPanel.svelte";
  import ScoreBoard from "./ScoreBoard.svelte";
  import Status from "./Status.svelte";
  import type { Config } from "./types.ts";

  type Phase =
    | { kind: "playerTurn" }
    | { kind: "aiTurn" }
    | { kind: "gameOver"; result: "win" | "lose" | "tie" };

  const AI_THINK_MS = 500;
  const PASS_DISPLAY_MS = 1000;
  const BLINK_TIMEOUT_S = 3;
  const HINT_HIGHLIGHT_TIMEOUT_S = 7;

  // states
  let config = $state<Config>({
    board_size: 8,
    strength: 5,
  });
  let player = $state<Piece>(Piece.Black);
  let board = $state(init_othello(config.board_size));
  let phase = $state<Phase>(initialPhase());
  let lastPlayed = $state<Date>(new Date());

  // derived
  const ai_piece = $derived(player === Piece.Black ? Piece.White : Piece.Black);
  const turn = $derived<"player" | "ai">(phase.kind === "aiTurn" ? "ai" : "player");
  const currentPlaying = $derived(turn === "player" ? player : ai_piece);
  const turnIsPlayable = $derived(placeable(board, currentPlaying) > 0);

  // reset on board size change
  $effect(() => {
    config.board_size;
    reset();
  });

  // single phase-driven effect: decide what to do on entry to each phase.
  // every scheduled timer has a cleanup so phase changes (incl. reset) cancel
  // pending work — this is what prevents the post-loss "AI plays first" race.
  $effect(() => {
    if (phase.kind === "gameOver") return;

    const sidePiece = phase.kind === "playerTurn" ? player : ai_piece;
    const otherPiece = phase.kind === "playerTurn" ? ai_piece : player;

    if (placeable(board, sidePiece) === 0) {
      if (placeable(board, otherPiece) === 0) {
        phase = { kind: "gameOver", result: computeResult() };
        return;
      }
      const t = setTimeout(() => {
        phase = phase.kind === "playerTurn" ? { kind: "aiTurn" } : { kind: "playerTurn" };
      }, PASS_DISPLAY_MS);
      return () => clearTimeout(t);
    }

    if (phase.kind === "aiTurn") {
      const t = setTimeout(() => {
        board = generate_ai_play(board, ai_piece, config.strength);
        phase = { kind: "playerTurn" };
        lastPlayed = new Date();
      }, AI_THINK_MS);
      return () => clearTimeout(t);
    }
    // playerTurn with playable moves: wait for click handler
  });

  // hint / blink (UI only)
  let highlightClickable = $state(false);
  let turnBlinking = $state(false);
  $effect(() => {
    lastPlayed;
    highlightClickable = false;
    if (phase.kind !== "playerTurn") return;
    const t = setTimeout(() => {
      highlightClickable = true;
    }, HINT_HIGHLIGHT_TIMEOUT_S * 1000);
    return () => clearTimeout(t);
  });
  $effect(() => {
    lastPlayed;
    turnBlinking = false;
    if (phase.kind === "gameOver") return;
    const t = setTimeout(() => {
      turnBlinking = true;
    }, BLINK_TIMEOUT_S * 1000);
    return () => clearTimeout(t);
  });

  // procedures / transitions
  function initialPhase(): Phase {
    return player === Piece.Black ? { kind: "playerTurn" } : { kind: "aiTurn" };
  }
  function reset() {
    board = init_othello(config.board_size);
    phase = initialPhase();
    lastPlayed = new Date();
  }
  function play_player(point: Point) {
    if (phase.kind !== "playerTurn") return;
    board = place_at(board, player, point);
    phase = { kind: "aiTurn" };
    lastPlayed = new Date();
  }
  function computeResult(): "win" | "lose" | "tie" {
    const scores = count_score(board);
    if (scores.black === scores.white) return "tie";
    const winner = scores.black > scores.white ? Piece.Black : Piece.White;
    return winner === player ? "win" : "lose";
  }
</script>

<div class="mx-auto w-fit">
  <Status {board} {turn} human={player} blinking={turnBlinking} {turnIsPlayable} />
  <Board
    {board}
    onclick={(p: Point) => play_player(p)}
    clickable={phase.kind === "playerTurn"}
    {player}
    {highlightClickable}
  />
  <ScoreBoard {board} />
</div>
<ControlPanel reset={() => reset()} bind:config />

{#if phase.kind === "gameOver"}
  <div class="modal modal-open" role="dialog">
    <div class="modal-box text-center">
      <h2 class="text-2xl font-bold">
        {#if phase.result === "win"}
          あなたの勝ち！
        {:else if phase.result === "lose"}
          あなたの負け...
        {:else}
          引き分け
        {/if}
      </h2>
      <div class="modal-action justify-center">
        <button class="btn btn-primary" onclick={reset}>もう一度</button>
      </div>
    </div>
    <div class="modal-backdrop"></div>
  </div>
{/if}
