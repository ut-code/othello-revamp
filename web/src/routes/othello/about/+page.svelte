<script lang="ts">
  import Navbar from "~/othello/components/Navbar.svelte";
</script>

<Navbar currentPage="/othello/about" />

<div class="container mx-auto mt-20 max-w-xl">
  <h1 class="text-3xl font-bold">このAIについて</h1>

  <h2 class="mt-4 text-2xl font-bold">AIの実装について</h2>
  <p class="mt-2">
    このAIは、いわゆるAIというより、ミニマックス戦略に基づく計算機です。
    ミニマックス戦略は、ゲーム理論における基本的な最適化手法で、
    最も良い手を計算的に導き出すためのアルゴリズムです。
  </p>

  <h2 class="mt-8 text-2xl font-bold">テクノロジー</h2>
  <p class="mt-2">このアプリケーションは、以下の技術を使用して開発されています：</p>

  <div class="mt-4 flex flex-wrap justify-center">
    <div class="w-1/2 p-4 md:w-1/4">
      <img src="/rust-logo.png" alt="Rust" class="mx-auto h-12 w-auto" />
      <p class="mt-2 text-center">Rust</p>
    </div>
    <div class="w-1/2 p-4 md:w-1/4">
      <img src="/wasm-logo.png" alt="WebAssembly" class="mx-auto h-12 w-auto" />
      <p class="mt-2 text-center">WebAssembly</p>
    </div>
  </div>

  <p class="mt-4">
    Rustは、安全性と並行性に優れたシステムプログラミング言語で、
    高速な実行と効率的なメモリ管理を可能にします。
    WebAssembly（Wasm）は、Webアプリケーションのパフォーマンスを向上させるための
    バイナリ形式で、Rustから直接コンパイルすることができます。
  </p>

  <h2 class="mt-8 text-2xl font-bold">パフォーマンス最適化</h2>
  <p class="mt-2">
    このアプリケーションは、WebAssemblyを使用することで、
    メモリの使用と処理速度の最適化を実現しています。
    Rustの型安全性とWebAssemblyの効率的な実行により、
    ゲームの計算を高速かつ効率的に行うことができます。
  </p>
</div>
