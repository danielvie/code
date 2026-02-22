# Walkthrough: Simple Python RAG System

This guide will walk you through how to set up and use this simple local Retrieval-Augmented Generation (RAG) system.

## Prerequisites

Before starting, ensure you have the following installed on your machine:
- **Python 3.12+**
- **[Task](https://taskfile.dev/)** (`task` runner installed)
- **[uv](https://docs.astral.sh/uv/)** (Python package installer and resolver)
- **[Ollama](https://ollama.com/)** running locally

### Pulling the Required Models

This project uses Ollama to run the models locally. You will need to pull both the text generation model and the embedding model.

Open your terminal and run:

```bash
# Pull the embedding model used for vector representations
ollama pull nomic-embed-text

# Pull the LLM used for answering questions
ollama pull hf.co/Qwen/Qwen3-8B-GGUF:Q6_K
```

## Setup

1. **Install Dependencies**
   
   To set up the Python virtual environment and install all necessary dependencies (like Langchain, Chroma, etc.), run the setup task:

   ```bash
   task setup
   ```
   This uses `uv sync` under the hood to ensure everything matches your `pyproject.toml` and `uv.lock`.

## Usage

1. **Adding Documents**
   
   Add your reading materials to the `data/` directory. The system supports the following formats:
   - Text files (`.txt`)
   - Markdown files (`.md`)
   - PDF files (`.pdf`)
   - Word Documents (`.docx`)

2. **Starting the RAG System**
   
   Start the interactive Question-Answering CLI by running:

   ```bash
   task run
   ```
   
   *Alternatively, you can run `uv run python main.py`.*

3. **Interacting with the CLI**
   
   When the app starts, it will automatically check the `data/` directory for any new files that haven't been added to the vector database yet. It will prompt you to ask if you want to index them. Type `y` or `yes` to accept.

   Once the system is ready, you can type your questions at the `Q: ` prompt.

   **Commands within the CLI:**
   - **`/sync`**: Checks the `data/` folder for new files and offers to index them manually.
   - **`/list`**: Lists all documents currently indexed in the Chroma vector database.
   - **`exit`**, **`quit`**, or **`q`**: Exits the program.

## Cleaning Up

If you ever want to reset the vector database or completely clean the virtual environment and lock file, use the clean task:

```bash
task clean
```

*(Note: Cleaning deletes the `chroma_db` folder, the `.venv`, and `uv.lock`.)*
