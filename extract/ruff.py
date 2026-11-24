"""Ruff's fixtures, fixed by Ruff itself: each linter's test table (`mod.rs`) names a rule and its fixture; the
fixture fixed with that rule alone (`--fix --unsafe-fixes --preview`) is the good side, split into pairs. The
structural linters only: flake8-simplify, flake8-return, Pylint's refactor rules (PLR), perflint,
flake8-comprehensions and refurb. Usage: ruff.py <ruff clone> <ruff binary> <gen folder>"""

import json
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile

sys.path.insert(0, str(pathlib.Path(__file__).parent))
import split  # noqa: E402
from common import Out  # noqa: E402

LINTERS = {"flake8_simplify": None, "flake8_return": None, "pylint": "PLR", "perflint": None,
           "flake8_comprehensions": None, "refurb": None}
clone, ruff, gen = pathlib.Path(sys.argv[1]), sys.argv[2], sys.argv[3]
sha = subprocess.run(["git", "-C", str(clone), "rev-parse", "--short", "HEAD"],
                     capture_output=True, text=True).stdout.strip()
rules = {r["name"]: r for r in json.loads(subprocess.run([ruff, "rule", "--all", "--output-format", "json"],
                                                         capture_output=True, text=True, check=True).stdout)}
kebab = lambda name: re.sub(r"(?<!^)(?=[A-Z])", "-", name).lower()

out = Out(gen, "ruff", "py")
missing, unfixed = set(), set()
fixtures = clone / "crates" / "ruff_linter" / "resources" / "test" / "fixtures"
with tempfile.TemporaryDirectory() as tmp:
    for linter, prefix in LINTERS.items():
        table = (clone / "crates" / "ruff_linter" / "src" / "rules" / linter / "mod.rs").read_text()
        for name, path in sorted(set(re.findall(r'test_case\(Rule::(\w+),\s*Path::new\("([^"]+)"\)', table))):
            rule = rules.get(kebab(name))
            src = fixtures / linter / path
            if rule is None or not src.exists() or not path.endswith(".py"):
                missing.add(name)
                continue
            code = rule["code"]
            if prefix and not code.startswith(prefix):
                continue
            work = pathlib.Path(tmp) / f"{code}-{src.name}"
            shutil.copy(src, work)
            subprocess.run([ruff, "check", "--isolated", "--no-cache", "--preview", "--unsafe-fixes", "--fix",
                            "--exit-zero", "--quiet", "--select", code, str(work)], capture_output=True)
            bad, good = src.read_text(), work.read_text()
            if bad == good:
                unfixed.add(code)
                continue
            rel = src.relative_to(clone)
            for b, g, line in split.pairs(bad, good, "python"):
                out.add(f"{code}-{rule['name']}", linter, f"{src.stem}-L{line}", b, g,
                        f"https://github.com/astral-sh/ruff/blob/{sha}/{rel}#L{line}")
out.close()
print(f"ruff: {len(unfixed)} rules with no fix in their fixture, {len(missing)} test cases not found")
