# code-snippet-pairs

Pairs of snippets that codeweight's default weights are tuned on: a bad snippet and its good counterpart, with
the same behaviour, the good one simpler. It should weigh less. `tune.py` does the tuning. The code taken from other
projects stays under their licenses: [THIRD-PARTY.md](THIRD-PARTY.md). The rest is MIT: [LICENSE](LICENSE).

## The layout

| path | what |
|------|------|
| `hand/<lang>/` | the 25 hand-picked pairs, from the linters' own examples (below) |
| `gen/<source>/<rule>/` | the generated pairs, from the linters' and fixers' tests ([below](#the-generated-pairs-gen)); never edited by hand |
| `mine/<set>/` | the pairs from our own projects' commits, kept out of git: their code isn't ours to publish ([below](#the-mined-pairs-mine)) |
| `mine/MINING.md` | how a `mine/` set is made: the steps and the subagents' brief |
| `mine/merge.py` | merges the subagents' index files into a set's `index.tsv` and checks the pairs |
| `mine/anonymize.py` | renames a set's names to useless ones (`Class1`, `method1`, `var1`, `CONST1`), its strings to `str1`, drops its comments; `mine/anonymize-keep.txt` lists the JDK and library names it leaves |
| `extract/` | the extractors that make `gen/` (`run.sh`), and the lists they skip: `semantic.tsv`, `broken.tsv`, `heavier.tsv` |
| `check.py` | weighs a folder's pairs under the defaults, a verdict per pair |
| `tune.py` | tunes the defaults on every pair; `tune-counts.json` is its counts (`tune.py extract`) |
| `calib/<lang>/` | the docs weight's calibration: a snippet beside its descriptions of equal cognitive volume, `<name>.<ext>`, `<name>.md` in prose and `<name>.dense.md` a terse spec ([below](#the-docs-weights-calibration-calib-2026-11-24)) |
| `calib.py` | fits the `[docs]` weights to `calib/`: each description to weigh what its snippet does |
| `THIRD-PARTY.md`, `licenses/` | the projects the snippets are taken from, their licenses and copyrights; the licenses' texts |

A pair is `<name>.bad.<ext>` beside `<name>.good.<ext>`. `check.py` and `tune.py` find every one under the folder.
A pair under `gen/` is generated and its rule is its folder. Any other pair is hand-picked: a rule of its own, at
three times a generated pair's weight in the tuning, in `hand/` and `mine/` alike.

## The hand-picked pairs (`hand/`)

The first pairs: 31 of them, 25 since the six Java pairs from OpenRewrite went (2026-11-24, below). They were
taken on 2026-11-19 from the linters' own pairs, a rule's example and its fix. Each one keeps the source's code.
Statements are wrapped in a function where the language needs one (Rust, Go), Clippy's hidden `# ` lines are shown,
staticcheck's placeholders are made concrete, and ESLint's one-line test strings are laid out on several lines.

`python3 check.py [codeweight] [folder]` weighs each pair under the defaults and per category (a run per category,
its weight 1, the others 0). A count is a sum of `nest(depth)`, not a plain count. Each pair gets a verdict:

- `ok`: the good snippet weighs less.
- `broken`: it doesn't, but the difference has a positive component, so other weights could fix it.
- `no-positive`: the good one has as much of every category as the bad one, or more. No weights can fix it.
- `invisible`: no category count differs. It's a change the model doesn't see (a name, an API, a literal).

A pair one side of which doesn't parse cleanly is an error: the check leaves it out and fails, naming it.

It writes `results.tsv` (a line per pair, with its difference by category) and `rules.tsv` (the verdicts per rule)
into the folder. It prints a summary, and each pair when there are few. It leaves `gen/` out: run it on `gen/` by
itself. `hand/results.tsv` and `hand/rules.tsv` are from `check.py <codeweight> hand`.

There are no Java pairs: the six there were, from OpenRewrite's rewrite-static-analysis, are under the Moderne Source
Available License, a copy for internal use only, so they went on 2026-11-24.

| pair | source | why the good one is better |
|------|--------|----------------------------|
| [python/needless_bool](hand/python/needless_bool.bad.py) | [Ruff SIM103](https://github.com/astral-sh/ruff/blob/main/crates/ruff_linter/src/rules/flake8_simplify/rules/needless_bool.rs) | a branch returning True / False is the condition itself |
| [python/collapsible_else_if](hand/python/collapsible_else_if.bad.py) | [Ruff PLR5501](https://github.com/astral-sh/ruff/blob/main/crates/ruff_linter/src/rules/pylint/rules/collapsible_else_if.rs) | an `if` alone in an `else` is an `elif`: one level less |
| [python/reimplemented_builtin](hand/python/reimplemented_builtin.bad.py) | [Ruff SIM110](https://github.com/astral-sh/ruff/blob/main/crates/ruff_linter/src/rules/flake8_simplify/rules/reimplemented_builtin.rs) | a search loop with an early return is `any` |
| [python/manual_list_comprehension](hand/python/manual_list_comprehension.bad.py) | [Ruff PERF401](https://github.com/astral-sh/ruff/blob/main/crates/ruff_linter/src/rules/perflint/rules/manual_list_comprehension.rs) | a loop that appends under a condition is a comprehension |
| [python/dict_get](hand/python/dict_get.bad.py) | [Ruff SIM401](https://github.com/astral-sh/ruff/blob/main/crates/ruff_linter/src/rules/flake8_simplify/rules/if_else_block_instead_of_dict_get.rs) | a membership branch around a lookup is `dict.get` with a default |
| [python/too_many_return_statements](hand/python/too_many_return_statements.bad.py) | [Ruff PLR0911](https://github.com/astral-sh/ruff/blob/main/crates/ruff_linter/src/rules/pylint/rules/too_many_return_statements.rs) | an `elif` chain mapping values is a dict: data, not branches |
| [python/enumerate_for_loop](hand/python/enumerate_for_loop.bad.py) | [Ruff SIM113](https://github.com/astral-sh/ruff/blob/main/crates/ruff_linter/src/rules/flake8_simplify/rules/enumerate_for_loop.rs) | a hand-kept counter is `enumerate` |
| [rust/collapsible_if](hand/rust/collapsible_if.bad.rs) | [Clippy collapsible_if (style)](https://github.com/rust-lang/rust-clippy/blob/main/clippy_lints/src/collapsible_if.rs) | an `if` alone in an `if` is one `if` with `&&` |
| [rust/explicit_counter_loop](hand/rust/explicit_counter_loop.bad.rs) | [Clippy explicit_counter_loop (complexity)](https://github.com/rust-lang/rust-clippy/blob/main/clippy_lints/src/loops/mod.rs) | a hand-kept counter is `enumerate` |
| [rust/manual_find](hand/rust/manual_find.bad.rs) | [Clippy manual_find (complexity)](https://github.com/rust-lang/rust-clippy/blob/main/clippy_lints/src/loops/mod.rs) | a search loop with an early return is `find` |
| [rust/manual_flatten](hand/rust/manual_flatten.bad.rs) | [Clippy manual_flatten (complexity)](https://github.com/rust-lang/rust-clippy/blob/main/clippy_lints/src/loops/mod.rs) | an `if let Some` wrapping a loop's body is `flatten` |
| [rust/while_let_loop](hand/rust/while_let_loop.bad.rs) | [Clippy while_let_loop (complexity)](https://github.com/rust-lang/rust-clippy/blob/main/clippy_lints/src/loops/mod.rs) | a `loop` that breaks on a failed match is `while let` |
| [rust/manual_filter](hand/rust/manual_filter.bad.rs) | [Clippy manual_filter (complexity)](https://github.com/rust-lang/rust-clippy/blob/main/clippy_lints/src/matches/mod.rs) | a `match` with an `if` inside is `Option::filter`: a closure for two branches |
| [rust/manual_unwrap_or](hand/rust/manual_unwrap_or.bad.rs) | [Clippy manual_unwrap_or (complexity)](https://github.com/rust-lang/rust-clippy/blob/main/clippy_lints/src/matches/mod.rs) | a `match` giving a default is `unwrap_or` |
| [go/s1031_nil_check_around_loop](hand/go/s1031_nil_check_around_loop.bad.go) | [staticcheck S1031](https://github.com/dominikh/go-tools/blob/master/simple/s1031/s1031.go) | ranging over a nil slice is a no-op: the guard is dead weight |
| [go/s1001_loop_copy](hand/go/s1001_loop_copy.bad.go) | [staticcheck S1001](https://github.com/dominikh/go-tools/blob/master/simple/s1001/s1001.go) | an element-by-element loop is `copy` |
| [go/s1008_bool_return](hand/go/s1008_bool_return.bad.go) | [staticcheck S1008](https://github.com/dominikh/go-tools/blob/master/simple/s1008/s1008.go) | a branch returning true / false is the condition itself |
| [go/s1011_loop_append](hand/go/s1011_loop_append.bad.go) | [staticcheck S1011](https://github.com/dominikh/go-tools/blob/master/simple/s1011/s1011.go) | appending one by one is one `append` with `...` |
| [go/s1017_trim_prefix](hand/go/s1017_trim_prefix.bad.go) | [staticcheck S1017](https://github.com/dominikh/go-tools/blob/master/simple/s1017/s1017.go) | a `HasPrefix` branch around a slice is `TrimPrefix` |
| [go/s1000_single_case_select](hand/go/s1000_single_case_select.bad.go) | [staticcheck S1000](https://github.com/dominikh/go-tools/blob/master/simple/s1000/s1000.go) | a one-case `select` is a plain receive |
| [js/no_else_return](hand/js/no_else_return.bad.js) | [ESLint no-else-return, foo2](https://github.com/eslint/eslint/blob/main/tests/lib/rules/no-else-return.js) | an `else` after a `return` is the rest of the function |
| [js/no_lonely_if](hand/js/no_lonely_if.bad.js) | [ESLint no-lonely-if](https://github.com/eslint/eslint/blob/main/tests/lib/rules/no-lonely-if.js) | an `if` alone in an `else` is an `else if` |
| [js/no_unneeded_ternary](hand/js/no_unneeded_ternary.bad.js) | [ESLint no-unneeded-ternary](https://github.com/eslint/eslint/blob/main/tests/lib/rules/no-unneeded-ternary.js) | `? true : false` is the condition itself |
| [js/no_useless_return](hand/js/no_useless_return.bad.js) | [ESLint no-useless-return](https://github.com/eslint/eslint/blob/main/tests/lib/rules/no-useless-return.js) | a `return;` at a function's end does nothing |
| [ts/prefer_optional_chain](hand/ts/prefer_optional_chain.bad.ts) | [typescript-eslint prefer-optional-chain](https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/eslint-plugin/docs/rules/prefer-optional-chain.mdx) | a chain of `&&` guards is `?.` |

### The first check (2026-11-19, the defaults of `02c46f2`)

30 pairs hold, with no pair lacking a positive component. One is broken by the weights:

- `java/nested_ternary_to_switch`: 14.10 against 21.10. The `switch` expression's three arms count as statements
  (6 by nesting), and its cases as one branch more than the two ternaries. A nested ternary weighs less than the
  `switch` it becomes.

This set was picked from the structural rules (simplify, complexity, return, nesting), so it says little about
point 2: each of these fixes removes a branch, a loop, a jump or a statement. The pairs with no positive component
(extract a method, rename, an idiom that moves no count) are still to be added from the same sources.

## The generated pairs (`gen/`)

The linters' and fixers' tests, a source per folder: `gen/<source>/<rule>/<case>.bad|good.<ext>`. Each source has
an `index.tsv` (the rule, its group, the case, its origin as a link at the commit used). The pairs are generated,
never edited by hand. `extract/run.sh <sources folder> [codeweight]` fetches the sources (sparse clones at the
pinned commits), installs Ruff into a venv there, makes `gen/` again and checks it.

| source | where the pairs come from | how | groups kept |
|--------|---------------------------|-----|-------------|
| `clippy` | `tests/ui/*.rs` beside `*.fixed` | split | `complexity`, `style` |
| `staticcheck` | go-tools' `testdata/**/*.go` beside `*.golden`, a fix per `-- name --` section | split | `simple` (S), `quickfix` (QF), `stylecheck` (ST) |
| `eslint` | `RuleTester` invalid cases with an `output` | as is | every `meta.type` but `layout` |
| `typescript-eslint` | the same, `packages/eslint-plugin/tests/rules` | as is | the same |
| `ruff` | each rule's fixture (from its linter's `mod.rs`), fixed by `ruff --select <rule> --fix --unsafe-fixes --preview` | split | flake8-simplify, flake8-return, Pylint's PLR, perflint, flake8-comprehensions, refurb |
| `errorprone` | refactoring tests, `addInputLines` / `addOutputLines` | split | every checker, grouped by its `@BugPattern` tags, else its severity |

A **split** source has a whole file before and after, with every fix of the rule applied at once. `extract/split.py`
diffs the two and widens each change to the units that hold it: the top-level items, then a function's statements
(by bracket depth, and by indentation in Python). The pair is those units, wrapped in their function's header. So a
`fn main` with 30 cases gives 30 pairs. A change on one side only (a statement removed, a line blanked, one added)
joins the rewrite right after it, else the one right before (`let d;` dropped before the `if` that becomes
`let d = if`). With no rewrite next to it, it takes the unchanged unit beside it as context on both sides (a
`return None` added after the last statement). The ESLint and typescript-eslint cases are read from the test files as text, never run:
only the `code` and `output` that are plain strings, and no case with a parser of its own. Any pair whose text is
the same as one already written is dropped in every source. So is a pair that is the same once comments and
whitespace are gone, or one of imports only, and a pair one side of which codeweight can't parse cleanly
(`$CODEWEIGHT`, else the one on the PATH).

What isn't here:
- The rules of `extract/semantic.tsv`, whose fix changes the behaviour (see the tuning below).
- OpenRewrite's recipes, because of their license (above).
- Ruff's other linters (pyupgrade and the style ones).
- Clippy's `pedantic`, `perf`, `correctness` and the other groups.
- Error Prone's `error` checks are kept but are no rewrites: their fix mends a bug.

### The first check of `gen/` (2026-11-19, the defaults of `02c46f2`)

8,940 pairs, after 72 that don't parse were dropped:

| | ok | broken | no-positive | invisible |
|---|---:|---:|---:|---:|
| clippy | 1,865 | 92 | 229 | 1,075 |
| staticcheck | 109 | 8 | 5 | 40 |
| eslint | 763 | 65 | 44 | 907 |
| typescript-eslint | 508 | 44 | 17 | 976 |
| ruff | 705 | 26 | 96 | 177 |
| errorprone | 442 | 76 | 175 | 496 |
| all | 4,392 | 311 | 566 | 3,671 |

- **Invisible, 41%.** This is the bulk of the linters' fixes: `yoda`, `curly`, `no-useless-rename`, type
  assertions, `consistent-type-imports`, casts. They aren't about complexity, and the model is right not to see
  them. Leaving these out, 83% of the pairs hold.
- **The simplify linters fit best**, as a share of the pairs that are ok, invisible ones included:
  flake8-comprehensions 83%, flake8-simplify 79%, staticcheck's `simple` 77%, Clippy's `complexity` 66%.
  ESLint is at 43% and typescript-eslint at 36%, because most of their fixes are invisible.
- **Broken, 311 pairs.** The weights' choices show:
  - A `switch` weighs more than the if-chain or ternary it replaces: Error Prone's `IfChainToSwitch`, `RefactorSwitch`.
  - A call weighs more than the operators it replaces: Clippy's `manual_div_ceil` (`(x + 7) / 8` → `x.div_ceil(8)`),
    `partialeq_to_none` (`x != None` → `x.is_some()`), `manual_is_multiple_of`.
  - A `function` and a `lambda` tie: ESLint's `object-shorthand`, `{f: function() {}}` → `{f() {}}`.
- **No-positive, 566 pairs.** The good side adds something without dropping anything: a call for a macro
  (Clippy's `redundant_pattern_matching`, 90 of them), a statement for a declarator (ESLint's `one-var`), a
  `return None` (Ruff's `RET503`), a field for a literal (`math.pi`, refurb's `FURB152`). These are for
  the report, not the solver.

`gen/rules.tsv` lists every rule's verdicts, the most broken first; `gen/results.tsv` lists every pair with its
difference by category.

## The mined pairs (`mine/`)

The pairs mined from the commits of our own projects, a set per folder (`mine/<set>/`), the pure refactors read by
hand, before the commit as bad, after as good (`mine/MINING.md` says how). Their code is a company's, so the sets are
kept out of git (`.gitignore`), even anonymized by `mine/anonymize.py` (every name of the project's `Class1`,
`method1`, `var1`, `CONST1`, every string `str1`, no comments; the JDK's and the libraries' names stay). Only the
scripts that make them are here.

One set so far, `parsed_from_projects/` (2026-11-20): 66 Java pairs from 1,081 commits of one service. Under the
defaults of `df92986`, 57 of them hold, 4 are broken and 5 have no positive component, most of them an extraction
that weighs its new method. `tune.py` finds a set's pairs like the hand-picked ones (each its own rule, 3 times a
generated pair's weight); none are in `tune-counts.json`, so the defaults were tuned without them, and a clone tunes
the same.

## The tuning (`tune.py`, 2026-11-19)

`tune.py` tunes the defaults on every pair here, `hand/`, `mine/` and `gen/`, with a simple evolutionary algorithm. Its score, lower
being better, was first the objective 3 × (pairs whose bad snippet weighs less) + 1 × (pairs that tie); now
each pair in it is weighed by its rule, plus the distance from the defaults (below):

- `python3 tune.py extract [codeweight] [folder]` runs codeweight once per count and writes each pair's difference
  (bad minus good) into `tune-counts.json`: a run per category and depth cap (`max_depth = m`, linear, factor 1,
  adds the nodes at depth m or deeper), per parameter cap (`step = 1`, `max = m`), and per flat category under each
  of the 16 sets of `[conditions]`, for each pair's difference and its good side. A candidate's score is then exact,
  with no run of codeweight.
- `python3 tune.py search [--seed N] [--gens N] [--workers N] [--holdout F | --folds K --fold I | --test-source S]
  [--delta D] [--tau T] [--alpha A] [--lam L] [--unweighted] [--pin GENE=VALUE]... [--out FILE]` searches: (40 + 80)
  over 3000 generations, a uniform crossover and a mutation that shakes every real value at once, x to
  x × U(0.9, 1.1) + U(-0.1, 0.1), the shake falling to a hundredth of that by the last generation, and now and then
  changes one discrete gene (the model, `max_depth`, a condition); no grid but 0.001, `statement` fixed at 1; a difference up to 0.01 is a tie. `--objective diff` scores by the differences instead
  (`--wrong`, `--cap`, `--raw`, `--power`), `hinge` and `logistic` by a margin (`--margin`), `--tiebreak` adds the
  differences to the order score (below). At the
  end each gene moves back toward its default (the whole way, else a half, a quarter, a tenth) while the score gets
  no worse. A generation is scored on every core (`--workers`): about 30 s a search. `--holdout 0.3` fits on 70% of the rules (not of the
  pairs: a rule's pairs are near copies) and scores the rest, `--folds 5 --fold 2` holds out a fifth of them,
  `--test-source clippy` a source; `--pin factor=1` keeps a gene. Its reports give worse, same, better, `fragile`
  (in the right order by under `delta`, ln(bad / good) < 0.05) and the loss, per source.
- `python3 tune.py verify FILE|- [codeweight] [folder]` gives a weights file's (`-`: the defaults) counts and loss by
  codeweight's own totals; run it outside a repo with a `codeweight.toml` of its own, which would be found first
  (codeweight's own repo has none).

Its result became the defaults: 6077 → 4890
(worse 764 → 561, ties 3785 → 3207; 3172 pairs tie under any weights).

| source | pairs | worse | ties | ok | objective before → after |
|---|---:|---:|---:|---:|---:|
| clippy | 3,261 | 156 | 875 | 2,230 | 1950 → 1343 |
| errorprone | 1,189 | 206 | 444 | 539 | 1239 → 1062 |
| eslint | 1,779 | 65 | 873 | 841 | 1132 → 1068 |
| hand-picked | 31 | 1 | 0 | 30 | 3 → 3 |
| ruff | 1,004 | 89 | 152 | 763 | 533 → 419 |
| staticcheck | 162 | 4 | 33 | 125 | 71 → 45 |
| typescript-eslint | 1,545 | 40 | 830 | 675 | 1149 → 950 |

The nesting factor 1.5 is the corpora's choice: on the pairs every factor from 1.05 to 1.5 scores the same.
`hand/results.tsv` and `hand/rules.tsv` are still those of the checks above, under the former defaults.

Then `macro`, a Rust macro apart from `call`: 1.15 on every seed, 4890 → 4837.

`extract/semantic.tsv` lists the rules whose fix changes the behaviour (a bug mended, a check or a guarantee added): 94
rules. Their 560 pairs were deleted from `gen/` on 2026-11-19, 8,380 left, and the extractors skip
these rules (`extract/common.py`), so a new `gen/` has none of them. On the 8,411 pairs left, the defaults
then score 4124. The tables above are of the checks before, with those pairs.

The pairs the splitting got wrong were dropped too, on 2026-11-19: 8,391 pairs left, 406 of them worse and 2,855
the same, objective 4073. `extract/split.py` now drops a pair in which an unchanged line is on one side only, the two
sides not the same code (a `return None` turned into a comment had the comment go with the statement before it):
34 pairs, 32 of them now written as the whole function, both sides right. `extract/broken.tsv` lists the 18 it can't tell,
skipped by name (`extract/common.py`): a fix cut in two (a `result = []` removed apart from its loop, a declaration
hoisted out of a `switch`, code moved shown as removed in one pair and added in another) and two pairs of different
statements; four more, whose good side holds the fixes of the statements after it too, were found among the worse.

Of the 406 pairs then worse, the 292 whose fix rightly weighs no less were deleted too, on 2026-11-19: `extract/heavier.tsv`
lists them with the reason, skipped by name like `extract/broken.tsv` (a rule's option for the longer form: ESLint's
`one-var`, `object-shorthand`, `operator-assignment`; code or a check added: `RET503`, `new_without_default`,
`ConstantPatternCompile`, `unit_arg`; the same thing in another form, no simpler: `type` and `interface`,
`enumerate` and `range(len(...))`, one API for another). The 110 left worse are the fixes that make the code simpler
and should weigh less: a loop for a comprehension or a call (`FURB122`, `FURB142`, `PERF401`), an assignment out of
the cases (`RefactorSwitch`), `matches!`, `match` or `then_some` for a call or an `if` (`manual_map`, `single_match`,
`obfuscated_if_else`), `x === false` for `!x`. 8,095 pairs, 2,855 the same, objective 3185. A pair skipped by name
still keeps its later copies out (a copy of it under another case name isn't written).

Then `gen/` was thinned to what the tuning can tell apart (`tune.py dedup`, a step of `extract/run.sh` after
`tune.py extract`), on 2026-11-19. A pair whose difference is nothing under any weights, the two sides the same
elements (`yoda`'s `1 == x` against `x == 1`, `curly`'s braces), is deleted: 2,815, each a tie that only added a
constant. Of a rule's pairs with the same difference (the same counts of each category at each depth, parameter
place and condition) only the shortest is kept: a linter's tests try one rewrite on many variants
(`prefer-optional-chain` 325 pairs, 53 differences; `no-extra-boolean-cast` 297, 12), and each counted again, so a
rule weighed by how many tests it has. 6,641 deleted, 1,454 left (1,423 generated, the 31 hand-picked kept): 41
worse, 13 the same, objective 136; with one more broken pair (`FURB171_0-L57`) gone, 1,453: 40 worse, objective
133. Then `assignments`, constructors, the `constructor` category and every operator counted (no `each_unary`): 123, 36 worse. A new category or condition could tell apart pairs merged now: `run.sh` without
the `dedup` step brings them back.

### The score by rule (2026-11-19)

The objective counted every pair alike, so a rule with fifty tests weighed fifty times one with a single test
(`prefer-optional-chain` has 52 pairs, most of the 422 rules one or two), and it saw only which side wins: an order
held by 0.05 counted as one held by 50. The score now:

- a pair's loss is 3 when its bad side weighs less, 1 on a tie, plus `alpha` × a soft margin on r = ln((bad + 1) /
  (good + 1)), 1 / (1 + exp((r − delta) / tau)): ~0.7 at a tie, 0.5 at r = delta, ~0 past it;
- a pair weighs 1 / sqrt(its rule's pairs), a hand-picked one 3 times that;
- plus `lam` × the distance from the defaults, so a weight no pair asks for stays where it is.

Under the defaults r has a median of 0.30, a tenth of the pairs under 0.05. Each setting below was fitted on four
fifths of the rules and scored on the rest, five times over (seed 1; every pair held out once):

| setting | worse | same | fragile | 3 × worse + same |
|---|---:|---:|---:|---:|
| the objective before (`--unweighted --alpha 0 --lam 0.01`) | 46 | 10 | 80 | 148 |
| by rule, alpha 0, lam 0.3 | 44 | 11 | 82 | 143 |
| by rule, alpha 0, lam 1 | 39 | 15 | 95 | 132 |
| by rule, alpha 0.25, lam 0.3 | 50 | 6 | 51 | 156 |
| by rule, alpha 0.5, lam 0.3 | 48 | 7 | 31 | 151 |
| by rule, alpha 0.5, delta = tau = 0.02 | 49 | 7 | 8 | 154 |
| by rule, alpha 1, lam 0.3 | 56 | 6 | 22 | 174 |

With seed 2 (other folds) the objective before and the one by rule tie (162), alpha 0.25 is again worse (183). The
margin does what it was for, few pairs left near a tie, but puts more in the wrong order on the rules held out: it
is off by default (`alpha = 0`), `lam = 1`. The defaults saw every pair, held out or not, so a stronger pull toward
them is favoured here more than it should be; the comparisons at one `lam` hold.

On all pairs, three seeds of that score agree: `macro` 1.15 → 1.75, `case` 2 → 2.4, `lambda` 2 → 1.65, `jump`
1.55 → 1.35, the rest as it was (with `lam = 0.3` the nesting goes exponential, 2.05-2.45, and is not taken).
Worse 36 → 32, the same 15 → 12, loss 47.24.

Then a row of accesses: an access weighs `access × min(1 + step × place, max)` by its place in a
row (`a.b.c`: `.b` 0, `.c` 1), as asked 0.5, 1, then 1.5. `extract` counts the accesses by place (a run per cap,
every other run flat) and `search` tunes `[accesses] step` and `max`. Worse 32 → 39, the same 12 → 13: a lone index
or slice at 0.5 is now below the call that replaces it (`l[:]` and `l.copy()`); no pair moves with `step` or `max`.

### A search without a grid (2026-11-19)

The search before (a grid of 0.05, a mutation of one or two genes, 200 generations, 16,000 candidates) left most
weights where they were. It now shakes every value at once, ever more gently, over 3000 generations (240,000
candidates, 30 s on 24 cores). On all pairs it finds far more: worse 39 → 33, the same 13 → 5 (seeds 1-3 agree to
0.005; seed 4 goes exponential, 2.0, worse 31), mostly by `access` 0.5 → 1.1 and `argument` 0.05 → 0.28.

On five folds of the rules (seed 1), against the old search on the same folds, from the same defaults:

| search                          | train worse | same | held out worse | same | loss  |
|---------------------------------|-------------|------|----------------|------|-------|
| the old one                     | 128         | 44   | 42             | 10   | 58.42 |
| shaken                          | 124         | 26   | 48             | 6    | 65.64 |
| shaken, `access` pinned at 0.5  | 137         | 26   | 42             | 6    | 59.91 |
| shaken, `lam = 3`               | 135         | 38   | 41             | 8    | 58.46 |

The better fit doesn't carry over to the rules held out: what the free search finds is mostly the pairs' own noise.
On all pairs, `access` pinned at 0.5 gives worse 35, the same 5; `lam = 3` worse 34, the same 12.

Free of the grid, the search splits ties by hairs: `bool_op` 0.548 against `operator` 0.549, `access` 0.5 against
`call` 0.495, wins of 0.001 or 0.005 that the reports, to 0.01, show as ties (`verify` found 14 where the search
counted 5). A candidate's difference up to 0.01 is now a tie (`EPS`): a win has to show. `verify` ties two totals
only when they show the same (`SHOWN`). Refit so, `access` pinned at 0.5, into the defaults: `call`
0.49, `macro` 1.6, `constructor` 0.949, `argument` 0.316, `bool_op` 0.544, `operator` 0.556, the rest as it was;
`verify`: worse 35, the same 5, loss 53.28 (from 39, 13).

### A score by the differences (2026-11-19)

`--objective diff`: the pairs' differences summed (each times its pair's weight, as above), maximized, a pair in the
wrong order times 20 (`--wrong`). Each with `access` pinned at 0.5, no pull toward the defaults (`lam = 0`):

| difference                              | worse | same | fragile | what it does                                   |
|-----------------------------------------|-------|------|---------|------------------------------------------------|
| the order score (the defaults)          | 35    | 5    | 92      |                                                |
| bad - good (`--raw`)                    | 70    | 16   | 47      | most weights at the bound, 8                   |
| (bad - good) / (bad + good)             | 46    | 30   | 53      | `branch`, `loop`, `lambda`, `jump` at 8, `function` 0 |
| the same, a right pair's at most 0.05   | 50    | 22   | 72      |                                                |
| the same, at most 0.1                   | 42    | 30   | 44      |                                                |
| the same, at most 0.2                   | 40    | 22   | 37      | `constructor` 6.7, `catch` 5.9, nesting exponential 2 |

A sum of differences grows without end as the weights grow apart: the search pushes them to the bounds, a pair
already clear made clearer outweighing one made wrong. A cap on a right pair's difference holds them in, but they
still move far (`branch` 3 → 2-2.8, `loop` 3 → 3.6-6.6), and the 20 makes a tie cheap against a pair a hair wrong:
the ties go up (`call` and `access` both 0.5 tie `l[:]` with `l.copy()`). More pairs in the right order by a margin
(fragile 92 → 37), but more in the wrong one; not taken.

A right pair's difference to the power 0.5 (`--power 0.5`, its root), a wrong one's times 100 (`--wrong 100`):

| difference                              | worse | same | fragile | what it does                                   |
|-----------------------------------------|-------|------|---------|------------------------------------------------|
| (bad - good) / (bad + good)             | 40    | 20   | 53      | `constructor` and `catch` at 8, `loop` 6.4, nesting exponential 2 |
| bad - good (`--raw`)                    | 45    | 26   | 67      | `constructor` and `class` at 8, `lambda` 0.85  |
| relative, `access` free                 | 37    | 15   | 34      | `access` 6.7, `call` 2.9, `branch` 7.3, `loop` 8, nesting 3 |

The root holds the weights in no better than the cap: still a sum that grows as they grow apart. With a wrong pair
100 times dearer, a tie costs nothing against two pairs that ask opposite things: the search ties them (`bool_op`
2.331 and `branch` 2.332, `case` 2.322). Not taken.

### Other scores, held out (2026-11-19)

Each fitted on all pairs and on five folds of the rules (1500 generations, seed 1, `access` pinned at 0.5), r a
pair's relative difference, (bad - good) / (bad + good):

- `order`: the score above (`lam = 1`).
- `--tiebreak B --cap 0.2`: the order score less B times the pairs' r (each at most 0.2): among candidates alike in
  order, the pairs clearer apart.
- `hinge --margin M`: max(0, M - r) / M per pair: a pair r of M or more past costs nothing, a tie 1, a wrong one more
  the more wrong; nothing for spreading the weights apart, as the sums above had.
- `logistic --margin M`: log2(1 + exp(-r / M)), the hinge made smooth.

| score                               | all worse | same | fragile | train worse | same | held out worse | same |
|-------------------------------------|-----------|------|---------|-------------|------|----------------|------|
| `order`, `lam = 1`                  | 35        | 5    | 92      | 138         | 26   | **39**         | 6    |
| tie-break 0.001, `lam = 1`          | 35        | 5    | 92      | 137         | 28   | 40             | 6    |
| tie-break 0.001, `lam = 0`          | 33        | 5    | 56      | 127         | 29   | 53             | 6    |
| tie-break 0.0001, `lam = 0`         | 33        | 5    | 56      | 128         | 28   | 54             | 6    |
| hinge 0.05, `lam = 0`               | 50        | 6    | 24      | 193         | 24   | 72             | 6    |
| hinge 0.05, `lam = 1`               | 47        | 5    | 31      | 183         | 26   | 66             | 6    |
| hinge 0.1, `lam = 0`                | 51        | 6    | 13      | 197         | 24   | 66             | 5    |
| logistic 0.05, `lam = 0`            | 48        | 7    | 17      | 197         | 26   | 63             | 7    |

The order score with its pull toward the defaults holds out best. Free of the pull (`lam = 0`), the tie-break fits
more (127 against 138) and holds out far worse (53): the nesting goes exponential, 3, `class` 5 → 2.5, `loop` 3 →
1.3-1.6, `function` 1.55 → 1, what the pairs leave open filled by their noise. With the pull it is the order score
(the pull outweighs it). The margins (hinge, logistic) leave few pairs near a tie (fragile 92 → 13-31) and put many
more in the wrong order, held out too. The defaults stay; the weights still at round values are those no pair moves
off their old defaults.

## The docs weight's calibration (`calib/`, 2026-11-24)

A docs weight and a code weight are the same thing, what a text costs to read and keep, so the `[docs]` weights
should put a Markdown file on the code's scale. `calib/` holds what calibrates them: a real snippet (its comments
dropped, a Java method in a bare class) beside descriptions of it, each of the same cognitive volume. A description
says everything the code does, in its order, as much as a reader needs to rewrite it without the code: every
input, case, condition, loop, error and result. It says nothing the code doesn't (no why, no examples). Two styles:

- `<name>.md`, prose, as our docs read: sentences, a list where the code has cases, the names and literals in
  inline code.
- `<name>.dense.md`, a terse spec: expressions in inline code, operators and arrows (`→`, `∀`, `⇔`), a table of
  cases. The same content, in fewer words and more symbols, so that the two kinds of run can be told apart.

| example | source |
|---------|--------|
| `rust/is_table_separator`, `rust/destination_length`, `rust/url_length` | codeweight's `src/markdown.rs` |
| `rust/parse_diff_tree` (prose only) | codeweight's `src/git.rs` |
| `python/crossover` | `tune.py` |
| `java/Retry`, `java/HexOf` | [cactoos](https://github.com/yegor256/cactoos) (MIT), `func/Retry.apply`, `bytes/HexOf.asBytes` |

`python3 calib.py [codeweight] [folder]` weighs each snippet (`--code`) and counts its descriptions' words,
function words, punctuation runs and symbol runs (`--docs` under one weight 1, the others 0, each in turn). It fits
the `[docs]` weights by the least squares of the relative misses: one factor for all four, the four apart, and
"halves", a function word and a punctuation run at half a word with the symbol apart.

### The calibration (2026-11-24, codeweight's code weights of `c66fb99`)

| description | code | words | function words | punctuation | symbols | code per run |
|-------------|-----:|------:|---------------:|------------:|--------:|-------------:|
| `java/HexOf` | 100.15 | 89 | 62 | 41 | 3 | 0.514 |
| `java/HexOf.dense` | 100.15 | 83 | 6 | 74 | 26 | 0.530 |
| `java/Retry` | 100.52 | 115 | 52 | 61 | 1 | 0.439 |
| `java/Retry.dense` | 100.52 | 60 | 5 | 63 | 12 | 0.718 |
| `python/crossover` | 74.10 | 77 | 39 | 62 | 1 | 0.414 |
| `python/crossover.dense` | 74.10 | 60 | 2 | 46 | 46 | 0.481 |
| `rust/destination_length` | 76.32 | 66 | 42 | 33 | 6 | 0.519 |
| `rust/destination_length.dense` | 76.32 | 52 | 9 | 55 | 33 | 0.512 |
| `rust/is_table_separator` | 50.49 | 59 | 27 | 18 | 6 | 0.459 |
| `rust/is_table_separator.dense` | 50.49 | 37 | 3 | 36 | 15 | 0.555 |
| `rust/parse_diff_tree` | 170.45 | 163 | 87 | 73 | 1 | 0.526 |
| `rust/url_length` | 89.85 | 93 | 59 | 48 | 13 | 0.422 |
| `rust/url_length.dense` | 89.85 | 69 | 15 | 61 | 36 | 0.496 |

| `[docs]` | word | function word | punctuation | symbol | worst miss | mean miss |
|----------|-----:|--------------:|------------:|-------:|-----------:|----------:|
| all 1 (the first defaults) | 1 | 1 | 1 | 1 | +142% | 101% |
| one factor | 0.489 | 0.489 | 0.489 | 0.489 | -32% | 10% |
| the four apart | 0.617 | 0.112 | 0.581 | 0.305 | -23% | 9% |
| halves | 0.639 | 0.320 | 0.320 | 0.619 | -33% | 10% |
| **the defaults** | 0.64 | 0.32 | 0.32 | 0.62 | -33% | 10% |

- **All 1 weighs a description twice its code**, and so a doc's change about twice its code's in a commit's line.
- **A run is worth about half a code weight**, 0.41 to 0.72, over three languages, sizes from 50 to 170 and both
  styles. A dense description has fewer runs for the same content, so each weighs a little more.
- **The scale is pinned, the shares aren't.** Every fit misses by 9-10% on the mean; what is left is the
  descriptions' own spread, the worst `java/Retry.dense` (-33%), the tersest. Fitted apart, the shares swing with
  the model: punctuation from 0.6 to 2 (to 2.1 with backticks counted as symbols, as inline code's backticks stand
  in for the code in it), function words near 0, the symbol from 0.3 to 0.8.
- **So the shares are a judgement, the halves**: a function word and a punctuation run are glue a reader passes
  over, half a word. The fit then sets the scale and the symbol: a word 0.64, a symbol 0.62. A third of prose's
  words are function words (none in a dense description, which is mostly code), so a word that says something
  weighs more than under one factor.
- Before function words, the halves on punctuation alone fitted word 0.54, punctuation 0.27, symbol 0.83, at 11% on
  the mean. The prose descriptions alone, before the dense ones, put a symbol near 0: they hold too few symbols to
  weigh them.
- The descriptions are the soft part: they are one writer's judgement of "the same volume".
