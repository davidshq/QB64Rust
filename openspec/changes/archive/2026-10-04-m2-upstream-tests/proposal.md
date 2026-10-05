# Proposal

## Why

No Rust test reads QB64pe's own test suite (404 programs), its 143 `qbasic_testcases`, or the 72,700 lines of
BASIC in the old compiler's sources, although "no panic, exact round trip, no false error, rejects what the old
compiler rejects" costs nothing to check on them. The next step, parser breadth over the whole language, needs
exactly that as its yardstick, and progress on the upstream suite needs an honest denominator: 125 of the 404
programs use array features deferred to `SOMEDAY.md`, so 279 are in reach (`study\22`). This change builds the
test base first, before more language work (order of `study\22` §5, step 2).

## What Changes

- Copy the text files of `..\QB64pe\tests\compile_tests` (3.1 MB of 239 MB; programs, expected output and errors,
  includes, sidecars) into `tests\upstream\compile_tests\` at the pinned commit, with QB64pe's licence and a
  script that redoes the copy; binary assets stay in the clone.
- Diagnostics carry a **"not supported yet" marker**: every diagnostic says whether it reports an error in the
  program or a construct the compiler does not handle yet. The command line shows the marker.
- Tier 1 gains, over the corpus, the upstream copy, the extracted QB64Fresh snippets, and (when the clone is
  present) `qbasic_testcases` and the old compiler's sources:
  - no panic and an exact round trip for every BASIC file;
  - **no false error**: a program the old compiler accepts gets no unmarked error, enforced by a shrink-only list
    of the programs that still do (`tests\known_false_errors.list`; parser breadth empties it);
  - **must reject**: a program the old compiler rejects gets at least one error, and once it gets an unmarked one
    it keeps getting one (shrink-only `tests\known_unsupported_rejections.list`);
  - the copy matches the clone (when present);
  - a **seeded mutation test**: byte-level edits of corpus programs give no panic and an exact round trip.
- Tier 2 over the upstream suite: the runner's `--list` works for `--suite compile`; `tests\upstream\pass.list`
  (ratchet) and `tests\upstream\deferred.list` (the 125, with the feature each needs). `qb64rust` accepts the
  `-f:<setting>=<value>` options the compile suite passes. Progress is reported as x of 279.
- QB64Fresh's 449 inline test snippets (`<qb64contain>\QB64Fresh\tests\integration_tests.rs`) extracted by a script
  into `tests\snippets\qb64fresh\`, deduplicated, each labelled by `qb64pe.exe` (an `.err` file when it rejects
  the snippet). Only the BASIC text is taken, not the assertions (`CLAUDE.md` rule 5).
- Refactor: `Ty`, `BinOp` and the conversion kind are defined once (in `sema`) and used by `ir` and `codegen-cpp`
  (`study\20` §3.4). No behaviour changes.
- A CI job for tier 2 (the slice list and the upstream pass list) on the QB64pe release, if the release can build
  `qb64rust`'s output; otherwise the reason is recorded.

## Capabilities

### New Capabilities
- `testing/upstream-tests`: the copy of QB64pe's test suite, the external inputs read from the clone, the
  QB64Fresh snippets, and the tier-1 and tier-2 checks over them (lists, denominator, provenance).

### Modified Capabilities
- `compiler/pipeline`: new requirement that diagnostics for constructs not handled yet are marked as such.
- `compiler/cli`: the diagnostics output shows the marker; the compile command accepts `-f:` settings.
- `testing/compiler-tests`: the tier-1 corpus checks use the marker and the shared lists; new requirement for the
  seeded mutation test.

## Impact

- New: `tests\upstream\`, `tests\snippets\`, `tests\known_false_errors.list`,
  `tests\known_unsupported_rejections.list`, `tools\upstream\copy_upstream_tests.py`,
  `tools\snippets\extract_qb64fresh_snippets.py`, test files in `crates\driver\tests\`.
- Changed: `crates\base` (`Diagnostic` gains the marker), every "not supported yet" site in `crates\syntax` and
  `crates\sema` (48 messages), `crates\driver` (rendering, `-f:` options), `crates\ir` and `crates\codegen-cpp`
  (shared enums), `tools\legacy_tests\run_legacy_tests.py` (`--list` for `--suite compile`), `.gitattributes`,
  possibly `.github\workflows\rust.yml` or a new workflow, `tools\repo_check` allowances if upstream files need
  them, `crates\README.md`, `tests\corpus\README.md`, `CLAUDE.md` layout, `STATUS.md`.
- Tier-1 time grows (about 900 more files through the front end); budget: `cargo test` stays under a minute in
  debug on the development machine.
- No change to generated code.
