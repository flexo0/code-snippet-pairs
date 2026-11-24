"""Clippy's `tests/ui`: each `X.rs` beside its `X.fixed`, split into pairs; the lints of the complexity and style
groups only. Usage: clippy.py <rust-clippy clone> <gen folder>"""

import pathlib
import re
import sys

sys.path.insert(0, str(pathlib.Path(__file__).parent))
import split  # noqa: E402
from common import Out  # noqa: E402

KEEP = {"complexity", "style"}
clone, gen = pathlib.Path(sys.argv[1]), sys.argv[2]
commit = (clone / ".git").exists() and __import__("subprocess").run(
    ["git", "-C", str(clone), "rev-parse", "--short", "HEAD"], capture_output=True, text=True).stdout.strip()

groups = {}
for f in (clone / "clippy_lints" / "src").rglob("*.rs"):
    for m in re.finditer(r"declare_clippy_lint!\s*\{.*?pub\s+(\w+),\s*(\w+),", f.read_text(), re.S):
        groups[m.group(1).lower()] = m.group(2)


def lints_of(rs, stderr):
    """The file's lint: its stem's if it is one, else the lints its stderr names."""
    stem = rs.stem
    while stem:
        if stem in groups:
            return [stem]
        stem = stem.rsplit("_", 1)[0] if "_" in stem else ""
    names = re.findall(r"clippy::([a-z0-9_-]+)", stderr.read_text()) if stderr.exists() else []
    return sorted({n.replace("-", "_") for n in names if n.replace("-", "_") in groups})


out = Out(gen, "clippy", "rs")
files = skipped = 0
ui = clone / "tests" / "ui"
for fixed in sorted(ui.rglob("*.fixed")):
    rs = fixed.with_name(fixed.name.split(".")[0] + ".rs")
    if not rs.exists():
        continue
    lints = lints_of(rs, fixed.with_suffix(".stderr"))
    kept = [lint for lint in lints if groups[lint] in KEEP]
    if not kept:
        skipped += 1
        continue
    files += 1
    rule = kept[0]
    rel = rs.relative_to(clone)
    for bad, good, line in split.pairs(rs.read_text(), fixed.read_text(), "rust"):
        case = f"{fixed.name.removesuffix('.fixed').replace('.', '_')}-L{line}"
        out.add(rule, groups[rule], case, bad, good,
                f"https://github.com/rust-lang/rust-clippy/blob/{commit}/{rel}#L{line}")
out.close()
print(f"clippy: {files} test files kept, {skipped} of other groups")
