<script lang="ts">
  import { onMount } from 'svelte';

  let file: File | null = null;
  let uploading = false;
  let message = '';
  let error = '';
  let dropzoneActive = false;

  function handleFileSelect(e: Event) {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      file = target.files[0];
      error = '';
      message = '';
    }
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    dropzoneActive = false;
    if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
      file = e.dataTransfer.files[0];
      error = '';
      message = '';
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    dropzoneActive = true;
  }

  function handleDragLeave() {
    dropzoneActive = false;
  }

  async function uploadFile() {
    if (!file) return;

    uploading = true;
    error = '';
    message = '';

    const formData = new FormData();
    formData.append('file', file);

    try {
      // Use Vite env var or fallback to localhost
      const apiUrl = import.meta.env.VITE_API_URL || 'http://localhost:8000';
      const response = await fetch(`${apiUrl}/upload`, {
        method: 'POST',
        body: formData
      });

      if (!response.ok) {
        throw new Error('Failed to upload document');
      }

      const data = await response.json();
      message = data.message || 'Document uploaded successfully!';
      file = null;
    } catch (err: any) {
      error = err.message || 'Something went wrong during upload.';
    } finally {
      uploading = false;
    }
  }
</script>

<svelte:head>
  <title>Upload Document - BrainRAG</title>
</svelte:head>

<div class="container hero">
  <div class="hero-content animate-fade-in">
    <h1 class="title">Feed Your Brain</h1>
    <p class="subtitle">Upload your documents and let our localized LLM turn them into instant answers.</p>

    <div 
      class="dropzone {dropzoneActive ? 'active' : ''} {file ? 'has-file' : ''}"
      on:drop={handleDrop}
      on:dragover={handleDragOver}
      on:dragleave={handleDragLeave}
      role="region"
      aria-label="File upload dropzone"
    >
      <input 
        type="file" 
        id="fileInput" 
        class="file-input" 
        on:change={handleFileSelect}
        accept=".txt,.pdf,.docx,.md"
      />
      <label for="fileInput" class="dropzone-label">
        {#if file}
          <div class="file-info">
            <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="file-icon"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path><polyline points="14 2 14 8 20 8"></polyline></svg>
            <span class="file-name">{file.name}</span>
            <span class="file-size">{(file.size / 1024).toFixed(1)} KB</span>
          </div>
        {:else}
          <div class="upload-prompt">
            <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="upload-icon"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="17 8 12 3 7 8"></polyline><line x1="12" y1="3" x2="12" y2="15"></line></svg>
            <span class="prompt-text"><strong>Click to upload</strong> or drag and drop</span>
            <span class="prompt-hint">TXT, PDF, DOCX (Max 10MB)</span>
          </div>
        {/if}
      </label>
    </div>

    {#if file}
      <button 
        class="btn-primary upload-btn animate-fade-in" 
        on:click={uploadFile} 
        disabled={uploading}
      >
        {uploading ? 'Uploading...' : 'Process Document'}
      </button>
    {/if}

    {#if error}
      <div class="alert error animate-fade-in">{error}</div>
    {/if}
    {#if message}
      <div class="alert success animate-fade-in">{message}</div>
    {/if}
  </div>
</div>

<style>
  .hero {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: calc(100vh - 8rem);
  }

  .hero-content {
    background: var(--bg-card);
    padding: 3rem;
    border-radius: 24px;
    border: 1px solid var(--border-color);
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5), 0 8px 10px -6px rgba(0, 0, 0, 0.5);
    text-align: center;
    max-width: 600px;
    width: 100%;
  }

  .title {
    font-size: 2.5rem;
    font-weight: 800;
    margin-bottom: 0.5rem;
    background: linear-gradient(135deg, #e0e7ff 0%, #a5b4fc 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .subtitle {
    color: var(--text-muted);
    font-size: 1.1rem;
    margin-bottom: 2.5rem;
    line-height: 1.6;
  }

  .dropzone {
    border: 2px dashed var(--border-color);
    border-radius: 16px;
    background-color: rgba(15, 23, 42, 0.5);
    transition: all 0.2s ease;
    margin-bottom: 1.5rem;
    position: relative;
    overflow: hidden;
  }

  .dropzone.active {
    border-color: var(--primary);
    background-color: rgba(99, 102, 241, 0.1);
  }

  .dropzone.has-file {
    border-color: var(--primary);
    border-style: solid;
    background-color: rgba(30, 41, 59, 0.8);
  }

  .file-input {
    position: absolute;
    width: 0;
    height: 0;
    opacity: 0;
  }

  .dropzone-label {
    display: block;
    padding: 3rem 2rem;
    cursor: pointer;
    width: 100%;
    height: 100%;
  }

  .upload-prompt {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
  }

  .upload-icon {
    color: var(--primary);
    margin-bottom: 0.5rem;
    transition: transform 0.2s;
  }

  .dropzone:hover .upload-icon {
    transform: translateY(-5px);
  }

  .prompt-text {
    font-size: 1.05rem;
  }

  .prompt-text strong {
    color: var(--primary);
  }

  .prompt-hint {
    font-size: 0.85rem;
    color: var(--text-muted);
  }

  .file-info {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
  }

  .file-icon {
    color: var(--primary);
    margin-bottom: 0.5rem;
  }

  .file-name {
    font-weight: 500;
    font-size: 1.1rem;
    word-break: break-all;
  }

  .file-size {
    font-size: 0.9rem;
    color: var(--text-muted);
  }

  .btn-primary {
    background: linear-gradient(to right, var(--primary), #818cf8);
    color: white;
    padding: 1rem 2rem;
    border-radius: 12px;
    font-size: 1.1rem;
    font-weight: 600;
    width: 100%;
    transition: transform 0.1s, box-shadow 0.2s;
    box-shadow: 0 4px 14px 0 rgba(99, 102, 241, 0.39);
  }

  .btn-primary:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(99, 102, 241, 0.5);
  }

  .btn-primary:disabled {
    opacity: 0.7;
    cursor: not-allowed;
    transform: none;
  }

  .alert {
    margin-top: 1.5rem;
    padding: 1rem;
    border-radius: 8px;
    font-size: 0.95rem;
    text-align: left;
  }

  .alert.error {
    background-color: rgba(239, 68, 68, 0.1);
    color: #fca5a5;
    border: 1px solid rgba(239, 68, 68, 0.2);
  }

  .alert.success {
    background-color: rgba(34, 197, 94, 0.1);
    color: #86efac;
    border: 1px solid rgba(34, 197, 94, 0.2);
  }
</style>
