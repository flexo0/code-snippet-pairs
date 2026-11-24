# Third-party code

Most snippets here are taken from other projects: their rule examples, their tests and their sources. Each is cut
to a pair (or to a calibration snippet) and changed: wrapped in a function or a class where the language needs one,
its placeholders made concrete, its one-line test strings laid out on several lines, its comments dropped. The code
stays under its project's license, whose text is in `licenses/`, as of the commit it was taken from.

| what | from | commit | license | copyright | the license's text |
|------|------|--------|---------|-----------|--------------------|
| `hand/rust/`, `gen/clippy/` | [rust-clippy](https://github.com/rust-lang/rust-clippy) | `23a80940ad` | MIT or Apache-2.0 | The Rust Project Contributors | [COPYRIGHT](licenses/clippy-COPYRIGHT), [MIT](licenses/clippy-LICENSE-MIT), [Apache-2.0](licenses/clippy-LICENSE-APACHE) |
| `hand/python/`, `gen/ruff/` | [Ruff](https://github.com/astral-sh/ruff) | `248231e993` | MIT (and, for the rules ported from other linters, theirs, listed in its license) | 2022 Charles Marsh | [LICENSE](licenses/ruff-LICENSE) |
| `hand/js/`, `gen/eslint/` | [ESLint](https://github.com/eslint/eslint) | `48207902fc` | MIT | OpenJS Foundation and other contributors | [LICENSE](licenses/eslint-LICENSE) |
| `hand/ts/`, `gen/typescript-eslint/` | [typescript-eslint](https://github.com/typescript-eslint/typescript-eslint) | `a53584297a` | MIT | 2019 typescript-eslint and other contributors | [LICENSE](licenses/typescript-eslint-LICENSE) |
| `gen/errorprone/` | [Error Prone](https://github.com/google/error-prone) | `dab49c372f` | Apache-2.0 | The Error Prone Authors | [COPYING](licenses/error-prone-COPYING) |
| `hand/go/`, `gen/staticcheck/` | [staticcheck](https://github.com/dominikh/go-tools) | `8371db75e9` | MIT | 2016 Dominik Honnef | [LICENSE](licenses/go-tools-LICENSE) |
| `calib/java/Retry.java`, `calib/java/HexOf.java` | [cactoos](https://github.com/yegor256/cactoos) | `master`, 2026-11-24 | MIT | 2017-2026 Yegor Bugayenko | [LICENSE](licenses/cactoos-LICENSE.txt) |

The commits are those `extract/run.sh` fetches `gen/` from. The hand-picked pairs were taken from the same projects'
docs and sources on 2026-11-19, at their main branch then.

The rest is not from these projects, and is under the MIT license of [LICENSE](LICENSE): the scripts and the docs,
`mine/`'s (its pairs, from our own projects' commits, are kept out of git) and the other snippets of `calib/`
(codeweight's own code, MIT too, and `tune.py`).
