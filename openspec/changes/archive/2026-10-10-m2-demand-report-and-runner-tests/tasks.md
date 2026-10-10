# Tasks

Both groups come from the eighth review (`study\29` §4, §5; the user: "do recs", 2026-10-10) and go before step 9a.
Text with backslashes goes through the Edit and Write tools (`CLAUDE.md` rule 7); no subagents (rule 1).

## 1. The demand report

- [x] 1.1 `base::lift_error_cap()`: a process-wide switch `Diagnostics::push` reads (design D2). Verify: the cap
  tests of `base` and `driver\tests\frontend.rs` still pass (nothing in a test process lifts it).
- [x] 1.2 `crates\driver\src\report.rs`: the programs of files and folders (`.bas` in any case, `.err` neighbours
  left out), one program's marked messages and count of other errors, the table (programs, alone, uses; design
  D1), the text. Unit tests of the table and its order.
- [x] 1.3 `qb64rust --report unsupported <file or folder>... [--include-root <dir>]` in `main.rs`; more than one
  input stays an error without `--report`. CLI tests for the three scenarios of the spec delta (`cli.rs`); the
  folder test has a program with 105 `LPRINT`s, counted in full.
- [x] 1.4 Run it over the sets. Found: the game has 1,740 marked errors in 38 messages (`LINE` 899, `COLOR` 190,
  `LOCATE` 145, `PAINT` 89, `DRAW` 71; `study\28` §4 counted the same three by hand); the corpus has 27 programs
  held back, 14 of them by a single message each; upstream's top rows are arrays as `TYPE` members (152
  programs), `REDIM` (148), `$Unstable` (85), `LBOUND` of an expression (63), `_DEST` (54). Upstream's `qb64pe\`
  folder shows 10 programs "with other errors": their included files are the old compiler's sources, which only
  the clone holds (tier 1 reads them from there).

## 2. The runner's tests

- [x] 2.1 `tools\legacy_tests\test_run_legacy_tests.py` (`unittest`): the pure rules (`norm_output`, `norm_text`,
  `is_old_compiler`, `new_compiler_rejection`, `load_normalize`, `apply_normalize`, `bash_exit_code`,
  `git_sees_text`, `write_recorded`, `write_expected`, `load_list`, `load_known_failures`, `written_by_builds`).
- [x] 2.2 `corpus_test` with `run_proc` replaced by a stand-in (design D3): every outcome named in the spec delta.
  41 tests, 0.2 s.
- [x] 2.3 Check that the tests bite: 15 seeded mistakes in the runner, one at a time, by a scratch script (not
  kept). Found: all 15 caught.
- [x] 2.4 `rust.yml`, job `test`: `python -m unittest discover -s tools/legacy_tests -p "test_*.py"`.

## 3. Close

- [x] 3.1 `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`,
  `python -m ruff format --check .`, `python -m ruff check .`, `tools\repo_check\check_repo.py --untracked`: clean.
- [x] 3.2 Documents: `crates\README.md` (the option), `tools\legacy_tests\README.md` (the tests), `CLAUDE.md`
  layout, `STATUS.md`, `DECISIONS.md`, `study\29` §8.
- [x] 3.3 Not done, and why: CI's `tier2` job still builds `qb64pe.exe` from the reference commit (proposal, "Not
  in this change").
