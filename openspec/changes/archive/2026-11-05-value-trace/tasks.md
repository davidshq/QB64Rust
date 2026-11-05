# Tasks

Standing rules: CLAUDE.md.

## 1. The option and the empty record

- [x] 1.1 Add `--trace-values` to `crates\driver\src\main.rs`, a `trace` field to `compile.rs`, and a `Trace`
      value in `lib.rs` built from `coverage_files`, as `--coverage` is (design D7). Check how the build cache
      keys a build and add the option to its key if needed. Verify with `cargo test -p qb64rust-driver`: the
      `--help` text names the option, and a CLI test shows that the C++ of a program built without it has not
      changed.
- [x] 1.2 Add `crates\codegen-cpp\src\trace.rs` and the run-time block `crates\codegen-cpp\src\trace_rt.h`, taken
      in with `include_str!` (design D3). Start with the writer of an empty record (`trace 1`, the files, `end`),
      registered with `atexit` and `at_quick_exit`. Verify with a CLI test in `crates\driver\tests\it\cli.rs`: a
      run with `QB64RUST_TRACE` set writes the file lines with the same hashes as a coverage record. A run without
      it writes nothing. Two runs write two records.
- [x] 1.3 Add the `trace` mode to the front-end tests (`crates\driver\tests\it\frontend.rs`, beside `coverage`),
      with a snapshot of a small program's C++ fragments under `tests\frontend\`. Verify that the snapshot test
      passes, and that `LAYOUT.md`'s row for `tests\frontend\` names the new mode.

## 2. The stores

- [x] 2.1 Add `Op::stores` to `crates\ir\src\lib.rs`, beside `Op::may_raise` (design D5): a `match` over `Op` with
      no `_` arm that places each operation in one of the four groups, or in none. Verify with unit tests in the
      `ir` crate: one operation of each group, and `Release` in none.
- [x] 2.2 Number the entries in `trace.rs` (`Entries::entry`, design D2): scalars, arrays, member paths, and
      parameters per procedure. `Storage::Temp` variables get none. Verify with unit tests in `trace.rs`: two `i`s
      in two procedures, `x(1)` and `x(2)` as one entry, `p.pos.x` as its own entry, `a%` and `a$` as two entries
      named `A`.
- [x] 2.3 Emit the record call where each store by name is made (design D1): in `Emitter::store` for a variable,
      an element (under the store's guard, at `tmp_long`) and a place through a member (under `mbr_ok`); in
      `copy_whole` for each scalar member; and after a built-in that is given a place, through an address taken
      once when the place has an index. Check every target of `INPUT`, `LINE INPUT` and `READ` in `io.rs`: one that
      does not go through `Emitter::store` gets the same call. Verify with a CLI test of the spec's scenarios
      "A FOR loop", "SWAP", "An array", "A member path", "A _BIT", "A store that an error skips" and
      "A FUNCTION in an index". Add a `_BIT * 3` array element, a statement below `$CHECKING:OFF`, and a statement
      that a handler retries with `RESUME`, which counts its store twice.
- [x] 2.4 Record the arguments passed by reference (design D1), for an `Op::Call` and for an `ExprKind::CallProc`
      in an expression. A variable or a member with no index is read after the statement and recorded when it
      differs from the entry's last recorded value. An element or a whole array gets an `alias` line with a
      counter. Verify with a CLI test of the scenarios "A SUB that changes its argument", "A SUB that leaves its
      argument alone" and "An array passed to a SUB", and of a `FUNCTION` that changes its argument inside an
      expression.
- [x] 2.5 Count the bulk stores and the stores at an address by site (design D5) and write the `untraced` lines.
      Verify with a CLI test: `POKE` twice on one line, a `_MEMPUT` and a graphics `GET` into an array each give a
      line with no name and the right count. `b() = a()`, `ERASE b` and `REDIM b(5)` each give a line that names
      `B()`.
- [x] 2.6 Check "The same program" now, before the values are kept. Add `--trace-values` to the test runner, as
      `--coverage` was added (`tools\legacy_tests\run_legacy_tests.py`). Build the corpus suite and the
      verification families `v18` and `v24` with the option, and compare each program's output with its recorded
      output. Verify that the runner's own tests pass and that every program matches. A program that does not
      match is a bug to fix here, not a row to list.

      *Notes, 2026-11-05.* The run-time part that keeps the values was written together with the record calls,
      so this run already kept the values. Every program gave its recorded output:
      - the whole corpus, 521 tests: 514 pass, 7 known failures, none failing. These are the counts of the
        build without the option;
      - `v18`: 67 of 67 pass;
      - `v24`: 371 pass, 1 known failure (`v24_h_member_store`, `QB64PE-DIVERGENCES.md` D-004), none failing.

      `slice/s75_sound_wait` ended with exit code 3221226356 once at `--jobs 8` and passed when the runner ran
      it again alone. It then passed 5 of 5 times alone with the option. `STATUS.md` already lists it as
      crashing under load. The runner's own tests: 299 pass.

## 3. The values

- [x] 3.1 Keep the distinct values, counts, first sites, store counts, after-call counts and ranges (design D3,
      D4). Key numbers by their bits, and strings by their text, or by length and hash above 4096 bytes. A `NaN`
      leaves the range as it is. Write the texts at exit (`%.9g`, `%.17g`, `%.21Lg`, decimal). Verify with CLI
      tests of "A run's record" from `compiler/cli`, and of "The limit", "A range past the limit" and "A SINGLE"
      from `compiler/value-trace`. A string test checks a tab and a line end inside a value.
- [x] 3.2 Check the ends of a run as CLI tests: the scenario "A runtime error", and "The window closed during a
      loop". Then run the check of task 2.6 once more, with the values kept. This whole-corpus run is done once
      for this change and does not join `just full`; the CLI tests of this change are the permanent check.
      Verify that every program matches, and write the counts of the run in this task's notes.

      *Notes, 2026-11-05.* The run of task 2.6 is this run. It was made with the values kept, and nothing in
      the emitter or the run-time part changed after it, so it was not made a second time. Its counts: corpus
      514 pass and 7 known failures of 521; `v18` 67 of 67; `v24` 371 pass and 1 known failure. The corpus
      runs wrote 495 records, each whole, with 5,838 entries, 1,683 untraced lines and 361 alias lines. The
      window test closes the window with PowerShell's `CloseMainWindow`, so it is a Windows test.
- [x] 3.3 Measure the cost on Civil War Strategy's copy `g12_game`: the run time of one script with and without the
      option. Write the numbers in `tools\trace\README.md`, under "Cost". Verify that the README gives them.

## 4. The report tool

- [x] 4.1 Write `tools\trace\value_trace.py` (design D8): read and merge records, check file hashes through
      `tools\coverage\coverage.py`'s reader, and print the text report, `--json` and `--var`. Print an array's
      aliases and bulk stores under its block. Write `tools\trace\test_trace.py` with the spec's scenarios "Two
      runs merged", "An alias in the report" and "Records of another program", and a record with a cut entry.
      Verify that `python -m pytest tools\trace` passes and that `just quick` runs the new tests. If it does not,
      add them to `tools\check`'s Python test step.
- [x] 4.2 Write `tools\trace\README.md`: the record format, version 1 (design D6), the string key above 4096
      bytes, what is not traced, that a variable's starting value is not a store, and an example run. Add a row
      for `tools\trace\` to `LAYOUT.md`. Verify that the README's example commands run as written.

## 5. Documents and closing

- [x] 5.1 Add a section "Recording the values of variables" to `MANUAL.md`, after "Measuring which lines run",
      and the option to its table of options. Verify that the commands in the section run as written.
- [x] 5.2 Add a `DECISIONS.md` row for the value-trace build: an option of our own, no QB64pe equivalent, so no
      `QB64PE-DIVERGENCES.md` row. Update `STATUS.md`'s "Next". Run `just quick` in the background and report what
      ran. Verify with `openspec validate value-trace --strict`.

## 6. Bulk stores that bring new values (user, 2026-11-05: "do it")

Added after the first build. The report left an array filled by `b() = a()` or `GET #` without values or a
range (design D9).

- [x] 6.1 Record every element after a copy of a whole array (`CopyArray`), read through the target's descriptor
      where the copy is made. Record the elements `_ARRAYCOPY` of one dimension wrote. Verify with the CLI test
      `trace_records_bulk_stores`: `B()` holds the four values copied, a `_BIT` array too, and `C()` holds only
      the two elements `_ARRAYCOPY a(1 TO 2) TO c(4)` wrote.
- [x] 6.2 Record every element after `GET #` and `_MEMGET` into a whole array, and the elements `_UCHARPOS`
      fills, read by the array's name. Verify with the same test (`F()`, `G()`) and the `trace` snapshot of
      `tests\frontend\value_trace_trace.bas` (`_UCHARPOS`).
- [x] 6.3 Record each element of a member array inside a copy of a whole type, a `SWAP` or a `GET #` of one, in
      a loop over the member's bounds. Verify with the same test (`Q.V()`).
- [x] 6.4 Trace a copy of a type that owns `_DYNAMIC` member arrays through an index: the copy's last part
      leaves the target's address in a temporary (`copy_owner`'s `hold`). Verify with the same test: `O().N`
      holds 3, and `f1` in `o(f1(i)) = one` is called three times, as in the build without the option.
- [x] 6.5 Read the strings of every `FIELD` again after `FIELD` and after `GET #` into a record buffer
      (`trace_fields`). Verify with the same test: `FA` holds four NUL bytes, `abcd` twice and `1234`, and the
      record has no untraced line for the `FIELD` or the `GET #`.
- [x] 6.6 Keep as counted lines: `ERASE`, `DIM`, `REDIM`, a whole `_MEM`, a `_DYNAMIC` member array inside a
      copy, and the bulk stores whose target needs an index (`_ARRAYCOPY` of several dimensions, `GET #` from an
      element on, `_MEMGET` into a member array through an index). Verify with `trace_counts_untraced_stores`.
- [x] 6.7 Update the delta spec ("Bulk stores that are traced", "Bulk stores that are counted", "The strings
      of a FIELD"), the design (D9), the proposal, `tools\trace\README.md`, `MANUAL.md` and `crates\README.md`.
      Verify with `openspec validate value-trace --strict`.
- [x] 6.8 Run the check of task 2.6 again, and the trace CLI tests on Windows and in WSL.

      *Notes, 2026-11-05.* Every program built with the option gave its recorded output again: the whole
      corpus 514 pass and 7 known failures of 521, none failing; `v18` 67 of 67; `v24` 371 pass and 1 known
      failure (`v24_h_member_store`, D-004). No program was run a second time by the runner. The corpus runs
      wrote 495 whole records with 5,956 entries (5,838 before) and 1,507 untraced lines (1,683 before). The
      nine `cli::trace_…` tests pass on Windows; the eight that are not Windows-only pass in WSL with `g++`.

- [x] 6.9 Fix the three findings of the review of section 6 (2026-11-05), each with a test.
      - `_FLOAT` elements of an array variable were read 32 bytes apart; they are 16 bytes apart. `b() = a()`
        of 1.5, 2.5, 3.5, 4.5 recorded 1.5, 3.5, 0, 0. Test: `FL2()` in `trace_records_bulk_stores`.
      - The count `_UCHARPOS` returns was held in a variable with no starting value. A call in a part of a value
        that is not evaluated (`_IIF`, `_ANDALSO`) then recorded elements by an unset count. The variable now
        starts at 0 and is put back to 0 after use. Test: the `trace` snapshot.
      - A copy of an array of a type with `_DYNAMIC` members, with other bounds, raises error 9 and is still
        made. Its records stood under the test for a pending error, so it was not recorded. Test: `OB().N` in
        `trace_records_bulk_stores`.

      The corpus run was not made again: the fixes change only what the traced build records. The nine
      `cli::trace_…` tests pass on Windows and the eight that are not Windows-only in WSL.

## Workflow follow-up

- Sync the delta specs into `openspec\specs\` and archive the change after the user's review.
