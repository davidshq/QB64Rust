# Tasks

Standing rules: `CLAUDE.md`.

## 1. Decision

- [x] 1.1 `DECISIONS.md`: the row for this change (user, 2026-10-20: the icon before the C++, the library programs
  from the C++ build's messages mapped to the BASIC line, no search of header text). Check: the row names this
  change's design.

## 2. The icon (design D1)

- [x] 2.1 `is_icon` and `check_exe_icon` in `crates\driver\src\lib.rs`: a file that is there but is no icon
  directory is an error at the metacommand. Unit tests of `is_icon`: one image, five images, text, too short,
  type 2 (a cursor), an entry past the end of the file. Check: `cargo test -p qb64rust-driver`.
- [x] 2.2 Front-end test `tests\frontend\exeicon_not_icon_errors.bas` (`check-fail`) with a text file
  `tests\frontend\inc\not_an_icon.ico`; its snapshot holds the error at the line. Check: the snapshot, and
  `real_programs_have_no_diagnostics` still passes (`cws.ico`).

## 3. The C++ build's errors (design D2, D3, D4)

- [x] 3.1 `BuildError` in `crates\driver\src\build.rs`: `Toolchain { output, folder }` from a failed `make`,
  `Other` for the rest; its `Display` is today's text. Check: tier 1, and a failed build prints what it printed
  before (`v37_x30` before 3.3).
- [x] 3.2 `crates\driver\src\cpp_errors.rs`: `read` (the toolchain's output to failures, or none) and
  `library_proc` (each failure to a library procedure, or none). Unit tests on the five measured outputs, paths
  relative: each read and matched; and gives up for an error in `qbx.cpp`, in a header, of `llvm-rc`, an
  undefined symbol that is no library name, and a name of a `DYNAMIC` block. Check: `cargo test -p
  qb64rust-driver`.
- [x] 3.3 `main.rs`: a mapped failure is printed as the front end's errors with the summary line, exit status 1,
  the build folder removed unless `--keep-build`; anything else as before. Check: `qb64rust -x -q -m` on each of
  the five programs in its folder under `target\legacy-tests\verification\` gives an error at the line the spec
  names and `1 error`.

## 4. The suite

- [x] 4.1 The six programs (`v29_e16_not_an_icon`, `v37_x04`, `x17`, `x23`, `x30`, `x31`) leave
  `tests\verification\known.list` for `pass.list`. Check: `run_legacy_tests.py --suite verification` on the six
  passes, and `verification_lists_are_exact` passes.

## 5. Close

- [x] 5.1 `STATUS.md`: the known bug removed, "Next" updated.
- [x] 5.2 `python tools\legacy_tests\counts.py` (the counts of `tests\verification\README.md`).
- [x] 5.3 The closing runs (`python tools\close\close.py --plan`, then `--check`).
- [x] 5.4 `python tools\testtime\test_time.py`.
