# Use a standard Python image
FROM python:3.11-slim

# Set the working directory
WORKDIR /app

# Install build tools and dependencies.
# The 'build-essential' package includes the C++ compiler ('g++').
RUN apt-get update && \
    apt-get install -y --no-install-recommends build-essential && \
    rm -rf /var/lib/apt/lists/*

# Copy the requirements file into the container
COPY requirements.txt .

# Install dependencies from requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

# Copy the rest of the application code
COPY . .

# Generate gRPC code from .proto files
RUN python -m grpc_tools.protoc -I. --python_out=. --grpc_python_out=. proto/service.proto

# The command to run the gRPC server when the container starts
CMD ["python", "-u", "server.py"]
# CMD python server.py