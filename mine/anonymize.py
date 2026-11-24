"""Anonymizes a mined pair set in place, so no name of the code it came from is left: every class, method,
variable, parameter, field and constant gets a useless name (`Class1`, `method1`, `var1`, `CONST1`), every string
literal `"str1"`, and the comments go. A pair's two sides share one mapping, so they stay the same code but for the
refactor. The names of the JDK and of the libraries (`anonymize-keep.txt`) stay: they are the refactor (`stream`,
`BitSet`, `computeIfAbsent`). Numbers, `this`, keywords and the structure stay, so the weights don't change: run
`check.py` before and after, `results.tsv` must be the same.

Usage: python3 anonymize.py <pair set folder>      (Java pairs: `<case>.bad.java` beside `<case>.good.java`)
Run it once on a new set, before anything is committed: a second run renames the new names again."""

import collections
import pathlib
import re
import sys

KEYWORDS = set("""abstract assert boolean break byte case catch char class const continue default do double else enum
extends final finally float for goto if implements import instanceof int interface long native new package private
protected public return short static strictfp super switch synchronized this throw throws transient try void volatile
while var record yield true false null sealed permits when non-sealed""".split())
TOKEN = re.compile(r'''//[^\n]*|/\*.*?\*/|"""(?:\\.|[^\\])*?"""|"(?:\\.|[^"\\\n])*"|'(?:\\.|[^'\\\n])*'|[A-Za-z_$][\w$]*|\d[\w.]*|\s+|.''', re.S)

keep = {"T": set(), "M": set(), "F": set()}
for line in (pathlib.Path(__file__).parent / "anonymize-keep.txt").read_text().splitlines():
    if line and not line.startswith("#"):
        kind, *names = line.split()
        keep[kind] |= set(names)


class Names:
    """One pair's mapping from the original names and strings to the new ones, numbered in order of appearance."""

    def __init__(self):
        self.names, self.strings, self.count = {}, {}, collections.Counter()

    def name(self, word, role):
        if word not in self.names:
            kind = ("CONST" if re.fullmatch(r"[A-Z][A-Z0-9_]+", word) else "Class") if word[0].isupper() \
                else ("method" if role == "method" else "var")
            self.count[kind] += 1
            self.names[word] = f"{kind}{self.count[kind]}"
        return self.names[word]

    def string(self, text):
        if text not in self.strings:
            self.strings[text] = f"str{len(self.strings) + 1}"
        return self.strings[text]


def anonymize(source, names):
    tokens = [m.group() for m in TOKEN.finditer(source)]
    significant = [i for i, t in enumerate(tokens) if not t.isspace() and not t.startswith(("//", "/*"))]
    out = []
    for pos, i in enumerate(significant):
        prev = tokens[significant[pos - 1]] if pos else ""
        nxt = tokens[significant[pos + 1]] if pos + 1 < len(significant) else ""
        t = tokens[i]
        if re.match(r"[A-Za-z_$]", t) and t not in KEYWORDS:
            method = nxt == "(" or prev == "::"
            if t[0].isupper():
                tokens[i] = t if t in keep["T"] else names.name(t, "type")
            elif method:
                tokens[i] = t if t in keep["M"] else names.name(t, "method")
            elif t in keep["F"] and (prev == "." or t == "log"):
                pass
            else:
                tokens[i] = names.name(t, "var")
        elif t.startswith('"') and t not in ('""',):
            tokens[i] = f'"{names.string(t)}"'
    for i, t in enumerate(tokens):
        if t.startswith(("//", "/*")):
            tokens[i] = ""
    text = "".join(tokens)
    # a line a comment emptied goes (when no literal or comment spans lines), the rest loses its trailing blanks
    lines, original = text.split("\n"), source.split("\n")
    if len(lines) == len(original):
        lines = [l for l, o in zip(lines, original) if l.strip() or not o.strip()]
    return "\n".join(l.rstrip() for l in lines)


folder = pathlib.Path(sys.argv[1])
for bad in sorted(folder.glob("*.bad.java")):
    good = bad.with_name(bad.name.replace(".bad.", ".good."))
    names = Names()
    for f in (bad, good):
        f.write_text(anonymize(f.read_text(), names))
print(f"anonymized {len(list(folder.glob('*.bad.java')))} pairs in {folder}")
