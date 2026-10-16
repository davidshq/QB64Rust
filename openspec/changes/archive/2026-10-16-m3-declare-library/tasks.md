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
- [x] 3.2 `codegen-cpp\src\library.rs`: the header included once (D-023; reworked: once per name, below), the prototypes with their linkage, the
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
  **Reworked for the answer of 2026-10-16 (`DECISIONS.md`: D-023 and D-024 not built):** a header is
  `#include`d at each block, once per name that finds it, as the old compiler does; nothing is de-duplicated.
  Where that fails in C++ (a header without a guard found twice, `x16`) the driver rejects the program first
  (task 4.1). `library_cpp.bas` includes `v37h` (which has `#pragma once`) at its three blocks.
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
  **Reworked with task 3.2 (the answer of 2026-10-16 applied to this case too):** each `DYNAMIC` block declares
  and loads its own handle, as the old compiler does, and the driver rejects a second block whose handle has the
  name of an earlier one's (measured by task 4.4, `v37_x67`: the old compiler's C++ fails, "redefinition of
  'DLL_kernel32'"). The `_2` names and the shared handle are gone; `dll_handle` is public for the driver.

## 4. The lookup and the build in the driver

- [x] 4.1 `driver\src\library.rs`: the lookup of design D2 with the clone as the old compiler's folder, its
  errors at the block's line; each header given by its canonical path, so that D-023 sees one file as one path
  (task 3.3; *D-023 not built: the canonical path serves the check below*). Verify: unit tests in a temporary folder for each step of the order (a `.a`
  beside with `./`, without it `x18`, a header from an included file's folder, `.hpp`, a version, System32, an
  empty name); `x01`, `x02`, `x52`, `x53` give the old compiler's text.
  As built: `find` (one name) and `resolve` (the program); unit tests `library_files_and_headers_in_the_old_
  compilers_order`, `dlls_in_the_old_compilers_order`. A "not found" error stands at the name, the others at the
  block. D-023 is not built (`DECISIONS.md` 2026-10-16): the canonical path serves the check instead, a header
  found a second time that has neither `#pragma once` nor any `#if` line is an error before the C++ ("… has no
  include guard …: the old compiler includes it twice and its C++ fails", `x16`; `has_guard` errs towards
  accepting, leaving any header with an `#if` to the C++ compiler). Two `DYNAMIC` blocks whose handles have one
  name are an error at the second (task 3.3, `x67`). The lookup runs for a build, `-z` and the `cpp` dump, and
  only when a block has a name: then the clone is needed (`--qb64pe-root`), also for `-z`. The `LoadLibrary`
  name of a DLL found by `./` is its full path, as the old compiler's; it stays in the C++, not in a diagnostic.
  **Fixed by the review of group 4 (2026-10-16):** the guard check rejected a header that only declares, which the
  old compiler builds when two blocks name it (measured, `v37_x68`, a new program: ` 3  2.5  0`). A header found
  a second time is now an error only when it has no guard and defines something (`defines`: outside comments,
  literals and preprocessor lines, a `{` that opens neither `extern "C"` nor a namespace; unit test
  `definitions_in_a_header`; a definition without a brace, `int x;`, is left to the C++ compiler). `has_guard` no
  longer takes any `#if` for a guard: `#pragma once`, or an `#ifndef X` / `#if !defined(X)` whose `X` a later
  `#define` defines (`#ifdef __cplusplus` is none). Also fixed: a handle clash of two different DLLs (`"my.lib"`,
  `"my_lib"`) says to rename one, not to join the blocks (one DLL by the file the lookup found, `Found::dll_file`,
  by its canonical path: `./v37dll` and `v37dll` beside the source are one, `a/x` and `b/x` two); a block without a header whose last name finds no file (a folder only) is an error, where it built
  with C linkage before (the old compiler writes no prototype and its C++ fails, measured, `v37_x69`, new;
  `qb64pe.bas` 4490, 5856, 13808); a library file whose path holds a `$`, which `make` expands, is an error at
  its name (measured, `v37_x70`, new, recorded from a folder `lib$dir` given to the runner with
  `--verification-root`: the old compiler's link gets `…\libir\v37c.a`, "no such file or directory", and it says
  "C++ compilation failed"; `qb64pe.bas` 13741 hands the path to `make` as this compiler does); `defines` reads the scope a `{` opens from the words before it, so a macro
  without a `;` before `extern "C" {` is no definition; `has_guard` and `defines` both err towards accepting
  (their doc comments list how); an `nm` that cannot be run is one error per block, naming `nm.exe` within the clone, not its full path.
- [x] 4.2 Linkage by the clone's `nm` over all files of a block (D-024; *not built: the last file only*), "Could not find sub/function …" and
  "Unable to resolve multiple instances …"; a C++ match gives `Linkage::Cpp` the text of `nm`'s line from the
  name to the first `)` (task 3.3). Verify: unit tests on a C and a C++ library built from
  `verification\v37_lib\` (skipped with a message when the clone's toolchain is not there).
  As built: D-024 is not built: a declaration is looked for in the file of the block's last name only, as the old
  compiler does (`x24` is rejected with its message). `linkage_in` reads `nm`'s lines as the old compiler does
  (` name(` in `-g` is C++, two of them an error; else a line ending in ` name` in `-g`, then in `-D`), unit test
  `linkage_from_nm`; `nm` runs once per file and option. The error names the library as written with the file's
  extension (`'./v37c.a'`), where the old compiler names its full path (no folder of the machine in a
  diagnostic, as D-015). The tests on built libraries are the CLI tests of task 4.3, `#[ignore]`d as every test
  that needs the clone.
- [x] 4.3 The library files on the link command, the bytes of libraries and headers in the build cache's key;
  the LSP reports the lookup's errors if its crate can reach the function (else noted in `crates\README.md`);
  `mark_libraries` and the rest of its CLI test go (`-z`, a build and the demand report no longer mark a block).
  Verify: a unit test of the key (a changed header changes it); CLI tests (`--ignored`): a C `.a`, a C++ `.a`, a
  `.o`, a DLL beside the program and error 260, D-023 and D-024 (*rejected, not built*), each library built into
  a temporary folder.
  As built: `Settings::libraries` and `Settings::headers`; the libraries follow `ExtraLinkerFlags` on the link
  command (`qb64pe.bas` 13726, 13741), a path with a space in quotes. The key of a program without libraries is
  unchanged, so the build cache stays valid. Unit tests `build_key_follows_the_libraries_and_headers`,
  `libraries_on_the_link_command`. The LSP's crate does not depend on the driver: it reports no lookup error
  (`crates\README.md`). The demand report runs no lookup (it reads no library). CLI tests:
  `library_files_are_linked_and_called` (C `.a`, C++ `.a`, `.o`, `CUSTOMTYPE` of the C++ library, DLL),
  `a_function_the_dll_does_not_export_is_error_260`, `library_errors_before_the_cpp` (`x18`, `x13`, `x24`, `x16`,
  a guarded header and one that only declares named twice accepted, `x67`, two DLLs whose names give one handle,
  `x69`, a `$` in a library's path); `a_library_block_in_the_report_and_the_dumps` (not ignored: a
  block with no name needs no clone). The 8 upstream programs pass run from the clone (2026-10-16); they stay on
  `tests\known_clean_not_passing.list` until task 5.2.
- [x] 4.4 `verification\v37_*` compiled and run with this compiler. Verify: every accepted program prints what
  the old compiler's prints and every rejected one is rejected with its reason, apart from D-023 (`x16`) and
  D-024 (`x24`), which now compile and run (*not built: both rejected*); the C++ failures of the old compiler
  (`x04`, `x17`, `x23`, `x30`, `x31`, `x57`) fail here too; two `DYNAMIC` blocks of one DLL (a new `v37_x`
  program, recorded with `qb64pe.exe` first) compile and run here (task 3.3; *reworked: rejected*); `x56`–`x59`
  give the old compiler's result with and without `QB64RUST_NO_FOLD=1`.
  As built (2026-10-16, 77 programs with `v37_x67`, recorded first: the old compiler's C++ fails, "redefinition
  of 'DLL_kernel32'"; then 79 with `x68` and `x69`, recorded first by the review of group 4): 31 print what the
  old compiler's print (`x68` among them); 44 are rejected as the old compiler rejects them, each by the reason
  it gives (the wording differs; `x13`, `x24` and `x51` by its own message, the library named as written),
  `x16`, `x67` and `x69` now before the C++; `x04`, `x17`, `x23`, `x30`, `x31` fail in C++ as the old compiler's
  do. Not equal: `v37_b` ("not supported yet: unsigned literals", the literal
  `18446744073709551614` above `_INTEGER64`, no library form; with that literal given `~&&` a copy prints what
  `v37_b` records), `x08` and `x22` (kept so by task 2.2), and **`x57`**: with folding on it compiles and runs
  (`1 - 1` is written `0ll`, which C++ takes as a null pointer), without folding it fails in C++ as the old
  compiler's does; put to the user (`STATUS.md`). `x56`, `x58`, `x59` are equal both ways. Run by a script of the
  session, not kept: the runner's `--suite verification` records with the old compiler only.

## 5. Slice programs and upstream

- [x] 5.1 Console slice programs in `tests\corpus\slice\`, a header in each `<name>.files\` (the header's text
  taken from `v37_lib\v37h.h`): numbers by value and by reference and the kinds of argument; strings and a TYPE;
  `ALIAS` with C++ names and overloads; `CUSTOMTYPE`; the C runtime with no name and `kernel32` with `DYNAMIC`;
  recorded with `qb64pe.exe`. Verify: each passes on its first run with this compiler; they join `slice.list`.
  As built (2026-10-16): `s121_library_numbers` (`v37_b`, `v37_c`; also the `BYVAL` 0 forms `(0)`, `0&` and a
  `CONST` 0 of `x56`, `x58`, `x59`, a result truncated by its cast, `x63`, two blocks naming the header and a
  block in a SUB), `s122_library_strings_types` (`v37_d`, the TYPE of `v37_e`, a `NULL` string result, `x39`),
  `s123_library_alias` (`v37_e`'s `ALIAS` forms, `x60`), `s124_library_customtype` (the header part of `v37_h`;
  a `.a` needs building, which the CLI tests do), `s125_library_runtime_dll` (`v37_a`, and `kernel32` with four
  declarations). The first four have `v37h.h` in their `.files\`, its first comment line naming its source; the
  block names `"v37h"` (`s123`: `"./v37h"`), found in the program's folder. `v37_b`'s literal
  `18446744073709551614` is written with `~&&`. Each recorded output equals the `v37` measurement's lines; all
  five passed on their first run.
- [x] 5.2 Upstream: `declare_library/test`, `declare_library_dynamic/test`, the four `declare_library_header`
  programs, `declare_library_static/test` and `auto_include/am-trigger` join `pass.list` (310 to 318 of 399).
  First the `tier2` job's step "Upstream pass list" in `rust.yml` drops `--compile-tests
  tests/upstream/compile_tests` (design D4: the copy lacks `lib-windows.a` and `lib.dll`, the clone the job
  checks out has them), with the runner's `--compile-tests` help text ("CI passes the copy …") and
  `tests\upstream\README.md` ("CI runs the same list from this copy") brought in line. Verify: the 8 pass run by
  themselves from the clone, as CI will run them; tier 1's list tests pass; the demand report's first line is no
  longer `DECLARE LIBRARY`.
  As built (2026-10-16): the 8 pass run alone from the clone; they left `tests\known_clean_not_passing.list`.
  `rust.yml`'s job comment, which said the copy holds every file the list needs, now says the clone, and the
  step has a comment why. The demand report over `tests\upstream` begins with `_SNDOPEN` (6).

## 6. Close

- [x] 6.1 Documents: `DIVERGENCES.md` (D-023, D-024), `SOMEDAY.md` (under "QB64pe behaviours to review": the
  block's prototypes beside a header, a result truncated by its cast (`x63`), a `STRING * n` member with no NUL
  (`x66`); what stayed marked; `errtest`'s handlers), `crates\README.md` (also: a header's own includes are not in
  the build cache's key),
  `tests\upstream\README.md`, `tests\corpus\README.md`, `verification\README.md` (`v37*` and any new numbers; `CLAUDE.md` holds
  no counts or catalogue since 2026-10-16), `ROADMAP.md`, `STATUS.md` (its line in the steps table, and the items
  of "Waiting on the user" the change settles or adds). Verify: `check_repo.py --untracked` is clean.
  As built (2026-10-16): D-023 and D-024 are not built, so `DIVERGENCES.md` has no rows for them: a paragraph
  below the table says they were proposed and not built and that the ids are not reused (no register row either
  for `x16`, `x67`, `x69`: rejecting a program whose build fails with the old compiler is no divergence).
  `SOMEDAY.md`: "Left marked by `m3-declare-library`" (`x22`, `x08`, `errtest`, Linux and macOS) and three rows
  under "QB64pe behaviours to review" (`x23`/`x30`, `x63`, `x66`); accepting D-023 and D-024 was already under
  "Other ideas". `verification\README.md` already named `x67`–`x70`. "Waiting on the user" keeps its two items
  (the answer applied to `x67` and `x69`; `x57`), which this change does not settle.
  Answered by the user, 2026-10-16 (`DECISIONS.md`): the rejection of `x67` and `x69` before the C++ is
  confirmed; `x57` is left as built, and gets no row in `DIVERGENCES.md`.
- [x] 6.2 Closing runs, once (working rule 9): tier 1 with the command-line tests that build, the runner's tests,
  the slice list with and without folding, the upstream and differential pass lists, the two full suites, the
  demand reports, the shrink-only lists regenerated (the call-site check is not affected: no built-in changes;
  say so); `python tools\testtime\test_time.py`. Verify: numbers in `STATUS.md`; nothing on a pass list fails;
  the files to stage and a commit message named.
  As built (2026-10-16, release build, `--jobs 8 --build-cache`): tier 1 322 tests (at the close of group 5;
  only documents and the runner's help text changed after it, which no tier 1 test reads apart from the list
  checks, unchanged since), the 30 command-line tests that build, the runner's 73 tests; slice list 436 of 436,
  also with `QB64RUST_NO_FOLD=1`; upstream pass list 318 of 318; differential 59 of 59; full corpus 455 of 461
  (the slice list and 19 `.err` programs; `218_fre` and the 5 known failures as before); full upstream suite 318
  pass, 85 rejected (72 at the compile, 13 `.err` with only marked errors), 1 known failure, none wrong; demand
  reports and the lists in `STATUS.md`; the lists regenerated, unchanged. The call-site check was not run: no
  built-in changed. `tools\testtime\test_time.py` run.

## Workflow follow-up

- Archive the change after the user has staged and committed it (`/opsx:archive`).
