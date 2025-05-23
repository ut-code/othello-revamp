<h1 class="my-10 text-center text-4xl font-bold tracking-tight text-gray-900 dark:text-white">
  Welcome to Boardgame AI!
</h1>

<center>
  <a href="/othello" class="btn btn-primary max-w-sm">オセロ</a>
</center>

<p id="turn-part"></p>
<p id="which-is-human"></p>
<div id="content">
  <table id="board"></table>
</div>
<div id="counter" class="hide">
  <div id="numBlack"></div>
  <div id="numWhite"></div>
</div>
<div id="plus-counter">
  <div id="numBlackPlus" class="hide"></div>
  <div id="numWhitePlus" class="hide"></div>
</div>
<div id="modal">
  <div id="start-dialog" class="dialog hide">
    <h1>Let's play 6x6 reversi!!</h1>
    <button id="start-button">プレイする</button>
  </div>
  <div id="finish-dialog" class="dialog hide">
    <button id="restart-button">トップに戻る</button>
  </div>
</div>

<style>
  :global(html) {
    min-height: 100%;
  }
</style>
