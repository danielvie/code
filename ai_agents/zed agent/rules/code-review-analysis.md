---
name: code-review-analysis
description: >
  Perform comprehensive code reviews with best practices, security checks, performance analysis, 
  and constructive feedback. Use when reviewing pull requests, analyzing code quality, checking 
  for security vulnerabilities, validating test coverage, or providing code improvement suggestions.
---

# Code Review Analysis

## Overview

Systematic code review process covering code quality, security, performance, maintainability, 
and testing. This skill provides a structured approach to reviewing code changes with 
constructive, actionable feedback following industry standards.

## When to Use

- Reviewing pull requests and merge requests before approval
- Analyzing code quality issues in existing codebase
- Checking for security vulnerabilities in new code
- Validating test coverage for new features
- Providing constructive feedback to developers
- Ensuring coding standards compliance
- Mentoring junior developers through code review
- Performance reviewing database queries or algorithms

## Core Pattern

The code review follows a systematic 5-phase approach:

1. **Initial Assessment** - Understand the scope and context of changes
2. **Code Quality Analysis** - Evaluate readability, maintainability, and complexity
3. **Security Review** - Identify vulnerabilities and security best practices
4. **Performance Review** - Check for inefficient patterns and optimization opportunities
5. **Testing Review** - Validate test coverage and quality

## Quick Reference

```bash
# Check the changes
git diff main...feature-branch

# Review file changes summary
git diff --stat main...feature-branch

# Check commit history
git log main...feature-branch --oneline
```

**Quick Checklist:**
- [ ] PR description is clear and complete
- [ ] Changes match the stated purpose
- [ ] No unrelated changes included
- [ ] Tests are included
- [ ] Documentation is updated

## Implementation

### Phase 1: Initial Assessment

Before diving into code details, establish context:

```bash
# Check the changes
git diff main...feature-branch

# Review file changes
git diff --stat main...feature-branch

# Check commit history
git log main...feature-branch --oneline
```

**Checklist:**
- [ ] PR description is clear and complete
- [ ] Changes match the stated purpose
- [ ] No unrelated changes included
- [ ] Tests are included
- [ ] Documentation is updated

---

### Phase 2: Code Quality Analysis

#### Readability

```python
# ❌ Poor readability - cryptic names, no types
def p(u,o):
    return u['t']*o['q'] if u['s']=='a' else 0

# ✅ Good readability - descriptive names, type hints
def calculate_order_total(user: User, order: Order) -> float:
    """Calculate order total with user-specific pricing."""
    if user.status == 'active':
        return user.tier_price * order.quantity
    return 0
```

#### Complexity Reduction

```javascript
// ❌ High cognitive complexity - deeply nested
function processData(data) {
  if (data) {
    if (data.type === "user") {
      if (data.status === "active") {
        if (data.permissions && data.permissions.length > 0) {
          // deeply nested logic
        }
      }
    }
  }
}

// ✅ Reduced complexity with early returns
function processData(data) {
  if (!data) return null;
  if (data.type !== "user") return null;
  if (data.status !== "active") return null;
  if (!data.permissions?.length) return null;

  // main logic at top level
}
```

**Code Quality Checklist:**
- [ ] Variable/function names are descriptive
- [ ] Functions are small and focused
- [ ] No deep nesting (max 2-3 levels)
- [ ] Comments explain "why" not "what"
- [ ] Consistent coding style
- [ ] No code duplication (DRY principle)

---

### Phase 3: Security Review

#### SQL Injection Prevention

```python
# ❌ Vulnerable to SQL injection
query = f"SELECT * FROM users WHERE email = '{user_email}'"

# ✅ Parameterized query
cursor.execute("SELECT * FROM users WHERE email = ?", (user_email,))
```

#### XSS Prevention

```javascript
// ❌ XSS vulnerable - renders raw HTML
element.innerHTML = userInput;

// ✅ Safe rendering
element.textContent = userInput;
// or use framework escaping: {{ userInput }} in templates
```

#### Authentication & Authorization

```typescript
// ❌ Missing authorization check
app.delete("/api/users/:id", async (req, res) => {
  await deleteUser(req.params.id);
  res.json({ success: true });
});

// ✅ Proper authorization
deleteUserHandler = [
  requireAuth,
  async (req, res) => {
    if (req.user.id !== req.params.id && !req.user.isAdmin) {
      return res.status(403).json({ error: "Forbidden" });
    }
    await deleteUser(req.params.id);
    res.json({ success: true });
  }
];
```

**Security Checklist:**
- [ ] No SQL injection vulnerabilities
- [ ] Input validation on all endpoints
- [ ] Output encoding for user data
- [ ] Proper authentication checks
- [ ] Authorization checks for sensitive operations
- [ ] No secrets in code
- [ ] Secure dependency versions

---

### Phase 4: Performance Review

#### Database Query Optimization

```javascript
// ❌ N+1 query problem
const users = await User.findAll();
for (const user of users) {
  user.orders = await Order.findAll({ where: { userId: user.id } });
}

// ✅ Eager loading
const users = await User.findAll({
  include: [{ model: Order }],
});
```

#### Algorithm Efficiency

```python
# ❌ Inefficient list operations
result = []
for item in large_list:
    if item % 2 == 0:
        result.append(item * 2)

# ✅ List comprehension
result = [item * 2 for item in large_list if item % 2 == 0]
```

**Performance Checklist:**
- [ ] No N+1 query problems
- [ ] Proper use of indexes
- [ ] No unnecessary computations in loops
- [ ] Efficient data structures used
- [ ] Caching opportunities identified
- [ ] No memory leaks

---

### Phase 5: Testing Review

#### Test Coverage Example

```javascript
describe("User Service", () => {
  // ✅ Tests edge cases
  it("should handle empty input", () => {
    expect(processUser(null)).toBeNull();
  });

  it("should handle invalid data", () => {
    expect(() => processUser({})).toThrow(ValidationError);
  });

  // ✅ Tests happy path
  it("should process valid user", () => {
    const result = processUser(validUserData);
    expect(result.id).toBeDefined();
  });
});
```

**Testing Checklist:**
- [ ] Unit tests for new functions
- [ ] Integration tests for new features
- [ ] Edge cases covered (null, empty, invalid)
- [ ] Error cases tested
- [ ] Mock/stub usage is appropriate
- [ ] Tests are deterministic (no flakiness)

---

### Best Practices

#### Error Handling

```typescript
// ❌ Silent failures - never do this
try {
  await saveData(data);
} catch (e) {
  // empty catch
}

// ✅ Proper error handling
try {
  await saveData(data);
} catch (error) {
  logger.error("Failed to save data", { error, data });
  throw new DataSaveError("Could not save data", { cause: error });
}
```

#### Resource Management

```python
# ❌ Resources not closed
file = open('data.txt')
data = file.read()
process(data)

# ✅ Proper cleanup with context manager
with open('data.txt') as file:
    data = file.read()
    process(data)
```

---

## Common Mistakes

### ❌ Don't Do This

1. **Be overly critical or personal** - Focus on code, not the developer
2. **Nitpick minor style issues** - Use automated tools (linters) for style
3. **Block on subjective preferences** - Distinguish preference from best practice
4. **Review too many changes at once** - Limit to ~400 lines per review
5. **Forget to check tests** - Untested code is incomplete
6. **Ignore security implications** - Security is non-negotiable
7. **Rush the review** - Take time to understand the changes
8. **Leave vague comments** - "This is wrong" without explanation

### ✅ Do This Instead

1. **Be constructive and respectful** - "Consider using X pattern because..."
2. **Explain the "why"** - Help developers learn from feedback
3. **Provide code examples** - Show the improved version
4. **Ask questions if unclear** - "What happens if Y is null?"
5. **Acknowledge good practices** - Positive reinforcement matters
6. **Focus on important issues** - Prioritize security, bugs, maintainability
7. **Consider the context** - Is this a prototype or production code?
8. **Offer to pair program** - For complex issues, collaborate

---

## Real-World Impact

Effective code reviews:
- **Reduce bugs by 60-90%** before reaching production
- **Improve team knowledge sharing** and code ownership
- **Enforce consistency** across the codebase
- **Mentor junior developers** through constructive feedback
- **Prevent security vulnerabilities** from being deployed
- **Maintain code quality** as the team scales

---

## Review Templates

### SQL Migration Template

```sql
-- Migration: [description]
-- Created: [date]

BEGIN;

-- Up migration
-- TODO: Add schema changes
-- CREATE TABLE IF NOT EXISTS ...
-- ALTER TABLE ...

-- Down migration (rollback)
-- TODO: Add rollback statements
-- DROP TABLE IF EXISTS ...

COMMIT;
```

### Schema Validation Script

```bash
#!/bin/bash
# validate-schema.sh - Validate database schema
# Usage: ./validate-schema.sh <schema_file>

set -euo pipefail

SCHEMA_FILE="${1:?Usage: $0 <schema_file>}"

echo "Validating schema: $SCHEMA_FILE"

# Validation checks:
# - Check SQL syntax
# - Verify foreign key references
# - Check index definitions
# - Validate naming conventions
# - Check for missing constraints

echo "Schema validation complete."
```