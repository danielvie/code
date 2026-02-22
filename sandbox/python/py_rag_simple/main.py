import os
import sys
from pathlib import Path

from langchain_community.document_loaders import TextLoader, PyPDFLoader, Docx2txtLoader
from langchain_text_splitters import RecursiveCharacterTextSplitter
from langchain_chroma import Chroma
from langchain_ollama import OllamaEmbeddings, ChatOllama
from langchain.chains import create_retrieval_chain
from langchain.chains.combine_documents import create_stuff_documents_chain
from langchain_core.prompts import ChatPromptTemplate

DATA_DIR = Path("data")
DB_DIR = Path("chroma_db")
MODEL_NAME = "hf.co/Qwen/Qwen3-8B-GGUF:Q6_K"
EMBEDDING_MODEL = "nomic-embed-text"

def get_loader(file_path):
    ext = file_path.suffix.lower()
    if ext in [".txt", ".md"]:
        return TextLoader(str(file_path), encoding="utf-8")
    elif ext == ".pdf":
        return PyPDFLoader(str(file_path))
    elif ext == ".docx":
        return Docx2txtLoader(str(file_path))
    return None

def sync_documents(vector_store):
    indexed_files = set()
    try:
        db_data = vector_store.get()
        if db_data and "metadatas" in db_data and db_data["metadatas"]:
            for meta in db_data["metadatas"]:
                if meta and "source" in meta:
                    indexed_files.add(meta["source"])
    except Exception as e:
        print(f"Warning checking db: {e}")

    available_files = []
    for f in DATA_DIR.glob("**/*"):
        if f.is_file() and f.suffix.lower() in [".txt", ".md", ".pdf", ".docx"]:
            available_files.append(f)

    new_files = [f for f in available_files if str(f.absolute()) not in indexed_files]

    if new_files:
        print(f"\nFound {len(new_files)} new document(s) in '{DATA_DIR}':")
        for f in new_files:
            print(f"  - {f.name}")
            
        ans = input("Do you want to add them to the RAG system? [y/N] ")
        if ans.lower() in ["y", "yes"]:
            documents = []
            for f in new_files:
                loader = get_loader(f)
                if loader:
                    print(f"Loading {f.name}...")
                    try:
                        docs = loader.load()
                        for d in docs:
                            d.metadata["source"] = str(f.absolute())
                        documents.extend(docs)
                    except Exception as e:
                        print(f"Failed to load {f.name}: {e}")
            
            if documents:
                print("Splitting documents...")
                text_splitter = RecursiveCharacterTextSplitter(chunk_size=1000, chunk_overlap=200)
                splits = text_splitter.split_documents(documents)
                
                print(f"Adding {len(splits)} chunks to vector store...")
                vector_store.add_documents(documents=splits)
                print("Documents added successfully.")
            else:
                print("No documents were loaded.")
        else:
            print("Skipped adding new documents.")
    else:
        print("\nNo new documents found. Everything is up to date.")

def list_documents(vector_store):
    try:
        db_data = vector_store.get()
        indexed_files = set()
        if db_data and "metadatas" in db_data and db_data["metadatas"]:
            for meta in db_data["metadatas"]:
                if meta and "source" in meta:
                    indexed_files.add(meta["source"])
        
        if not indexed_files:
            print("\nNo documents currently indexed in the database.")
        else:
            print(f"\nFound {len(indexed_files)} document(s) in the database:")
            for f in sorted(list(indexed_files)):
                print(f"  - {Path(f).name}")
    except Exception as e:
        print(f"Error listing documents: {e}")

def main():
    if not DATA_DIR.exists():
        DATA_DIR.mkdir()

    print("Initializing embeddings...")
    embeddings = OllamaEmbeddings(model=EMBEDDING_MODEL)

    print("Initializing vector store...")
    vector_store = Chroma(
        collection_name="rag_collection",
        embedding_function=embeddings,
        persist_directory=str(DB_DIR)
    )

    sync_documents(vector_store)

    print("\nInitializing model...")
    # Initialize the LLM
    try:
        llm = ChatOllama(model=MODEL_NAME, temperature=0)
    except Exception as e:
        print(f"Failed to initialize LLM: {e}")
        return

    system_prompt = (
        "You are an assistant for question-answering tasks. "
        "Use the following pieces of retrieved context to answer the question. "
        "If you don't know the answer, say that you don't know. "
        "Use three sentences maximum and keep the answer concise.\n\n"
        "{context}"
    )

    prompt = ChatPromptTemplate.from_messages([
        ("system", system_prompt),
        ("human", "{input}"),
    ])

    question_answer_chain = create_stuff_documents_chain(llm, prompt)
    retriever = vector_store.as_retriever()
    rag_chain = create_retrieval_chain(retriever, question_answer_chain)

    print("\n--- RAG System Ready ---")
    print("Type your questions below. Type 'exit' or 'quit' to stop.")

    while True:
        try:
            query = input("\nQ: ")
        except (KeyboardInterrupt, EOFError):
            print()
            break
            
        if query.lower().strip() in ["exit", "quit", "q"]:
            break

        if query.lower().strip() == "/sync":
            sync_documents(vector_store)
            continue
            
        if query.lower().strip() == "/list":
            list_documents(vector_store)
            continue
            
        if not query.strip():
            continue

        print("Thinking...")
        try:
            response = rag_chain.invoke({"input": query})
            print(f"A: {response['answer']}")
        except Exception as e:
            print(f"Error generating answer: {e}")

if __name__ == "__main__":
    main()
