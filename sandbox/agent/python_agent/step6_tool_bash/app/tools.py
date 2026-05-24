import json
import platform
import subprocess

from openai.types.chat import ChatCompletionFunctionToolParam

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
                    "description": "The path to the file to read",
                }
            },
            "required": ["file_path"],
        },
    },
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
                    "description": "The path of the file to write to",
                },
                "content": {
                    "type": "string",
                    "description": "The content to write to the file",
                },
            },
        },
    },
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
                "command": {"type": "string", "description": "The command to execute"}
            },
        },
    },
}


def tool_read(tool_call_function) -> str:
    args = tool_call_function.function.arguments
    filepath = json.loads(args)["file_path"]

    try:
        print(f"reading `{filepath}`")
        with open(filepath, "r", encoding="utf-8") as f:
            content = f.read()
            print("content", content)
    except OSError as e:
        content = f"Error reading file: {e}"

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
    
    print(f"command: `{command}`")
    result = subprocess.run(
        command,
        shell=True,
        capture_output=True,
        text=True
    )
    
    return result.stdout
