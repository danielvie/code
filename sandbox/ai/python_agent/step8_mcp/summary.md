# app/main.py Summary

## Overview
A CLI chatbot application that interfaces with OpenRouter API to process user prompts and execute tools when needed.

## Actions Performed

### 1. Environment Setup
- Reads `OPENROUTER_API_KEY` from environment variables (required)
- Reads `OPENROUTER_BASE_URL` from environment variables (defaults to `https://openrouter.ai/api/v1`)

### 2. Command Line Interface
- Accepts a required `-p` argument for the user prompt

### 3. Client Initialization
- Creates an OpenAI client configured with the API key and base URL

### 4. Message Loop
- Sends user prompt to the model (`minimax/minimax-m2.7`) via OpenRouter
- Processes responses in a loop:
  - **If tool calls are present**: Executes each tool using `tool_executer()`, appends results as tool messages, and continues the loop
  - **If no tool calls**: Prints the assistant's response content and exits

## Execution Flow
```
User Prompt → OpenRouter API → Assistant Response
                                    ↓
                         [Tool Calls?]
                           /    \
                         Yes      No
                          ↓       ↓
                   Execute Tools  Print & Exit
                          ↓
                   Append Results
                          ↓
                   Continue Loop
```

## Key Components
- **client**: OpenAI client for API communication
- **messages**: List of conversation messages (user, assistant, tool results)
- **tool_get_tools()**: Retrieves available tools
- **tool_executer()**: Executes tool calls returned by the model