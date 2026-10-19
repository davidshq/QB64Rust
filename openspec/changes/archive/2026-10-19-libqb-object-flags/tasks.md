# Tasks

Standing rules: `CLAUDE.md`.

## 1. The flags and the stamp (design D1, D2, D3)

- [x] 1.1 `program_makefile` and `build_with_make`: no `CXXFLAGS_EXTRA`; `-f Makefile -f <build>/program.mk`
  gives `[-O2 ]-fwrapv -I<copy>/internal/c` to `$(QB_QBX_OBJ)` and `embedded.o`. Unit test of the makefile's text.
  Checked with `make -n -B` on a kept build folder: `qbx.cpp` with `-fwrapv`, `libqb\src\qbs.cpp` without.
- [x] 1.2 `objects.stamp`: `objects_stamp`, `refresh_objects`, `remove_built`; `lock_objects` refreshes alone. Unit
  test: objects without a stamp are removed and the sources kept, objects under the stamp kept, another toolchain
  removes them again. `.gitignore` names `/libqb/objects.stamp`.
- [x] 1.3 The build key's version 5.
- [x] 1.4 Checked for real: a slice program built with the release compiler removes the old objects, builds those
  it needs, and writes the stamp.

## 2. Close

- [x] 2.1 `STATUS.md`: the known bug removed, "Next" item 2's note on flags.
- [x] 2.2 The closing runs (`python tools\close\close.py --plan`, then `--check`), shared with `const-rulings`.
- [x] 2.3 `python tools\testtime\test_time.py`.
