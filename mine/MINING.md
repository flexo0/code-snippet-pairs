# Mining pairs from a repo's commits

How the pairs of `mine/` are made: subagents read an author's commits and write the pure refactors they find as
pairs. `parsed_from_projects/` was made this way (see the README). Below are the steps, then the brief each subagent
gets.

## The steps

1. **The list.** List the commits to mine, oldest first, skipping those of the set's `commits.txt`:

       git -C <repo> log --reverse --author=<name> --since=<date> --until=<date> \
           --format='%h %ad %s' --date=short -- '*.java'

2. **The chunks.** Split it into K chunks, one per sonnet subagent (`split -n l/K`). About 10 commits a chunk is a
   good size.
3. **The agents.** Start one sonnet subagent per chunk with the brief below, its placeholders filled. Each sonnet subagent
   gets its own `{CHUNK}` and `{INDEX}`, e.g. `<scratch>/chunk-K.txt` and `<scratch>/index-K.tsv`. All of them
   share `{OUT_DIR}`, the set's folder `mine/<set>/`.
4. **The merge.** `CODEWEIGHT=<codeweight> python3 mine/merge.py <scratch> mine/<set>` reports:
   - a pair with one side missing, or no index line;
   - an index line with no files;
   - a pair whose sides are the same but for comments and whitespace;
   - a copy of another pair;
   - a side that doesn't parse.

   Once it's clean, run it again with `--write` to write `mine/<set>/index.tsv`.
5. **The review.** Read every pair against its commit (`git show <h> -- <file>`), with the brief's rules in
   mind. The agents' mistakes are mostly these:
   - a type that is on one side only;
   - parts left untrimmed;
   - a helper called but missing, or a call site left out;
   - an incidental change kept in;
   - a refactor that isn't better on its own.
6. **The anonymizing.** After the review, `python3 mine/anonymize.py mine/<set>` renames every name, string and
   comment of the pairs in place (run it once). Look at what it kept: a library name missing from
   `mine/anonymize-keep.txt` was renamed and the refactor with it, so add it, restore the pairs and run it again.
7. **The check.** Run `python3 check.py <codeweight> mine/<set>`, the codeweight a build with the current
   defaults. Note the result in the README.
8. **The record.** Add the full hashes of every commit the agents read to `mine/<set>/commits.txt`, one per line,
   so the next run skips them.

## The brief

Fill in:
- `{REPO_NAME}` and `{REPO_PATH}`: the repo to mine.
- `{CHUNK}` and `{N}`: the subagent's list file and how many commits it has.
- `{DONE_LIST}`: `mine/<set>/commits.txt`.
- `{OUT_DIR}`: `mine/<set>/`.
- `{INDEX}`: the subagent's own index file.
- `{CODEWEIGHT}`: a current build. An older one rejects the newer categories.

````markdown
# Mining pure refactors from {REPO_NAME} into codeweight pairs

codeweight weighs code: every element has a weight, and an element weighs more the deeper it is nested. Its
default weights are tuned on *pairs*. A pair is a bad snippet and a good one that behave the same, the good one
simpler; the good one should weigh less. We are making pairs from real refactors the repo's author made.

**The repo is read only:** `{REPO_PATH}`. Use only `git log`, `git show`, `git diff` and `git grep <rev>`. Never
checkout, commit, stash or write in it.

**Your commits:** the file `{CHUNK}`, one line per commit: hash, date, subject. There are {N} of them. For each
one:
- look at the stat first (`git show --stat <h>`), then the Java diff (`git show <h> -- '*.java'`, or file by
  file);
- see the code before and after with `git show <h>~1:<path>` and `git show <h>:<path>`.

Use the subject to choose what to look at first, but don't trust it. A commit called "fix" or "wip" may hold a pure
refactor in part of it, and one called "refactor" may change behaviour. Skip SQL, configs and generated code. Skip
the commits listed in `{DONE_LIST}` (full hashes, one per line): they were mined before.

## What to take

A **pure refactor** is a change to a unit (a method, a few methods, a class) after which the unit does exactly the
same thing, written more simply or more clearly. Some examples:
- an `if` chain or nested `if`s flattened into guard clauses;
- a loop turned into a stream, or a stream into a loop;
- duplicated code merged into a helper, or a helper inlined;
- a long method split;
- a boolean expression simplified;
- a needless variable or temporary removed;
- a provably redundant null check, cast, `continue` or parameter dropped;
- a dead parameter that every caller passes the same constant for, removed along with its branch;
- hand-written code replaced by a utility or library call.

It may be only part of a commit, as long as the unit you take changed only by the refactor. The rest of the commit
can add a feature or fix a bug. If the same unit also got a behaviour change, skip the unit. Behaviour changes
include a new condition, a new field set, a changed log text, a different exception, `<` for `<=`, or a new
parameter used for something new. Don't rewrite the code to separate the two. There is one exception: when the
refactor is one self-contained statement and the behaviour change is in other statements of the method, you may
take that statement alone (see trimming below).

### It must be the same behaviour, provably

Check each of these before you write a pair; when in doubt, leave it out:
- **Null.** Can anything be null on one side and not the other? An `Optional` chain against a null check, a
  `@Nullable` value, unboxing.
- **Exceptions.** Something that threw before and doesn't now, or the reverse. Empty collections count:
  `in ()` is an SQL error, `= ANY('{}')` finds nothing.
- **Evaluation.** The number of calls, side effects, order, short-circuiting, laziness, floating-point rounding.
- **Other callers.** A public or protected method that changed: look for every caller with `git grep <h>~1`. A
  protected field or list that subclasses may change. A method that gained a guard inside while it had callers
  elsewhere.
- **Library semantics.** For example, Spring's `env.matchesProfiles(...)` also looks at the default profiles when
  no profile is active. SQL `is` against `=` on a column that can be null. Lombok's `@RequiredArgsConstructor`
  takes the parameters in field order.
- **A helper the good side calls.** Check whether it existed before the commit (`git grep <name> <h>~1`). If it
  did, it is a library call and stays out of the pair. If the commit added it, it goes into the good side's type.
  If the commit changed it, skip the pair.

### It must be better on its own

The good side must be simpler or clearer by itself, without the commit's other changes. Skip a refactor that only
pays off because of them. The cases rejected before:
- **Extracted only for a new feature.** A helper or constructor extracted only so that new code can reuse it,
  where the pair alone shows a single use that isn't clearer. Splitting a long method into named steps is fine on
  its own; moving two setters into a constructor that takes a service is not.
- **Holder-array state.** An extraction that passes mutable state in and out through one-element arrays, or that
  extracts dead code (a branch under a constant `false`).
- **Unwrapped `Optional`.** An `Optional` the API returns, turned into `orElse(null)` plus an `if`. The reverse is
  fine: `Optional.ofNullable(x).map(f).orElse(null)` becoming `x == null ? null : f(x)`.
- **Gimmicky syntax.** For example `switch (d) { case double v when v < 0.5 -> … }` for a plain `if`/`else if`
  chain.

Also skip:
- **Invisible changes.** codeweight can't see renames alone, formatting, imports, comments, annotations alone, a
  type or generic changed alone, visibility, or code moved unchanged to another class or file.
- **Speed rewrites.** Caching, a `BitSet` for a `Set`, arrays for records, a manual loop for speed: the author made
  the code heavier on purpose. The reverse, removing such an optimization so the code gets simpler while doing the
  same, may be taken.

Quality over quantity: a wrong pair hurts the tuning. Expect few pairs; most commits are features and fixes.

## How to write a pair

Files go into `{OUT_DIR}`, named `<hash10>-<slug>.bad.java` (before the commit) and `<hash10>-<slug>.good.java`
(after). The slug is a few lowercase words joined by `_`, saying what changed: `guard_clauses`, `loop_to_stream`,
`extract_helper`. Several pairs from one commit get different slugs.

**Types:**
- Each side wraps its members in their type's declaration, with the original name and kind (`class X {`,
  `interface`, `enum`, `record`). Leave out the package, the imports and the annotations on the type.
- Any type that a refactor's member lives in, before or after, goes on **both** sides. Leave it empty
  (`class PickupPlace {\n}`) if nothing of it changed on that side. A helper moved into an existing enum: the enum
  is on both sides (with its constants), and the helper is only in the good side's.
- A type is on one side only when the commit really added it (good only) or deleted it (bad only).
- If the code moved from one existing class to another, give both sides the same wrapper name.
- **Members.** Both sides hold the same members, plus what the refactor added (an extracted helper, on the good
  side) or removed (an inlined helper or a deleted delegate, on the bad side).
- **Call sites.** If a helper replaces code in several places, every such place goes in, on both sides, when it
  changed only by the refactor. If the refactor changes a method's signature, the call sites whose only change is
  the new call go in too.

**Trimming.** Cut each side down to what the refactor changed. A sibling statement, a branch of an `if`/`else if`
chain, a member, a log line, a comment block, a validation, or a parameter nothing uses: if it is the same on
both sides and the change doesn't depend on it, remove it from both. What to keep:
- **The nesting around the change.** The enclosing loops, `if`s, `try`, lambdas: an element's weight depends on
  its depth, so cutting them changes the difference.
- **What the changed code depends on.** The declarations of the variables it uses, and the statement that uses
  its result, if that's what makes it meaningful.
- **The removed side of a dead parameter.** The `if` on that parameter.

Remove only; don't rewrite what's left, beyond the minimum to keep it coherent: an argument list shortened, a
return type changed to `void` once the `return` is gone.

**Text:**
- Keep the original text, re-indented if needed.
- If the commit also made an incidental change inside the pair, put it back to the bad side's form on the good
  side, so the pair holds only the refactor. Examples: an added annotation, `@Nullable` → `@Nonnull`, a field
  access turned into an accessor call, spacing.
- One refactor idea per pair. A pair may be large (a whole method of 80 lines) if that is what the refactor
  touched after trimming.

**Parsing.** Each side must parse cleanly. Run `{CODEWEIGHT} scan --code --by file <bad> <good>`: it prints each file's
weight, and a `warning: ... has syntax errors` line if one doesn't parse. Fix your wrapping until there is no
warning.

If the original uses syntax the parser can't read (statements before `super(...)`, preview features), leave that
member out of both sides and say so in a `//` comment on the good side. If the pair then no longer makes sense,
drop it.

A lower weight on the good side is NOT a requirement. Record the pair whatever the weights say, and don't pick
pairs by them.

## Index

For each pair, append a line to `{INDEX}`, tab-separated, with no header:
`<case>\t<commit hash10>\t<file path in the repo>\t<kind>\t<why the good one is better, one short phrase>`
- `<case>` is `<hash10>-<slug>`.
- `<kind>` is one of: `guard_clauses`, `flatten_nesting`, `extract_method`, `inline`, `dedup`, `loop_to_stream`,
  `stream_to_loop`, `simplify_condition`, `simplify_expression`, `remove_redundant`, `optional`, `switch`,
  `library_call`, `restructure`, `other`.

## Report

When done, reply with a short report:
- how many commits you looked at;
- how many pairs you wrote, by kind;
- for each pair that is near but not exact, one line on what differs: an edge case, a library semantic, a member
  you had to leave out;
- a few refactors you saw but skipped, a line each with the reason.

Don't paste the pairs.
````
