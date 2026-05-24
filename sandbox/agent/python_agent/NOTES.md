```python
messages = [{ role: "user", content: prompt }]

loop:
    response = call_api(messages)
    append response message to messages

    if response has no tool_calls:
        print response.content
        exit

    for each tool_call in response.tool_calls:
        result = execute_tool(tool_call)
        append {
            role: "tool",
            tool_call_id: tool_call.id,
            content: result
        } to messages
```
