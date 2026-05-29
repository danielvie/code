# Code Snippets Reference

Code snippets must be readable and syntax-colored. Use dark `pre` blocks with inline JavaScript highlighting.

## Code Block Style

```css
pre {
  margin: 12px 0;
  padding: 14px;
  overflow-x: auto;
  border-radius: 8px;
  border: 1px solid #293640;
  background: var(--code);
  color: #dce8e5;
  font-size: 13px;
  line-height: 1.45;
}

pre code {
  padding: 0;
  border: 0;
  background: transparent;
  color: inherit;
  font-size: inherit;
}
```

## Syntax Colors

Use token classes:

```css
.tok-comment { color: #91a8a2; font-style: italic; }
.tok-keyword { color: #7dd3fc; font-weight: 700; }
.tok-type { color: #a7f3d0; }
.tok-fn { color: #fde68a; }
.tok-string { color: #fda4af; }
.tok-number { color: #c4b5fd; }
.tok-literal { color: #f9a8d4; font-weight: 700; }
.tok-operator { color: #93c5fd; }
```

## Syntax Highlighter Rules

Use a small inline highlighter if no external libraries are allowed. It should:

- escape `&`, `<`, and `>` before injecting HTML;
- highlight `//` comments;
- highlight line-start `#` comments for config snippets;
- highlight quoted strings;
- highlight numbers, hex values, floats, and simple units such as `20K`;
- highlight known keywords;
- highlight known literals;
- highlight known project types;
- treat capitalized identifiers as types;
- treat words followed by `(` as function names;
- highlight common operators and punctuation.

Use this pattern:

```js
const syntaxKeywords = new Set([
  "async", "await", "const", "float", "if", "message", "oneof", "return",
  "static", "string", "struct", "uint32", "uint32_t", "void",
]);

const syntaxLiterals = new Set(["false", "true", "y", "n"]);

const syntaxTypes = new Set([
  "BlePacket", "DataSet", "MatrixData", "SignalStep", "Telemetry",
  "UiCommand", "UiCommandResult", "Uint8Array",
]);

const escapeHtml = (text) =>
  text.replace(/[&<>]/g, (char) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;" })[char]);

const colorToken = (className, text) =>
  `<span class="${className}">${escapeHtml(text)}</span>`;

const highlightCodeBlock = (code) => {
  const source = code.textContent;
  let html = "";
  let index = 0;

  while (index < source.length) {
    const rest = source.slice(index);

    if (rest.startsWith("//")) {
      const end = source.indexOf("\n", index);
      const next = end === -1 ? source.length : end;
      html += colorToken("tok-comment", source.slice(index, next));
      index = next;
      continue;
    }

    if (source[index] === "#" && (index === 0 || source[index - 1] === "\n")) {
      const end = source.indexOf("\n", index);
      const next = end === -1 ? source.length : end;
      html += colorToken("tok-comment", source.slice(index, next));
      index = next;
      continue;
    }

    const stringMatch = rest.match(/^"([^"\\]|\\.)*"|^'([^'\\]|\\.)*'/);
    if (stringMatch) {
      html += colorToken("tok-string", stringMatch[0]);
      index += stringMatch[0].length;
      continue;
    }

    const numberMatch = rest.match(/^(0x[\da-fA-F]+|\d+(?:\.\d+)?f?|\d+K)\b/);
    if (numberMatch) {
      html += colorToken("tok-number", numberMatch[0]);
      index += numberMatch[0].length;
      continue;
    }

    const wordMatch = rest.match(/^[A-Za-z_][\w:]*/);
    if (wordMatch) {
      const word = wordMatch[0];
      const afterWord = source.slice(index + word.length);
      if (syntaxKeywords.has(word)) html += colorToken("tok-keyword", word);
      else if (syntaxLiterals.has(word)) html += colorToken("tok-literal", word);
      else if (syntaxTypes.has(word) || /^[A-Z][\w:]*$/.test(word)) html += colorToken("tok-type", word);
      else if (/^\s*\(/.test(afterWord)) html += colorToken("tok-fn", word);
      else html += escapeHtml(word);
      index += word.length;
      continue;
    }

    if (/^[{}()[\];,.=:+\-*/&|!<>]/.test(rest)) {
      html += colorToken("tok-operator", source[index]);
      index += 1;
      continue;
    }

    html += escapeHtml(source[index]);
    index += 1;
  }

  code.innerHTML = html;
};

document.querySelectorAll("pre code").forEach(highlightCodeBlock);
```

Adapt `syntaxKeywords`, `syntaxTypes`, and `syntaxLiterals` to the document topic. For example:

- protocol documents should include `message`, `oneof`, `uint32`;
- firmware documents should include `static`, `volatile`, `IRAM_ATTR`, `uint32_t`;
- TypeScript/UI documents should include `type`, `interface`, `async`, `await`, `Uint8Array`.

## Code Snippet Content Rules

- Keep snippets short: ideally 5 to 25 lines.
- Use snippets to illustrate contracts, types, state transitions, or timing-critical logic.
- Do not paste huge functions unless the whole function is the subject of the study.
- Use inline `<code>` for filenames, fields, constants, and function names in paragraphs and tables.
- Escape `<`, `>`, and `&` in raw HTML snippets unless the highlighter reads from `textContent` safely.
