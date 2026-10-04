# Tasks

Starts after `m2-procedures-and-errors` is archived (design, Context).

## 1. Shared enums (D8)

- [ ] 1.1 `ir` re-exports `Ty`, `BinOp`, `ConvKind` (as `Conv`) from `sema`; remove `ir`'s copies and the mapping
  functions in `lower.rs`; adjust `codegen-cpp`. Verify: `cargo test` with no snapshot changed; `cargo clippy
  --workspace --all-targets -- -D warnings` clean; `cargo tree -p qb64rust-ir` unchanged.

## 2. The marker (D3)

- [ ] 2.1 `base::Diagnostic` gains `unsupported`; `Diagnostics::unsupported`; `is_real_error`; the renderer prints
  `error: not supported yet: <message>` and the summary `N errors (M not supported yet)`. Unit tests in `base`.
- [ ] 2.2 Move the 48 "not supported yet" sites in `syntax` and `sema` to the marker, removing the words from the
  messages. Verify: `grep -rn "not supported yet" crates --include=*.rs` finds only the renderer and tests;
  snapshot diffs show only the new rendering (review each `.snap.new`).
- [ ] 2.3 Parser: errors at a known BASIC word or operator it does not handle are marked (D3 list), naming the
  token. Verify: parse snapshots for `x = a MOD b`, `IF a < b THEN`, `PRINT #1, x`, and one genuine syntax error
  (`x = 5 6`) that stays unmarked.
- [ ] 2.4 CLI spec scenarios "Unknown statement" and "Summary" as tests in `crates\driver\tests\cli.rs`.

## 3. Inputs and the two lists (D1, D2, D4)

- [ ] 3.1 `tools\upstream\copy_upstream_tests.py`; run it; `tests\upstream\SOURCE.md`, `LICENSE-QB64pe.txt`,
  `.gitattributes` entries. Verify: 404 `.bas` copied; `python tools\repo_check\check_repo.py` passes (add
  `ALLOWED` entries with reasons if upstream text needs them); a second run changes nothing (`git status`).
- [ ] 3.2 `crates\driver\tests\inputs.rs` with the input sets of D2 (clone sets skipped without the clone);
  move the checks from `corpus.rs` into it (round trip, slice list, rejected). Test: the copy equals the clone
  (when present). Verify: `cargo test`; the same test with `QB64RUST_QB64PE_ROOT` pointing at a missing folder
  passes and prints the skip note.
- [ ] 3.3 The two lists (D4) with `QB64RUST_UPDATE_LISTS=1`; generate them. Verify: a following `cargo test`
  passes; deleting one entry makes it fail with "new entries", adding a bogus one with "stale entries" (checked
  once, then restored). Record the counts (accepted programs with a real error, rejected programs with only marked
  errors, per set) in `tests\upstream\README.md`.
- [ ] 3.4 Measure `cargo test` time in debug before and after 3.2–3.3; if over a minute, apply the fallback of the
  design's first risk. Record the time in `crates\README.md`.

## 4. Snippets (D5)

- [ ] 4.1 `tools\snippets\extract_qb64fresh_snippets.py`; run it against QB64Fresh (`<qb64contain>`) and
  `..\QB64pe\qb64pe.exe`; `tests\snippets\qb64fresh\SOURCE.md`. Verify: count before and after deduplication in
  `SOURCE.md`; no local path in any `.err` (`check_repo.py`); a second run changes no file; the lists of 3.3
  regenerated and reviewed.

## 5. Mutation test (D6)

- [ ] 5.1 `crates\driver\tests\mutate.rs`. Verify: runs in under 10 s in debug; temporarily adding a panic on a
  rare byte sequence to the lexer makes it fail with a reproducible report (checked once, then removed). Any real
  panic or round-trip failure it finds is fixed in this task, with the mutant added to `tests\frontend\` as a
  `parse-ok` or `check-fail` test.

## 6. Tier 2 over upstream (D7, D9)

- [ ] 6.1 Check the QB64pe release for what the build needs (D9) and record the answer in `design.md`.
- [ ] 6.2 `qb64rust`: `-f:OptimizeCppProgram`, `-f:StripDebugSymbols`, other `-f:` settings rejected; CLI spec
  scenarios "Compile suite's settings" and "Unknown setting" as tests (the first `#[ignore]`d like the other build
  tests). `run_legacy_tests.py`: `--list` for `--suite compile`.
- [ ] 6.3 `tests\upstream\deferred.list` (125 entries, checked by eye) and the tier-1 check that no program is in
  both lists and that prints "x of N" with N computed. Verify: N is 279.
- [ ] 6.4 Run `--suite compile --qb64 target\release\qb64rust.exe` over the whole upstream suite once (not a pass
  criterion; about as long as the old compiler's run); put every passing program into `tests\upstream\pass.list`.
  Verify: the `--list` run passes all of them, also with `QB64RUST_NO_FOLD=1`; `git -C ..\QB64pe status
  --porcelain` unchanged.
- [ ] 6.5 If 6.1 said yes: the CI job of D9. Verify: it runs green on a push of the branch.

## 7. Documentation

- [ ] 7.1 `tests\upstream\README.md` (D7), `crates\README.md` (tests table: `inputs.rs`, `mutate.rs`, the lists and
  `QB64RUST_UPDATE_LISTS`, the marker), `tests\corpus\README.md` (checks moved), `study\19` (tier-1 contents,
  tier-2 upstream run, CI), `CLAUDE.md` layout (`tests\upstream\`, `tests\snippets\`, the lists, the two tools),
  `STATUS.md` (step 2 done, the false-error and upstream numbers as the baseline for parser breadth). Verify:
  `openspec validate m2-upstream-tests --strict` passes; `check_repo.py` passes.
