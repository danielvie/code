<script>
  import './styles.css';
</script>

<nav class="navbar">
  <div class="container nav-content">
    <a href="/" class="logo">BrainRAG</a>
    <div class="nav-links">
      <a href="/">Upload</a>
      <a href="/chat">Chat</a>
      <a href="/docs">Documents</a>
    </div>
  </div>
</nav>

<main class="main-content">
  <slot />
</main>

<style>
  .navbar {
    background-color: rgba(30, 41, 59, 0.8);
    backdrop-filter: blur(12px);
    border-bottom: 1px solid var(--border-color);
    position: sticky;
    top: 0;
    z-index: 50;
  }

  .nav-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
    height: 4rem;
  }

  .logo {
    font-size: 1.25rem;
    font-weight: 700;
    letter-spacing: -0.025em;
    background: linear-gradient(to right, #818cf8, #c084fc);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .nav-links {
    display: flex;
    gap: 1.5rem;
  }

  .nav-links a {
    font-size: 0.95rem;
    font-weight: 500;
    color: var(--text-muted);
    transition: color 0.2s;
  }

  .nav-links a:hover {
    color: var(--text-main);
  }

  .main-content {
    min-height: calc(100vh - 4rem);
    padding: 2rem 0;
  }
</style>
