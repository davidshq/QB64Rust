# Tasks

Standing rules: CLAUDE.md. What this change does differently: its measurements are of this compiler's own debug
build under the toolchain's `lldb` (design D4), recorded in `design.md`'s "Measured" section, not in
`verification\`; they run as `#[ignore]`d CLI tests at the end, after the build and the file exist.

## 1. The debug build

- [x] 1.1 `DECISIONS.md`: the row of 2026-10-24 on Path A amended as design D1 says (`-f:StripDebugSymbols=false`
  leaves the executable unstripped as `qb64pe` does; `--debug` is the debug build). Verify: the row reads so.
- [x] 1.2 `driver`: `--debug` (`Options`, `Settings.debug`), `-f:StripDebugSymbols=true|false` parsed like
  `OptimizeCppProgram` (`Settings.strip`), `program_makefile` with `-g` and without `-O2` for a debug build,
  `STRIP_SYMBOLS=n` for either, the console line when optimisation is overridden, both in the build cache's key.
  Unit tests beside `program_makefile`'s (`-g -fwrapv`; `--debug` with optimize gives no `-O2`) and the key's.
  Verify: `cargo test -p qb64rust-driver`; a usage line names `--debug`.
- [x] 1.3 CLI tests (`--ignored`): `debug_build_same_output` (a slice program built with `--debug` prints its
  recorded output), `strip_debug_symbols_false_keeps_symbols` (`nm` of the toolchain lists local symbols, type `t`,
  for `-f:StripDebugSymbols=false` and none for the default; first written as "`QBMAIN` and not for the default",
  but `--strip-unneeded` keeps the global `QBMAIN`, so the spec's scenario was corrected), `debug_build_is_cached_apart` (with
  `QB64RUST_BUILD_CACHE`, a plain build then a debug build both build, a second debug build is a hit).
  `tests\upstream\README.md`: a line that the suite's executables are unstripped only if the runner's compile
  line is documented there. Verify: the three tests pass.

## 2. The debug symbol file

- [x] 2.1 `codegen-cpp\src\debug.rs` (design D2): `DebugInfo`, `reach`, `collect`, `to_json`; `emit_debug` in
  `lib.rs`; the file hash shared with `coverage.rs`. Unit tests: `reach` for each storage and shape; `to_json` of a
  two-variable program is valid JSON with `version` 1; the layouts constants against `qbs.h`'s first two members
  (the risk of the design). Verify: `cargo test -p qb64rust-codegen-cpp`; `--dump cpp` of every `cpp` snapshot
  unchanged (`cargo test` shows no snapshot change).
- [x] 2.2 `driver`: the file written beside the executable after a debug build (`debug_path`), untouched
  otherwise; `--dump debug-info`; `dump_debug_info` (design D3). The test mode `debug-info` in
  `driver\tests\frontend.rs` and the mode table of `crates\README.md` (there is no `tests\frontend\README.md`); programs `tests\frontend\debug_info_storages.bas`
  (each storage, a static and a dynamic array, a string, a `_BIT * 3`), `debug_info_types.bas` (a user type with a
  fixed string, a `STRING` member, a member array, a `_DYNAMIC` member, nested types), `debug_info_include.bas`
  (a FUNCTION in `inc\` and a variable declared there), each with its snapshot reviewed by eye against the C++
  (`--dump cpp`). Verify: `cargo test`; each scenario of `compiler/debug-info` that tier 1 can see has its
  program.
- [x] 2.3 CLI tests (`--ignored`): `debug_build_writes_symbol_file` (the file exists, parses, `version` 1, names
  `SUB_SHOW`), `plain_build_removes_symbol_file` (a stale file goes; it stayed until the review of 2026-10-25,
  design D3), and `failed_build_removes_symbol_file` (tier 1). `crates\README.md`: `--debug` and the dump in
  "Building and running". Verify: the tests pass.

## 3. The measurement

- [x] 3.1 `support::lldb()` (the clone's `lldb.exe`, `-b`, `QB64PE_NOPROMPT=y`, a deadline, the output as text)
  in `driver\tests\support\mod.rs`; the six tests of design D4 in `cli.rs`, each asserting what `lldb` printed
  (the breakpoint's location, the frame's `file:line` after each `next`, the values read, the stop in `error`).
  A test whose expectation is the open question (how many stops per line; the retry) asserts the shape of the
  output and prints the stop sequence with `--nocapture`, so the fact is read from the log, not guessed.
  `crates\README.md` "Tests": the `lldb` tests in the `--ignored` row. Verify: `cargo test -p qb64rust-driver
  --test cli -- --ignored --nocapture debug_ > target\debug-lldb.log 2>&1` passes (`--nocapture` added at the task: without it the
  stop sequences stay in the captured output), the log holds the stop sequences.
- [x] 3.2 `design.md` "Measured": one line per test with what `lldb` did (stops per line, the two-statement
  line, the one-line loop, the call, the retry, each storage's expression, the error stop), and what the proxy's
  design takes from each. A finding that contradicts a delta spec changes the spec here. Verify: the section has
  the six lines; `openspec validate m5-debug-build-symbols --strict` passes.

- [x] 3.3 (Added at the review, user, 2026-10-25.) A procedure of an included file: its header and `END` lines
  under its own file (`procs.rs`, `lines_in(Some(proc.file), …)`; design "Measured", "Found on the way"), the
  `cpp` snapshots that change reviewed (`errorlocation_cpp`, `url_cpp`: `#line` file names only), and the CLI test
  `debug_step_into_an_included_function`. Verify: `cargo test`; the test passes.
- [x] 3.4 (Added at the review, user, 2026-10-25.) A procedure closed in an included file: `end_file` in `Proc`
  (`sema`, `ir`), the footer's `#line` under it, the symbol file's `end_file`; the program
  `tests\frontend\debug_info_end_file.bas` with `inc\debug_info_end.bi`, and `debug_info_library.bas` (a library
  function is left out, spec "Procedures with no body"). An earlier `<exe>.debug.json` is removed
  with the earlier executable by every build (design D3). The `.bas` and `.bi` of the `debug-info` tests are kept
  with LF (`.gitattributes`), since their snapshots hold the hash of their bytes. Verify: `cargo test`; the CLI
  tests that build pass.

## 4. Closing

- [x] 4.1 (Done 2026-10-25, after tasks 3.4: `--check --known debug_next_steps_by_line` passes. That test's
  `lldb` hung on the `next` over `PRINT` in the CLI run and was killed at 120 s, after every earlier step had
  stopped where expected; alone it passed in 6 s, and it passed in the first closing run, so it is a hang under
  load, in `STATUS.md`'s "Still to watch". The first closing run's clipboard and `c15_resize` failures did not
  recur. The test `debug_info_library.bas`, added after them, reran tier 1 with fmt and clippy only, the rest
  `--not-repeated` since nothing else reads it.) The closing runs: `python tools\close\close.py --plan`, each command run by its own, then
  `tools\close\close.py --check` passes (the slice list and the upstream pass list rebuild once for the new cache
  key, as design D1 says). Then the counts of the test READMEs by `tools\legacy_tests\counts.py`, `STATUS.md`'s
  "Next" names the next change of M5 (the DAP proxy, `study\36` §4), and `python tools\testtime\test_time.py`.
  Verify: `--check` passes.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
