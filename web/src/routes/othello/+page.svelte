<script lang="ts">
  import {
    Piece,
    count_score,
    generate_ai_play,
    init_othello,
    place_at,
    placeable,
  } from "boardgame-ai";
  import Board from "./Board.svelte";

  // states
  let board_size = 8;
  let strength = 15;
  let board = $state(init_othello(board_size));
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

  // effects
  // play ai when it's ai's turn
  $effect(() => {
    if (turn === "ai") {
      setTimeout(() => {
        play_ai();
        turn = "player";
      }, 500);
    }
  });
  // stuck prevention
  $effect(() => {
    if (turn === "player" && !placeable(board, player)) {
      setTimeout(() => {
        turn = "ai";
      }, 1000);
    }
  });
  $effect(() => {
    if (!placeable(board, player) && !placeable(board, ai_piece)) {
      finalize();
    }
  });

  // procedures / transitions
  function reset() {
    turn = player === Piece.Black ? "player" : "ai";
    board = init_othello(board_size);
  }
  function play_ai() {
    board = generate_ai_play(board, ai_piece, strength);
    turn = "player";
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
    reset();
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

<h1 class="text-5xl font-bold">Welcome to Othello AI!</h1>
<center>
  <Board
    {board}
    onclick={(point) => {
      board = place_at(board, player, point);
      turn = "ai";
    }}
    clickable={turn === "player"}
    {player}
  />
</center>
<button class="btn btn-error align-right" onclick={reset}>Reset</button>
