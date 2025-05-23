<script lang="ts">
  type Props = {
    leftLink: {
      href: string;
      text: string;
      class?: string;
    };
    title: string;
    linkList: { href: string; text: string }[];
    currentPage: string;
  };
  const { currentPage, leftLink, title, linkList }: Props = $props();
</script>

<nav class="navbar">
  <a href={leftLink.href} class="navbar-start {leftLink.class ?? 'logo'}">{leftLink.text}</a>
  <span class="navbar-center logo">{title}</span>
  <div class="navbar-end nav-links">
    {#each linkList as link}
      <a href={link.href} class={currentPage === link.href ? "active" : ""}>{link.text}</a>
    {/each}
  </div>
</nav>

<style>
  .navbar {
    background-color: #2c3e50;
    padding: 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    z-index: 1000;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .logo {
    color: white;
    font-size: 1.5rem;
    font-weight: bold;
    text-decoration: none;
  }

  .nav-links {
    display: flex;
    gap: 2rem;
  }

  .nav-links a {
    color: white;
    text-decoration: none;
    font-size: 1rem;
    transition: color 0.3s ease;
  }

  .nav-links a:hover {
    color: #3498db;
  }

  .nav-links a.active {
    color: #3498db;
  }

  @media (max-width: 768px) {
    .nav-links {
      display: none;
    }
  }
</style>
