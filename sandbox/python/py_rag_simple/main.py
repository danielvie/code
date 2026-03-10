from pathlib import Path

from langchain.chains import create_retrieval_chain
from langchain.chains.combine_documents import create_stuff_documents_chain
from langchain_chroma import Chroma
from langchain_community.document_loaders import Docx2txtLoader, PyPDFLoader, TextLoader
from langchain_core.prompts import ChatPromptTemplate
from langchain_ollama import ChatOllama, OllamaEmbeddings
from langchain_text_splitters import RecursiveCharacterTextSplitter

# ==========================================
# 1. CONFIGURATION
# ==========================================
DATA_DIR = Path("data")          # Where your PDF, TXT, etc. files are located
DB_DIR = Path("chroma_db")        # Where the vector store will be saved
MODEL_NAME = "qwen3.5:2b"         # Model used for answering questions
EMBEDDING_MODEL = "bge-m3:latest" # Model used to transform text into numbers (vectors)
# EMBEDDING_MODEL = "nomic-embed-text:v1.5"

# ==========================================
# 2. DOCUMENT PROCESSING (Ingestion)
# ==========================================

def get_loader(file_path):
    """Selects the correct loader based on the file extension."""
    ext = file_path.suffix.lower()
    if ext in [".txt", ".md"]:
        return TextLoader(str(file_path), encoding="utf-8")
    elif ext == ".pdf":
        return PyPDFLoader(str(file_path))
    elif ext == ".docx":
        return Docx2txtLoader(str(file_path))
    return None

def ingest_documents(vector_store):
    """Educational process: Load -> Split -> Add to Vector Store."""
    
    # Identify local files
    available_files = [f for f in DATA_DIR.glob("**/*") 
                       if f.is_file() and f.suffix.lower() in [".txt", ".md", ".pdf", ".docx"]]
    
    if not available_files:
        print(f"Warning: No compatible files found in {DATA_DIR}")
        return

    # Ask before indexing (for user control)
    print(f"\nFiles found: {[f.name for f in available_files]}")
    ans = input("Do you want to (re)index these documents? [y/N] ")
    if ans.lower() not in ["y", "yes"]:
        return

    documents = []
    for f in available_files:
        print(f"-> Loading: {f.name}")
        loader = get_loader(f)
        if loader:
            try:
                # Load file content
                docs = loader.load()
                # Add metadata to know the source later
                for d in docs:
                    d.metadata["source"] = f.name
                documents.extend(docs)
            except Exception as e:
                print(f"Error loading {f.name}: {e}")

    if documents:
        # CHUNKING: Break large texts into smaller pieces
        # This helps the model focus only on relevant parts.
        print("-> Splitting documents into chunks...")
        text_splitter = RecursiveCharacterTextSplitter(
            chunk_size=800, 
            chunk_overlap=100
        )
        splits = text_splitter.split_documents(documents)

        # STORAGE: Send to ChromaDB
        # Embeddings are automatically generated here by Chroma using Ollama
        print(f"-> Adding {len(splits)} chunks to the vector store...")
        vector_store.add_documents(documents=splits)
        print("Success: Documents indexed.")

# ==========================================
# 3. RAG CORE (Retrieval & Generation Chain)
# ==========================================

def setup_rag_chain(vector_store):
    """Sets up the 'chain' that unites Retrieval and Generation."""
    
    # Initialize the Language Model (LLM)
    # On macOS, we use streaming to prevent the app from appearing frozen.
    llm = ChatOllama(
        model=MODEL_NAME, 
        temperature=0, 
        streaming=True, # Response appears bit by bit
        reasoning=False # Disable internal "thinking" mode if supported
    )

    # Define the assistant's BEHAVIOR (Prompt)
    system_prompt = (
        "You are a helpful assistant. Use the CONTEXT below to answer the question.\n"
        "If you don't know the answer, say you couldn't find the information.\n"
        "Be direct and answer in at most 3 sentences.\n\n"
        "CONTEXT:\n{context}"
    )

    prompt = ChatPromptTemplate.from_messages([
        ("system", system_prompt),
        ("human", "{input}"),
    ])

    # Create the document processing chain
    combine_docs_chain = create_stuff_documents_chain(llm, prompt)
    
    # Turn the vector store into a Retriever
    retriever = vector_store.as_retriever(search_kwargs={"k": 3}) # Retrieve the 3 most similar chunks

    # Join everything into a Retrieval Chain
    return create_retrieval_chain(retriever, combine_docs_chain)

# ==========================================
# 4. INTERFACE AND EXECUTION (Main Loop)
# ==========================================

def main():
    # Ensure directories exist
    DATA_DIR.mkdir(exist_ok=True)

    print(f"--- Simple RAG System ---")
    print(f"Model: {MODEL_NAME} | Embeddings: {EMBEDDING_MODEL}")

    # Initialize LangChain components
    embeddings = OllamaEmbeddings(model=EMBEDDING_MODEL)
    
    vector_store = Chroma(
        collection_name="rag_learning_collection",
        embedding_function=embeddings,
        persist_directory=str(DB_DIR),
    )

    # Initial ingestion
    ingest_documents(vector_store)

    # Prepare the response chain
    rag_chain = setup_rag_chain(vector_store)

    print("\nReady! Type your question or 'exit' to quit.")
    
    while True:
        try:
            query = input("\nYOU: ").strip()
        except EOFError:
            break
            
        if query.lower() in ["exit", "quit", "q", "sair"]:
            break
        if not query:
            continue

        print("ASSISTANT: ", end="", flush=True)
        
        try:
            # The 'for' loop allows printing each piece of the response as it arrives.
            full_response = ""
            for chunk in rag_chain.stream({"input": query}):
                # The streaming response comes in chunks
                if "answer" in chunk:
                    content = chunk["answer"]
                    print(content, end="", flush=True)
                    full_response += content
            print() # New line at the end
            
        except Exception as e:
            print(f"\nError generating answer: {e}")

if __name__ == "__main__":
    main()
