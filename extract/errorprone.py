"""Error Prone's refactoring tests: each `addInputLines(name, ...)` and the `addOutputLines(name, ...)` after it
are a fixture and its fixed text (string literals, a line each, or a text block), split into pairs. A checker's
group is its `@BugPattern` tags (style, simplification, fragile_code, ...), else its severity (error, warning,
suggestion): an error's fix mends a bug, it doesn't keep the behaviour.
Usage: errorprone.py <error-prone clone> <gen folder>"""

import pathlib
import re
import subprocess
import sys

sys.path.insert(0, str(pathlib.Path(__file__).parent))
import split  # noqa: E402
from common import Out  # noqa: E402

clone, gen = pathlib.Path(sys.argv[1]), sys.argv[2]
sha = subprocess.run(["git", "-C", str(clone), "rev-parse", "--short", "HEAD"],
                     capture_output=True, text=True).stdout.strip()
ESC = {"n": "\n", "t": "\t", "r": "\r", "b": "\b", "f": "\f", "s": " ", '"': '"', "'": "'", "\\": "\\", "\n": ""}


def unescape(s):
    return re.sub(r"\\(u[0-9a-fA-F]{4}|[0-7]{1,3}|.)",
                  lambda m: chr(int(m.group(1)[1:], 16)) if m.group(1)[0] == "u"
                  else chr(int(m.group(1), 8)) if m.group(1)[0].isdigit() else ESC.get(m.group(1), m.group(1)),
                  s, flags=re.S)


def text_block(raw):
    """A text block's value: the incidental indentation (the least, the closing line's too) removed."""
    lines = raw.split("\n")[1:]  # the opening line ends at `"""`
    ind = [len(ln) - len(ln.lstrip()) for ln in lines[:-1] if ln.strip()]
    if not lines[-1].strip():
        ind.append(len(lines[-1]))
    m = min(ind) if ind else 0
    body = "\n".join(ln[m:].rstrip() for ln in lines)
    return unescape(body)


def args(src, i):
    """The string arguments of a call whose `(` is at i: [value or None for a non-string], and the index after."""
    out, cur, depth, j, n = [], [], 1, i + 1, len(src)
    plain = True
    while j < n and depth:
        if src.startswith('"""', j):
            k = src.find('"""', j + 3)
            while k > 0 and src[k - 1] == "\\":
                k = src.find('"""', k + 1)
            cur.append(text_block(src[j + 3:k]))
            j = k + 3
        elif src[j] == '"':
            k = j + 1
            while src[k] != '"':
                k += 2 if src[k] == "\\" else 1
            cur.append(unescape(src[j + 1:k]))
            j = k + 1
        elif src.startswith("//", j):
            j = src.find("\n", j)
        elif src[j] in "([{":
            depth += 1
            plain = False
            j += 1
        elif src[j] in ")]}":
            depth -= 1
            j += 1
        elif src[j] == "," and depth == 1:
            out.append("".join(cur) if plain and cur else None)
            cur, plain = [], True
            j += 1
        else:
            if not src[j].isspace() and src[j] != "+":
                plain = False
            j += 1
    out.append("".join(cur) if plain and cur else None)
    return out, j


tags = {}
for f in (clone / "core" / "src" / "main").rglob("*.java"):
    m = re.search(r"@BugPattern\((.*?)\)\s*(public\s+)?(final\s+)?class\s+(\w+)", f.read_text(), re.S)
    if m:
        t = re.search(r"tags\s*=\s*\{?([^}=]*)", m.group(1))
        names = re.findall(r"\b([A-Z][A-Z_]{3,})\b", t.group(1)) if t else []
        sev = re.search(r"severity\s*=\s*(?:\w+\.)*(ERROR|WARNING|SUGGESTION)", m.group(1))
        tags[m.group(4)] = "+".join(sorted({x.lower() for x in names})) or (sev.group(1).lower() if sev else "")

out = Out(gen, "errorprone", "java")
for test in sorted((clone / "core" / "src" / "test").rglob("*Test.java")):
    src = test.read_text()
    if "addOutputLines" not in src:
        continue
    checker = test.stem.removesuffix("Test")
    group = tags.get(checker) or "untagged"
    rel = test.relative_to(clone)
    pending = {}
    for m in re.finditer(r"\.add(Input|Output)Lines\(", src):
        a, _ = args(src, m.end() - 1)
        if not a or a[0] is None or any(x is None for x in a[1:]):
            continue
        name, text = a[0], "\n".join(a[1:])
        line = src.count("\n", 0, m.start()) + 1
        if m.group(1) == "Input":
            pending[name] = (text, line)
        elif name in pending:
            bad, at = pending.pop(name)
            for b, g, ln in split.pairs(bad, text, "java"):
                out.add(checker, group, f"L{at}-{ln}", b, g, f"https://github.com/google/error-prone/blob/{sha}/{rel}#L{at}")
out.close()
