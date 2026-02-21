<script lang="ts">
  import { onMount } from 'svelte';
  import DefectCard from './DefectCard.svelte';

  let catalog: any = null;
  let defects: any = null;
  let loadingCatalog = true;
  let loadingDefects = false;
  let error = '';

  let showPipeline = false; // Add toggle state

  // Derived defect array to handle OSLC JSON-LD format variations
  let defectItems: any[] = [];
  $: {
    if (defects) {
      if (Array.isArray(defects)) {
        defectItems = defects;
      } else if (defects['@graph']) {
        defectItems = defects['@graph'].filter((d: any) => d['@type'] === 'oslc_cm:Defect' || (Array.isArray(d['@type']) && d['@type'].includes('oslc_cm:Defect')));
      } else if (defects['@type'] === 'oslc_cm:Defect') {
        defectItems = [defects];
      } else {
        defectItems = [];
      }
    } else {
      defectItems = [];
    }
  }

  // Creation states
  let newTitle = '';
  let newDesc = '';

  onMount(async () => {
    try {
      const response = await fetch('/catalog', {
        headers: { 'Accept': 'application/ld+json' }
      });
      if (!response.ok) throw new Error(`Failed to fetch catalog: ${response.status} ${response.statusText}`);
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
        headers: { 'Accept': 'application/ld+json' }
      });
      if (!response.ok) throw new Error(`Failed to fetch defects: ${response.status} ${response.statusText}`);
      defects = await response.json();
    } catch (err: any) {
      alert(err.message || 'Error querying defects');
    } finally {
      loadingDefects = false;
    }
  }

  async function createDefect() {
    try {
      const title = newTitle.trim() || "Sample Defect";
      const desc = newDesc.trim() || "This is a test defect created from the Svelte client.";
      
      const response = await fetch('/provider/1/defects', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/ld+json'
        },
        body: JSON.stringify({
          "prefixes": {
            "rdf": "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
            "dcterms": "http://purl.org/dc/terms/",
            "oslc": "http://open-services.net/ns/core#",
            "oslc_cm": "http://open-services.net/ns/cm#"
          },
          "dcterms:title": title,
          "dcterms:description": desc
        })
      });
      if (!response.ok) throw new Error('Failed to create defect');
      
      // Reset inputs
      newTitle = '';
      newDesc = '';
      
      await queryDefects(); // refresh list
    } catch (err: any) {
      alert(err.message || 'Error creating defect');
    }
  }

  async function updateDefect(event: CustomEvent<any>) {
    const { id: defectId, title: editTitle, desc: editDesc } = event.detail;
    try {
      const response = await fetch(`/provider/1/defects/${defectId}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/ld+json'
        },
        body: JSON.stringify({
          "prefixes": {
            "rdf": "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
            "dcterms": "http://purl.org/dc/terms/",
            "oslc": "http://open-services.net/ns/core#",
            "oslc_cm": "http://open-services.net/ns/cm#"
          },
          "dcterms:title": editTitle,
          "dcterms:description": editDesc
        })
      });
      if (!response.ok) throw new Error('Failed to update defect');
      await queryDefects();
    } catch (err: any) {
      alert(err.message || 'Error updating defect');
    }
  }

  async function deleteDefect(event: CustomEvent<any>) {
    const { id: defectId } = event.detail;
    try {
      const response = await fetch(`/provider/1/defects/${defectId}`, {
        method: 'DELETE',
        headers: { 'Accept': 'application/json' }
      });
      if (!response.ok) throw new Error('Failed to delete defect');
      await queryDefects();
    } catch (err: any) {
      alert(err.message || 'Error deleting defect');
    }
  }

  async function deleteAllDefects() {
    try {
      const response = await fetch('/provider/1/defects', {
        method: 'DELETE',
        headers: { 'Accept': 'application/json' }
      });
      if (!response.ok) throw new Error('Failed to delete defects');
      await queryDefects(); // refresh list to show empty state
    } catch (err: any) {
      alert(err.message || 'Error deleting defects');
    }
  }
</script>

<main class="min-h-screen w-full bg-gray-dark text-gray-light font-sans selection:bg-primary/30 p-4 md:p-8">
  <div class="max-w-6xl mx-auto">
    <header class="text-center mb-12 lg:mb-16 pt-8">
      <h1 class="text-4xl md:text-6xl font-extrabold bg-gradient-to-r from-primary via-secondary to-accent bg-clip-text text-transparent mb-4 tracking-tight drop-shadow-sm">
        OSLC Proof of Concept
      </h1>
      <p class="text-gray-mid text-lg md:text-xl font-medium max-w-2xl mx-auto">A modern functional ecosystem facilitating seamless engineering data integration.</p>
    </header>

    {#if loadingCatalog}
      <div class="flex justify-center items-center py-20">
        <div class="text-center text-gray-mid text-xl animate-pulse font-semibold flex items-center gap-3">
          <svg class="animate-spin h-6 w-6 text-primary" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
          Loading OSLC Catalog...
        </div>
      </div>
    {:else if error}
      <div class="bg-red-900/20 text-red-400 p-6 rounded-sm text-center border border-red-500/30 backdrop-blur-sm max-w-2xl mx-auto shadow-lg shadow-red-900/10">
        <p class="font-medium flex items-center justify-center gap-2">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
          {error}
        </p>
      </div>
    {:else if catalog}
      <div class="flex flex-col gap-6">
        
        <!-- Control Panel (Top Row) -->
        <div class="bg-gray-dark/50 backdrop-blur-md rounded-md p-6 shadow-xl border border-gray-mid/20 hover:border-secondary/50 transition-all duration-300 flex flex-col md:flex-row gap-6 items-center justify-between relative overflow-hidden group">
          <div class="absolute -top-16 -right-16 w-40 h-40 bg-primary/10 rounded-full blur-3xl group-hover:bg-primary/20 transition-all duration-700 pointer-events-none"></div>

          <!-- Catalog Information -->
          <div class="relative z-10 flex-1 w-full text-center md:text-left">
            <div class="inline-flex items-center gap-2 px-3 py-1 rounded-sm bg-secondary/10 text-secondary text-xs font-bold uppercase tracking-wider mb-2 border border-secondary/20">
              Service Provider
            </div>
            <h2 class="text-2xl md:text-3xl font-bold mb-1 text-gray-light leading-tight">{catalog['dcterms:title'] || catalog.title || 'Service Provider Catalog'}</h2>
            <p class="text-gray-mid text-sm md:text-base">{catalog['dcterms:description'] || catalog.description || 'OSLC Catalog'}</p>
          </div>
            
          <!-- Action Buttons Form -->
          <div class="relative z-10 flex flex-col md:flex-row gap-4 items-start shrink-0">
            <form on:submit|preventDefault={createDefect} class="flex flex-col gap-2 w-full md:w-64 border-r border-gray-mid/10 md:pr-4">
               <!-- Creation Form -->
               <input type="text" bind:value={newTitle} placeholder="Defect Title (optional)" class="bg-[#0b1121] border border-gray-mid/20 rounded px-3 py-1.5 text-sm focus:ring-2 focus:ring-accent outline-none text-white w-full" />
               <input type="text" bind:value={newDesc} placeholder="Description (optional)" class="bg-[#0b1121] border border-gray-mid/20 rounded px-3 py-1.5 text-sm focus:ring-2 focus:ring-accent outline-none text-white w-full" />
               
               <button 
                type="submit"
                class="bg-accent/70 hover:bg-accent/90 px-4 py-2 text-sm w-full mt-1"
               >
                 <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" /></svg>
                 Create Defect
               </button>
            </form>
            
            <div class="flex flex-col gap-2 w-full md:w-auto h-full justify-center">
              <!-- Query Action (Blue) -->
              <button 
                class="bg-primary px-5 py-2 text-sm" 
                on:click={queryDefects} 
                disabled={loadingDefects}
              >
              {#if loadingDefects}
                <svg class="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
                Querying...
              {:else}
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd" /></svg>
                Query Defects
              {/if}
            </button>

            <!-- Delete All Action (Red) -->
            <button 
              class="bg-red-600/40 hover:bg-red-600/60 hover:ring-red-500/40 shadow-red-600/20 px-5 py-2 text-sm" 
              on:click={deleteAllDefects} 
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" /></svg>
              Delete All
            </button>

            </div>
          </div>
        </div>
        
        <!-- Defects Data (Bottom Full Width) -->
        {#if defects}
          <!-- Defect Cards Collection -->
          {#if defectItems.length > 0}
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-4">
              {#each defectItems as defect}
                <DefectCard 
                  {defect} 
                  on:updateDefect={updateDefect} 
                  on:deleteDefect={deleteDefect} 
                />
              {/each}
            </div>
          {/if}

          <div class="bg-[#172338] rounded-sm p-8 shadow-2xl border border-gray-mid/20 hover:border-primary/50 transition-all duration-300 flex flex-col relative overflow-hidden group">
            <div class="absolute -bottom-16 -left-16 w-40 h-40 bg-secondary/10 rounded-full blur-3xl group-hover:bg-secondary/20 transition-all duration-700"></div>

            <div class="relative z-10 flex flex-col h-full">
              <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-6">
                <div class="flex items-center gap-4">
                  <h2 class="text-2xl font-bold text-gray-light flex items-center gap-3">
                    <span class="w-1.5 h-6 rounded-full bg-secondary shadow-[0_0_10px_rgba(138,49,179,0.5)]"></span>
                    Results Pipeline
                  </h2>
                  <span class="bg-gray-mid/10 text-gray-mid text-xs font-bold px-3 py-1.5 rounded-sm border border-gray-mid/20 whitespace-nowrap">
                    {defectItems.length} items found
                  </span>
                </div>
                <!-- Inline Toggle -->
                <button 
                  class="bg-secondary/20 hover:bg-secondary/40 text-secondary border border-secondary/30 text-xs px-4 py-1.5 rounded-sm transition-all font-bold"
                  on:click={() => showPipeline = !showPipeline}
                >
                  {showPipeline ? 'Hide Raw Data' : 'Show Raw Data'}
                </button>
              </div>
              
              {#if showPipeline}
                {#if defectItems.length === 0}
                  <div class="flex-1 flex flex-col items-center justify-center text-gray-mid opacity-80 border-2 border-dashed border-gray-mid/20 rounded-sm p-8 mt-2 min-h-[250px]">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-14 w-14 mb-4 text-gray-mid/50" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" /></svg>
                    <p class="font-medium text-lg text-gray-light">No defects found</p>
                    <p class="text-sm mt-1 text-center">The current query returned an empty dataset from the provider.</p>
                  </div>
                {:else}
                  <div class="bg-[#0b1121] p-5 rounded-md overflow-x-auto border border-gray-mid/10 shadow-inner flex-1 bg-opacity-80 max-h-96 overflow-y-auto mt-2">
                    <pre class="text-sm font-mono text-primary/90 leading-relaxed tracking-wide">{JSON.stringify(defectItems, null, 2)}</pre>
                  </div>
                {/if}
              {/if}
            </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</main>

<style>
  :global(body) {
    background-color: var(--color-gray-dark);
  }
</style>
