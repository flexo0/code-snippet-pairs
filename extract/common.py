"""What the extractors share: writing a source's pairs and its index."""

import hashlib
import json
import re
import os
import pathlib
import shutil
import subprocess


# The rules whose fix changes the behaviour (a bug mended, a check or a guarantee added): never written.
SEMANTIC = {line.split("\t")[0] for line in
            (pathlib.Path(__file__).parent / "semantic.tsv").read_text().splitlines()[1:]}
# The pairs the splitting gets wrong (a fix cut in two, two sides that aren't the same code): never written.
BROKEN = {line.split("\t")[0] for line in
          (pathlib.Path(__file__).parent / "broken.tsv").read_text().splitlines()[1:]}
# The pairs whose fix rightly weighs no less (a style chosen, code or a check added, the same in another form):
# never written.
HEAVIER = {line.split("\t")[0] for line in
           (pathlib.Path(__file__).parent / "heavier.tsv").read_text().splitlines()[1:]}

COMMENT = re.compile(r"/\*.*?\*/|//[^\n]*|^[ \t]*#(?![\[!])[^\n]*", re.S | re.M)  # not a Rust attribute


def trivial(bad, good):
    """Whether a pair is no rewrite of code: the same once comments and whitespace are gone, or imports only."""
    b, g = (COMMENT.sub("", t) for t in (bad, good))
    if re.sub(r"\s+", "", b) == re.sub(r"\s+", "", g):
        return True
    from split import IMPORT
    return all(IMPORT.match(line) for t in (b, g) for line in t.splitlines() if line.strip())


class Out:
    """A source's folder of generated pairs, `gen/<source>/<rule>/<case>.bad|good.<ext>`, and its `index.tsv`."""

    def __init__(self, root, source, ext):
        self.dir = pathlib.Path(root) / source
        if self.dir.exists():
            shutil.rmtree(self.dir)
        self.dir.mkdir(parents=True)
        self.ext, self.rows, self.seen = ext, [], set()

    def add(self, rule, group, case, bad, good, origin, ext=None):
        """Writes a pair, unless the same one (by text) is already written, it is trivial, its rule is in
        `semantic.tsv` or it is in `broken.tsv` or `heavier.tsv`; returns whether it was."""
        key = hashlib.sha1((bad + "\0" + good).encode()).hexdigest()
        if key in self.seen:
            return False
        self.seen.add(key)  # a pair skipped below keeps its later copies out too
        if (f"{self.dir.name}/{rule}" in SEMANTIC or f"{self.dir.name}/{rule}/{case}" in BROKEN | HEAVIER
                or trivial(bad, good)):
            return False
        ext = ext or self.ext
        d = self.dir / rule
        d.mkdir(exist_ok=True)
        base, k = case, 1
        while (d / f"{case}.bad.{ext}").exists():  # the same name from another fixture (another Go version)
            k += 1
            case = f"{base}-{k}"
        (d / f"{case}.bad.{ext}").write_text(bad.rstrip("\n") + "\n")
        (d / f"{case}.good.{ext}").write_text(good.rstrip("\n") + "\n")
        self.rows.append((rule, group, case, f"{rule}/{case}.bad.{ext}", origin))
        return True

    def unparsed(self):
        """The pairs (their bad file, relative) a side of which doesn't parse cleanly, by codeweight's warnings;
        codeweight is $CODEWEIGHT, else the one on the PATH."""
        cw = os.environ.get("CODEWEIGHT", "codeweight")
        warned = set()
        for only in ("--code", "--tests"):
            r = subprocess.run([cw, "scan", only, "--json", "-v", str(self.dir)], capture_output=True, text=True,
                               check=True)
            warned |= {pathlib.Path(w["path"]).resolve() for w in json.loads(r.stdout)["warnings"] if w["path"]}
        out = set()
        for rule, _g, case, bad, _o in self.rows:
            b = (self.dir / bad).resolve()
            if b in warned or b.with_name(b.name.replace(".bad.", ".good.")) in warned:
                out.add(bad)
        return out

    def close(self):
        """Drops the pairs that don't parse, writes the index."""
        drop = self.unparsed()
        for bad in drop:
            (self.dir / bad).unlink()
            (self.dir / bad.replace(".bad.", ".good.")).unlink()
        for d in self.dir.iterdir():
            if d.is_dir() and not any(d.iterdir()):
                d.rmdir()
        self.rows = [r for r in self.rows if r[3] not in drop]
        with open(self.dir / "index.tsv", "w") as f:
            f.write("rule\tgroup\tcase\tbad\torigin\n")
            for r in self.rows:
                f.write("\t".join(r) + "\n")
        print(f"{self.dir.name}: {len(self.rows)} pairs, {len({r[0] for r in self.rows})} rules, "
              f"{len(drop)} that don't parse dropped")
