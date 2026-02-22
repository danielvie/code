Create a new project to learn RAG.

## Project Description

This project is a simple RAG (Retrieval-Augmented Generation) system that uses a local LLM to answer questions about a given document.

## Project Structure

```
py_rag/
├── docs/
│   ├── INTRO.md
│   └── ...
├── src/
│   └── ...
├── tests/
│   └── ...
├── pyproject.toml
├── Taskfile.yml
└── .gitignore
```

## Goals

- [ ] Create a backend API for the RAG system. 
- [ ] Create a frontend for the RAG system. 

## Tech Stack

- Python 3.12+
- FastAPI
- Svelte
- Docker
- Docker Compose

## Instructions

- Use Taskfile to manage the project.
- Use Docker Compose to manage the project.
- Use Docker to manage the project.

## Specs

- The backend API should have the following endpoints:
    - `POST /upload` - Upload a document.
    - `POST /ask` - Ask a question.
    - `GET /docs` - List all documents.
    - `GET /docs/{id}` - Get a document.
    - `DELETE /docs/{id}` - Delete a document.
- The frontend should have the following features:
    - Upload a document (hero page).
    - Ask a question (chat interface).
    - List all documents.
    - Get a document.
    - Delete a document.
- The documents are stored in the `data/` directory.
    - They can be text, pdf or docx format.
