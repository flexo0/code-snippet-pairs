"""Splits a linter's fixture and its fixed version into pairs: the smallest units (top-level items, then a
function's statements) that hold each change, the rest of the file left out.

A unit is found by bracket depth, strings and comments skipped (Rust, Go, Java, JS / TS), and by indentation
too in Python.
"""

import difflib
import re

OPEN, CLOSE = "{[(", "}])"


class Lexer:
    """Tracks bracket depth line by line, skipping strings, chars and comments; its state spans lines."""

    def __init__(self, lang):
        self.lang = lang
        self.depth = 0
        self.state = None  # None, "block", ('"',), ("raw", hashes), ("`",)

    def line(self, s):
        """Feeds a line; returns the code of it (strings and comments blanked), for the end-of-statement check."""
        out = []
        i, n = 0, len(s)
        while i < n:
            st = self.state
            if st == "block":
                j = s.find("*/", i)
                if j < 0:
                    return "".join(out)
                i, self.state = j + 2, None
                continue
            if st is not None:
                kind = st[0]
                if kind == '"' or kind == "'":
                    if s[i] == "\\":
                        i += 2
                        continue
                    if s[i] == kind:
                        self.state = None
                        out.append(kind)
                    i += 1
                    continue
                if kind in ('"""', "'''"):
                    j = s.find(kind, i)
                    if j < 0:
                        return "".join(out)
                    i, self.state = j + 3, None
                    out.append('"')
                    continue
                if kind == "raw":
                    end = '"' + "#" * st[1]
                    j = s.find(end, i)
                    if j < 0:
                        return "".join(out)
                    i, self.state = j + len(end), None
                    out.append('"')
                    continue
                if kind == "`":
                    j = s.find("`", i)
                    if j < 0:
                        return "".join(out)
                    i, self.state = j + 1, None
                    out.append("`")
                    continue
            c = s[i]
            if s.startswith("//", i) or (self.lang == "python" and c == "#"):
                break
            if s.startswith("/*", i):
                self.state, i = "block", i + 2
                continue
            if self.lang == "rust" and c == "r" and re.match(r'r#*"', s[i:]) and (i == 0 or not s[i - 1].isalnum()):
                m = re.match(r'r(#*)"', s[i:])
                self.state, i = ("raw", len(m.group(1))), i + len(m.group(0))
                continue
            if self.lang == "python" and s.startswith(('"""', "'''"), i):
                self.state, i = (s[i:i + 3],), i + 3
                out.append('"')
                continue
            if c == '"':
                self.state, i = ('"',), i + 1
                out.append('"')
                continue
            if c == "'":
                if self.lang == "rust":
                    m = re.match(r"'(\\.[^']*|[^\\'])'", s[i:])
                    if m:  # a char literal, else a lifetime or a label
                        i += len(m.group(0))
                        out.append("'c'")
                        continue
                    out.append(c)
                    i += 1
                    continue
                self.state, i = ("'",), i + 1
                out.append("'")
                continue
            if c == "`" and self.lang in ("go", "js", "ts"):
                self.state, i = ("`",), i + 1
                out.append("`")
                continue
            if c in OPEN:
                self.depth += 1
            elif c in CLOSE:
                self.depth -= 1
            out.append(c)
            i += 1
        if self.state is not None and self.state[0] in ('"', "'") and self.lang != "rust":
            self.state = None  # a JS / Java / Go string doesn't span lines
        return "".join(out)


CONTINUES = ("&&", "||", ",", "(", "{", "[", ".", "+", "-", "*", "/", "=", "|", "&", ":", "?", "=>", "->")


def units(lines, lang, base):
    """The units of `lines` at bracket depth `base`: [(start, end)], end exclusive. A unit starts on a line of
    code at that depth and ends on one back at it that ends a statement or an item; comments and attributes
    before it are left out (they weigh nothing)."""
    lx = Lexer(lang)
    lx.depth = base
    codes, depths = [], []
    for s in lines:
        before = lx.depth
        codes.append(lx.line(s).strip())
        depths.append((before, lx.depth))
    out, start = [], None
    for i, code in enumerate(codes):
        before, after = depths[i]
        if start is None:
            if not code or before != base or code.startswith(("#[", "#![", "@")) and after == base:
                continue
            start = i
        if after != base or not code:
            continue
        nxt = next((c for c in codes[i + 1:] if c), "")
        if nxt.startswith(("else", "catch", "finally", "while (")) and lang != "go":
            continue  # a `}` before `else`: the statement goes on
        if lang == "go":
            ends = not code.endswith(CONTINUES) or code.endswith(("++", "--"))
        else:
            ends = code.endswith((";", "}")) or lang in ("js", "ts") and not code.endswith(CONTINUES)
        if ends:
            out.append((start, i + 1))
            start = None
    if start is not None:
        out.append((start, len(lines)))
    return out


def units_py(lines):
    """Python's units at indentation 0: a statement with its deeper lines, its open brackets' lines, its
    `elif` / `else` / `except` / `finally` clauses and the decorators before it."""
    lx = Lexer("python")
    starts = []  # (line, is a clause that continues the statement before)
    for i, s in enumerate(lines):
        before, state = lx.depth, lx.state
        code = lx.line(s)
        if before or state or not code.strip() or s[:1].isspace():
            continue
        clause = re.match(r"(elif|else|except|finally|case)\b", code.strip()) is not None
        starts.append((i, clause))
    out, deco = [], None
    for k, (i, clause) in enumerate(starts):
        end = starts[k + 1][0] if k + 1 < len(starts) else len(lines)
        while end > i + 1 and not lines[end - 1].strip():
            end -= 1
        if clause and out:
            out[-1] = (out[-1][0], end)
        elif lines[i].lstrip().startswith("@"):
            deco = i if deco is None else deco
        else:
            out.append((i if deco is None else deco, end))
            deco = None
    return out


def norm(text, lang):
    """The text without whitespace and line comments: what a weight can't see."""
    return re.sub(r"\s+", "", re.sub(r"#[^\n]*" if lang == "python" else r"//[^\n]*", "", text))


IMPORT = re.compile(r"\s*(import|use|package|extern\s+crate)[\s{*]|\s*from\s+\S+\s+import\s")  # not import.meta


def hunks(bad, good, ub, ug):
    """Groups of (bad units, good units) the diff's changes fall into, overlapping changes merged. A change on
    one side only (a statement removed, a line blanked, one added) joins the rewrite next to it: `let d;` dropped
    before `if` that becomes `let d = if`. Imports never join: a changed import is no pair."""
    def unit_of(us, a, b):
        if a == b:  # an insertion point: the unit it is in or right at the end of (`return None` added last)
            return {k for k, (s, e) in enumerate(us) if s < a <= e}
        return {k for k, (s, e) in enumerate(us) if s < b and e > a}

    def merge(adjacent):
        merged = True
        while merged:
            merged = False
            for a in range(len(groups)):
                for b in range(len(groups)):
                    if a != b and adjacent(groups[a], groups[b]):
                        groups[a][0] |= groups[b][0]
                        groups[a][1] |= groups[b][1]
                        del groups[b]
                        merged = True
                        break
                if merged:
                    break

    def is_import(lines, us, side):
        return all(IMPORT.match(lines[us[k][0]]) for k in side)

    groups = []
    sm = difflib.SequenceMatcher(None, bad, good, autojunk=False)
    for op, i1, i2, j1, j2 in sm.get_opcodes():
        if op == "equal":
            continue
        gb, gg = unit_of(ub, i1, i2), unit_of(ug, j1, j2)
        if i1 == i2 and gb and j1:  # added at a unit's end: that unit on the other side too
            gg |= unit_of(ug, j1 - 1, j1)
        if j1 == j2 and gg and i1:
            gb |= unit_of(ub, i1 - 1, i1)
        groups.append([gb, gg])
    merge(lambda x, y: x[0] & y[0] or x[1] & y[1])  # the same unit

    def one_sided(y):
        side = 0 if y[0] and not y[1] else 1 if y[1] and not y[0] else None
        return None if side is None or is_import(*((bad, ub) if side == 0 else (good, ug)), y[side]) else side

    def before(x, y):  # y one-sided, right before x's units on its side
        s = one_sided(y)
        return s is not None and x[0] and x[1] and max(y[s]) == min(x[s]) - 1

    def after(x, y):
        s = one_sided(y)
        return s is not None and x[0] and x[1] and min(y[s]) == max(x[s]) + 1
    merge(before)  # a declaration goes with the statement after it, first
    merge(after)

    def text(lines, us, k):
        return norm("\n".join(lines[us[k][0]:us[k][1]]), "")

    for g in groups:  # still one-sided: the unchanged unit next to it, on both sides, as its context
        s = one_sided(g)
        if s is None:
            continue
        (lines, us), (olines, ous) = ((bad, ub), (good, ug))[::1 if s == 0 else -1]
        n = min(g[s]) - 1 if min(g[s]) > 0 else max(g[s]) + 1
        if n >= len(us):
            continue
        same = [m for m in range(len(ous)) if text(olines, ous, m) == text(lines, us, n)]
        if same:
            g[s].add(n)
            g[1 - s].add(min(same, key=lambda m: abs(m - n)))
    groups = [g for g in groups if g[0] and g[1]
              and not (is_import(bad, ub, g[0]) and is_import(good, ug, g[1]))]
    out = []
    for gb, gg in groups:
        if not gb and not gg:
            continue
        rb = (min(gb), max(gb) + 1) if gb else None
        rg = (min(gg), max(gg) + 1) if gg else None
        out.append((rb, rg))
    return out


FN = {
    "rust": re.compile(r"^\s*(pub(\([^)]*\))?\s+)?((const|async|unsafe|extern\s+\"[^\"]*\")\s+)*fn\s"),
    "go": re.compile(r"^func\s"),
    "java": re.compile(r"^\s*[\w<>\[\],.?\s@]*\b\w+\s*\([^;]*\)\s*(throws [\w.,\s]+)?\{\s*$"),
    "python": re.compile(r"^\s*(async\s+)?def\s"),
}
CONTAINER = {
    "rust": re.compile(r"^\s*(pub(\([^)]*\))?\s+)?(unsafe\s+)?(impl|mod|trait)\b"),
    "java": re.compile(r"^\s*[\w\s@]*\b(class|interface|enum|record)\s"),
    "python": re.compile(r"^\s*class\s"),
}


def body_start(lines, s, e, lang):
    """The index of the line after the one that opens a unit's body, or None."""
    if lang == "python":
        lx = Lexer("python")
        for k in range(s, e):
            code = lx.line(lines[k]).rstrip()
            if not lx.depth and not lx.state and code.endswith(":") and not lines[k].lstrip().startswith("@"):
                return k + 1 if k + 1 < e else None
        return None
    for k in range(s, e):
        code = re.sub(r"//.*", "", lines[k]).rstrip()
        if code.endswith("{"):
            return k + 1
    return None


def pairs(bad_text, good_text, lang, base=0):
    """The pairs of a fixture and its fixed text: [(bad snippet, good snippet, bad line)], line 1-based."""
    bad, good = bad_text.split("\n"), good_text.split("\n")
    if lang == "python":
        ub, ug = units_py(bad), units_py(good)
    else:
        ub, ug = units(bad, lang, base), units(good, lang, base)
    same = [(i + k, j + k) for i, j, n in difflib.SequenceMatcher(None, bad, good, autojunk=False)
            .get_matching_blocks() for k in range(n) if norm(bad[i + k], lang)]
    out = []
    for rb, rg in hunks(bad, good, ub, ug):
        if rb is None or rg is None:
            continue  # a unit only added or only removed: not a rewrite of one thing
        bs, be = ub[rb[0]][0], ub[rb[1] - 1][1]
        gs, ge = ug[rg[0]][0], ug[rg[1] - 1][1]
        if any((bs <= i < be) != (gs <= j < ge) for i, j in same):
            continue  # an unchanged line in one snippet only: the two sides are not the same code
            # (a statement turned into a comment leaves that comment to the statement before)
        btext, gtext = "\n".join(bad[bs:be]), "\n".join(good[gs:ge])
        if norm(btext, lang) == norm(gtext, lang):
            continue
        one = rb[1] - rb[0] == 1 and rg[1] - rg[0] == 1
        head_b, head_g = bad[bs:be], good[gs:ge]
        first = lambda ls: next((l for l in ls if l.strip() and not l.strip().startswith(("#", "@", "//"))), "")
        if one and (FN.get(lang) and FN[lang].match(first(head_b)) and FN[lang].match(first(head_g))
                    or CONTAINER.get(lang) and CONTAINER[lang].match(first(head_b))
                    and CONTAINER[lang].match(first(head_g))):
            kb, kg = body_start(bad, bs, be, lang), body_start(good, gs, ge, lang)
            if lang == "python" and kb and kg:
                ib_lines, ig_lines = bad[kb:be], good[kg:ge]
                ind = lambda ls: min((len(x) - len(x.lstrip()) for x in ls if x.strip()), default=0)
                nb, ng = ind(ib_lines), ind(ig_lines)
                inner = pairs("\n".join(x[nb:] for x in ib_lines), "\n".join(x[ng:] for x in ig_lines), lang)
                if inner:
                    for ib, ig, line in inner:
                        out.append(("\n".join(bad[bs:kb]) + "\n" + "\n".join(" " * nb + x if x else x for x in ib.split("\n")),
                                    "\n".join(good[gs:kg]) + "\n" + "\n".join(" " * ng + x if x else x for x in ig.split("\n")),
                                    kb + line))
                    continue
            elif kb and kg and kb < be and kg < ge:
                inner = pairs("\n".join(bad[kb:be - 1]), "\n".join(good[kg:ge - 1]), lang, 0)
                if inner:
                    for ib, ig, line in inner:
                        out.append(("\n".join(bad[bs:kb]) + "\n" + ib + "\n" + bad[be - 1],
                                    "\n".join(good[gs:kg]) + "\n" + ig + "\n" + good[ge - 1], kb + line))
                    continue
        out.append((btext, gtext, bs + 1))
    return out
