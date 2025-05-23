<script lang="ts">
  import {
    Piece,
    Point,
    count_score,
    generate_ai_play,
    init_othello,
    place_at,
    placeable,
  } from "boardgame-ai";
  import Board from "./Board.svelte";
  import ControlPanel from "./ControlPanel.svelte";
  import ScoreBoard from "./ScoreBoard.svelte";
  import Status from "./Status.svelte";
  import type { Config } from "./types.ts";

  // states
  let config = $state<Config>({
    board_size: 8,
    strength: 5,
  });
  let board = $state(init_othello(config.board_size));
  $effect(() => {
    reset(config.board_size);
    lastPlayed = new Date();
  });

  let player = $state<Piece>(Piece.Black);
  let turn = $state<"player" | "ai">("player");
  const ai_piece = $derived(
    (() => {
      switch (player) {
        case Piece.Black:
          return Piece.White;
        case Piece.White:
          return Piece.Black;
      }
    })(),
  );
  const currentPlaying = $derived(turn === "player" ? player : ai_piece);
  const turnIsPlayable = $derived(placeable(board, currentPlaying) > 0);

  // effects
  // play ai when it's ai's turn
  $effect(() => {
    if (turn === "ai") {
      setTimeout(() => {
        if (turn !== "ai") return;
        play_ai();
        turn = "player";
      }, 500);
    }
  });
  // stuck prevention
  $effect(() => {
    if (!placeable(board, currentPlaying)) {
      setTimeout(() => {
        turn = turn === "player" ? "ai" : "player";
      }, 1000);
    }
  });
  $effect(() => {
    if (!placeable(board, player) && !placeable(board, ai_piece)) {
      finalize();
    }
  });

  // timeout 処理
  let lastPlayed = $state<Date>();
  let highlightClickable = $state(false);
  let turnBlinking = $state(false);

  const BLINK_TIMEOUT = 3; /* seconds */
  const HINT_HIGHLIGHT_TIMEOUT = 7; /* seconds */
  $effect(() => {
    lastPlayed;
    highlightClickable = false;
    if (turn === "player") {
      const to = setTimeout(() => {
        highlightClickable = true;
      }, HINT_HIGHLIGHT_TIMEOUT * 1000);
      return () => clearTimeout(to);
    }
  });
  $effect(() => {
    lastPlayed;
    turnBlinking = false;
    const to = setTimeout(() => {
      turnBlinking = true;
    }, BLINK_TIMEOUT * 1000);
    return () => clearTimeout(to);
  });

  // procedures / transitions
  function reset(board_size: number) {
    turn = player === Piece.Black ? "player" : "ai";
    board = init_othello(board_size);
    // setting lastPlayed to undefined twice will not trigger highlightClickable reset
    // because of how svelte evaluates value equality
    lastPlayed = new Date();
  }
  function play_player(point: Point) {
    board = place_at(board, player, point);
    turn = "ai";
    lastPlayed = new Date();
  }
  function play_ai() {
    board = generate_ai_play(board, ai_piece, config.strength);
    lastPlayed = new Date();
  }
  function finalize() {
    const scores = count_score(board);
    console.log({ black: scores.black, white: scores.white });
    // todo: make a better result
    const finalState: "win" | "lose" | "tie" = isPlayerWinning(scores);
    switch (finalState) {
      case "win":
        alert("you win!");
        break;
      case "lose":
        alert("you lose...");
        break;
      case "tie":
        alert("tie!");
        break;
      default:
        finalState satisfies never;
    }
    reset(config.board_size);
  }

  // helper functions
  function whoisPlaying(piece: Piece): "player" | "ai" {
    return player === piece ? "player" : "ai";
  }
  function isPlayerWinning(scores: { black: number; white: number }): "win" | "lose" | "tie" {
    // nested ternary operator lmao
    return scores.black === scores.white
      ? "tie"
      : whoisPlaying(scores.black > scores.white ? Piece.Black : Piece.White) === "player"
        ? "win"
        : "lose";
  }
</script>

<h1 class="mb-20 text-5xl font-bold">Othello AI</h1>

<div class="mx-auto w-fit">
  <Status {board} {turn} human={player} blinking={turnBlinking} {turnIsPlayable} />
  <Board
    {board}
    onclick={(p: Point) => play_player(p)}
    clickable={turn === "player"}
    {player}
    {highlightClickable}
  />
  <ScoreBoard {board} />
</div>
<ControlPanel reset={() => reset(config.board_size)} bind:config />
