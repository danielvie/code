<script lang="ts">
    import { onMount } from "svelte";

    let messages: { role: "user" | "assistant"; content: string }[] = [];
    let question = "";
    let asking = false;

    async function askQuestion() {
        if (!question.trim()) return;

        const userQuestion = question.trim();
        messages = [...messages, { role: "user", content: userQuestion }];
        question = "";
        asking = true;

        try {
            const apiUrl =
                import.meta.env.VITE_API_URL || "http://localhost:8000";
            const response = await fetch(
                `${apiUrl}/ask?question=${encodeURIComponent(userQuestion)}`,
                {
                    method: "POST",
                },
            );

            if (!response.ok) {
                throw new Error("Failed to get an answer");
            }

            const data = await response.json();
            messages = [
                ...messages,
                { role: "assistant", content: data.answer },
            ];
        } catch (error) {
            messages = [
                ...messages,
                {
                    role: "assistant",
                    content:
                        "Sorry, an error occurred while fetching the answer.",
                },
            ];
        } finally {
            asking = false;
        }
    }

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === "Enter" && !e.shiftKey) {
            e.preventDefault();
            askQuestion();
        }
    }
</script>

<svelte:head>
    <title>Chat - BrainRAG</title>
</svelte:head>

<div class="container chat-layout">
    <div class="chat-container">
        <div class="chat-header">
            <h2>Ask BrainRAG</h2>
            <p>Ask questions about your uploaded documents</p>
        </div>

        <div class="messages-area">
            {#if messages.length === 0}
                <div class="empty-state">
                    <div class="icon-wrapper">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="48"
                            height="48"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="1.5"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            ><path
                                d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"
                            ></path></svg
                        >
                    </div>
                    <h3>Start a conversation</h3>
                    <p>
                        Type your question below to explore the knowledge base.
                    </p>
                </div>
            {/if}

            {#each messages as msg}
                <div class="message {msg.role}">
                    <div class="message-content">
                        {msg.content}
                    </div>
                </div>
            {/each}

            {#if asking}
                <div class="message assistant typing">
                    <div class="message-content">
                        <span class="dot"></span>
                        <span class="dot"></span>
                        <span class="dot"></span>
                    </div>
                </div>
            {/if}
        </div>

        <div class="input-area">
            <textarea
                bind:value={question}
                on:keydown={handleKeydown}
                placeholder="Message BrainRAG..."
                rows="1"
            ></textarea>
            <button
                class="send-btn"
                on:click={askQuestion}
                disabled={asking || !question.trim()}
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="20"
                    height="20"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    ><line x1="22" y1="2" x2="11" y2="13"></line><polygon
                        points="22 2 15 22 11 13 2 9 22 2"
                    ></polygon></svg
                >
            </button>
        </div>
    </div>
</div>

<style>
    .chat-layout {
        display: flex;
        justify-content: center;
        height: calc(100vh - 8rem);
    }

    .chat-container {
        display: flex;
        flex-direction: column;
        width: 100%;
        max-width: 800px;
        background: var(--bg-card);
        border-radius: 20px;
        border: 1px solid var(--border-color);
        box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.5);
        overflow: hidden;
    }

    .chat-header {
        padding: 1.5rem;
        border-bottom: 1px solid var(--border-color);
        text-align: center;
        background: rgba(15, 23, 42, 0.4);
    }

    .chat-header h2 {
        font-size: 1.25rem;
        font-weight: 700;
        margin-bottom: 0.25rem;
    }

    .chat-header p {
        font-size: 0.9rem;
        color: var(--text-muted);
    }

    .messages-area {
        flex: 1;
        padding: 1.5rem;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .empty-state {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100%;
        text-align: center;
        color: var(--text-muted);
    }

    .icon-wrapper {
        background: rgba(99, 102, 241, 0.1);
        color: var(--primary);
        padding: 1.5rem;
        border-radius: 50%;
        margin-bottom: 1.5rem;
    }

    .empty-state h3 {
        font-size: 1.25rem;
        color: var(--text-main);
        margin-bottom: 0.5rem;
    }

    .message {
        display: flex;
        animation: fadeIn 0.3s ease-out;
    }

    .message.user {
        justify-content: flex-end;
    }

    .message.assistant {
        justify-content: flex-start;
    }

    .message-content {
        max-width: 80%;
        padding: 1rem 1.25rem;
        border-radius: 18px;
        font-size: 0.95rem;
        line-height: 1.5;
    }

    .message.user .message-content {
        background: linear-gradient(
            to right,
            var(--primary),
            var(--primary-hover)
        );
        color: white;
        border-bottom-right-radius: 4px;
    }

    .message.assistant .message-content {
        background: rgba(51, 65, 85, 0.6);
        border: 1px solid var(--border-color);
        border-bottom-left-radius: 4px;
    }

    .input-area {
        padding: 1.5rem;
        background: rgba(15, 23, 42, 0.4);
        border-top: 1px solid var(--border-color);
        display: flex;
        gap: 1rem;
        align-items: flex-end;
    }

    textarea {
        flex: 1;
        background: var(--bg-dark);
        border: 1px solid var(--border-color);
        border-radius: 12px;
        padding: 1rem 1.25rem;
        color: var(--text-main);
        font-family: inherit;
        font-size: 0.95rem;
        resize: none;
        max-height: 150px;
        transition: border-color 0.2s;
    }

    textarea:focus {
        outline: none;
        border-color: var(--primary);
    }

    .send-btn {
        background: var(--primary);
        color: white;
        width: 3.2rem;
        height: 3.2rem;
        border-radius: 12px;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 0.2s;
    }

    .send-btn:hover:not(:disabled) {
        background: var(--primary-hover);
        transform: translateY(-2px);
    }

    .send-btn:disabled {
        opacity: 0.5;
        background: var(--border-color);
        cursor: not-allowed;
    }

    /* Typing dots animation */
    .typing .message-content {
        display: flex;
        align-items: center;
        gap: 4px;
        padding: 1.25rem;
    }

    .dot {
        width: 6px;
        height: 6px;
        background-color: var(--text-muted);
        border-radius: 50%;
        animation: bounce 1.4s infinite ease-in-out both;
    }

    .dot:nth-child(1) {
        animation-delay: -0.32s;
    }
    .dot:nth-child(2) {
        animation-delay: -0.16s;
    }

    @keyframes bounce {
        0%,
        80%,
        100% {
            transform: scale(0);
        }
        40% {
            transform: scale(1);
        }
    }
</style>
