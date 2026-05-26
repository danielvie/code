In this stage, you'll add support for the `Bash` tool.

### The `Bash` Tool

The `Bash` tool enables the LLM to run shell commands. It gives the model direct access to the command line to perform actions like deleting files, creating directories, or running scripts.

You'll need to advertise the `Bash` tool in your request and execute it when the model requests it.

Here is an example of the `Bash` tool's specification:

```js
{
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
```

### Executing the `Bash` Tool

When the model requests a `Bash` tool call:

1. Parse the arguments to extract the `command`
2. Execute the command using your language's shell execution capabilities (e.g., `subprocess.run()` in Python, `child_process.exec()` in Node.js)
3. Capture both stdout and stderr from the command
4. Return the command output (or an error message if it failed) to the model as a tool message

For example, if the command is `rm README_old.md`, execute it and return the result (which will be empty if successful).