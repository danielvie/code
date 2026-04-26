---
name: git-commit
description: Use when writing Git commit messages. Enforces imperative mood, 50/72 character limits, and zero meta-commentary.
---

# Git Commit

## Overview
Generate high-quality, professional Git commit messages that summarize changes accurately and concisely.

## When to Use
- **Trigger**: When asked to "write a commit message", "commit these changes", or "summarize the diff".
- **Symptom**: Verbose commit messages, non-imperative subjects, or repetition of subject in body.

## Core Pattern
1. **Analyze Diff**: Identify the *primary* purpose of the changes.
2. **Subject Line**: 
   - Start with an imperative verb (Add, Fix, Update, Refactor).
   - Limit to 50 characters.
   - Capitalize first letter.
   - No trailing punctuation.
3. **Body (Optional)**:
   - Use only if it adds context not obvious from the subject.
   - Separate from subject with a blank line.
   - Wrap at 72 characters.
4. **Final Check (Strict Output)**: 
   - **MANDATORY**: Only return the commit message. 
   - **STOP**: Delete all additional meta-commentary, explanations, or quotes.
   - **No Preamble**: No "Here is the message..." or "Commit message:".

## Quick Reference
- **GOOD**: `Add Gmail authentication flow`
- **BAD**: `I added the Gmail authentication flow.`
- **GOOD**: `Fix typo in README`
- **BAD**: `Fixed typo in README.`

## Common Mistakes
- **Meta-commentary**: Including text like "Here is the commit message:" or "I hope this helps."
- **Narrative bodies**: "In this commit, I decided to..." (Delete).
- **Redundancy**: Repeating the subject in the first line of the body.
- **Punctuation**: Ending the subject line with a period.
- **Past Tense**: Using "Fixed" or "Added" instead of "Fix" or "Add".

## Red Flags
| Violation | Skill Fix |
| :--- | :--- |
| Including meta-commentary | **STOP**. Delete everything except the raw commit message. |
| Subject > 50 chars | Rewrite to capture the *impact*, not the list of files. |
| Body repeats subject | Delete the redundant lines in the body. |
| Past tense verb | Change to imperative (e.g., Fixed -> Fix). |
| Ending with `.` | Remove the trailing punctuation from the subject line. |
