# app/main.py
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from app.routers import tasks

# define origins for Frontend
origins = [
    "http://localhost",
    "http://localhost:5000",
    "http://localhost:5173",
]

app = FastAPI(
    title="ProcessLens Server",
    description="API Server",
)

# add CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include the router to attach all /api routes to the main app
app.include_router(tasks.router)


@app.get("/", tags=["Root"])
async def root():
    return {"message": "Welcome to the API. Check /docs for endpoints."}
