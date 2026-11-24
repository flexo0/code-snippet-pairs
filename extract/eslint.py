"""ESLint's and typescript-eslint's rule tests, read without running them: each invalid case with an `output` is
a pair, the code as the test has it. A rule's group is its `meta.type` (problem, suggestion, layout); the layout
rules (formatting) and the cases with a parser of their own are left out.
Usage: eslint.py <eslint clone> <typescript-eslint clone> <gen folder>"""

import pathlib
import re
import subprocess
import sys

sys.path.insert(0, str(pathlib.Path(__file__).parent))
import jscases  # noqa: E402
from common import Out  # noqa: E402

eslint, tseslint, gen = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2]), sys.argv[3]


def commit(clone):
    return subprocess.run(["git", "-C", str(clone), "rev-parse", "--short", "HEAD"],
                          capture_output=True, text=True).stdout.strip()


def meta_type(rule_file):
    m = re.search(r"""\btype:\s*["'](problem|suggestion|layout)["']""", rule_file.read_text()) \
        if rule_file.exists() else None
    return m.group(1) if m else "unknown"


def extract(source, clone, tests, rules, rule_ext, ext, repo):
    out = Out(gen, source, ext)
    sha = commit(clone)
    left = {"layout": 0, "parser": 0}
    for test in sorted((clone / tests).rglob(f"*.test.{ext}" if ext == "ts" else "*.js")):
        stem = test.name.split(".")[0]
        rule = test.parent.name if test.parent != clone / tests else stem  # a rule's folder of test files
        group = meta_type(clone / rules / f"{rule}.{rule_ext}")
        if group == "layout":
            left["layout"] += 1
            continue
        src = test.read_text()
        rel = test.relative_to(clone)
        for k, c in enumerate(jscases.cases(src)):
            if c["parser"]:
                left["parser"] += 1
                continue
            out.add(rule, group, f"{stem}-{k + 1:03d}", c["code"], c["output"],
                    f"https://github.com/{repo}/blob/{sha}/{rel}")
    out.close()
    print(f"{source}: left out {left['layout']} layout rules, {left['parser']} cases with a parser of their own")


extract("eslint", eslint, "tests/lib/rules", "lib/rules", "js", "js", "eslint/eslint")
extract("typescript-eslint", tseslint, "packages/eslint-plugin/tests/rules", "packages/eslint-plugin/src/rules",
        "ts", "ts", "typescript-eslint/typescript-eslint")
