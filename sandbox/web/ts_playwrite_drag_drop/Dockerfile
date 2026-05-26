# Use the Microsoft Playwright image based on Ubuntu Jammy which includes Node.js and Chromium
FROM mcr.microsoft.com/playwright:v1.43.0-jammy

# Install curl, python3, pip, and other necessary tools
RUN apt-get update && apt-get install -y python3 python3-pip curl && rm -rf /var/lib/apt/lists/*

# Install uv for Python dependency management
RUN curl -LsSf https://astral.sh/uv/install.sh | sh
ENV PATH="/root/.local/bin:$PATH"

WORKDIR /app

# Copy the Node.js package information and install
COPY package.json ./
RUN npm install

# Install Playwright browsers (chromium only since we only use it)
RUN npx playwright install chromium

# Copy Python requirements and install using uv
COPY pyproject.toml ./
RUN uv sync

# Copy all the rest of the application files
COPY . .

# Environment setup
ENV CI=1

# Command is typically overridden by the Taskfile, but setting a default
CMD ["npx", "playwright", "test"]
