<script lang="ts">
  import NavBar from "~/lib/NavBar.svelte";
  type Props = {
    currentPage: string;
  };
  const { currentPage }: Props = $props();
</script>

<NavBar
  {currentPage}
  leftLink={{
    href: "/",
    text: "Boardgame AI",
  }}
  title="Othello"
  linkList={[
    { href: "/othello", text: "Othello" },
    { href: "/othello/about", text: "About Othello AI" },
  ]}
/>
