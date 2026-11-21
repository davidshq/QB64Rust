# Tasks

Standing rules: CLAUDE.md. What this change does differently: the atomic-flags edit is not inside a test for
Emscripten (design D7). W0's design D4 kept every libqb edit inside one; that was Claude's rule, written on
2026-11-19. Nothing is recorded for the wasm platform (design D8).

## 1. The width as a value

- [x] 1.1 `Platform::pointer_bytes()` in `crates\base\src\platform.rs`: 8 for Windows and Linux, 4 for Wasm
  (design D1). Verify: a unit test for each platform; `cargo test -p qb64rust-base`.
- [x] 1.2 `sema::Program::pointer_bytes`, set in `check_with` from `parsed.flags.platform()`, and copied to
  `ir::Program` by `ir::lower` (design D1). Verify: a `sema` unit test that checks a program with the wasm
  flags and reads 4; `cargo test -p qb64rust-sema -p qb64rust-ir`.
- [x] 1.3 `Ty::stored_bits(self, pointer_bytes)` beside `int_bits`, and a doc comment on each of them that says
  which width it gives (design D2). Verify: unit tests for `_OFFSET`, `_UNSIGNED _OFFSET`, `LONG` and a
  `_BIT * 5` at both widths.

## 2. Sizes and layouts in sema and the IR

- [x] 2.1 `size_of(types, pointer_bytes, t)`, and its 8 callers (design D1). `mem_type` takes the width and
  adds the tail member `$_PAD` at 4 bytes only (design D3). `layout_bytes` (`check\places.rs`) gives a
  `STRING` and a dynamic slot `pointer_bytes` (design D5). Verify: `sema` unit tests at both widths. `LEN` of
  an `_OFFSET` is 8 or 4, of `_MEM` 56 or 40, and of the spec's `TYPE` of a `LONG`, an `_OFFSET` and a
  `STRING * 2` 14 or 10. The `_MEM` members are at offsets 0, 4, 20, 24, 28 and 32 at width 4.
- [x] 2.2 Read each of the 49 calls of `int_bits`, `qb_bits`, `radix_width`, `is_wider_than` and `conversion`
  in the 12 files of design D2's list. Move to `stored_bits` each call that describes a stored value: the
  literal ranges of `literal.rs`, the folds of `consteval.rs`, `qb_bits` and `radix_width`, `_MK$` and `_CV`.
  The calls of `codegen-cpp` follow in 3.2. Verify: the task's note lists every call with "computed" or
  "stored". A `sema` test gives `HEX$` of an `_UNSIGNED _OFFSET` of -1 a width of 8 digits at width 4, and a
  folded `_OFFSET` constant past 32 bits is cut at width 4. `cargo test -p qb64rust-sema`.
  Note (2026-11-21): the list of calls is design D2: the calls moved to `stored_bits`, then "Found while
  applying, and so not changed", then the calls that stay on `int_bits`. No separate note was written. Nothing
  was cut in `literal.rs` or `consteval.rs`, because there is no `_OFFSET` literal (design D2). So the check of
  a folded constant does not apply. The `HEX$` test is `new_types_special_cases` in `sema\src\builtins.rs`.

## 3. The generated C++

- [x] 3.1 `Layout::ty_size`, `member_size` and `layout` (`codegen-cpp\src\decl.rs`) take the width from the
  program: a `STRING` member, a dynamic slot and the owner alignment are `pointer_bytes` (design D5). Verify:
  `cargo test -p qb64rust-codegen-cpp`; the desktop snapshots do not change in this task.
- [x] 3.2 The stored widths of `codegen-cpp` (design D2): `input_type_code` (`io.rs`), the type codes of
  `mem.rs`, `bytes_letters` (`builtins.rs`) and `trace.rs`. Add them to 2.2's note. Verify: unit tests of
  `input_type_code` and of `mem.rs`'s code for `_OFFSET` at both widths.
  Note (2026-11-21): `trace.rs` is not changed: it reads an `_OFFSET` through its C type (design D2).
  `varptr_type` (`cmem.rs`) was changed in its place.
- [x] 3.3 The two size checks in `global.txt` (design D4), with the program's numbers. Update the snapshots in
  this task alone, so that its diff holds only these lines. Verify: `cargo test -p qb64rust-driver`. A C++ file
  built by hand with a wrong number stops `clang++` with the message.
- [x] 3.4 The `-z` comparison in `crates\driver\tests\it\wasm_target.rs` allows the size checks, and a new
  case with an `_OFFSET` shows that the fragments differ only in sizes (spec `compiler/wasm-target`, "The C++ is
  the same as for Linux"). Verify: `cargo test -p qb64rust-driver --test it wasm_target`.

## 4. The wasm target builds pointer types

- [x] 4.1 Remove the pointer rules of `crates\sema\src\target.rs` (design D6) and item 3 of its doc comment.
  Replace `pointer_sized_types` in `wasm_target.rs` with cases that build. Keep the audio word that takes an
  `_OFFSET` as a case that is still rejected for its part. Verify: `cargo test -p qb64rust-sema -p
  qb64rust-driver`. With the SDK present, the spec's scenarios "LEN of an _OFFSET", "_MEM", "A block of a LONG",
  "An unsigned _OFFSET of -1" and "A member array of strings" run under Node in `wasm_target.rs`.
- [x] 4.2 The family `v73` of design D8: programs whose output is the same at both widths through `$IF 32BIT`,
  recorded with `qb64pe.exe` on the host, one by one. Add the family's line to `verification\README.md`. Verify:
  each passes on Windows (`run_legacy_tests.py`, by name) and with `--platform wasm` on the host.
- [x] 4.3 Language reference (`CLAUDE.md` working rule 16): `docs\reference\entries\_offset.md`, `_mem.md`,
  `len.md`, and the chapter `docs\reference\chapters\wasm-target.md`, which loses `_OFFSET` and `_MEM` from its
  list of what is not available yet. Then `reference.py render` and `reference.py --check`. Verify: `--check`
  passes. `reference.py evidence _OFFSET` shows no page that still says the type is rejected on wasm.
  Note (2026-11-21): `len.md` is not edited. It gives no size of an `_OFFSET` or a `_MEM`, so nothing on it
  became wrong.

## 5. Atomic flags in libqb

- [x] 5.1 Write the edit files `tools\libqb\edits\atomic-thread-flags-*.edit` (design D7) for the flags it lists,
  with their `extern` declarations. Fix each place that does not compile with a `std::atomic` at that place. Verify:
  libqb builds for Windows and for wasm. `git diff` shows that no generated fragment changes. The wasm slice
  (`check wasm-slice`, 22 programs) passes on the host.
- [x] 5.2 Time one tight-loop program (a `FOR` loop of 100 million statements, `$CHECKING` on) on the host,
  before and after 5.1. Verify: both times are in the task's note. A slowdown over 5% is raised with the user
  before going on.
  Note (2026-11-20): 400 million statements, four interleaved runs each. Plain 1.38 s, every flag atomic 4.4 s,
  every flag but `qbevent` atomic 1.38 s. So `qbevent` stays plain (design D7). The user accepted this on
  2026-11-20.

## 6. The 270 programs

- [x] 6.1 Remove the `offset` lines of `tests\verification\known.wasm.list` and the reason from the runner's
  set and its comment at the top (spec `testing/golden-corpus`). Verify: the runner's check of the lists
  passes, and a line with `offset` fails it.
- [x] 6.2 Say how long the run of the 270 will take and why that size (`CLAUDE.md` working rule 6). Then run them
  in the VM with `--platform wasm`, by list if `check` can take one (design D8). Verify: the log is kept, and
  the task's note has the counts: pass, fail by reason.
  Note (2026-11-20): `check wasm-verification --where vm` takes no list and runs the whole family, about 2 to
  4.5 hours. The names of the 270 are in `target\scratch\2026-11-20\offset-programs.list`. A sample of 12 on the
  host: 9 pass, `v21_a_decls` and `v56_a1_scalar_offsets` differ by the pointer's width, `v66_a06_varseg_mem`
  is a known failure. Which run to start is asked of the user.
  Run (2026-11-20): the user chose the list. `check` got the step `wasm-rerun`, which runs
  `tests\wasm\rerun.list` for wasm in the VM. The 270 took 17.7 minutes there, with 2.3 minutes for the copy:
  230 pass, 25 fail, 15 known failures (they were known on Linux already). The log is
  `target\check\logs\failed\wasm-rerun-2026-11-20_203958.log`.
- [x] 6.3 Sort each failure (design D8). Fix the faults, then run only the failed programs again by list. List
  the others as `width`, `target` or another reason, saying what was seen. Verify: the failed programs pass or
  are listed; `run_legacy_tests.py`'s check of the lists passes.
  Note (2026-11-20): one fault. An `_INTEGER64` array was accepted for an `_OFFSET` array parameter on wasm,
  and the SUB then wrote 4-byte elements into 8-byte ones (`v24_x179_param_offset_to_integer64` printed 0 for
  5). `array_kind` (`crates\sema\src\check\proc.rs`) now uses the pointer's stored width, so it is a compile
  error on wasm, and a `LONG` array is accepted there. The other 24 show a 4-byte pointer in their output and
  are listed as `width`, each with what was seen; `v32_c_mk_cv` also shows quad precision's `_FLOAT` bytes.
  The 25 ran again by list in the VM (9.4 minutes): 25 known failures, none failing.

## 7. Records

- [x] 7.1 `DECISIONS.md` rows for D2 (computed in 64 bits, stored in 32) and D3 (`_MEM` is 40 bytes). A
  `DIVERGENCES-QB64PE.md` row for D3 that names the wasm target and says it is unmeasured. `LAYOUT.md` for the
  new edit file and the family `v73`. Verify: `check_repo.py` passes.
  Note (2026-11-20): `LAYOUT.md` needed no row: the edit files are under `libqb\`'s row, and the families are
  listed in `verification\README.md`. The divergence row is D-072.
- [x] 7.2 `STATUS.md`'s "Next": W0.5 built, the new pass count of the wasm verification run, and W1 next.
  Verify: `just quick` passes in the background. The handover says what ran and what stood. The desktop
  suites (`just full`, for the libqb edit of 5.1) are left to the user, as working rule 6 says.

## Workflow follow-up

- Archive the change after the user's review. Its delta specs are already in `openspec\specs\` (merged by hand
  on 2026-11-20 in task 4.3, because `reference.py --check` reads the merged specs), so the archive must not
  apply them a second time.
