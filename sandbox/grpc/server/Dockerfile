# Use Ubuntu 22.04 as it has decent gRPC packages in apt
FROM ubuntu:22.04

# Avoid interactive prompts during build
ENV DEBIAN_FRONTEND=noninteractive

# Install build tools and gRPC dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    git \
    libgrpc++-dev \
    libprotobuf-dev \
    protobuf-compiler-grpc \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy server-specific files (relative to build context root)
COPY server/CMakeLists.txt .
COPY server/server.cpp .

# Copy the shared proto file from the project root
COPY helloworld.proto .

# Build the C++ server
RUN mkdir build && cd build && \
    cmake .. && \
    make -j$(nproc)

# Expose the port
EXPOSE 50051

# Run the server
CMD ["./build/server"]