# Design

## Context

Background and numbers: `study\22` (§2 measurements, §4 what is taken from QB64pe and QB64Fresh). Today tier 1
reads `tests\corpus` (`crates\driver\tests\corpus.rs`: round trip, slice list without diagnostics, `.err` programs
rejected) and every `.bas` under `tests\` (`crates\syntax\tests\corpus_roundtrip.rs`). Diagnostics are
`{ severity, span, message }` (`crates\base`); 48 messages in `syntax` and `sema` say "not supported yet" in their
text only. The runner `tools\legacy_tests\run_legacy_tests.py` runs `--suite compile` from the clone's
`tests\compile_tests` with `-f:OptimizeCppProgram=true -f:StripDebugSymbols=false` (plus
`-f:GenerateLicenseFile=true` for the 18 tests with an `.<os>.license` file) and `-q -m -x`; `--list` works for
`--suite corpus` only. `qb64rust` rejects any option it does not know.

Measured for this design: `qb64pe` turns `OptimizeCppProgram` into `-O2` in `CXXFLAGS_EXTRA`
(`source\qb64pe.bas` 13728–13738; `-Og -g` with debug info, which the suite does not ask for). The clone's
`tests\compile_tests` has 404 `.bas`, 331 `.output`, 56 `.err`, 18 `.license`, 6 `.bi`, 6 `.bm`, 5 `.h`, 3 `.c`,
4 `.compile-from-base`, 2 `.noprompt`, 1 `.md`, 2 `.gitignore`; no `.flags`. The rest is binary (images, fonts,
sound, DLLs and static libraries). Folder names include `name with spaces`, `single'quote'test`, `parens()`,
`dollar$sign$test`.

`m2-procedures-and-errors` is archived before this change starts (its delta of "Tier 1 corpus checks" is the base
of this one).

## Goals / Non-Goals

**Goals:**
- Every BASIC file we can get goes through the front end on every `cargo test`: no panic, exact round trip.
- A machine-checked definition of "no false error" and "stays rejected", with lists that only shrink, so parser
  breadth (next step) has a number to drive to zero.
- Upstream progress as "x of 279", end to end, with the existing runner.
- The duplicated `Ty`/`BinOp`/conversion enums of `sema` and `ir` become one.

**Non-Goals:**
- Making more programs pass. No language feature is added; generated code does not change.
- Copying `qbasic_testcases` or the old compiler's sources (third-party or not needed; `CLAUDE.md`, 2026-10-04).
- A coverage-guided fuzzer (`SOMEDAY.md`).
- Old compiler error texts: `.err` files are used for "must reject" only.

## Decisions

### D1. Layout of the copy
`tests\upstream\compile_tests\<category>\<file>` mirrors the clone, text files only, selected by extension:
`.bas .bi .bm .h .c .output .err .license .noprompt .compile-from-base .md` (not `.gitignore`, which would change
git's behaviour inside our tree). `tests\upstream\SOURCE.md` names the commit (`16f629784e`), the date, the licence
and the script; `tests\upstream\LICENSE-QB64pe.txt` is the clone's `licenses\license_qb64.txt`.
`tools\upstream\copy_upstream_tests.py <clone>` deletes and rewrites `tests\upstream\compile_tests\` (never
touches the lists), and refuses to run if the clone's HEAD is not the commit in `SOURCE.md` unless given
`--commit-ok` (then it updates `SOURCE.md`). `.gitattributes`: `tests/upstream/**/*.output`, `*.err`, `*.license`
and the `.bas`/`.bi`/`.bm` files are `-text`, so the bytes stay as upstream has them.
*Why a copy and not a submodule or a clone in CI:* the clone is ours read-only and 239 MB; the text is 3.1 MB and
MIT; the libqb copy at M3 sets the same precedent (a plain copy at a pinned commit).
*Repo check:* `tools\repo_check` runs over the copy; if upstream text trips rule 2 (a path in a test), the file
and text are added to `ALLOWED` with the reason, not edited.

### D2. Input sets and the program list
One test binary, `crates\driver\tests\inputs.rs`, owns every whole-program input set (the existing checks in
`corpus.rs` move into it; `corpus_roundtrip.rs` stays as the `syntax`-only round trip). Each set gives
`(name, path, verdict)` where verdict is `Accepted`, `Rejected` or `IncludeOnly`:

| Prefix | Root | Verdict |
|---|---|---|
| `corpus/` | `tests\corpus` | `Rejected` with `.err`, else `Accepted` |
| `upstream/` | `tests\upstream\compile_tests` | `.bas`: `Rejected` with `.err`, else `Accepted`; `.bi`/`.bm`: `IncludeOnly` |
| `snippets/` | `tests\snippets` | `Rejected` with `.err`, else `Accepted` |
| `qbasic/` | clone `tests\qbasic_testcases` | `Accepted` (all compile, `baselines\`) |
| `qb64pe-source/` | clone `source`, `internal\support` | `source\qb64pe.bas`: `Accepted`; every other file `IncludeOnly` |

The clone is found as the driver finds it (`--qb64pe-root` logic: `QB64RUST_QB64PE_ROOT`, else `..\QB64pe`); when
missing, the two clone sets print "skipped: no QB64pe clone" and contribute nothing. Every file of every set gets
the no-panic and round-trip check; `IncludeOnly` files get nothing else (they are not programs on their own).

### D3. The marker
`base::Diagnostic` gains `unsupported: bool`; `Diagnostics::unsupported(span, message)` creates one. The 48 sites
whose message says "not supported yet" switch to it and drop the words from the message (the renderer adds
them). Rendering: `error: not supported yet: <message>`; the summary `N errors (M not supported yet)` when M > 0.
`Diagnostic::is_real_error()` = error and not marked.

Parser errors at syntax it does not know yet: when the parser fails at a token that is a known BASIC word or
operator it does not handle (a keyword in `keywords.rs`, or one of `MOD`, `\`, `^`, `=`, `<>`, `<`, `>`, `<=`,
`>=`, `AND`, `OR`, `NOT`, `XOR`, `EQV`, `IMP` in an expression; `#` and `;` file numbers), the diagnostic is
marked, with the token named ("operator `MOD`"). Any other parse error stays unmarked. This rule is a heuristic;
its failures show up as entries in `known_false_errors.list`, which is the point of the list.
*Rejected:* a separate severity level. The language server needs "is this a real error?" as a yes/no on an error,
and the VS Code problem list should still show the item as an error in the editor during the transition.

### D4. The two lists
`tests\known_false_errors.list` (an `Accepted` program with at least one real error) and
`tests\known_unsupported_rejections.list` (a `Rejected` program whose errors are all marked). Format: `prefix/
relative/path.bas`, sorted, `#` comments, one entry per line; a trailing comment after the entry is allowed (the
first real error, for reading). The test computes the exact set for each list, compares, and on a difference
fails with "new entries" and "stale entries"; with `QB64RUST_UPDATE_LISTS=1` it rewrites the lists (keeping the
header comment) and passes. A `Rejected` program with **no** error fails always (no list can excuse it). Entries
of the clone sets are checked only when the clone is present (they are neither new nor stale otherwise).

The slice list check (no diagnostics at all) stays as it is.

### D5. Snippets from QB64Fresh
`tools\snippets\extract_qb64fresh_snippets.py <QB64Fresh dir> <qb64pe.exe>`: reads every `r#"..."#` raw string
assigned to `source` in `tests\integration_tests.rs`, removes the common indentation, normalises line ends to CR
LF (as QB64pe sources are), drops snippets whose text is identical to an earlier one, and writes
`tests\snippets\qb64fresh\<module>__<test>.bas`. Each is compiled with `qb64pe.exe -z -q -w` in a scratch folder
(never in the repo); a non-zero exit writes `<name>.err` with the compiler's message, normalised like the corpus
`.err` files (no paths). `SOURCE.md`: QB64Fresh's commit, the count before and after deduplication, the rule-5 note
(inputs only, the user's MIT code; assertions not taken, verdicts are `qb64pe.exe`'s). Snippets that need input or
never end do not matter: nothing is run. The script is in the repo; `<qb64contain>` stays out of it (path passed on
the command line).

### D6. Mutation test
`crates\driver\tests\mutate.rs`: a small xorshift generator seeded with a constant (`QB64RUST_MUTATE_SEED` overrides
it, `QB64RUST_MUTATE_COUNT` the count). For each corpus program, 20 mutants; each applies one to three edits from:
delete a range of up to 16 bytes, insert 1–8 random bytes (any value, including 0x00, CR, LF, `"`, `'`, `_`),
duplicate a range, swap two ranges. Each mutant goes through lexer, parser and `sema` (not lowering). Checks: no
panic (`catch_unwind`), exact round trip. On failure: program name, mutant number, seed, and the mutant's bytes
written to `target\mutate-failure.bas`. Budget about 5,500 front-end runs, under 10 s in debug.
*Why not proptest:* no new dependency for a 60-line generator; determinism and the printed reproduction are what
matter.

### D7. Tier 2 over upstream
- `run_legacy_tests.py`: `--list` also applies to `--suite compile` (names `category/name`); the guard that limits
  `--list` to `--suite corpus` is relaxed for it.
- `qb64rust`: `-f:OptimizeCppProgram=true` adds `-O2` to `CXXFLAGS_EXTRA` (beside `-fwrapv`, so overflow still
  wraps), `=false` adds nothing; `-f:StripDebugSymbols=…` is ignored; any other `-f:` setting is the error "setting
  `<name>` is not supported yet" (so the 18 licence tests fail at compile, which is honest).
- `tests\upstream\deferred.list`: the 125 programs, each `category/name  <feature>` (`$UNSTABLE:TYPEFIELDS`,
  `REDIM _RETAIN`, whole-array assignment, `_ARRAYCOPY`), found by the patterns of `study\22` §2 and checked by eye
  for false matches. `tests\upstream\pass.list`: starts with whatever passes today (expected: a handful of the 25
  of `study\22` §2).
- `tests\upstream\README.md`: what the folder is, the two lists, the tier-2 command, and the progress line
  "x of 279" (the number of non-deferred programs is computed by the tier-1 test that also checks that no program
  is in both lists, and printed).

### D8. Shared enums
`Ty`, `BinOp` and `ConvKind` stay defined in `sema`; `ir` re-exports them (`pub use qb64rust_sema::{Ty, BinOp,
ConvKind as Conv}`) and `lower.rs` drops its three mapping functions. `ir` already depends on `sema`, so the
dependency graph does not change. *Why not a new crate:* step 5 of `study\22` turns `Ty` into a type table owned by
`sema`; a separate crate now would move again then. The IR's ABI-neutrality is unaffected (the enums name no C
type). Snapshots of `typed`, `ir` and `cpp` tests must not change.

### D9. CI
First check that the Windows QB64pe release (`vscode-extension.yml` downloads it) has `Makefile`, `internal\c` with
`qbx.cpp`, and the toolchain at `internal\c\c_compiler\bin\mingw32-make.exe`, and that `qb64rust -x` builds a slice
program against it. If yes: a job in `rust.yml` (or its own workflow, path-filtered like `rust.yml`) that builds
`qb64rust` in release, caches the release and the libqb objects it builds, and runs tier 2 with
`tests\corpus\slice.list` and `tests\upstream\pass.list`. If no: record the reason in `study\19` and stop.

*Checked (task 6.1, 2026-10-04): yes.* The release `qb64pe_win-x64-4.7.0-GLFW.7z` has `Makefile`, `internal\c\qbx.cpp`,
`internal\c\libqb.cpp`, `internal\temp` and `internal\c\c_compiler\bin\mingw32-make.exe`. `qb64rust -q -x
--qb64pe-root <release>` built `tests\corpus\slice\s01_*.bas` from scratch, and the program's output equals the
recorded `.output`. The first attempt found a bug: the `#line` directives wrote `\` in the source path as the
octal escape `\134`, which clang rejects in an unevaluated string; the runner only ever passed bare file names, so
no test had shown it. Fixed in `codegen-cpp` (`line_name`: only `\\` and `\"` escapes), with a unit test.

## Risks / Trade-offs

- **Tier-1 time.** About 900 more files (404 + 12 includes + ~450 snippets + 143 + 51) plus 5,500 mutants. →
  Measure after each task; if `cargo test` in debug grows past a minute, run the clone sets only in release
  (`#[cfg_attr(debug_assertions, ignore)]`) and say so in `crates\README.md`.
- **The marker heuristic hides real syntax errors** (a program with a genuine error at `MOD` reported as
  unsupported). → Acceptable while it can only make us *reject* more, never accept; the must-reject check still
  sees an error. The language server shows marked errors differently, not never.
- **Large initial lists.** `known_false_errors.list` will hold most of the 404 + 143 at first. → That is the
  measurement; the list is printed as a count in the test output, and parser breadth works it down.
- **Upstream copy drifts from the clone** when the clone is updated. → The comparison test fails; rerun the script
  with `--commit-ok`.
- **Snippet labels depend on the scratch environment** (a snippet that `$INCLUDE`s a missing file). → They are
  labels of what `qb64pe.exe` says for that text alone, which is what our front end sees too.

## Open Questions

None blocking. Whether the CI job is possible is answered by task 6.1.
