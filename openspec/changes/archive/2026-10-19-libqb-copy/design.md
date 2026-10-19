# Design

## Context

`crates\driver\src\build.rs` builds a program by running the clone's `mingw32-make` with `-C <clone>`: QB64pe's
`Makefile` there includes the `build.mk` files of `internal\c\libqb` and `internal\c\parts`, compiles the
program's `qbx.cpp` (a copy in the build folder, `QB_QBX_SRC`) and links it with libqb's objects, which it builds
into the clone's `internal\c` when one is missing (`*.o`, `*.a`, git-ignored there). The toolchain is the clone's
`internal\c\c_compiler`, downloaded by QB64pe's setup and tracked by no git. `library.rs` runs the clone's `nm`
and looks for `DECLARE LIBRARY` files relative to the clone, as the old compiler does relative to its own
folder. The runner (`tools\legacy_tests`), `callsite.py` and the CLI tests lock the clone (`clone_lock.py`):
alone for `qb64pe.exe`, shared for the new compiler, whose builds also take the objects' lock around `make`.

What `make` reads outside `internal\c` (measured in the `Makefile` at `16f629784e`): the `Makefile` itself,
`licenses\license_*.txt` (only with `GENERATE_LICENSE=y`) and, only with `BUILD_QB64` (never set here),
`internal\source`. The toolchain is named in five variables, `AR`, `CC`, `CXX`, `OBJCOPY`, `WINDRES`
(`Makefile` 101–105), each relative to `internal\c`.

The commit's tracked files: 1,200 under `internal\c` (1,093 in `parts`, 99 in `libqb`), 30 in `licenses`, 26.8 MB
in all. None holds a control byte the repo check flags; nine lines hold paths the check would flag, none private
(comments in `filesystem.cpp` naming the user's temp folder and the Windows folder, a debugging file name in
`graphics.cpp`, three lines in curl).

## Goals / Non-Goals

**Goals:**

- Every build of the new compiler reads libqb from this repo and writes nothing into the clone.
- The copy is QB64pe's files at the reference commit, byte for byte, and a check says so.
- Runs of the new compiler stop locking the clone, so a recording with `qb64pe.exe` can run beside them.

**Non-Goals:**

- Any change to libqb's sources (the copy makes later fixes possible; this change makes none). The first is
  `_COTH` (`DIVERGENCES.md` D-031, a change of its own after this one); `copy_libqb.py --check` then needs a list
  of the copy's own edits, which that change adds.
- Owning the toolchain: it stays the clone's.
- A layout for an installed compiler (the `.vsix` change decides where an installed `qb64rust` finds libqb).
- CI changes: `tier2` already checks the clone out at the commit and unpacks the toolchain into it.

## Decisions

### D1. The copy mirrors the clone's layout, in `libqb\`

`libqb\Makefile`, `libqb\internal\c\…`, `libqb\licenses\…`, `libqb\COPYING.txt` (QB64pe's MIT licence) and
`libqb\SOURCE.md` (commit, date, what was copied and how to redo it). The same relative layout as the clone, so
the `Makefile` runs unchanged with `make -C libqb` and its `internal\c\…` paths resolve. The name says what the
folder is to this project; a second runtime of M6 would sit beside it. Alternative: only `internal\c` at the
repo root as `internal\c` (the literal "copy of `internal\c`"), with the `Makefile` beside it: rejected, as it
puts QB64pe's top-level names among this repo's.

The files are taken from the commit's blobs (`git -C <clone> archive` / `git show`, not the working tree), so
line ends are the committed ones whatever the clone's `core.autocrlf`; `.gitattributes` marks `libqb/** -text`
so this repo stores them unchanged. The copy's own `internal\c\.gitignore` (QB64pe's) comes with it;
this repo's `.gitignore` adds what builds write in the copy: `libqb/**/*.o`, `libqb/**/*.a`, `libqb/**/*.tmp`.

Code: `tools\libqb\copy_libqb.py` (new), with `test_copy_libqb.py`, the shape of
`tools\upstream\copy_upstream_tests.py`: `--clone <dir>` (default `..\QB64pe`), `--commit` (default: the one
in `SOURCE.md`, else the clone's `HEAD`), writes the copy and `SOURCE.md`; `--check` compares each file with the
commit's blob and lists differing, missing and extra files (git-ignored files of the copy, the built objects, do
not count as extra), exit 1 on any.

### D2. The toolchain is the clone's, passed by name

`build_with_make` runs the clone's `mingw32-make` with `-C <copy>` and adds `AR=`, `CC=`, `CXX=`, `OBJCOPY=`,
`WINDRES=` naming the clone's `internal\c\c_compiler\bin\*.exe` (backslashes, as the `Makefile` writes them).
Command-line variables override the `Makefile`'s `:=` assignments. `QB_QBX_SRC` and the copy of `qbx.cpp` come
from the copy, and `CXXFLAGS_EXTRA`'s `-I` names the copy's `internal\c`. `nm` (`library.rs`) and the
`DECLARE LIBRARY` lookup stay with the clone. A relative word of `-f:ExtraLinkerFlags` that names a file of the
clone is passed with its full name there (found while building it: `make -C` would otherwise look for it in the
copy, where the old compiler, linking in its own folder, finds it in the clone; `extra_link_args`), and the build
key hashes that file of the clone, as before. Alternative: a junction `libqb\internal\c\c_compiler` to the
clone's (the copy's `.gitignore` already ignores the name): rejected, a hidden step outside the repo that a
fresh checkout or a worktree would lack.

Code: `crates\driver\src\build.rs` (`build_with_make`, a `Roots { clone, libqb }` passed where `root` is now).

### D3. Finding the copy

`build::find_libqb(option)`: `--libqb-root`, else `QB64RUST_LIBQB_ROOT`, else `libqb` next to the crate the
binary was built from (`CARGO_MANIFEST_DIR\..\..\libqb`), counted only if it holds `internal\c\qbx.cpp`; the
error names the option and the variable, as `find_root`'s does. Looked for only when an executable is built, so
`-z`, `--dump` and the language server need no copy. The copy of the compiler a run tests (`target\test-bin`)
keeps the path fixed at build time, so it finds the same copy, as it finds the same clone; a compiler built in a
worktree finds the worktree's copy.

Code: `crates\driver\src\build.rs` (`find_libqb`), `crates\driver\src\main.rs` (the option and the usage line).

### D4. The build key

`build_key` stamps the copy's `internal\c` and hashes the copy's `qbx.cpp` and `Makefile`, and the clone's
`c_compiler\bin` executables as now. The clone's `settings\config.ini` leaves the key: it mattered because
`qb64pe.exe` builds libqb objects in the clone with its options (`test-while-building` D1), and it never builds
the copy's. `written_by_builds` stays as it is (the `qbx<N>.cpp` rule simply never matches in the copy). The
key's version goes to 4, so no entry made against the clone is reused.

Code: `crates\driver\src\build.rs` (`build_key`, `compiler_settings` removed with its unit test, the other
build-key tests moved to the copy's layout).

### D5. Runs of the new compiler take no clone lock

A run of the new compiler reads from the clone only the toolchain and `DECLARE LIBRARY`'s files, which a compile
with `qb64pe.exe` does not write (it writes `internal\temp`, `internal\c\qbx*` and libqb objects in the clone,
none of which the new compiler reads after D2). So the runner takes the clone's lock alone for `qb64pe.exe`
and none for `qb64rust`, with one exception found while building it: a compile-suite run on the clone's own
`tests\compile_tests` (and the qbasic and format suites) runs each program with its test folder in the clone as
the working directory, where some programs write files and a run of `qb64pe` runs the same programs; such a run
of `qb64rust` takes the lock shared, as before (`clone_lock_kind`, from whether any program of the run lies in
the clone). The CLI tests that build run no program in the clone and stop taking it
(`crates\driver\tests\support\mod.rs`). The objects' lock (`QB64RUST_OBJECTS_LOCK`, the same file under the main
checkout's `target\qb64pe-clone`) is kept: it now guards the copy's objects, which builds of the new compiler
write side by side. A worktree's copy has objects of its own, so its runs could lock separately; they keep the
main checkout's lock, as now, which only costs a wait while one of them builds objects. `callsite.py` runs
`qb64pe.exe` and keeps the lock alone.

Code: `tools\legacy_tests\run_legacy_tests.py` (`main`, the lock around the run), `clone_lock.py`
(`clone_lock_kind`, `name_objects_lock`, the docstring),
`crates\driver\tests\support\mod.rs`, `tools\legacy_tests\README.md` ("Locks"), `CLAUDE.md` working rule 9's line
on locks.

### D6. The repo check leaves the copy out

`tools\repo_check\check_repo.py` skips `libqb\` except `SOURCE.md`, for both the path and the control-byte
checks: the files are third-party bytes, `copy_libqb.py --check` holds them to the commit, and rule 2 is about
what this project writes. Alternative: nine `ALLOWED` entries: rejected, as a new upstream commit would bring
new ones each time.

## Risks / Trade-offs

- **The first build after a checkout builds all of libqb** (the copy has no objects): about 550 objects, a few
  minutes, once per flag set, as for a fresh clone. The CLI tests and runner take the objects' lock alone for it.
  → The first closing run pays it; later runs reuse the objects.
- **27 MB in the repo, for good** (a cost that stays). Decided on 2026-10-02; git compresses C sources to well
  under that.
- **An executable could differ from before** while its code does not: object files can hold the source's path
  (`__FILE__` in libqb's asserts), now the copy's. Not a behaviour change; the slice list and the upstream pass
  list check behaviour, and the pass cache runs every program again (every executable changes).
- **Two copies of libqb on one machine** (the clone's, which `qb64pe.exe` uses, and this one): a fix made in the
  copy later is not in the recordings. → That is the point of the copy; a fix that changes output is a
  `DIVERGENCES.md` row, as for the compiler.
- **libqb's objects take the driver's flags** (found at the close): the `Makefile` adds `CXXFLAGS_EXTRA` to every
  object, so the copy's libqb objects are built with `-fwrapv`, and with `-O2` when the first build that makes an
  object asks for `OptimizeCppProgram`. Before, most objects in the clone had been built by `qb64pe.exe` (113 of
  the 551 objects both hold differ in code, the other 438 only in the header's time stamp). Not new in kind: the
  driver passed the same flags before, and an object it built in the clone took them, and which build makes an
  object first was already a matter of order. Every closing run passes. → A follow-up can give `-fwrapv` to the
  program's `qbx.cpp` only and build libqb with the flags `qb64pe.exe` uses (not in this change's scope).
- **`s75_sound_wait` crashed once under load** at the close (exit code 0xC0000374, heap corruption, with
  `--jobs 8`; it passed alone and in the rerun). It is timing-sensitive (`SOUND`), so the run did not fail. Not
  seen in an earlier kept log; whether the audio part of libqb has a race under load, or the flags above play a
  part, is not known. → Watch it in the next runs; with the copy, libqb can now be fixed here.
- **Moving the reference commit** now means running `copy_libqb.py` as well as moving the clone; `SOURCE.md` and
  `DECISIONS.md` 2026-10-09 say so.
