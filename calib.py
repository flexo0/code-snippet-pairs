"""Calibrates the docs weight on calib/: a snippet beside its descriptions, each of equal cognitive volume.

python3 calib.py [codeweight] [folder]

A snippet `<name>.<ext>` has a description `<name>.md` in prose and may have more, `<name>.<style>.md` (`dense`:
a terse spec of expressions, operators and tables). Prints each description's code weight (its snippet's), its
words, function words, punctuation runs and symbol runs, and the [docs] weights that make a description weigh
what its code does: one factor for all four, and some apart.
"""
import json
import pathlib
import subprocess
import sys
import tempfile

here = pathlib.Path(__file__).resolve().parent
cw = sys.argv[1] if len(sys.argv) > 1 else "codeweight"
folder = pathlib.Path(sys.argv[2] if len(sys.argv) > 2 else here / "calib").resolve()
KEYS = ["word", "function_word", "punctuation", "symbol"]


def weight(only, path, config=None):
    """A file's weight under the defaults, or under a weights file merged over them."""
    args = [cw, "scan", only, "--json", str(path)] + (["--config", config] if config else [])
    return json.loads(subprocess.run(args, capture_output=True, text=True, check=True).stdout)["total"]


examples = []  # (name, code weight, [words, function words, punctuation runs, symbol runs])
with tempfile.TemporaryDirectory() as t:
    configs = []
    for key in KEYS:
        f = pathlib.Path(t) / f"{key}.toml"
        f.write_text("[docs]\n" + "".join(f"{k} = {1 if k == key else 0}\n" for k in KEYS))
        configs.append(str(f))
    for md in sorted(folder.rglob("*.md")):
        stem = md.name.split(".")[0]
        code = [p for p in md.parent.glob(stem + ".*") if p.suffix != ".md"]
        if len(code) != 1:
            sys.exit(f"{md.relative_to(folder)}: no snippet, or more than one, beside it")
        examples.append((str(md.relative_to(folder))[:-3], weight("--code", code[0]),
                         [weight("--docs", md, c) for c in configs]))

print(f"{'description':<30} {'code':>7} {'words':>6} {'funct':>6} {'punct':>6} {'symbols':>7} {'code/run':>8}")
for name, c, n in examples:
    print(f"{name:<30} {c:>7.2f} {n[0]:>6.0f} {n[1]:>6.0f} {n[2]:>6.0f} {n[3]:>7.0f} {c / sum(n):>8.3f}")


def solve(a, b):
    """x of a x = b, by Gaussian elimination with partial pivoting."""
    m = [row[:] + [v] for row, v in zip(a, b)]
    size = len(m)
    for i in range(size):
        p = max(range(i, size), key=lambda r: abs(m[r][i]))
        m[i], m[p] = m[p], m[i]
        for r in range(size):
            if r != i:
                f = m[r][i] / m[i][i]
                m[r] = [x - f * y for x, y in zip(m[r], m[i])]
    return [m[i][size] / m[i][i] for i in range(size)]


def fit(groups):
    """The weights that minimize sum((sum(w n) / c - 1)^2), a weight per group of count columns, each a column
    and its share of the group's weight (`[(0, 1), (2, 0.5)]`: column 2 weighs half of column 0); a column in no
    group weighs 0."""
    rows = [[sum(n[j] * f for j, f in g) / c for g in groups] for _, c, n in examples]
    a = [[sum(r[i] * r[j] for r in rows) for j in range(len(groups))] for i in range(len(groups))]
    b = [sum(r[i] for r in rows) for i in range(len(groups))]
    w = [0.0] * len(KEYS)
    for g, v in zip(groups, solve(a, b)):
        for j, f in g:
            w[j] = v * f
    return w


def miss(w):
    """Each description's weight under these [docs] weights, against its code's: the worst and the mean miss."""
    m = [sum(x * y for x, y in zip(w, n)) / c - 1 for _, c, n in examples]
    return max(m, key=abs), sum(abs(x) for x in m) / len(m)


fits = [
    ("all 1", [1, 1, 1, 1]),
    ("one factor", fit([[(0, 1), (1, 1), (2, 1), (3, 1)]])),
    ("all apart", fit([[(0, 1)], [(1, 1)], [(2, 1)], [(3, 1)]])),
    ("halves", fit([[(0, 1), (1, 0.5), (2, 0.5)], [(3, 1)]])),
    ("defaults", [0.64, 0.32, 0.32, 0.62]),
]
for label, w in fits:
    worst, mean = miss(w)
    print(f"{label:<11} " + "  ".join(f"{k} {v:.3f}" for k, v in zip(KEYS, w))
          + f"   worst miss {worst:+.0%}, mean {mean:.0%}")
