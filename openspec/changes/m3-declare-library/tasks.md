# Tasks

Order: group 1, then 2, 3, 4, 5, 6. Each of groups 2 to 5 ends with `cargo fmt --check`, `cargo clippy
--workspace --all-targets -- -D warnings` and `cargo test` (tier 1) green and with its tests, lists and
`crates\README.md` lines in place, so the change can stop after any of them.

Standing rules for every task: work inline, no subagents; nothing staged or committed by Claude: each group ends
by naming the files to stage and a commit message; text with backslashes goes through the Edit and Write tools or
a script file; `python tools\repo_check\check_repo.py --untracked` is clean before a group is reported done.
**Test cadence** (`CLAUDE.md` working rule 9): while working, tier 1 and only the programs the work touches
(`--glob`, `--category`, or a list with `--pass-cache target\pass-cache`), each run's output sent to a log under
`target\`; tier 2, the run without folding and the full suites run once, in group 6. A behaviour of the old
compiler that looks questionable is implemented its way and listed in `SOMEDAY.md` ("QB64pe behaviours to
review"), apart from the two divergences of the proposal. The libraries of `verification\v37_*` are built first
with `python verification\v37_lib\build_libs.py`.

## 1. Measurements

- [x] 1.1 Measure with `qb64pe.exe` before the proposal (`verification\v37_*`, 65 programs, recorded 2026-10-15;
  11 more by task 1.3):
  `v37_a`–`v37_j` and `v37_x01`–`x55`, the C sources and headers in `v37_lib\` with `build_libs.py`, an included
  file in `v37_inc\`. Verify: every program has its `.compile.txt` and `.out.txt`.
- [x] 1.2 Record the facts: a `v37` paragraph in `study\00` §5, `v37*` in `CLAUDE.md`'s layout table, the
  `DECISIONS.md` row (the user's choice of the change, `errtest` left out, Claude's defaults), the built
  libraries git-ignored. Verify: `check_repo.py --untracked` is clean.
- [x] 1.3 By the review of the proposal, `v37_x56`–`x66` (11 programs, recorded 2026-10-15): which `BYVAL` 0 is
  written `NULL` (`(0)`, `0&`, a `CONST` 0 are, `1 - 1` is not; an overloaded function given `NULL`), the
  default types of a FUNCTION and a parameter told apart by a non-integer value (`x35`–`x38` could not), a
  `STRING * n` member followed by another member (no NUL: C reads on). Facts in `study\00` §5; design D3, D4
  and the spec follow them, with the review's other points (the upstream binaries, a name in two libraries,
  the line of the `nm` errors, a header's own includes). Verify: every program has its `.compile.txt` and
  `.out.txt`; `check_repo.py --untracked` is clean.

## 2. The block and its declarations in `sema`

- [x] 2.1 `Library`, `LibraryName`, `External` and `Arg::Value` in `sema\src\lib.rs` (design D1); the block no
  longer marked (`check\blocks.rs`); pass 1 enters each declaration as a `Proc` with its `External`, in a SUB too,
  with the default types (`x35`–`x38`), `ALIAS` as written (`x45`). Verify: a `typed` front-end test of a block
  with each form; the follow-on marks of the 8 upstream programs are gone (`qb64rust -z` on each prints no
  "not supported yet" for them).
  As built: a module of its own, `sema\check\library.rs`; declarations are entered after the program's own
  procedures (so a clash is reported at the library's line wherever it stands) and before any parameters.
  `Library` has no `file` field: its `span` names the file. The typed test is `library_typed.bas` (with
  `inc\library.bi`). A parameter named like a built-in (`BYVAL val AS LONG`) is accepted: QB64pe's
  `declare_library_header` tests use one, and a library parameter is no BASIC variable. Until groups 3 and 4 the
  driver marks each block "not supported yet" before `-z`, a build, the `ir` and `cpp` dumps and the demand
  report (`mark_libraries` in `driver\src\lib.rs`; by the review of group 2, which found the dumps writing a stub
  for a library procedure and the report counting the 8 programs as clean; CLI test
  `a_library_block_is_marked_until_it_builds`), so `-z` on the 8 upstream programs prints that one mark at the
  block and nothing at the uses; the 8 are on
  `tests\known_clean_not_passing.list` until task 5.2. `blocks_marked.bas` lost its library part and
  `blocks_recovery`'s snapshot its mark. `Arg::Value` and `Arg::Null` reach the IR's facts, dump and validation
  and the emitter's `args` (the value as it is, `NULL`): the IR shares `sema`'s `Arg`, so the build needed them
  now; their check stays with tasks 3.1 and 3.2.
- [x] 2.2 The rejected declarations and names (`x03`–`x07`, `x10`–`x12`, `x15`, `x25`, `x26`, `x28`, `x29`,
  `x32`, `x40`, `x44`, `x46`–`x48`, `x19`–`x21` as the parser gives them now), each with the old compiler's
  reason. Verify: `library_errors.bas` (`check-fail`) holds each form; its snapshot reviewed against the
  `.compile.txt` files.
  As built: `x05`, `x07` and `x12` are the parser's errors (a name without quotes, two names without a comma,
  `STATIC` after a declaration; before, `x05` was "expected the end of the statement", `x07` was accepted, `x12`
  was "not supported yet"); `x46`, `x47`, `x19`–`x21` keep the parser's. `x04` (`CUSTOMTYPE` without a name) is
  no `sema` error: the old compiler's C++ fails, and so will this compiler's (task 4.4). `x08` (`ALIAS` in
  `DECLARE SUB`, which the old compiler accepts) and `x09` stay as they were, as design D1 says. Not in the list
  and marked: `x22`, a library FUNCTION beside a program SUB of the name ("not supported yet"; which a use names
  was not measured).
- [x] 2.3 Calls: a library SUB and FUNCTION called through the shared checks, a `BYVAL` argument as
  `Arg::Value`, the rejected uses (`x14`, `x33`, `x34`, `x41`, `x50`, `x54`). Verify: `library_errors.bas` holds
  the uses; a `typed` test shows `Ref`, `Temp` and `Value` arguments; the demand report over `tests\upstream`
  no longer names `DECLARE LIBRARY`.
  Since the review of group 2 the report names the driver's mark instead ("building a program with `DECLARE
  LIBRARY` …", task 2.1), at the block and nowhere else; what this check asked for is that `sema`'s mark and
  the follow-on marks at the uses are gone.
  As built, beside the design: **`Arg::Null`**, a second new kind of argument. Which `BYVAL` argument is C++
  `NULL` is decided in `sema` from the argument as written (`null_arg`: `0`, `0%`, `0&`, in parentheses, a
  `CONST` whose value is integer 0), not by `codegen-cpp` from its C++ text as D3 first said: `sema` folds
  `1 - 1` into a 0, so the emitter could not tell it from `0`. `0%` is taken with `0&` (not measured); `0#`, `0.0` and
  `_FALSE` are not (not measured). `x34` (a SUB as a value) was already "name already in use"; `x50` was already
  an error. Met on the way, not of this group: `v37_b` has `18446744073709551615` (above the `_INTEGER64` range),
  "not supported yet: unsigned literals", which task 4.4 needs.

## 3. The IR and the C++

- [x] 3.1 The IR carries the libraries and the external part of a procedure; `Arg::Value` in the IR, its dump
  and validation; the `ir` and `cpp` dumps no longer call `mark_libraries` (`driver\src\main.rs` and the
  front-end tests' `frontend.rs`), and the CLI test `a_library_block_is_marked_until_it_builds` loses its dump
  part. Verify: `library_ir.bas` (`ir`).
  As built: `Library`, `LibraryKind`, `LibraryName`, `External` are `sema`'s, re-exported by the IR (as `Arg` is).
  Validation: a library procedure's library exists, it has no statements, one `BYVAL` flag per parameter and no
  array parameter; a `BYVAL` parameter takes `Arg::Value` or `Arg::Null` and no other parameter does; a `Value`
  is a number (unit test `library_procedures`). A `Value` is not checked to have the parameter's type: `sema`
  leaves an integer literal to a `DOUBLE` parameter an integer (the old compiler's text, which picks a C++
  overload). The CLI test now checks that both dumps succeed and show the call.
- [x] 3.2 `codegen-cpp\src\library.rs`: the header included once (D-023), the prototypes with their linkage, the
  call, its arguments (place, copy, string, TYPE, value with no cast, `NULL` for `Arg::Null`, which `sema`
  decides: task 2.3) and the result's cast and string result (design D3), with
  `Resolved` as an input of `emit`. Verify: `library_cpp.bas` (`cpp`) compared line by line with the
  old compiler's C++ of `v37_b`, `v37_c`, `v37_d`, `v37_e`, `v37_f` (kept by hand under `target\`, not in the
  repo).
  As built, beside the design: `ResolvedLibrary` has `headers: Vec<PathBuf>` (a block may name several headers),
  not one `Option`; `Resolved::linkage` maps each declaration to its linkage, C where it has none, so
  `Resolved::default()` is "nothing found". The libraries' text stands before the program's prototypes in
  `regsf.txt`, block by block, not at each block's place (a program prototype names no library type). An
  `#include` writes the path with `/` (GCC reads a backslash there as it is; the old compiler doubles it). A block
  with no name writes no prototype (measured, `v37_a`: the C runtime's functions are declared already). A
  `BYVAL` value: `Widen`, `Truncate` and `Nearest` conversions are left out and C converts; `RoundEven` stays
  (`qbr`), as the old compiler writes them. A library procedure gets no `mainK`/`dataK`/`freeK`; K counts program
  procedures. `args` was split into `arg` (one argument), which the library's arguments reuse. Compared with the
  old compiler's C++ (`qb64pe.exe -z` of `v37_a`–`v37_h`, under `target\v37cpp\`): the same apart from layout and
  two folded forms. **Found:** folding writes `h_b%%(300)` as `h_b(44)` (C gives 44 for 300 too) and `1 - 1` to a
  `BYVAL _OFFSET` as `0ll`, which C++ also takes as a null pointer, so `v37_x57`, whose C++ fails with the old
  compiler, compiles here with folding on and fails with `QB64RUST_NO_FOLD=1` (the 2026-10-08 rule: a failed
  compile may be fixed; task 4.4 records it). The CLI's `cpp` dump uses `Resolved::default()` until task 4.1, so
  a named block appears there as one without a header.
- [x] 3.3 `DYNAMIC` and `CUSTOMTYPE`: their typedefs and pointers, the start-up lines in `maindata.txt` with
  errors 259 and 260, the `extern "C"` line of a `CUSTOMTYPE` C name. Verify: `library_cpp.bas` holds a block of
  each, compared with the old compiler's C++ of `v37_g` and `v37_h`.
  As built: the start-up lines come first in `maindata.txt`, before the variables, as the old compiler's. The
  handle's name is the old compiler's (`qb64pe.bas` 4526–4567, 5243–5249): `DLL_` and the written name without its
  version (`.1.0`, `:1`) and folder, each byte that is no letter or digit an `_` (`"my.lib"` gives `DLL_my_lib`;
  unit test `dll_handle_names`); `LoadLibrary` gets `ResolvedLibrary::dll`, or the written name with `.dll` when
  there is none.
  **Fixed by the review of group 3:** a `CUSTOMTYPE` function of C++ linkage without a header is declared as the old
  compiler declares it, `extern void <nm's demangled text>;` in `global.txt` (`qb64pe.bas` 13819–13827; it had no
  declaration, so its C++ failed): `Linkage::Cpp` carries that text, which task 4.2 takes from `nm`. Two `DYNAMIC`
  blocks of one DLL share one handle, declared and loaded once, and a different DLL whose handle name is taken gets
  `_2`, `_3`, …: the old compiler declares the handle once per block, so its C++ fails for both (read in
  `qb64pe.bas` 5206, 5271, not measured; task 4.4 measures it; a failed compile fixed by the 2026-10-08 rule; task 4.4 records it, task 6.1 decides whether it is a
  register row). The header de-duplication of D-023 compares paths as given: task 4.1 gives each header its
  canonical path.

## 4. The lookup and the build in the driver

- [ ] 4.1 `driver\src\library.rs`: the lookup of design D2 with the clone as the old compiler's folder, its
  errors at the block's line; each header given by its canonical path, so that D-023 sees one file as one path
  (task 3.3). Verify: unit tests in a temporary folder for each step of the order (a `.a`
  beside with `./`, without it `x18`, a header from an included file's folder, `.hpp`, a version, System32, an
  empty name); `x01`, `x02`, `x52`, `x53` give the old compiler's text.
- [ ] 4.2 Linkage by the clone's `nm` over all files of a block (D-024), "Could not find sub/function …" and
  "Unable to resolve multiple instances …"; a C++ match gives `Linkage::Cpp` the text of `nm`'s line from the
  name to the first `)` (task 3.3). Verify: unit tests on a C and a C++ library built from
  `verification\v37_lib\` (skipped with a message when the clone's toolchain is not there).
- [ ] 4.3 The library files on the link command, the bytes of libraries and headers in the build cache's key;
  the LSP reports the lookup's errors if its crate can reach the function (else noted in `crates\README.md`);
  `mark_libraries` and the rest of its CLI test go (`-z`, a build and the demand report no longer mark a block).
  Verify: a unit test of the key (a changed header changes it); CLI tests (`--ignored`): a C `.a`, a C++ `.a`, a
  `.o`, a DLL beside the program and error 260, D-023 and D-024, each library built into a temporary folder.
- [ ] 4.4 `verification\v37_*` compiled and run with this compiler. Verify: every accepted program prints what
  the old compiler's prints and every rejected one is rejected with its reason, apart from D-023 (`x16`) and
  D-024 (`x24`), which now compile and run; the C++ failures of the old compiler (`x04`, `x17`, `x23`, `x30`,
  `x31`, `x57`) fail here too; two `DYNAMIC` blocks of one DLL (a new `v37_x` program, recorded with `qb64pe.exe` first) compile and run here
  (task 3.3); `x56`–`x59` give the old compiler's result with and without `QB64RUST_NO_FOLD=1`.

## 5. Slice programs and upstream

- [ ] 5.1 Console slice programs in `tests\corpus\slice\`, a header in each `<name>.files\` (the header's text
  taken from `v37_lib\v37h.h`): numbers by value and by reference and the kinds of argument; strings and a TYPE;
  `ALIAS` with C++ names and overloads; `CUSTOMTYPE`; the C runtime with no name and `kernel32` with `DYNAMIC`;
  recorded with `qb64pe.exe`. Verify: each passes on its first run with this compiler; they join `slice.list`.
- [ ] 5.2 Upstream: `declare_library/test`, `declare_library_dynamic/test`, the four `declare_library_header`
  programs, `declare_library_static/test` and `auto_include/am-trigger` join `pass.list` (310 to 318 of 399).
  First the `tier2` job's step "Upstream pass list" in `rust.yml` drops `--compile-tests
  tests/upstream/compile_tests` (design D4: the copy lacks `lib-windows.a` and `lib.dll`, the clone the job
  checks out has them), with the runner's `--compile-tests` help text ("CI passes the copy …") and
  `tests\upstream\README.md` ("CI runs the same list from this copy") brought in line. Verify: the 8 pass run by
  themselves from the clone, as CI will run them; tier 1's list tests pass; the demand report's first line is no
  longer `DECLARE LIBRARY`.

## 6. Close

- [ ] 6.1 Documents: `DIVERGENCES.md` (D-023, D-024), `SOMEDAY.md` (under "QB64pe behaviours to review": the
  block's prototypes beside a header, a result truncated by its cast (`x63`), a `STRING * n` member with no NUL
  (`x66`); what stayed marked; `errtest`'s handlers), `crates\README.md` (also: a header's own includes are not in
  the build cache's key),
  `tests\upstream\README.md`, `tests\corpus\README.md`, `CLAUDE.md`'s layout table (counts), `ROADMAP.md`,
  `STATUS.md`. Verify: `check_repo.py --untracked` is clean.
- [ ] 6.2 Closing runs, once (working rule 9): tier 1 with the command-line tests that build, the runner's tests,
  the slice list with and without folding, the upstream and differential pass lists, the two full suites, the
  demand reports, the shrink-only lists regenerated (the call-site check is not affected: no built-in changes;
  say so); `python tools\testtime\test_time.py`. Verify: numbers in `STATUS.md`; nothing on a pass list fails;
  the files to stage and a commit message named.

## Workflow follow-up

- Archive the change after the user has staged and committed it (`/opsx:archive`).
