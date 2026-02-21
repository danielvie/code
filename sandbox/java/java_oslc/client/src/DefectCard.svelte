<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  
  export let defect: any;
  
  let isEditing = false;
  let editTitle = '';
  let editDesc = '';

  const dispatch = createEventDispatcher();

  function startEdit() {
    isEditing = true;
    editTitle = defect['dcterms:title'] || '';
    editDesc = defect['dcterms:description'] || '';
  }

  function cancelEdit() {
    isEditing = false;
  }

  function handleSave() {
    dispatch('updateDefect', { 
        id: defect['dcterms:identifier'], 
        title: editTitle, 
        desc: editDesc 
    });
    isEditing = false;
  }

  function handleDelete() {
    dispatch('deleteDefect', { id: defect['dcterms:identifier'] });
  }
</script>

<div class="bg-gray-dark/40 backdrop-blur-md rounded-md p-5 border border-gray-mid/20 hover:border-primary/50 transition-all flex flex-col h-full shadow-lg relative group">
  {#if isEditing}
    <!-- Edit Mode -->
    <div class="flex flex-col h-full z-10">
      <input type="text" bind:value={editTitle} class="bg-[#0b1121] border border-primary/50 rounded px-3 py-2 text-sm mb-3 focus:ring-1 focus:ring-primary outline-none text-white w-full" placeholder="Title" />
      <textarea bind:value={editDesc} class="bg-[#0b1121] border border-primary/50 rounded px-3 py-2 text-sm mb-4 focus:ring-1 focus:ring-primary outline-none text-white w-full h-24 resize-none" placeholder="Description"></textarea>
      <div class="flex gap-2 mt-auto">
        <button class="bg-primary flex-1 py-1.5 text-xs rounded-sm hover:brightness-110 active:brightness-90 font-bold text-white transition-all shadow-md shadow-primary/20" on:click={handleSave}>Save</button>
        <button class="bg-gray-mid/20 flex-1 py-1.5 text-xs rounded-sm hover:bg-gray-mid/30 text-white font-bold transition-all" on:click={cancelEdit}>Cancel</button>
      </div>
    </div>
  {:else}
    <!-- View Mode -->
    <div class="flex flex-col h-full z-10">
      <div class="flex justify-between items-start mb-2 gap-2">
        <h3 class="font-bold text-lg text-gray-light leading-tight line-clamp-2">{defect['dcterms:title'] || 'Untitled'}</h3>
        <span class="text-[10px] bg-secondary/20 text-secondary border border-secondary/30 px-2 py-0.5 rounded opacity-80 shrink-0">Defect</span>
      </div>
      <p class="text-sm text-gray-mid mb-5 line-clamp-3">{defect['dcterms:description'] || 'No description provided.'}</p>
      
      <div class="text-[11px] text-gray-mid/50 mb-4 mt-auto break-all font-mono opacity-60">ID: {defect['dcterms:identifier']}</div>
      
      <!-- Action Row -->
      <div class="flex gap-2 justify-end mt-auto opacity-0 group-hover:opacity-100 transition-opacity duration-200">
        <button 
          class="bg-blue-600/20 hover:bg-blue-600/40"
          on:click={startEdit}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" viewBox="0 0 20 20" fill="currentColor"><path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z" /></svg>
          Edit
        </button>
        <button 
          class="bg-red-600/20 hover:bg-red-600/40"
          on:click={handleDelete}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" /></svg>
          Delete
        </button>
      </div>
    </div>
  {/if}
</div>
