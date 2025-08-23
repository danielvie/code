# Stage 1: Builder
# Use the official Bun image, which includes Bun, Node.js, and all necessary tools.
FROM oven/bun:1.1.18 as builder

# Set the working directory
WORKDIR /app

# Copy package.json and tsconfig.json to leverage Docker's layer caching.
COPY package.json ./


# Copy the rest of the application files
COPY . .

# Install dependencies using Bun
RUN bun install

# Generate gRPC code from .proto files.
# `bun run gen` will use the `grpc-tools` devDependency to create the stubs.
RUN mkdir gen
RUN bun run gen

# Stage 2: Final
# Use a minimal Bun image for the final production container.
# FROM oven/bun:1.1.18-slim

# # Set the working directory
# WORKDIR /app

# # Copy all the files from the builder stage.
# # This includes the node_modules, generated code, and the source files.
# COPY --from=builder /app /app

# # Run the TypeScript client.
# # This command will connect to a server at `host.docker.internal`
# # `host.docker.internal` is a special DNS name that resolves to the host machine's IP address from within a Docker container.
# # This is required for the client to communicate with a server running on the host machine (e.g., the Python server).
CMD ["bun", "run", "client.ts"]