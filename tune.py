"""Tunes the default weights on the pairs with a simple evolutionary algorithm.

The score, lower is better: each pair's loss times its weight, summed, plus lam times the candidate's distance from
the defaults. A pair's loss is its order, 3 when its bad snippet weighs less than its good one and 1 when the two
weigh the same, plus alpha times a soft margin on r = ln((bad + 1) / (good + 1)): 1 / (1 + exp((r - delta) / tau)),
about 0.7 at a tie, 0.5 at r = delta, ~0 once r is a few tau past it. The margin sees an order held by a hair, which
the counts alone don't (`fragile` counts those, r under delta); it is relative and capped. It is off by default
(alpha 0): it cut the fragile pairs, but cost order on the rules held out. A pair's weight is 1 / sqrt(its rule's
pairs), three times that for a hand-picked one (`hand/`, `mine/`): a linter's rule with fifty tests doesn't weigh
fifty times one with a single test. The distance keeps what no pair asks for at its default. Over every pair of the
folder, `hand/`, `mine/` and `gen/` alike; a pair one side of which doesn't parse cleanly is left out (`check.py`
names them). The reports give the plain counts too: worse, same, better.

What it tunes: the base weights (`statement` fixed at 1: a pair fixes only ratios), `[nesting] model`, `factor`
and `max_depth`, `[parameters] step` and `max`, `[accesses] step` and `max`, and the four `[conditions]`. `[nesting] flat` stays as it is.

How: a snippet's weight is linear in its counts, so codeweight is run once per count, not once per candidate
(`extract`): a run per category and depth cap (a run with `max_depth = m`, linear, `factor = 1` adds the nodes at
depth m or deeper), per parameter cap (`step = 1`, `max = m` adds the parameters at place m - 1 or later), the
same per access cap (`[accesses]`, the accesses at place m - 1 or later in their row; every other run flat), and
per flat category under each of the 16 sets of conditions (they add flat nodes only). A candidate's score is
then exact, from each pair's counts (its difference, bad minus good, and its good side's), with no run of codeweight;
`verify` scores a weights file by codeweight's own totals.

The search: (mu + lambda), a uniform crossover and a mutation that shakes every real value at once, x to
x * U(0.9, 1.1) + U(-0.1, 0.1) (the weights, the nesting factor, the steps and maxes; no grid but 0.001), ever more
gently (the shake's scale falls from 1 to GENTLE over the generations), and now and then changes one discrete gene; at the end each gene moves back toward its default while the score gets no worse. `--holdout F` fits on a share of the rules
(not of the pairs: a rule's pairs are near copies) and scores the rest; `--test-source S` holds out a source.

Usage:
  python3 tune.py extract [codeweight] [folder]         the counts, into <folder>/tune-counts.json
  python3 tune.py search [--seed N] [--gens N] [--workers N] [--holdout F | --folds K --fold I | --test-source S]
                         [--delta D] [--tau T] [--alpha A] [--lam L] [--unweighted] [--pin GENE=VALUE]... [--out FILE]
  python3 tune.py verify FILE [codeweight] [folder]     the counts and loss of a weights file, by codeweight's totals
  python3 tune.py dedup [folder]                        deletes the generated pairs the tuning can't tell apart

`case` is kept below `branch` (`BELOW`).
"""

import argparse
import collections
import concurrent.futures
import json
import math
import multiprocessing
import operator
import os
import pathlib
import random
import subprocess
import sys
import tempfile
import tomllib

here = pathlib.Path(__file__).parent
CATS = ["statement", "call", "macro", "constructor", "argument", "branch", "case", "loop", "try", "catch", "lambda", "function", "class",
        "bool_op", "jump", "operator", "access", "parameter"]
FLAT = ["call", "macro", "constructor", "argument", "bool_op", "operator", "access"]  # [nesting] flat of the defaults, parameter aside
NESTED = [c for c in CATS if c not in FLAT and c != "parameter"]
CONDITIONS = ["casts", "let_patterns", "magic_numbers", "nullish_operator"]
COMBOS = 1 << len(CONDITIONS)
DEPTHS = 11  # depth 0..10, the last one "10 or deeper": max_depth is tuned up to 10
PLACES = 12  # parameter place 0..11, the last one "11 or later"
EPS = 0.01  # a candidate's difference up to a report's last digit (0.01) is a tie: a win must show in the reports
SHOWN = 1e-6  # verify's: two totals as the reports show them (to 0.01) tie when they are the same
HAND = 3  # a hand-picked pair's weight in the loss, against a generated one's 1
SMOOTH = 1  # added to both sides' weights in the margin: a statement's weight, so a tiny pair's ratio isn't wild
# The margin wanted, its softness and its weight against a pair in the wrong order, the pull toward the defaults.
# Off by default: on five folds of the rules any margin cost order on the rules held out (README).
LOSS = {"delta": 0.05, "tau": 0.05, "alpha": 0.0, "lam": 1.0}
# The other objective (`--objective diff`): the pairs' differences, bad minus good, summed, a wrong one times WRONG;
# each relative to its pair's size, (bad - good) / (bad + good), unless `--raw` (which scaling every weight up wins).
DIFF = {"objective": "order", "wrong": 20.0, "raw": False, "cap": 0.0, "power": 1.0}  # cap: a right pair's most (0 none); power: a right pair's difference to it

def read_defaults(path=here.parent / "codeweight" / "src" / "default.toml"):
    """The defaults the search starts from and is anchored to: the repo's src/default.toml."""
    t = tomllib.loads(path.read_text())
    return {"weights": {c: float(t["weights"][c]) for c in CATS}, "model": t["nesting"]["model"],
            "factor": float(t["nesting"]["factor"]), "max_depth": int(t["nesting"]["max_depth"]),
            "step": float(t["parameters"]["step"]), "max": float(t["parameters"]["max"]),
            "astep": float(t["accesses"]["step"]), "amax": float(t["accesses"]["max"]),
            "conditions": [bool(t["conditions"][c]) for c in CONDITIONS]}


DEFAULTS = read_defaults()


# ---------------------------------------------------------------------------------------------------- extract

def weights_toml(one, accesses="[accesses]\nstep = 0\nmax = 1\n"):
    """One category at 1, the others 0; every access alike unless `accesses` says otherwise."""
    return accesses + "[weights]\n" + "".join(f"{k} = {1 if k == one else 0}\n" for k in CATS)


def conditions_toml(bits):
    return "[conditions]\n" + "".join(f"{c} = {'true' if b else 'false'}\n" for c, b in zip(CONDITIONS, bits))


def scan(cw, folder, config_text):
    """Each file's weight (its tests' too, as check.py) and the files with warnings, under a weights file."""
    weights, warned = collections.Counter(), set()
    with tempfile.NamedTemporaryFile("w", suffix=".toml", delete=False) as f:
        f.write(config_text)
    try:
        for only in ("--code", "--tests"):
            args = [cw, "scan", only, "--json", "--by", "file", "-v", str(folder)]
            if config_text:
                args += ["--config", f.name]
            r = json.loads(subprocess.run(args, capture_output=True, text=True, check=True).stdout)
            for row in r["rows"]:
                weights[str(pathlib.Path(row["path"]).resolve())] += row["weight"]
            warned |= {str(pathlib.Path(w["path"]).resolve()) for w in r["warnings"] if w["path"]}
    finally:
        pathlib.Path(f.name).unlink()
    return weights, warned


def pairs_of(folder, warned):
    """(name, bad, good) of every pair whose sides parse cleanly, and the names of those left out."""
    out, left = [], []
    for bad in sorted(folder.rglob("*.bad.*")):
        name, ext = bad.name.split(".bad.")
        good = bad.with_name(f"{name}.good.{ext}")
        rel = str(bad.relative_to(folder).parent / name)
        if str(bad) in warned or str(good) in warned:
            left.append(rel)
        else:
            out.append((rel, str(bad), str(good)))
    return out, left


def extract(cw, folder):
    jobs = {"base": ""}
    for c in NESTED:
        for m in range(DEPTHS):
            jobs[f"nest/{c}/{m}"] = f'[nesting]\nmodel = "linear"\nfactor = 1\nmax_depth = {m}\n' + weights_toml(c)
    for m in range(1, PLACES + 1):
        jobs[f"param/{m}"] = f"[parameters]\nstep = 1\nmax = {m}\n" + weights_toml("parameter")
        jobs[f"acc/{m}"] = weights_toml("access", f"[accesses]\nstep = 1\nmax = {m}\n")
    for combo in range(COMBOS):
        bits = [bool(combo >> i & 1) for i in range(len(CONDITIONS))]
        for c in FLAT:
            jobs[f"flat/{combo}/{c}"] = conditions_toml(bits) + weights_toml(c)
    with concurrent.futures.ThreadPoolExecutor(6) as ex:
        res = dict(zip(jobs, ex.map(lambda t: scan(cw, folder, t), jobs.values())))
    warned = res["base"][1]
    pairs, left = pairs_of(folder, warned)

    def counts(value):
        """A pair's counts, each run's weight given by value(run): bad minus good, or the good side's."""
        nest = {}
        for c in NESTED:
            cum = [value(f"nest/{c}/{m}") for m in range(DEPTHS)]
            at_least = [cum[0]] + [cum[m] - cum[m - 1] for m in range(1, DEPTHS)]  # nodes at depth >= m
            per = [at_least[d] - (at_least[d + 1] if d + 1 < DEPTHS else 0) for d in range(DEPTHS)]
            if any(per):
                nest[c] = per
        cum = [value(f"param/{m}") for m in range(1, PLACES + 1)]
        at_least = [cum[0]] + [cum[m] - cum[m - 1] for m in range(1, PLACES)]  # parameters at place >= m
        param = [at_least[k] - (at_least[k + 1] if k + 1 < PLACES else 0) for k in range(PLACES)]
        cum = [value(f"acc/{m}") for m in range(1, PLACES + 1)]
        at_least = [cum[0]] + [cum[m] - cum[m - 1] for m in range(1, PLACES)]  # accesses at place >= m
        acc = [at_least[k] - (at_least[k + 1] if k + 1 < PLACES else 0) for k in range(PLACES)]
        flat = [{c: v for c in FLAT if (v := value(f"flat/{combo}/{c}"))} for combo in range(COMBOS)]
        return {"nest": nest, "param": param if any(param) else [], "acc": acc[1:] if any(acc[1:]) else [],
                "flat": flat, "default": value("base")}

    rows = []
    for name, b, g in pairs:
        row = {"pair": name, **counts(lambda key: res[key][0][b] - res[key][0][g])}
        row["good"] = counts(lambda key: res[key][0][g])
        rows.append(row)
    out = folder / "tune-counts.json"
    out.write_text(json.dumps({"pairs": rows, "left_out": left}))
    print(f"{len(rows)} pairs, {len(left)} left out, into {out}")


# ---------------------------------------------------------------------------------------------------- model

def source_of(name):
    """A generated pair's source; `hand` or `mine` for the others, by their top folder."""
    parts = pathlib.PurePath(name).parts
    return parts[1] if parts[0] == "gen" else parts[0]


def rule_of(name):
    """A generated pair's source and rule; a hand-picked pair is a rule of its own."""
    parts = pathlib.PurePath(name).parts
    return parts[1:3] if parts[0] == "gen" else (name,)


def pair_weights(names, weighted=True):
    """Each pair's weight in the loss: 1 / sqrt(its rule's pairs), HAND times that for a hand-picked one (1 each
    when not weighted)."""
    if not weighted:
        return [1.0] * len(names)
    size = collections.Counter(rule_of(n) for n in names)
    return [(1 if n.startswith("gen/") else HAND) / math.sqrt(size[rule_of(n)]) for n in names]


def pair_loss(bad, good, loss):
    """A pair's loss: 3 when its bad side weighs less, 1 when the two tie, and alpha times a soft margin, about 1
    at a tie, 0.5 at ln(bad / good) = delta, ~0 past it: an order barely held counts, a clear one doesn't."""
    order = 3 if bad < good - EPS else 1 if bad <= good + EPS else 0
    r = math.log((bad + SMOOTH) / (good + SMOOTH)) if bad + SMOOTH > 0 and good + SMOOTH > 0 else 0.0
    x = (loss["delta"] - r) / loss["tau"]
    return order + loss["alpha"] * (1 / (1 + math.exp(-x)) if x > -40 else 0.0)


class Pairs:
    """The pairs, as sparse terms over a coefficient vector: each one's difference (bad minus good) and good side,
    the same ones merged."""

    def __init__(self, rows, loss=None):
        self.loss = loss or LOSS
        # coefficient index: nested (cat, depth), parameter place, flat cat
        self.idx = {}
        for c in NESTED:
            for d in range(DEPTHS):
                self.idx[("n", c, d)] = len(self.idx)
        for k in range(PLACES):
            self.idx[("p", k)] = len(self.idx)
        for k in range(1, PLACES):
            self.idx[("a", k)] = len(self.idx)
        for c in FLAT:
            self.idx[("f", c)] = len(self.idx)
        merged = collections.defaultdict(lambda: [0, 0.0])  # key -> [pairs, their weight]
        for r, c in zip(rows, pair_weights([r["pair"] for r in rows], self.loss.get("weighted", True))):
            m = merged[(self.key(r), self.key(r["good"]))]
            m[0] += 1
            m[1] += c
        self.groups = []  # (pairs, weight, [difference terms per combo], [good side terms per combo])
        for (diff, good), (n, c) in merged.items():
            self.groups.append((n, c, [diff[0] + f for f in diff[1]], [good[0] + f for f in good[1]]))
        self.size = len(rows)
        # per combo, each group's terms as (indices, values), for a quick sum: (n, c, diff, good)
        self.fast = [[(n, c, tuple(zip(*diffs[combo])) or ((), ()), tuple(zip(*goods[combo])) or ((), ()))
                      for n, c, diffs, goods in self.groups] for combo in range(COMBOS)]

    def key(self, r):
        base = []
        for c, per in sorted(r["nest"].items()):
            base += [(self.idx[("n", c, d)], v) for d, v in enumerate(per) if v]
        base += [(self.idx[("p", k)], v) for k, v in enumerate(r["param"]) if v]
        base += [(self.idx[("a", k)], v) for k, v in enumerate(r.get("acc", []), 1) if v]
        flats = tuple(tuple((self.idx[("f", c)], v) for c, v in sorted(f.items())) for f in r["flat"])
        return tuple(base), flats

    def coefficients(self, g):
        w = g["weights"]
        coef = [0.0] * len(self.idx)
        for c in NESTED:
            for d in range(DEPTHS):
                depth = min(d, g["max_depth"])
                nest = 1 + g["factor"] * depth if g["model"] == "linear" else g["factor"] ** depth
                coef[self.idx[("n", c, d)]] = w[c] * nest
        for k in range(PLACES):
            coef[self.idx[("p", k)]] = w["parameter"] * min(1 + g["step"] * k, g["max"])
        for k in range(1, PLACES):  # an access past place 0: what its place adds over the flat count's weight
            coef[self.idx[("a", k)]] = w["access"] * (min(1 + g["astep"] * k, g["amax"]) - 1)
        for c in FLAT:
            coef[self.idx[("f", c)]] = w[c]
        return coef

    def objective(self, g):
        """(score, worse, same, loss) under a candidate, the score lower being better: the pairs' weighted loss
        plus lambda times its distance from the defaults."""
        coef = self.coefficients(g)
        at, mul = coef.__getitem__, operator.mul
        combo = sum(1 << i for i, b in enumerate(g["conditions"]) if b)
        objective = self.loss.get("objective", "order")
        if objective in ("diff", "hinge", "logistic"):
            return self.by_difference(g, coef, combo)
        margin = self.loss["alpha"] != 0
        worse = same = 0
        loss = 0.0
        for n, c, (di, dv), (gi, gv) in self.fast[combo]:
            d = sum(map(mul, map(at, di), dv))
            if margin:
                good = sum(map(mul, map(at, gi), gv))
                loss += c * pair_loss(good + d, good, self.loss)
            elif d < -EPS:
                worse += n
                loss += 3 * c
            elif d <= EPS:
                same += n
                loss += c
        tiebreak = self.loss.get("tiebreak", 0.0)
        if tiebreak:  # the order first; among candidates alike in it, the pairs clearer apart
            cap = self.loss.get("cap", 0.0) or 1.0
            loss -= tiebreak * sum(c * min(r, cap) for c, r in self.relative(at, combo))
        if margin:
            worse = sum(n for n, _, (di, dv), _ in self.fast[combo] if sum(map(mul, map(at, di), dv)) < -EPS)
            same = sum(n for n, _, (di, dv), _ in self.fast[combo] if abs(sum(map(mul, map(at, di), dv))) <= EPS)
        return loss + self.loss["lam"] * distance(g), worse, same, loss

    def relative(self, at, combo):
        """(weight, relative difference) of each group: (bad - good) / (bad + good), in -1..1."""
        mul = operator.mul
        for n, c, (di, dv), (gi, gv) in self.fast[combo]:
            d = sum(map(mul, map(at, di), dv))
            size = 2 * sum(map(mul, map(at, gi), gv)) + d
            yield c, (d / size if size > 0 else 0.0)

    def by_difference(self, g, coef, combo):
        """(score, worse, same, loss) by the differences: the loss is minus their sum, each pair's difference (bad
        minus good, relative to their sum unless raw) times its weight, a wrong one's times `wrong`."""
        at, mul = coef.__getitem__, operator.mul
        wrong, raw = self.loss.get("wrong", DIFF["wrong"]), self.loss.get("raw", DIFF["raw"])
        cap, power = self.loss.get("cap", DIFF["cap"]), self.loss.get("power", DIFF["power"])
        objective, m = self.loss.get("objective"), self.loss.get("margin", 0.05)
        worse = same = 0
        total = 0.0
        if objective == "hinge":  # short of the margin m, in m: a tie 1, a pair wrong by m 2; past it 0
            total = -sum(c * max(0.0, m - r) / m for c, r in self.relative(at, combo))
        elif objective == "logistic":  # a soft hinge: a tie 1, ~0 a few m past it, ~ -r / m / ln 2 when wrong
            total = -sum(c * (math.log2(1 + math.exp(-r / m)) if r / m > -40 else -r / m / math.log(2))
                         for c, r in self.relative(at, combo))
        for n, c, (di, dv), (gi, gv) in self.fast[combo] if objective == "diff" else ():
            d = sum(map(mul, map(at, di), dv))
            if not raw:
                good = sum(map(mul, map(at, gi), gv))
                size = 2 * good + d
                d = d / size if size > 0 else 0.0
            if d < 0:
                d *= wrong
            else:
                d = min(d, cap) if cap else d
                d = d ** power if power != 1 else d
            total += c * d
        for n, _, (di, dv), _ in self.fast[combo]:
            d = sum(map(mul, map(at, di), dv))
            if d < -EPS:
                worse += n
            elif d <= EPS:
                same += n
        return -total + self.loss["lam"] * distance(g), worse, same, -total

    def fragile(self, g):
        """The pairs in the right order by a margin r under delta: a small change of weights flips them."""
        coef = self.coefficients(g)
        combo = sum(1 << i for i, b in enumerate(g["conditions"]) if b)
        n_fragile = 0
        for n, _, diffs, goods in self.groups:
            d = sum(coef[i] * v for i, v in diffs[combo])
            good = sum(coef[i] * v for i, v in goods[combo])
            if d > EPS and math.log((good + d + SMOOTH) / (good + SMOOTH)) < self.loss["delta"]:
                n_fragile += n
        return n_fragile


# ---------------------------------------------------------------------------------------------------- search

TUNED = [c for c in CATS if c != "statement"]
# Weights kept below others, whatever the pairs say: a switch's case below an if (its default, as an else, free),
# so that a switch weighs less than the if-chain it replaces.
BELOW = [("case", "branch")]
W_MAX = 8.0


def clamp(x, lo, hi):
    """A value within its bounds, to 0.001: no grid but the digits a weights file shows."""
    return round(min(max(x, lo), hi), 3)


def distance(g):
    """How far a candidate is from the defaults: the score's pull toward them, and the tie-break."""
    d = sum(abs(math.log((g["weights"][c] + 0.1) / (DEFAULTS["weights"][c] + 0.1))) for c in TUNED)
    d += (g["model"] != DEFAULTS["model"]) + abs(g["factor"] - DEFAULTS["factor"])
    d += abs(g["max_depth"] - DEFAULTS["max_depth"]) / 10 + abs(g["step"] - DEFAULTS["step"])
    d += abs(g["max"] - DEFAULTS["max"]) / 2 + 0.5 * sum(g["conditions"])
    d += abs(g["astep"] - DEFAULTS["astep"]) + abs(g["amax"] - DEFAULTS["amax"]) / 2
    return d


BOUNDS = {"step": (0, 1), "max": (1, 4), "astep": (0, 2), "amax": (1, 5)}


def fix(g):
    g["factor"] = clamp(g["factor"], 0 if g["model"] == "linear" else 1, 3)
    g["max_depth"] = int(min(max(g["max_depth"], 0), DEPTHS - 1))
    for k, (lo, hi) in BOUNDS.items():
        g[k] = clamp(g[k], lo, hi)
    for c in TUNED:
        g["weights"][c] = clamp(g["weights"][c], 0, W_MAX)
    for low, high in BELOW:
        g["weights"][low] = round(min(g["weights"][low], g["weights"][high] - 0.01), 3)
    return g


def copy(g):
    return {**g, "weights": dict(g["weights"]), "conditions": list(g["conditions"])}


GENES = TUNED + ["model", "factor", "max_depth", "step", "max", "astep", "amax"] + [f"cond{i}" for i in range(len(CONDITIONS))]
REAL = ["factor"] + list(BOUNDS)  # the real genes beside the weights
SHAKE = (0.1, 0.1)  # a mutation takes every real value x to x * U(1 - a, 1 + a) + U(-b, b), a and b times its scale
GENTLE = 0.01  # the scale at the last generation: it falls from 1 geometrically, the shakes ever gentler
SWITCH = 0.25  # the chance a mutation also changes one discrete gene: the model, max_depth or a condition


def shaken(x, rnd, scale):
    a, b = SHAKE[0] * scale, SHAKE[1] * scale
    return x * rnd.uniform(1 - a, 1 + a) + rnd.uniform(-b, b)


def mutate(g, rnd, scale=1.0):
    """Every real value shaken at once (the weights, the factor, the steps and maxes), by `scale` times SHAKE, and
    now and then one discrete gene changed."""
    g = copy(g)
    for c in TUNED:
        g["weights"][c] = shaken(g["weights"][c], rnd, scale)
    for k in REAL:
        g[k] = shaken(g[k], rnd, scale)
    if rnd.random() < SWITCH:
        gene = rnd.choice(["model", "max_depth"] + [f"cond{i}" for i in range(len(CONDITIONS))])
        if gene == "model":
            g["model"] = "exponential" if g["model"] == "linear" else "linear"
            g["factor"] = 1 + g["factor"] / 2 if g["model"] == "exponential" else 2 * (g["factor"] - 1)
        elif gene == "max_depth":
            g["max_depth"] += rnd.choice([-2, -1, 1, 2])
        else:
            i = int(gene[4:])
            g["conditions"][i] = not g["conditions"][i]
    return fix(g)


def crossover(a, b, rnd):
    g = copy(a)
    for c in TUNED:
        if rnd.random() < 0.5:
            g["weights"][c] = b["weights"][c]
    if rnd.random() < 0.5:
        g["model"], g["factor"] = b["model"], b["factor"]
    for k in ("max_depth", "step", "max", "astep", "amax"):
        if rnd.random() < 0.5:
            g[k] = b[k]
    g["conditions"] = [x if rnd.random() < 0.5 else y for x, y in zip(a["conditions"], b["conditions"])]
    return g


def pin(g, pins):
    """Sets the pinned genes (`weights.call=0.5`, `factor=1`, `model=linear`, `conditions=1111`) on a candidate."""
    for k, v in pins.items():
        if k.startswith("weights."):
            g["weights"][k[8:]] = float(v)
        elif k == "model":
            g["model"] = v
        elif k == "conditions":
            g["conditions"] = [c == "1" for c in v]
        else:
            g[k] = type(DEFAULTS[k])(float(v))
    return g


MODEL = None  # the pairs a search's workers score, set before they fork


def score_of(g):
    return MODEL.objective(g)[0]


def search(rows, seed, gens, mu=40, lam=80, pins=None, loss=None, log=print, workers=None):
    """The best candidate of a (mu + lambda) search, each generation's candidates scored on `workers` processes."""
    global MODEL
    model = MODEL = Pairs(rows, loss)
    rnd = random.Random(seed)
    pins = pins or {}
    start = pin(fix(copy(DEFAULTS)), pins)
    pop = [start] + [pin(mutate(start, rnd), pins) for _ in range(mu - 1)]
    workers = workers or os.cpu_count()
    with multiprocessing.get_context("fork").Pool(workers) as pool:
        chunk = max(1, lam // workers)
        scored = list(zip(pool.map(score_of, pop, chunk), pop))
        scored.sort(key=lambda t: (t[0], distance(t[1])))
        seen = {json.dumps(g, sort_keys=True) for g in pop}
        for gen in range(gens):
            scale = GENTLE ** (gen / max(gens - 1, 1))
            kids = []
            while len(kids) < lam:
                a, b = rnd.sample(scored, 2)
                kid = pin(mutate(crossover(a[1], b[1], rnd) if rnd.random() < 0.5 else a[1], rnd, scale), pins)
                k = json.dumps(kid, sort_keys=True)
                if k not in seen:
                    seen.add(k)
                    kids.append(kid)
            scored += list(zip(pool.map(score_of, kids, chunk), kids))
            scored.sort(key=lambda t: (t[0], distance(t[1])))
            scored = scored[:mu]
            if gen % 100 == 0 or gen == gens - 1:
                _, w, t, _ = model.objective(scored[0][1])
                log(f"gen {gen:5}: scale {scale:.3f}, best {scored[0][0]:.3f} (worse {w}, same {t}), "
                    f"worst kept {scored[-1][0]:.3f}, seen {len(seen)}")
    return shrink(model, scored[0][1], pins, log)


SHRINK = (1, 0.5, 0.25, 0.1)  # the shares of its way to the default a real gene tries, the whole way first


def toward_default(g, gene, share=1):
    """The candidate with one gene moved toward its default (a real one by a share of the way, a discrete one all of
    it), None when it doesn't move."""
    h = copy(g)
    if gene in h["weights"]:
        w, d = h["weights"][gene], DEFAULTS["weights"][gene]
        h["weights"][gene] = w + share * (d - w)
    elif gene.startswith("cond"):
        i = int(gene[4:])
        h["conditions"][i] = DEFAULTS["conditions"][i]
    elif gene == "model":
        h["model"], h["factor"] = DEFAULTS["model"], DEFAULTS["factor"]
    elif gene == "max_depth":
        h["max_depth"] += max(-1, min(1, DEFAULTS["max_depth"] - h["max_depth"]))
    else:
        h[gene] = h[gene] + share * (DEFAULTS[gene] - h[gene])
    h = fix(h)
    return None if json.dumps(h, sort_keys=True) == json.dumps(g, sort_keys=True) else h


def shrink(model, g, pins, log):
    """Moves each gene toward its default while the score gets no worse: what no pair asks for goes back."""
    best = model.objective(g)[0]
    moved = True
    while moved:
        moved = False
        for gene in GENES:
            if gene in pins or f"weights.{gene}" in pins or (gene.startswith("cond") and "conditions" in pins):
                continue
            for share in SHRINK:
                h = toward_default(g, gene, share)
                if h is not None and (score := model.objective(h)[0]) <= best:
                    g, best, moved = h, score, True
                    break
    log(f"shrunk toward the defaults where the score allows: score {best:.3f}, distance {distance(g):.2f}")
    return g


def toml_of(g):
    lines = ["[nesting]", f'model = "{g["model"]}"', f"factor = {g['factor']:g}", f"max_depth = {g['max_depth']}",
             "", "[parameters]", f"step = {g['step']:g}", f"max = {g['max']:g}",
             "", "[accesses]", f"step = {g['astep']:g}", f"max = {g['amax']:g}", "", "[conditions]"]
    lines += [f"{c} = {'true' if b else 'false'}" for c, b in zip(CONDITIONS, g["conditions"])]
    lines += ["", "[weights]"] + [f"{c} = {g['weights'][c]:g}" for c in CATS]
    return "\n".join(lines) + "\n"


def report(rows, g, title, loss):
    print(f"{title}: score {Pairs(rows, loss).objective(g)[0]:.2f}")
    by = collections.defaultdict(list)
    for r in rows:
        by[source_of(r["pair"])].append(r)
    print(f"  {'source':<20}{'pairs':>7}{'worse':>7}{'same':>7}{'better':>8}{'fragile':>9}{'loss':>9}")
    for src in sorted(by) + ["all"]:
        m = Pairs(rows if src == "all" else by[src], loss)
        _, w, s, ls = m.objective(g)
        print(f"  {src:<20}{m.size:>7}{w:>7}{s:>7}{m.size - w - s:>8}{m.fragile(g):>9}{ls:>9.2f}")


def split(rows, args):
    """(train, test): the test rows a source's (--test-source), a fold of the rules (--folds, --fold) or a share of
    them (--holdout)."""
    if args.test_source:
        test = {i for i, r in enumerate(rows) if source_of(r["pair"]) == args.test_source}
    elif args.folds:
        rules = sorted({rule_of(r["pair"]) for r in rows})
        random.Random(args.seed).shuffle(rules)
        held = set(rules[args.fold::args.folds])
        test = {i for i, r in enumerate(rows) if rule_of(r["pair"]) in held}
    else:
        rules = sorted({rule_of(r["pair"]) for r in rows})
        held = set(random.Random(args.seed).sample(rules, int(len(rules) * args.holdout)))
        test = {i for i, r in enumerate(rows) if rule_of(r["pair"]) in held}
    return [r for i, r in enumerate(rows) if i not in test], [r for i, r in enumerate(rows) if i in test]


def main_search(args):
    data = json.loads((here / "tune-counts.json").read_text())
    rows = data["pairs"]
    loss = {"delta": args.delta, "tau": args.tau, "alpha": args.alpha, "lam": args.lam,
            "weighted": not args.unweighted, "objective": args.objective, "wrong": args.wrong, "raw": args.raw, "cap": args.cap, "power": args.power,
            "margin": args.margin, "tiebreak": args.tiebreak}
    defaults = fix(copy(DEFAULTS))
    pins = dict(p.split("=", 1) for p in args.pin)
    print(f"{len(rows)} pairs, {len(Pairs(rows, loss).groups)} distinct, loss {loss}")
    if args.holdout or args.test_source or args.folds:
        train_rows, test_rows = split(rows, args)
        best = search(train_rows, args.seed, args.gens, pins=pins, loss=loss, workers=args.workers)
        for name, part in (("train", train_rows), ("held out", test_rows)):
            m = Pairs(part, loss)
            (_, w0, s0, l0), (_, w1, s1, l1) = m.objective(defaults), m.objective(best)
            print(f"{name}: {len(part)} pairs, defaults worse {w0} same {s0} fragile {m.fragile(defaults)} "
                  f"loss {l0:.2f}, tuned worse {w1} same {s1} fragile {m.fragile(best)} loss {l1:.2f}")
    else:
        best = search(rows, args.seed, args.gens, pins=pins, loss=loss, workers=args.workers)
    report(rows, defaults, "defaults", loss)
    report(rows, best, "tuned", loss)
    print(toml_of(best))
    if args.out:
        pathlib.Path(args.out).write_text(toml_of(best))


def main_verify(args):
    folder = pathlib.Path(args.folder).resolve()
    text = pathlib.Path(args.file).read_text() if args.file != "-" else ""
    weights, warned = scan(args.codeweight, folder, text)
    pairs, left = pairs_of(folder, warned)
    worse = sum(weights[b] < weights[g] - SHOWN for _, b, g in pairs)
    same = sum(abs(weights[b] - weights[g]) <= SHOWN for _, b, g in pairs)
    loss = sum(c * pair_loss(weights[b], weights[g], LOSS)
               for c, (_, b, g) in zip(pair_weights([n for n, _, _ in pairs]), pairs))
    print(f"{len(pairs)} pairs ({len(left)} left out): worse {worse}, same {same}, loss {loss:.2f}")


def dedup(folder):
    """Thins `gen/` to what the tuning can tell apart: a generated pair whose difference is nothing under any
    candidate (the two sides the same elements, `yoda`'s `1 == x` against `x == 1`) is deleted, and of a rule's
    pairs with the same difference only the shortest is kept: a linter's tests try one rewrite on many variants, and
    each would count again. The hand-picked pairs stay. Reads and rewrites <folder>/tune-counts.json (`extract`
    first)."""
    path = folder / "tune-counts.json"
    data = json.loads(path.read_text())
    rows = data["pairs"]
    model = Pairs(rows)
    size = lambda r: sum(len(f.read_text()) for f in (folder / r["pair"]).parent.glob(
        pathlib.Path(r["pair"]).name + ".*"))
    kept, seen, drop = [], set(), []
    for r in sorted(rows, key=lambda r: (size(r), r["pair"])):
        if not r["pair"].startswith("gen/"):
            kept.append(r)
            continue
        key = model.key(r)
        base, flats = key
        rule = tuple(pathlib.PurePath(r["pair"]).parts[1:3])
        if not any(base + f for f in flats) or (rule, key) in seen:
            drop.append(r["pair"])
            continue
        seen.add((rule, key))
        kept.append(r)
    gone = set(drop)
    for name in drop:
        for f in (folder / name).parent.glob(pathlib.Path(name).name + ".*"):
            f.unlink()
    for index in (folder / "gen").glob("*/index.tsv"):
        lines = index.read_text().splitlines(keepends=True)
        src = index.parent.name
        index.write_text("".join([lines[0]] + [line for line in lines[1:]
                                               if f"gen/{src}/{line.split(chr(9))[3].split('.bad.')[0]}" not in gone]))
        for d in index.parent.iterdir():
            if d.is_dir() and not any(d.iterdir()):
                d.rmdir()
    order = {r["pair"]: i for i, r in enumerate(rows)}
    data["pairs"] = sorted(kept, key=lambda r: order[r["pair"]])
    path.write_text(json.dumps(data))
    print(f"{len(drop)} pairs deleted, {len(kept)} left")


if __name__ == "__main__":
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = p.add_subparsers(dest="cmd", required=True)
    e = sub.add_parser("extract")
    e.add_argument("codeweight", nargs="?", default="codeweight")
    e.add_argument("folder", nargs="?", default=str(here))
    s = sub.add_parser("search")
    s.add_argument("--seed", type=int, default=1)
    s.add_argument("--gens", type=int, default=3000)
    s.add_argument("--workers", type=int, default=0, help="the processes scoring the candidates (0: one per core)")
    s.add_argument("--holdout", type=float, default=0, help="a share of the rules held out to score the result")
    s.add_argument("--folds", type=int, default=0, help="the rules in this many folds, one held out (--fold)")
    s.add_argument("--fold", type=int, default=0)
    s.add_argument("--test-source", help="a source held out to score the result: clippy, ruff, errorprone, ...")
    s.add_argument("--delta", type=float, default=LOSS["delta"], help="the margin ln(bad / good) wanted")
    s.add_argument("--tau", type=float, default=LOSS["tau"], help="how soft the margin is")
    s.add_argument("--alpha", type=float, default=LOSS["alpha"], help="the margin's weight, against 3 per worse")
    s.add_argument("--unweighted", action="store_true", help="every pair weighs 1, whatever its rule")
    s.add_argument("--lam", type=float, default=LOSS["lam"], help="the pull toward the defaults")
    s.add_argument("--objective", choices=["order", "diff", "hinge", "logistic"], default=DIFF["objective"],
                   help="order: the pairs in the wrong order and the ties; diff: the differences, summed; hinge, "
                        "logistic: each pair's relative difference short of a margin")
    s.add_argument("--margin", type=float, default=0.05, help="hinge, logistic: the relative difference wanted")
    s.add_argument("--tiebreak", type=float, default=0.0,
                   help="order: this times the pairs' relative differences (capped by --cap) taken off the score")
    s.add_argument("--wrong", type=float, default=DIFF["wrong"], help="diff: a wrong pair's difference times this")
    s.add_argument("--raw", action="store_true", help="diff: each difference as it is, not relative to its pair")
    s.add_argument("--cap", type=float, default=DIFF["cap"], help="diff: the most a right pair's difference counts")
    s.add_argument("--power", type=float, default=DIFF["power"], help="diff: a right pair's difference to this power, 0.5 its root")
    s.add_argument("--out")
    s.add_argument("--pin", action="append", default=[], help="a gene kept as given: factor=1, weights.call=0.5")
    v = sub.add_parser("verify")
    v.add_argument("file", help="a weights file, or - for the defaults")
    v.add_argument("codeweight", nargs="?", default="codeweight")
    v.add_argument("folder", nargs="?", default=str(here))
    d = sub.add_parser("dedup")
    d.add_argument("folder", nargs="?", default=str(here))
    a = p.parse_args()
    {"extract": lambda: extract(a.codeweight, pathlib.Path(a.folder).resolve()), "search": lambda: main_search(a),
     "verify": lambda: main_verify(a), "dedup": lambda: dedup(pathlib.Path(a.folder).resolve())}[a.cmd]()
