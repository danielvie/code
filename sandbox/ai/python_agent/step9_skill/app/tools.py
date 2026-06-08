import json
import subprocess
from pathlib import Path
from typing import cast

from openai.types.chat import (
    ChatCompletionFunctionToolParam,
    ChatCompletionMessageToolCall,
)

SKILLS_PATH = Path(".agents/skills")

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

tool_load_skill_spec: ChatCompletionFunctionToolParam = {
    "type": "function",
    "function": {
        "name": "LoadSkill",
        "description": "Load the full instructions for a matched local skill",
        "parameters": {
            "type": "object",
            "required": ["name"],
            "properties": {
                "name": {
                    "type": "string",
                    "description": "The skill name to load",
                }
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
    result = subprocess.run(command, shell=True, capture_output=True, text=True)

    return result.stdout


def tool_load_skill(tool_call_function) -> str:
    args = tool_call_function.function.arguments
    name = json.loads(args)["name"]
    skill_path = SKILLS_PATH / name / "SKILL.md"

    try:
        return skill_path.read_text(encoding="utf-8")
    except OSError as e:
        return f"Error loading skill: {e}"


def tool_get_tools() -> list[ChatCompletionFunctionToolParam]:
    return [
        tool_read_spec,
        tool_write_spec,
        tool_bash_spec,
        tool_load_skill_spec,
    ]


def tool_executer(tool_call) -> str:
    if tool_call.type != "function":
        raise RuntimeError(f"unsupported tool call type: {tool_call.type}")

    function_call = cast(ChatCompletionMessageToolCall, tool_call)
    functoin_name = function_call.function.name.lower()

    result = ""
    match functoin_name:
        case "read":
            result = tool_read(function_call)
        case "write":
            result = tool_write(function_call)
        case "bash":
            result = tool_bash(function_call)
        case "loadskill":
            result = tool_load_skill(function_call)

    return result
