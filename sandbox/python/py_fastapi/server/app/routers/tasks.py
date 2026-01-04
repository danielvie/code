# app/routers/tasks.py
from fastapi import APIRouter, HTTPException, UploadFile, File
import os
import aiofiles
from pydantic import BaseModel, Field

# .. CONSTANTS
UPLOAD_FOLDER = "uploaded_files"
os.makedirs(UPLOAD_FOLDER, exist_ok=True)


# .. SCHEMAS
# Schema for Task 1: Simple payload
class Task1Payload(BaseModel):
    data_id: int = Field(..., description="Unique ID for the data to be processed.")
    config: str


# Schema for Task 2: More complex, nested data structure
class Task2Payload(BaseModel):
    user_token: str
    target_value: float
    parameters: dict


# Schema for Task 3: Another simple payload
class Task3Payload(BaseModel):
    file_path: str
    timeout_seconds: int = 60


# --- APIRouter Initialization ---

router = APIRouter(
    prefix="/api",  # All routes will be /api/...
    tags=["Tasks"],  # Grouping in documentation
)

# --- POST Endpoints (Task 1, 2, 3) ---


@router.post("/task1")
async def execute_task1(payload: Task1Payload):
    """
    Initiates processing for a specific data ID with a given configuration.
    """
    print(f"Executing Task 1 for ID: {payload.data_id} with config: {payload.config}")

    # Simulate work
    # result = await perform_complex_operation(payload.data_id)

    return {"status": "Task 1 queued", "data_id": payload.data_id}


@router.post("/task2")
async def execute_task2(payload: Task2Payload):
    """
    Handles a complex request involving a user token and parameters.
    """
    if not payload.user_token.startswith("USR-"):
        raise HTTPException(status_code=400, detail="Invalid user token format.")

    # Simulate work
    # result = await handle_token_based_request(payload.user_token, payload.parameters)

    return {"status": "Task 2 accepted", "target": payload.target_value}


# --- GET Endpoint (Task 3) ---
@router.get("/task3")
async def get_task_info(job_id: str | None = None, status_filter: str = "active"):
    """
    Retrieves status or information about one or more tasks.

    If job_id is provided, returns details for that job.
    Otherwise, returns a list of jobs filtered by status.
    """
    if job_id:
        # Simulate fetching single job details from a database
        return {"job_id": job_id, "status": "completed", "result_url": f"/results/{job_id}"}

    # Simulate fetching a list of jobs
    return {
        "status_filter": status_filter,
        "jobs": [
            {"id": "j-101", "status": "active"},
            {"id": "j-102", "status": "completed"},
        ],
    }


@router.post("/task4")
async def task4_upload_file(file: UploadFile = File(...)):
    """
    Receives a file, saves it to the local UPLOAD_FOLDER, and returns metadata
    """

    try:
        file_path = os.path.join(UPLOAD_FOLDER, file.filename)

        async with aiofiles.open(file_path, "wb") as out_file:
            # read up to 1024 bytes
            while content := await file.read(1024):
                await out_file.write(content)

        return {
            "filename": file.filename,
            "content_type": file.content_type,
            "message": f"File '{file.filename}' successfully copied to '{UPLOAD_FOLDER}'.",
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"there was an error uploading the file: {e}")
    finally:
        await file.close()


@router.get("/task/{item_id}")
async def task_n(item_id: int):
    """
    Receives a file, saves it to the local UPLOAD_FOLDER, and returns metadata
    """

    return {"item_id": item_id, "message": f"mijn id nummer is: {item_id}"}
