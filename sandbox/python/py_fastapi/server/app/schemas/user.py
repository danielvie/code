# app/schemas/user.py
from pydantic import BaseModel, Field


# Schema for creating a new user (request body)
class UserCreate(BaseModel):
    username: str = Field(
        ..., min_length=4, max_length=50, description="Must be between 4 and 50 characters."
    )
    email: str
    # Use Field constraints for range validation
    age: int = Field(gt=18, description="User must be older than 18.")

    # Optional field with a default value
    is_active: bool = True
