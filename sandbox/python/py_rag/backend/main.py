import os
from fastapi import FastAPI, UploadFile, File, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from typing import List

app = FastAPI(title="RAG API")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"], # Allow all origins for dev
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

DATA_DIR = os.path.join(os.path.dirname(os.path.dirname(__file__)), "data")
os.makedirs(DATA_DIR, exist_ok=True)

@app.get("/")
def read_root():
    return {"message": "Welcome to the RAG API"}

@app.post("/upload")
async def upload_document(file: UploadFile = File(...)):
    if not file.filename:
        raise HTTPException(status_code=400, detail="No file uploaded")
    
    file_path = os.path.join(DATA_DIR, file.filename)
    try:
        with open(file_path, "wb") as f:
            content = await file.read()
            f.write(content)
        return {"filename": file.filename, "message": "Document uploaded successfully"}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/ask")
async def ask_question(question: str):
    # Stub implementation for asking a question
    return {"question": question, "answer": "This is a placeholder answer."}

@app.get("/documents")
async def list_documents() -> List[str]:
    files = os.listdir(DATA_DIR)
    return files

@app.get("/documents/{id}")
async def get_document(id: str):
    file_path = os.path.join(DATA_DIR, id)
    if not os.path.exists(file_path):
        raise HTTPException(status_code=404, detail="Document not found")
    # In a real app we might return the file content or metadata
    return {"filename": id, "path": file_path}

@app.delete("/documents/{id}")
async def delete_document(id: str):
    file_path = os.path.join(DATA_DIR, id)
    if not os.path.exists(file_path):
        raise HTTPException(status_code=404, detail="Document not found")
    try:
        os.remove(file_path)
        return {"message": f"Document {id} deleted successfully"}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
