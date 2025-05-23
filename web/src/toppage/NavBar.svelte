<script lang="ts">
  import NavBar from "~/lib/NavBar.svelte";
  type Props = {
    currentPage: string;
  };
  const { currentPage }: Props = $props();

  const festival = {
    href: "https://festival.utcode.net",
    text: "← 五月祭トップに戻る",
    class: "text-white",
  };
</script>

<NavBar
  {currentPage}
  leftLink={festival}
  title="Boardgame AI"
  linkList={[{ href: "/", text: "Home" }]}
/>
