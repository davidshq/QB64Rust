# Tasks

Standing rules: CLAUDE.md.

## 1. The groundwork

- [x] 1.1 Add `code: Option<&'static str>` to `Diagnostic` in `crates/base/src/lib.rs`, `None` from every existing
      constructor, and `Diagnostic::lint(span, code, msg)` (design D2). Verify with `cargo test -p qb64rust-base`
      and `cargo build` of the workspace.
- [x] 1.2 Add `crates/sema/src/lints.rs` with `run(prog, main, diags)`, the five code constants and the
      main-file filter (design D1, D3). Call it from `check_with` after the last `check_once`, only when there is
      no error. Verify with `cargo test -p qb64rust-sema`: nothing changes yet.
- [x] 1.3 Add the `warnings` mode to `crates/driver/tests/it/frontend.rs` (design D10), with a first fixture that
      has a `CONST` chain of `^` so the snapshot shows an existing warning. Verify that the frontend test passes,
      and that `LAYOUT.md`'s row for `tests/frontend/` names the mode.
- [x] 1.4 Pass the code to the client in `crates/lsp/src/analysis.rs`. Verify with `cargo test -p qb64rust-lsp`.

## 2. The four table lints

- [x] 2.1 `unused-label` (design D6). Fixture `tests/frontend/lints_unused_label_warnings.bas` with the spec's
      scenarios: a named label nothing names, one named only by `RESTORE`, `RESUME`, `ON ERROR GOTO` and
      `ON TIMER … GOSUB`, and a numbered program. Verify the snapshot shows only the expected warnings.
- [x] 2.2 `unused-procedure` (design D5). Add the set of declaring spans to `Symbols`, filled at the header and at
      `DECLARE` lines. Fixture `lints_unused_procedure_warnings.bas`: a SUB declared and never called, a FUNCTION
      that calls itself, a SUB named by `ON TIMER`, `SUB _GL`, a `DECLARE LIBRARY` routine. A second fixture,
      `lints_test.bas` with mode `warnings`, holds a test SUB and gives no warning. A third includes a toolbox
      file from `tests/frontend/inc/` that defines several SUBs, and calls one. Verify the snapshots and that
      go-to-definition tests in `crates/driver/tests/it/language_features.rs` still pass.
- [x] 2.3 `used-once` (design D4). Add `implicit` to `Var` and set it in `check/decl.rs`. Collect the `FOR` counter
      and `INPUT`/`LINE INPUT` targets in `lints.rs`. Fixture `lints_used_once_warnings.bas`: a misspelled read, a
      variable set and read, a declared one, `x` and `x$`, a loop counter, `INPUT` into a dummy, an implicit
      array used once, and the same in a SUB. A fixture with `OPTION _EXPLICIT` gives none. Verify the
      snapshots.
- [x] 2.4 `hides-shared` (design D7). Push to `Program.hidden_shared` in `check/decl.rs` and give the warning in
      `lints.rs`. Fixture `lints_hides_shared_warnings.bas`: a local `DIM` and a `REDIM` of a `DIM SHARED` name,
      a `SHARED score` in a SUB, and a local of the same name and another type. Check what the compiler does
      today with a parameter named as a `DIM SHARED` variable, and write the result in "Found while building".
      Verify the snapshot.

## 3. Unreachable code

- [x] 3.1 `unreachable` (design D8). Find the statement kinds that write no code and write them in "Found while
      building". Fixture `lints_unreachable_warnings.bas`: after `GOTO`, `END`, `EXIT SUB`, `EXIT DO` and
      `RETURN`, two dead statements giving one warning; `END` then a handler label; `END` then `DATA`; a
      single-line `IF … THEN GOTO`; `ERROR 5` followed by code; `ON n GOTO` followed by code. Verify the
      snapshot.

## 4. The CLI and the editor

- [x] 4.1 `--report warnings` in `crates/driver/src/report.rs` and `main.rs` (design D9). CLI tests for the
      spec's two scenarios, and for `-w` printing a lint line while the executable is still built (without a
      build where the test only needs the front end). Verify with `cargo test -p qb64rust-driver`.
- [x] 4.2 A check test in `crates/driver/tests/it/check.rs` for the `unused-label` code. Read every existing test
      that counts or compares warnings (`check.rs` `warning`, `cli.rs` around lines 1575 and 3694–3815) and
      adjust each so it keeps its point (design, "Risks"). Verify with `cargo test -p qb64rust-driver`.
- [x] 4.3 `MANUAL.md`: a section on the lints under "Options" or after it, with each code, what it means, and the
      main-file rule. Update the `-w` row. Verify the text against the snapshots.

## 5. QB64pe's warnings

- [x] 5.1 Write the `verification\v68_*` programs (design D11). For "Unused variable": a `DIM` never used, an
      implicit variable named once, set once, read once, a `FOR` counter, an `INPUT` target, an array used only in
      `LBOUND`, a FUNCTION's result, a parameter, `SHARED`, `COMMON SHARED`, a variable used only in an included
      file. Also an empty `SELECT CASE` with and without a statement before `END SELECT`, a `CONST` repeated with
      the same value and with another, and `CHAIN` and `RUN` under `$DEBUG`. Record each with `qb64pe -w -z`
      through the family tooling, a few programs at a time. Add the family's line to `verification\README.md`.
      Write in "Found while building" what decides each rule, and whether `used-once` keeps a case of its own.
- [x] 5.2 Build "Unused variable" in `lints.rs`, and "Empty SELECT CASE block" and "Duplicate constant
      definition" where the checker meets them, queued on `Program` (design D11). Build "Feature incompatible
      with $DEBUG mode" only if 5.1 shows it outside the IDE. Settle the overlap with `used-once` (spec, "One
      warning per variable"). Verify with a CLI test that compares our `-w` lines with the recorded ones for each
      `v68` program, and with frontend `warnings` snapshots.
- [x] 5.3 Add a `DIVERGENCES-QB64PE.md` row only for a difference 5.1 found that we keep. Verify the row names
      its `v68` program.

## 6. Measure and close

- [x] 6.1 Run `qb64rust --report warnings` over `tests/corpus`, `tests/upstream` and the programs of
      `tests/verification/pass.list` on the host (design D9: front end only). Read a sample of each lint's
      warnings, QB64pe's four included. Narrow any of our lints that is mostly wrong, and run it again. Write the
      counts and each narrowing in "Found while building".
- [x] 6.2 Records: a `DECISIONS.md` row for the lints. `SOMEDAY.md` rows for: silencing lints, or turning them on for included files; dead code after an `IF` whose every branch leaves;
      a procedure called only by unused ones. Update `LAYOUT.md` for `crates/sema/src/lints.rs` if `sema`'s
      files have rows. Update `STATUS.md`'s "Next". Verify with `python tools/repo_check/check_repo.py`.
- [x] 6.3 Run `just quick` in the background and read its log. Verify that it passes, or name what failed and why.

## Workflow follow-up

- Merge the delta specs into `openspec/specs/` and archive the change (`/opsx:archive`).
