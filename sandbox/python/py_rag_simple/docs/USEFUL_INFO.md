# Useful Information for RAG System

This document contains additional information about how the Retrieval-Augmented Generation (RAG) system works and how it can be modified.

## Why do we need `nomic-embed-text`?

In a Retrieval-Augmented Generation (RAG) system, you actually rely on **two different types of AI models** working together. Here is why you need `nomic-embed-text` in addition to your main Language Model (like Qwen):

1. **The Language Model (e.g., Qwen)**: This is the conversational brain. It reads the context and generates the human-readable answer. However, it cannot efficiently "search" through folders of huge documents by itself.
2. **The Embedding Model (`nomic-embed-text`)**: This is the search engine backbone of the RAG system. 

Here is exactly what `nomic-embed-text` does step-by-step:
* **During Setup:** When you add `.txt` or `.pdf` files to your `data/` folder, the embedding model reads the text and mathematically converts it into lists of numbers called "vectors" (or embeddings). These vectors capture the *semantic meaning* of sentences and paragraphs. The vectors are then stored in your vector database (Chroma).
* **When You Ask a Question:** The system takes your question and passes it through the exact same `nomic-embed-text` model to turn your question into a vector.
* **The Matchmaker:** The system then mathematically compares your question's vector against all the document vectors in the database to find the closest matches. It pulls those most relevant paragraphs and feeds them to the Language Model to generate your final answer.

Without `nomic-embed-text`, the system would have no fast or smart way to understand which documents in your folder are relevant to the question you just asked.

---

## Using an External API (e.g., OpenAI/ChatGPT)

If you want to use an external API (like OpenAI's ChatGPT) instead of running models locally with Ollama, you would need to change two main components in your `main.py` file: **the Embedding Model** and **the Language Model (LLM)**.

Here is what the transition in `main.py` would look like if you switch to OpenAI:

### 1. Update your Imports
Remove `langchain_ollama` and import `langchain_openai`.

**Before:**
```python
from langchain_ollama import OllamaEmbeddings, ChatOllama
```

**After:**
```python
from langchain_openai import OpenAIEmbeddings, ChatOpenAI
```

### 2. Update Model Variables
Set the model names to OpenAI models. Make sure you also have your `OPENAI_API_KEY` set as an environment variable.

**Before:**
```python
MODEL_NAME = "hf.co/Qwen/Qwen3-8B-GGUF:Q6_K"
EMBEDDING_MODEL = "nomic-embed-text"
```

**After:**
```python
MODEL_NAME = "gpt-4o-mini" # or "gpt-3.5-turbo", "gpt-4o", etc.
EMBEDDING_MODEL = "text-embedding-3-small"
```

### 3. Initialize the New Embeddings
Swap OllamaEmbeddings for OpenAIEmbeddings.
*(Note: If you switch embedding models, you will need to delete your old `chroma_db` folder because embeddings created by `nomic-embed-text` are mathematically incompatible with embeddings created by OpenAI!)*

**Before:**
```python
embeddings = OllamaEmbeddings(model=EMBEDDING_MODEL)
```

**After:**
```python
embeddings = OpenAIEmbeddings(model=EMBEDDING_MODEL)
```

### 4. Initialize the New LLM
Swap the `ChatOllama` instance for a `ChatOpenAI` instance.

**Before:**
```python
llm = ChatOllama(model=MODEL_NAME, temperature=0)
```

**After:**
```python
# It automatically looks for the OPENAI_API_KEY environment variable
llm = ChatOpenAI(model=MODEL_NAME, temperature=0) 
```

### Summary of required steps for OpenAI integration:
1. Run `uv add langchain-openai` (or `pip install langchain-openai`) to get the integration package.
2. Obtain and set an OpenAI API Key (`OPENAI_API_KEY`).
3. **Delete your current `chroma_db` folder** so it can rebuild the database with the new OpenAI embeddings.
4. Run the program. The rest of the Langchain pipeline (`create_stuff_documents_chain`, `create_retrieval_chain`, etc.) works exactly the same!
