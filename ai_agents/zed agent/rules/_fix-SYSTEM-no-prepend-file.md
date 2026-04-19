# Rule: No File Path in Content

## Description
When generating or writing file content via `edit_file`, do NOT include the file path in the first line of the content. The file path belongs only in the tool call's `path` parameter, never in the actual file content itself.

## Problem
The model sometimes contaminates file content by prepending the file path as the first line, like:
```
 ```md C:/SANDBOX/NOTES/OBSIDIAN/main/main/STUDIES/AI/articles/example.md
```

This results in corrupted files where the first line is a code fence with a path instead of the actual content.

## Examples

### ❌ INCORRECT - File path in content
The file content starts with:
 ```md C:/SANDBOX/NOTES/OBSIDIAN/main/main/STUDIES/AI/articles/Measuring the Impact of Early-2025 AI.md
# Measuring the Impact of Early-2025 AI...

### ✅ CORRECT - Clean content
The file content starts with:
# Measuring the Impact of Early-2025 AI...

## Applies To
All file types: `.md`, `.cpp`, `.py`, `.js`, `.ts`, `.rs`, `.json`, `.yaml`, etc.

## Instructions
1. When using `edit_file`, write ONLY the actual content that should be in the file
2. Do NOT prepend the file path to the content
3. Do NOT include line number references (e.g., `#L1-10`) in the content
4. The `path` parameter in the tool call is where the file path goes, not in the content