<script lang="ts">
  import { onMount } from 'svelte';

  let catalog: any = null;
  let defects: any = null;
  let loadingCatalog = true;
  let loadingDefects = false;
  let error = '';

  onMount(async () => {
    try {
      const response = await fetch('/catalog', {
        headers: { 'Accept': 'application/json-ld' }
      });
      if (!response.ok) throw new Error('Failed to fetch catalog');
      catalog = await response.json();
    } catch (err: any) {
      error = err.message || 'Error occurred while fetching Catalog';
    } finally {
      loadingCatalog = false;
    }
  });

  async function queryDefects() {
    loadingDefects = true;
    try {
      const response = await fetch('/provider/1/defects', {
        headers: { 'Accept': 'application/json-ld' }
      });
      if (!response.ok) throw new Error('Failed to fetch defects');
      defects = await response.json();
    } catch (err: any) {
      alert(err.message || 'Error querying defects');
    } finally {
      loadingDefects = false;
    }
  }

  async function createDefect() {
    try {
      const response = await fetch('/provider/1/defects', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json-ld',
          'Accept': 'application/json-ld'
        },
        body: JSON.stringify({
          title: "Sample Defect",
          description: "This is a test defect created from the Svelte client."
        })
      });
      if (!response.ok) throw new Error('Failed to create defect');
      await queryDefects(); // refresh list
    } catch (err: any) {
      alert(err.message || 'Error creating defect');
    }
  }
</script>

<main class="container">
  <header>
    <h1>OSLC Proof of Concept</h1>
    <p>A functional ecosystem facilitating seamless integration.</p>
  </header>

  {#if loadingCatalog}
    <div class="loading">Loading OSLC Catalog...</div>
  {:else if error}
    <div class="error">{error}</div>
  {:else if catalog}
    <div class="grid">
      <div class="card catalog-card">
        <h2>{catalog['dcterms:title'] || catalog.title || 'Service Provider Catalog'}</h2>
        <p>{catalog['dcterms:description'] || catalog.description || 'OSLC Catalog'}</p>
        <div class="actions">
          <button on:click={queryDefects} disabled={loadingDefects}>
            {loadingDefects ? 'Querying...' : 'Query Defects'}
          </button>
          <button class="secondary" on:click={createDefect}>Create Defect</button>
        </div>
      </div>
      
      {#if defects}
        <div class="card defects-card">
          <h2>Queried Defects</h2>
          {#if Array.isArray(defects) && defects.length === 0}
            <p>No defects found.</p>
          {:else}
            <pre>{JSON.stringify(defects, null, 2)}</pre>
          {/if}
        </div>
      {/if}
    </div>
  {/if}
</main>

<style>
  :global(body) {
    font-family: 'Inter', system-ui, sans-serif;
    background-color: #0f172a;
    color: #f8fafc;
    margin: 0;
    padding: 0;
  }
  .container {
    max-width: 1000px;
    margin: 0 auto;
    padding: 2rem;
  }
  header {
    text-align: center;
    margin-bottom: 3rem;
  }
  h1 {
    font-size: 2.5rem;
    background: linear-gradient(to right, #38bdf8, #818cf8);
    -webkit-background-clip: text;
    background-clip: text;
    color: transparent;
    margin-bottom: 0.5rem;
  }
  .grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 2rem;
  }
  @media (min-width: 768px) {
    .grid {
      grid-template-columns: 1fr 1fr;
    }
  }
  .card {
    background: #1e293b;
    border-radius: 12px;
    padding: 2rem;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
    border: 1px solid #334155;
    transition: transform 0.2s;
  }
  .card:hover {
    transform: translateY(-2px);
  }
  .actions {
    margin-top: 1.5rem;
    display: flex;
    gap: 1rem;
  }
  button {
    background: #3b82f6;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s;
  }
  button:hover:not(:disabled) {
    background: #2563eb;
  }
  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  button.secondary {
    background: #475569;
  }
  button.secondary:hover {
    background: #334155;
  }
  .loading {
    text-align: center;
    color: #94a3b8;
    font-size: 1.25rem;
  }
  .error {
    color: #ef4444;
    background: #450a0a;
    padding: 1rem;
    border-radius: 8px;
    text-align: center;
  }
  pre {
    background: #0f172a;
    padding: 1rem;
    border-radius: 8px;
    overflow-x: auto;
    font-size: 0.875rem;
    border: 1px solid #334155;
  }
</style>
