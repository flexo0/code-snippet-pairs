#!/usr/bin/env bash
# Fetches the linters' sources (sparse, at the commits the pairs were made from) into <sources> and makes
# ../gen from them, then checks it. Usage: run.sh <sources folder> [codeweight binary]
# Needs git, python3 (3.12), and a ruff of its own (installed into <sources>/ruff-venv).
set -euo pipefail
src=$(realpath -m "$1")
cw=${2:-codeweight}
export CODEWEIGHT=$cw # the extractors drop the pairs it can't parse cleanly
here=$(dirname "$(realpath "$0")")
gen=$here/../gen
mkdir -p "$src"

fetch() { # <dir> <url> <commit> <sparse patterns>...
	local d=$src/$1
	if [ ! -d "$d" ]; then
		git init -q "$d"
		git -C "$d" remote add origin "$2"
		git -C "$d" config remote.origin.promisor true
		git -C "$d" config remote.origin.partialclonefilter blob:none
		git -C "$d" sparse-checkout set --no-cone "${@:4}"
		git -C "$d" fetch -q --depth 1 --filter=blob:none origin "$3"
		git -C "$d" checkout -q FETCH_HEAD
	fi
}

fetch clippy https://github.com/rust-lang/rust-clippy 23a80940adb1f2f196cd18468ab9bbb254dee58e \
	'/tests/ui/**' '/clippy_lints/src/**'
fetch gotools https://github.com/dominikh/go-tools 8371db75e9daf10073dc2ba684c2a24a1d6ead95 \
	'/simple/**' '/quickfix/**' '/stylecheck/**'
fetch eslint https://github.com/eslint/eslint 48207902fccd37450823b8fd07736d84449ec8e7 \
	'/tests/lib/rules/**' '/lib/rules/**'
fetch tseslint https://github.com/typescript-eslint/typescript-eslint a53584297a1fa65f4cc91387b0a7da834ab73ed7 \
	'/packages/eslint-plugin/tests/rules/**' '/packages/eslint-plugin/src/rules/**'
ruff_patterns=()
for l in flake8_simplify flake8_return pylint perflint flake8_comprehensions refurb; do
	ruff_patterns+=("/crates/ruff_linter/src/rules/$l/mod.rs" "/crates/ruff_linter/resources/test/fixtures/$l/**")
done
fetch ruff https://github.com/astral-sh/ruff 248231e9936ecc0e44ecd3a69bc07899736e4e23 "${ruff_patterns[@]}"
fetch errorprone https://github.com/google/error-prone dab49c372f301d9563237705b9a19b13d8ced06f \
	'/core/src/test/java/com/google/errorprone/bugpatterns/**' '/core/src/main/java/com/google/errorprone/bugpatterns/**'

if [ ! -x "$src/ruff-venv/bin/ruff" ]; then
	python3 -m venv "$src/ruff-venv"
	"$src/ruff-venv/bin/pip" install -q ruff==0.17.5
fi

python3 -I "$here/clippy.py" "$src/clippy" "$gen"
python3 -I "$here/staticcheck.py" "$src/gotools" "$gen"
python3 -I "$here/eslint.py" "$src/eslint" "$src/tseslint" "$gen"
python3 -I "$here/ruff.py" "$src/ruff" "$src/ruff-venv/bin/ruff" "$gen"
python3 -I "$here/errorprone.py" "$src/errorprone" "$gen"
python3 -I "$here/../tune.py" extract "$cw"
python3 -I "$here/../tune.py" dedup # one pair per rule and difference the tuning can tell apart
python3 -I "$here/../check.py" "$cw" "$gen"
