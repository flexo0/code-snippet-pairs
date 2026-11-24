"""Weighs a pair set: each snippet's total under the defaults, and its per-category count (a run per category,
its weight 1, the others 0; a count is a sum of nest(depth), not a plain count). Each pair gets a verdict:

  ok           the good snippet weighs less
  broken       it doesn't, but the difference has a positive component: other weights could fix it
  no-positive  the good one has as much of every category as the bad one, or more: no weights can fix it
  invisible    no category count differs: the change is one the model doesn't see (a name, an API, a literal)

A pair a side of which doesn't parse cleanly is an error: the extractors drop such pairs, the hand-picked ones
must parse.

Writes `results.tsv` (a line per pair) and `rules.tsv` (the verdicts per rule) into the folder; prints a summary
per source and group (a generated source's `index.tsv` gives a rule's group), and each pair when there are few;
a `gen` folder below is left to a run of its own. Usage: python3 check.py [codeweight binary] [folder]"""

import collections
import json
import pathlib
import subprocess
import sys
import tempfile

here = pathlib.Path(__file__).parent
cw = sys.argv[1] if len(sys.argv) > 1 else "codeweight"
pairs = pathlib.Path(sys.argv[2] if len(sys.argv) > 2 else here).resolve()
CATS = ["statement", "call", "macro", "constructor", "argument", "branch", "case", "loop", "try", "catch", "lambda", "function", "class",
        "bool_op", "jump", "operator", "access", "parameter"]


def scan(config=None):
    """Each file's weight, its tests' too (a Rust file's `#[test]`s, a test file), and the files with warnings."""
    weights, warned = collections.Counter(), set()
    for only in ("--code", "--tests"):
        args = [cw, "scan", only, "--json", "--by", "file", "-v", str(pairs)] + (["--config", config] if config else [])
        r = json.loads(subprocess.run(args, capture_output=True, text=True, check=True).stdout)
        for row in r["rows"]:
            weights[str(pathlib.Path(row["path"]).resolve())] += row["weight"]
        warned |= {str(pathlib.Path(w["path"]).resolve()) for w in r["warnings"] if w["path"]}
    return weights, warned


total, warned = scan()
counts = {}
with tempfile.TemporaryDirectory() as t:
    for c in CATS:
        f = pathlib.Path(t) / f"{c}.toml"
        f.write_text("[weights]\n" + "".join(f"{k} = {1 if k == c else 0}\n" for k in CATS))
        counts[c] = scan(str(f))[0]

rows, unparsed = [], []
for bad in sorted(b for b in pairs.rglob("*.bad.*") if "gen" not in b.relative_to(pairs).parts):
    name, ext = bad.name.split(".bad.")
    good = bad.with_name(f"{name}.good.{ext}")
    b, g = str(bad), str(good)
    tb, tg = total[b], total[g]
    diff = {c: round(counts[c][b] - counts[c][g], 2) for c in CATS}
    diff = {c: v for c, v in diff.items() if v}
    if b in warned or g in warned:
        unparsed.append(str(bad.relative_to(pairs)))
        continue
    if tb > tg + 1e-9:
        verdict = "ok"
    elif not diff:
        verdict = "invisible"
    elif not any(v > 0 for v in diff.values()):
        verdict = "no-positive"
    else:
        verdict = "broken"
    rel = bad.relative_to(pairs)
    rows.append((str(rel.parent / name), tb, tg, verdict, " ".join(f"{c}{v:+g}" for c, v in diff.items())))

with open(pairs / "results.tsv", "w") as f:
    f.write("pair\tbad\tgood\tverdict\tdifference\n")
    for p, tb, tg, v, d in rows:
        f.write(f"{p}\t{tb:g}\t{tg:g}\t{v}\t{d}\n")

if len(rows) <= 50:
    for p, tb, tg, v, d in rows:
        print(f"{p:<40}{tb:7.2f}{tg:7.2f}{tb - tg:+7.2f}  {v:<11} {d}")
    print()
VERDICTS = ["ok", "broken", "no-positive", "invisible"]
group = {}
for index in pairs.glob("*/index.tsv"):
    for line in index.read_text().splitlines()[1:]:
        rule, grp = line.split("\t")[:2]
        group[f"{index.parent.name}/{rule}"] = grp
by, rules = collections.defaultdict(collections.Counter), collections.defaultdict(collections.Counter)
for p, *_, v, _d in rows:
    parts = pathlib.PurePath(p).parts
    rule = "/".join(parts[:2]) if len(parts) > 2 else parts[0]
    by[f"{parts[0]}/{group[rule]}" if rule in group else parts[0]][v] += 1
    by["all"][v] += 1
    rules[rule][v] += 1


def share(c):
    return c["ok"] / sum(c.values()) if c else 0


with open(pairs / "rules.tsv", "w") as f:
    f.write("rule\tgroup\t" + "\t".join(VERDICTS) + "\tok\n")
    for r in sorted(rules, key=lambda r: (-rules[r]["broken"], -rules[r]["no-positive"], r)):
        c = rules[r]
        f.write(f"{r}\t{group.get(r, '')}\t" + "\t".join(str(c[v]) for v in VERDICTS) + f"\t{share(c):.0%}\n")
print(f"{'':<28}" + "".join(f"{v:>12}" for v in VERDICTS) + f"{'ok':>8}")
for k in sorted(by, key=lambda k: (k == "all", k)):
    c = by[k]
    print(f"{k:<28}" + "".join(f"{c[v]:>12}" for v in VERDICTS) + f"{share(c):>8.0%}")
if unparsed:
    sys.exit(f"{len(unparsed)} pairs don't parse cleanly, left out: " + ", ".join(unparsed[:10]))
