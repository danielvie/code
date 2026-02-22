Create a new project to learn RAG.

## Project Description

This project is a simple RAG (Retrieval-Augmented Generation) system that uses a local LLM to answer questions about a given document.

## Goals

- Create a Simple RAG system in python to learn the concepts. 

## Tech Stack

- Python 3.12+
- uv for managing the project
- Langchain
- Ollama (model: `hf.co/Qwen/Qwen3-8B-GGUF:Q6_K`)

## Instructions

- Use Taskfile to manage the project.

## Specs

- The documents are stored in the `data/` directory.
    - They can be text, pdf or docx format.
- The interface is a simple terminal interface.
- I can add more files in the `data/` directory and the system should ask if i want to add them to the RAG system.
- The system should be able to answer questions about the documents.