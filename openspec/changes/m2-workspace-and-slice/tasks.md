# Tasks

## 1. Records before code (`specs/language/numeric-semantics`, design D11, D12)

- [ ] 1.1 Write `DIVERGENCES.md` (D12) with D-001 (LONG wrap) and D-002 (`_INTEGER64` wrap), each pointing to
  `verification\v11_wrap_o2` / `v12_wrap_int64` and `study\16` §8. Verify: no local paths; the two rows name the
  default and `-O2` behaviour as measured.
- [ ] 1.2 Write the seven `tests\corpus\slice\` programs and `SOURCE.md` (D11). Each starts with `$CONSOLE:ONLY`,
  uses no `PRINT` comma, and covers the scenarios of `specs/language/numeric-semantics` it is named for.
  Verify: each compiles with `qb64pe.exe`.
- [ ] 1.3 Add `--list <file>` to `tools\legacy_tests\run_legacy_tests.py` (one `<group>/<name>` per line, `#`
  comments) and document it in `tools\legacy_tests\README.md`. Write `tests\corpus\slice.list` (D11, 14 programs).
  Verify: `--suite corpus --list tests\corpus\slice.list` with the old compiler runs exactly the listed programs.
- [ ] 1.4 Record the slice group with the old compiler (`--suite corpus --category slice --record`), then check
  it. Compare each recorded result with the numeric spec's scenarios; where they disagree, correct the **spec**
  (it describes observed behaviour) and note it here. Verify: a second `--record` changes no file; the privacy
  grep of the golden-corpus change (D7 there) finds nothing; `s05_instr` answers the `INSTR` start-0 question.

## 2. Workspace (design D1, D2)

- [ ] 2.1 Root `Cargo.toml` (workspace, edition 2024, shared lints: `unsafe_code = "forbid"`,
  `clippy::all`), `rust-toolchain.toml` (1.88.0), the seven crates as empty libraries plus the `qb64rust` binary.
  Verify: `cargo build` and `cargo test` pass; `cargo tree` shows only `insta` and `serde_json` as external crates.
- [ ] 2.2 `base`: `FileId`, `Span`, `SourceMap` with line index (CR LF, LF, lone CR), byte columns, `Diagnostic`.
  Verify: unit tests for line/column of offsets in files with each line-end style, and at end of file.
- [ ] 2.3 `builtins`: `build.rs` reads `tools\builtins\builtins.json` and generates a table (name, kind, argument
  types, optional slots from `specialformat`, return type). Only `INSTR` is lowered in this change. Verify: a test
  finds `INSTR` with three slots, the first optional, return LONG.

## 3. Front end (design D3–D5, D10)

- [ ] 3.1 `syntax` lexer over `&[u8]`: keywords, identifiers with suffixes, numeric literals (all forms of
  `study\02` §1.2, typed later), string literals, `$` metacommands, comments (`'`, `REM`), `:`, line ends,
  trivia. Verify: token snapshot tests; a test with every byte 0x01–0xFF inside a string and a comment.
- [ ] 3.2 Lossless tree (green nodes + cursor) and the parser for the slice subset, with statement-family modules,
  the full precedence table, one-error-per-statement recovery, the 100-error cap and "not supported yet" for other
  keywords. Verify: tier-1 test over all `tests\corpus\**\*.bas`: no panic, exact byte round trip; parse
  snapshots for each statement form; a two-error file reports both.
- [ ] 3.3 Measure with `qb64pe.exe` what a different suffix on a DIMmed name does (D5) and record it in `design.md`.
  Verify: the measurement program and its result are in `verification\` (`v13_…`).
- [ ] 3.4 `sema`: symbols, suffix rules, implicit SINGLE, `DIM`, literal typing, operator computation types,
  explicit conversions, constant folding with `wrapping_*`; `--dump typed` text. Verify: `tests\frontend\` files
  with `typed` mode for every literal-typing and arithmetic-width scenario of the numeric spec; tier-1 test: the
  14 programs of `slice.list` give no diagnostics.
- [ ] 3.5 Test harness for `tests\frontend\` (mode lines, D10) with `insta`. Verify: a file without a mode line
  and a `check-fail` file without errors both fail the harness (checked once with temporary files, then removed).

## 4. Build path (design D7, D8)

- [x] 4.0 Ask the user to confirm D8 (libqb objects built in the clone's git-ignored folders).
  (Done 2026-10-03: approved; recorded in `design.md` D8 and `CLAUDE.md`.)
- [ ] 4.1 Record the old compiler's fragments (`qb64pe -z`) for each slice program into a scratch folder (never
  the repo) and note in `design.md` anything the Context table does not yet cover (string variables, `END`,
  `INSTR` raising). Verify: the Context table is complete for the slice.
- [ ] 4.2 Spike: build a hand-written fragment set (a `PRINT "hello"` program) through the Make overrides of D8,
  with `-fwrapv`. Verify: the executable prints `hello` and the `END` trailer under the corpus runner's console
  helper; the fragments, `qbx.o` and `.sym` are in the build folder.
- [ ] 4.3 Check the clone stays clean and libqb is reused. Verify: `git -C ..\QB64pe status --porcelain` is the
  same before and after a build; after a `qb64pe.exe` build of a `$CONSOLE:ONLY` program, a slice build does not
  rebuild `libqb_make_*.o` (timestamps unchanged). Record the time of a slice build (`study\19` §4).

## 5. IR, emitter, driver (design D6, D7, D9)

- [ ] 5.1 `ir`: types, lowering from the typed tree, text dump. Verify: `ir` snapshots for the lowering pairs:
  each PRINT item form, an assignment with each conversion kind, `INSTR` with and without the optional argument;
  `cargo tree -p qb64rust-ir` does not contain `codegen-cpp`.
- [ ] 5.2 `codegen-cpp`: the fragment set of D7. Verify: `cpp` snapshots for the same pairs; the emitted
  `passed` masks and casts agree with the fragments recorded in 4.1 for the same program.
- [ ] 5.3 `driver`: CLI of D9 and `specs/compiler/cli`, the build of D8, diagnostics output, exit status.
  Verify: each scenario of `specs/compiler/cli` by a test (the build ones marked `#[ignore]` unless the clone is
  present, and run by hand).

## 6. End to end (`specs/compiler/pipeline`, `specs/testing/compiler-tests`)

- [ ] 6.1 Tier 2: `run_legacy_tests.py --suite corpus --qb64 target\release\qb64rust.exe --list
  tests\corpus\slice.list`. Verify: all 14 pass. Note any program that needed a change to the spec or the
  divergence register.
- [ ] 6.2 Run the full corpus with `qb64rust` once (not a pass criterion). Verify: every program outside the list
  fails at compile time with a diagnostic, none crashes the compiler or produces a wrong executable; record the
  counts and the per-program compile time in `tests\corpus\README.md` (input for the runner's parallel option,
  `study\19` §5).

## 7. Documentation

- [ ] 7.1 `crates\README.md` (crate map, how to build and test, how to review snapshots) and a short section in
  `tests\corpus\README.md` for the `slice` group and `slice.list`.
- [ ] 7.2 Update `STATUS.md`, `study\00` §2 and §10, `CLAUDE.md` (layout table: `crates\`, `tests\frontend\`,
  `DIVERGENCES.md`; decisions taken here: own lossless tree over bytes, mode-line syntax, build via Make overrides,
  error cap 100). Verify: `openspec validate m2-workspace-and-slice --strict` passes.
