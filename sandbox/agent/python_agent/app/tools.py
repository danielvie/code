from collections.abc import Callable
from typing import cast
import json
from openai.types.chat import ChatCompletionFunctionToolParam, ChatCompletionMessageToolCall
import subprocess

tool_read_spec: ChatCompletionFunctionToolParam = {
  "type": "function",
  "function": {
    "name": "Read",
    "description": "Read and return the contents of a file",
    "parameters": {
      "type": "object",
      "properties": {
        "file_path": {
          "type": "string",
          "description": "The path to the file to read"
        }
      },
      "required": ["file_path"]
    }
  }
}

tool_write_spec: ChatCompletionFunctionToolParam = {
  "type": "function",
  "function": {
    "name": "Write",
    "description": "Write content to a file",
    "parameters": {
      "type": "object",
      "required": ["file_path", "content"],
      "properties": {
        "file_path": {
          "type": "string",
          "description": "The path of the file to write to"
        },
        "content": {
          "type": "string",
          "description": "The content to write to the file"
        }
      }
    }
  }
}

tool_bash_spec: ChatCompletionFunctionToolParam = {
  "type": "function",
  "function": {
    "name": "Bash",
    "description": "Execute a shell command",
    "parameters": {
      "type": "object",
      "required": ["command"],
      "properties": {
        "command": {
          "type": "string",
          "description": "The command to execute"
        }
      }
    }
  }
}

def tool_read(tool_call_function) -> str:
    args     = tool_call_function.function.arguments
    filepath = json.loads(args)["file_path"]
    
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()
        
    return content
        
def tool_write(tool_call_function) -> str:
    args = tool_call_function.function.arguments
    filepath = json.loads(args)["file_path"]
    content = json.loads(args)["content"]
    
    with open(filepath, "w", encoding="utf-8") as f:
        f.write(content)
        
    return f"Created {filepath} correctly!"
        
def tool_bash(tool_call_function) -> str:
    args = tool_call_function.function.arguments
    command = json.loads(args)["command"]
    
    result = subprocess.run(
        command,
        shell=True,
        capture_output=True,
        text=True
    )
    
    return result.stdout

ToolHandler = Callable[[ChatCompletionMessageToolCall], str]

TOOL_DISPATCHER: dict[str, ToolHandler] = {
    "read": tool_read,
    "write": tool_write,
    "bash": tool_bash,
}

def tool_executer(tool_call) -> str:
    tool_call_function = cast(ChatCompletionMessageToolCall, tool_call)
    tool_function_name = tool_call_function.function.name.lower()

    handler = TOOL_DISPATCHER.get(tool_function_name)
    if handler is None:
        raise RuntimeError(f"unsupported tool: {tool_function_name}")

    return handler(tool_call_function)
