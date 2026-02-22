<script lang="ts">
    import { onMount } from "svelte";

    let documents: string[] = [];
    let loading = true;
    let error = "";

    async function fetchDocuments() {
        loading = true;
        error = "";
        try {
            const apiUrl =
                import.meta.env.VITE_API_URL || "http://localhost:8000";
            const response = await fetch(`${apiUrl}/docs`);
            if (!response.ok) {
                throw new Error("Failed to fetch documents");
            }
            documents = await response.json();
        } catch (err: any) {
            error = err.message || "Could not load documents.";
        } finally {
            loading = false;
        }
    }

    async function deleteDocument(id: string) {
        if (!confirm(`Are you sure you want to delete ${id}?`)) return;

        try {
            const apiUrl =
                import.meta.env.VITE_API_URL || "http://localhost:8000";
            const response = await fetch(
                `${apiUrl}/docs/${encodeURIComponent(id)}`,
                {
                    method: "DELETE",
                },
            );
            if (!response.ok) {
                throw new Error("Failed to delete document");
            }

            // Refresh list
            documents = documents.filter((doc) => doc !== id);
        } catch (err: any) {
            alert(err.message || "Could not delete document.");
        }
    }

    onMount(() => {
        fetchDocuments();
    });
</script>

<svelte:head>
    <title>Documents - BrainRAG</title>
</svelte:head>

<div class="container docs-section">
    <div class="header-area">
        <div>
            <h1 class="page-title">Knowledge Base</h1>
            <p class="page-subtitle">
                Manage the documents uploaded to your localized LLM
            </p>
        </div>
        <button class="btn-refresh" on:click={fetchDocuments}>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="18"
                height="18"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                ><path
                    d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.3"
                /></svg
            >
            Refresh
        </button>
    </div>

    {#if loading}
        <div class="loading-state">
            <div class="spinner"></div>
            <p>Loading documents...</p>
        </div>
    {:else if error}
        <div class="alert error">{error}</div>
    {:else if documents.length === 0}
        <div class="empty-state">
            <div class="empty-icon">📁</div>
            <h3>No documents found</h3>
            <p>Upload some files to get started with the RAG system.</p>
            <a
                href="/"
                class="btn-primary"
                style="display: inline-block; margin-top: 1rem;"
                >Upload Document</a
            >
        </div>
    {:else}
        <div class="docs-grid animate-fade-in">
            {#each documents as doc}
                <div class="doc-card">
                    <div class="doc-icon">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            ><path
                                d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"
                            ></path><polyline points="14 2 14 8 20 8"
                            ></polyline></svg
                        >
                    </div>
                    <div class="doc-info">
                        <h3 class="doc-title">{doc}</h3>
                        <span class="doc-status">Indexed</span>
                    </div>
                    <button
                        class="btn-delete"
                        on:click={() => deleteDocument(doc)}
                        aria-label="Delete document"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="18"
                            height="18"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            ><polyline points="3 6 5 6 21 6"></polyline><path
                                d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
                            ></path><line x1="10" y1="11" x2="10" y2="17"
                            ></line><line x1="14" y1="11" x2="14" y2="17"
                            ></line></svg
                        >
                    </button>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .docs-section {
        max-width: 900px;
    }

    .header-area {
        display: flex;
        justify-content: space-between;
        align-items: flex-end;
        margin-bottom: 2.5rem;
    }

    .page-title {
        font-size: 2rem;
        font-weight: 700;
        margin-bottom: 0.5rem;
    }

    .page-subtitle {
        color: var(--text-muted);
    }

    .btn-refresh {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem 1rem;
        background: rgba(51, 65, 85, 0.5);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        color: var(--text-main);
        font-size: 0.9rem;
        transition: all 0.2s;
    }

    .btn-refresh:hover {
        background: var(--border-color);
    }

    .docs-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
        gap: 1.5rem;
    }

    .doc-card {
        background: var(--bg-card);
        border: 1px solid var(--border-color);
        border-radius: 16px;
        padding: 1.5rem;
        display: flex;
        align-items: center;
        gap: 1rem;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .doc-card:hover {
        border-color: var(--primary);
        transform: translateY(-2px);
        box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.3);
    }

    .doc-icon {
        width: 48px;
        height: 48px;
        border-radius: 12px;
        background: rgba(99, 102, 241, 0.1);
        color: var(--primary);
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
    }

    .doc-info {
        flex: 1;
        overflow: hidden;
    }

    .doc-title {
        font-size: 1.05rem;
        font-weight: 600;
        margin-bottom: 0.25rem;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .doc-status {
        font-size: 0.8rem;
        color: #34d399; /* emerald green */
        display: inline-flex;
        align-items: center;
        gap: 4px;
    }

    .doc-status::before {
        content: "";
        display: block;
        width: 6px;
        height: 6px;
        border-radius: 50%;
        background-color: #34d399;
    }

    .btn-delete {
        color: var(--text-muted);
        padding: 0.5rem;
        border-radius: 8px;
        transition: all 0.2s;
    }

    .btn-delete:hover {
        color: var(--danger);
        background: rgba(239, 68, 68, 0.1);
    }

    .loading-state,
    .empty-state {
        text-align: center;
        padding: 4rem 2rem;
        background: var(--bg-card);
        border-radius: 16px;
        border: 1px dashed var(--border-color);
    }

    .spinner {
        width: 40px;
        height: 40px;
        border: 3px solid rgba(255, 255, 255, 0.1);
        border-radius: 50%;
        border-top-color: var(--primary);
        animation: spin 1s ease-in-out infinite;
        margin: 0 auto 1.5rem;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    .empty-icon {
        font-size: 3rem;
        margin-bottom: 1rem;
        opacity: 0.5;
    }

    .empty-state h3 {
        font-size: 1.25rem;
        margin-bottom: 0.5rem;
    }

    .empty-state p {
        color: var(--text-muted);
    }

    .alert {
        padding: 1rem;
        border-radius: 8px;
        background-color: rgba(239, 68, 68, 0.1);
        color: #fca5a5;
        border: 1px solid rgba(239, 68, 68, 0.2);
    }
</style>
