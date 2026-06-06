In this stage, you'll add support for the `Write` tool.

### The `Write` Tool

The `Write` tool enables the LLM to write content to files. Like with the `Read` tool, you need to advertise the `Write` tool in your request and execute it when the model requests it.

Here's the tool specification:

```js
{
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
```

### Executing the `Write` Tool

When the model requests a `Write` tool call:

1. Parse the arguments to extract the `file_path` and `content`

2. Write the content to the file at the specified path:

   * If the file doesn't exist, create it
   * If the file exists, overwrite it with the new content

3. Append the result to messages as a tool message (just like with `Read`)