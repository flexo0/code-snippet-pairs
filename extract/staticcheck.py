"""staticcheck's (go-tools) golden tests: each `X.go` beside its `X.go.golden`, split into pairs; the simple (S),
quickfix (QF) and stylecheck (ST) checks. A golden file of `-- <fix> --` sections holds a fixed file per fix, each
its own pairs. Usage: staticcheck.py <go-tools clone> <gen folder>"""

import pathlib
import re
import subprocess
import sys

sys.path.insert(0, str(pathlib.Path(__file__).parent))
import split  # noqa: E402
from common import Out  # noqa: E402

clone, gen = pathlib.Path(sys.argv[1]), sys.argv[2]
commit = subprocess.run(["git", "-C", str(clone), "rev-parse", "--short", "HEAD"],
                        capture_output=True, text=True).stdout.strip()
out = Out(gen, "staticcheck", "go")
for golden in sorted(clone.glob("*/*/testdata/**/*.golden")):
    src = golden.with_name(golden.name.removesuffix(".golden"))
    if not src.exists():
        continue
    group, check = golden.relative_to(clone).parts[:2]
    text = golden.read_text()
    sections = re.split(r"^-- (.*) --$", text, flags=re.M)
    fixes = list(zip(sections[1::2], sections[2::2])) if len(sections) > 1 else [("", text)]
    rel = src.relative_to(clone)
    for k, (title, fixed) in enumerate(fixes):
        for bad, good, line in split.pairs(src.read_text(), fixed.lstrip("\n"), "go"):
            case = f"{src.stem}-L{line}" + (f"-fix{k + 1}" if len(fixes) > 1 else "")
            out.add(check.upper(), group, case, "package p\n\n" + bad, "package p\n\n" + good,
                    f"https://github.com/dominikh/go-tools/blob/{commit}/{rel}#L{line}")
out.close()
