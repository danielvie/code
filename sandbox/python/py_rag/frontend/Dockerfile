FROM node:22-alpine

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy source code
COPY . .

# Run development server (can be overridden in compose)
CMD ["npm", "run", "dev", "--", "--host"]
