# Use the same base image as GitLab CI
FROM python:3.11-slim

# Set working directory
WORKDIR /app

# Copy project files
COPY pyproject.toml uv.lock app.py ./
COPY templates/ ./templates/

# Install uv
RUN pip install uv

# Sync dependencies using uv
RUN uv sync

# Expose port 5000 for the static file server
EXPOSE 5000

# Serve static files using Python's http.server
CMD ["uv", "run", "python", "app.py"]