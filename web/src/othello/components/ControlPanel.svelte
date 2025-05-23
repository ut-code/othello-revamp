<script lang="ts">
  import type { Config } from "./types.ts";

  type Props = {
    reset: () => void;
    config: Config;
  };
  let { reset, config = $bindable() }: Props = $props();

  const selectClass = /* __css */ "border rounded-lg p-1";
</script>

<button class="btn btn-error align-right" onclick={reset}>盤面をリセット</button>

<div class="mt-2 space-y-4">
  <label for="board-size">盤面サイズ:</label>
  <select id="board-size" class={selectClass} bind:value={config.board_size}>
    {#each [4, 6, 8, 10, 12] as size}
      <option value={size}>{size}x{size}</option>
    {/each}
  </select>

  <label for="strength">AI の強さ:</label>
  <select id="strength" class={selectClass} bind:value={config.strength}>
    {#each [1, 2, 3, 4, 5, 6, 7, 8, 9] as s}
      <option value={s}>{s}</option>
    {/each}
  </select>
</div>
