FROM python:3.13-slim

WORKDIR /app

# Install uv
RUN pip install uv

# Copy the toml files
COPY pyproject.toml /app/

# Install dependencies via uv
RUN uv sync

# Copy source code
COPY main.py /app/main.py

CMD ["uv", "run", "uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
