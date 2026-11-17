from functools import cache
from pathlib import Path
import re
import subprocess
import tomllib
import unittest


ROOT = Path(__file__).resolve().parents[1]
LANGUAGE = ROOT / "languages" / "tlaplus"
EXAMPLES = sorted((ROOT / "examples").glob("*.tla"))
CAPTURE = re.compile(r"capture: \d+ - ([\w.]+), .*?text: `(.*?)`$", re.MULTILINE)


def tree_sitter(command, *args):
    result = subprocess.run(
        ["tree-sitter", command, "--lib-path", ".cache/tlaplus.so",
         "--lang-name", "tlaplus", *map(str, args)],
        cwd=ROOT, capture_output=True, text=True, encoding="utf-8",
    )
    if result.returncode:
        raise AssertionError(result.stdout + result.stderr)
    return result.stdout


@cache
def query(name, example):
    return tree_sitter("query", LANGUAGE / f"{name}.scm", ROOT / "examples" / example)


class SyntaxTests(unittest.TestCase):
    def test_metadata(self):
        manifest = tomllib.loads((ROOT / "extension.toml").read_text(encoding="utf-8"))
        config = tomllib.loads((LANGUAGE / "config.toml").read_text(encoding="utf-8"))
        self.assertIn(config["grammar"], manifest["grammars"])
        self.assertRegex(manifest["grammars"]["tlaplus"]["rev"], r"^[0-9a-f]{40}$")
        self.assertEqual(config["path_suffixes"], ["tla"])
        self.assertEqual(config["line_comments"], ["\\* "])
        self.assertEqual(config["block_comment"]["start"], "(*")
        self.assertEqual(config["block_comment"]["end"], "*)")
        self.assertIn("prefix", config["block_comment"])
        self.assertIn("tab_size", config["block_comment"])
        self.assertNotIn("language_servers", manifest)
        indent = re.compile(config["increase_indent_pattern"])
        self.assertRegex("Init ==", indent)
        self.assertRegex("Init ≜", indent)
        self.assertNotRegex("    /\\ count = 0", indent)
        # A prime changes a TLA+ state variable, so it must not insert a quote.
        self.assertNotIn("'", [pair["start"] for pair in config["brackets"]])

    def test_examples_parse_without_errors(self):
        for example in EXAMPLES:
            with self.subTest(example=example.name):
                tree_sitter("parse", "--quiet", example)

    def test_all_queries_compile(self):
        for path in sorted(LANGUAGE.glob("*.scm")):
            with self.subTest(query=path.name):
                query(path.stem, "Counter.tla")

    def test_highlight_captures(self):
        captures = set(CAPTURE.findall(query("highlights", "Counter.tla")))
        expected = {
            ("keyword", "MODULE"), ("keyword", "THEOREM"),
            ("type", "Counter"), ("constant", "Max"),
            ("function", "Init"), ("function", "f"),
            ("variable", "count"), ("variable.parameter", "n"),
            ("property", "value"), ("boolean", "TRUE"),
            ("number", "0"), ("number", "1.5"),
            ("number", "\\b101"), ("number", "\\o17"), ("number", "\\hFF"),
            ("operator", "=="), ("operator", "'"),
            ("operator", "\\in"), ("operator", "[]"),
            ("string", '"counter\\n"'), ("string.escape", "\\n"),
            ("comment", "\\* A bounded counter with a history of its values."),
            ("comment", " nested comments"),
        }
        self.assertFalse(expected - captures, f"Missing captures: {expected - captures}")
        unicode_captures = set(CAPTURE.findall(query("highlights", "Unicode.tla")))
        for symbol in ["∧", "□", "∈", "⇒"]:
            self.assertIn(("operator", symbol), unicode_captures)

    def test_bracket_pairs(self):
        pairs = set()
        for example in EXAMPLES:
            output = query("brackets", example.name)
            for match in re.split(r"  pattern: \d+\n", output)[1:]:
                captures = CAPTURE.findall(match)
                self.assertEqual([name for name, _ in captures], ["open", "close"])
                pairs.add(tuple(text for _, text in captures))
        expected = {
            ("(", ")"), ("[", "]"), ("{", "}"), ('"', '"'),
            ("<<", ">>"), ("⟨", "⟩"), ("〈", "〉"),
            ("[", "]_"), ("<<", ">>_"), ("⟨", "⟩_"),
        }
        self.assertFalse(expected - pairs, f"Missing bracket pairs: {expected - pairs}")
        self.assertNotIn(("<", ">"), pairs)

    def test_indent_and_override_captures(self):
        indents = query("indents", "Counter.tla")
        self.assertIn(" - indent,", indents)
        captures = set(CAPTURE.findall(indents))
        self.assertIn(("start", "=="), captures)
        self.assertIn(("start", "LET"), captures)
        self.assertIn(("end", "IN"), captures)
        overrides = set(CAPTURE.findall(query("overrides", "Counter.tla")))
        self.assertIn(("string", '"counter\\n"'), overrides)
        self.assertTrue(any(name == "comment.inclusive" for name, _ in overrides))


if __name__ == "__main__":
    unittest.main()
