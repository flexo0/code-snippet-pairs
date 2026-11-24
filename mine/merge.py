"""Merges the mining agents' index files into a set's `index.tsv` and checks every pair: both sides there, indexed,
not the same but for comments and whitespace, not a copy of another, parsing cleanly.

Usage: python3 merge.py <folder of index-*.tsv> <pair set folder> [--write]
codeweight is `$CODEWEIGHT`, else the one on the PATH. Without `--write` it only reports."""

import collections
import os
import pathlib
import re
import subprocess
import sys

indexes, pairs = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
cw = os.environ.get("CODEWEIGHT", "codeweight")

rows = {}
for f in sorted(indexes.glob("index-*.tsv")):
    for line in f.read_text().splitlines():
        if not line.strip():
            continue
        cols = line.split("\t")
        if len(cols) != 5:
            print("bad index line", f.name, line[:100])
            continue
        if cols[0] in rows:
            print("duplicate case", cols[0])
        rows[cols[0]] = cols
bads = {p.name.split(".bad.")[0] for p in pairs.glob("*.bad.*")}
goods = {p.name.split(".good.")[0] for p in pairs.glob("*.good.*")}
for c in sorted(bads ^ goods):
    print("one side only", c)
for c in sorted((bads & goods) - rows.keys()):
    print("not indexed", c)
for c in sorted(rows.keys() - (bads & goods)):
    print("indexed, no files", c)
    del rows[c]


def strip(text):
    """The text without comments and whitespace."""
    return re.sub(r"\s+", "", re.sub(r"//[^\n]*", "", re.sub(r"/\*.*?\*/", "", text, flags=re.S)))


seen = {}
for c in sorted(rows):
    b, g = (next(pairs.glob(f"{c}.{side}.*")).read_text() for side in ("bad", "good"))
    if strip(b) == strip(g):
        print("same but for comments/whitespace", c)
    key = (strip(b), strip(g))
    if key in seen:
        print("copy", c, "of", seen[key])
    seen[key] = c
out = subprocess.run([cw, "scan", "--code", "--by", "file", str(pairs)], capture_output=True, text=True)
for line in out.stderr.splitlines():
    print(line)
print(len(rows), "pairs;", collections.Counter(r[3] for r in rows.values()).most_common())
if "--write" in sys.argv:
    with open(pairs / "index.tsv", "w") as f:
        f.write("case\tcommit\tfile\tkind\twhy\n")
        for c in sorted(rows):
            f.write("\t".join(rows[c]) + "\n")
