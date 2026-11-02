# Source of `libqb/`

Commit: `16f629784e` (2026-09-30) of QB64pe (`github.com/QB64-Phoenix-Edition/QB64pe`), the reference clone's commit
(`DECISIONS.md` 2026-10-09).

`libqb/` holds QB64pe's `Makefile`, `internal/c/` (without the toolchain `internal/c/c_compiler`, which no git
tracks; the build takes it from the clone), `licenses/` and `COPYING.txt`: 1232 files, 26878907 bytes,
each the commit's file byte for byte under its relative path, but for the edits below. The new compiler builds
against it with `make -C libqb` (design of the OpenSpec change `libqb-copy`); the libqb objects it builds here are
git-ignored.

Edits of this project, applied by the tool after copying (each file's header says why; `QB64PE-DIVERGENCES.md` has the
row, `UPSTREAM.md` the fix as offered to QB64pe):

- `tools/libqb/edits/d012-error-exit-code-no.edit`: `internal/c/libqb/src/error_handle.cpp`
- `tools/libqb/edits/d012-error-exit-code.edit`: `internal/c/libqb/src/error_handle.cpp`
- `tools/libqb/edits/d031-coth.edit`: `internal/c/libqb/include/extended_math.h`
- `tools/libqb/edits/d048-console-eof-end.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/d048-console-eof-input-string.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/d048-console-eof-input.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/d049-bload-past-block.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/d049-bsave-past-block.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/d050-memnew-zero-lock.edit`: `internal/c/libqb/src/memblock.cpp`
- `tools/libqb/edits/d051-statuscode-missing-handle.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/d052-division-error-text.edit`: `internal/c/libqb/src/error_handle.cpp`
- `tools/libqb/edits/d052-division-error-trappable.edit`: `internal/c/libqb/src/error_handle.cpp`
- `tools/libqb/edits/d052-division-fatal-entry.edit`: `internal/c/libqb/src/error_handle.cpp`
- `tools/libqb/edits/d052-division-fatal-header.edit`: `internal/c/libqb/include/error_handle.h`
- `tools/libqb/edits/d052-division-signal-handler.edit`: `internal/c/qbx.cpp`
- `tools/libqb/edits/d053-bit-numbers-modulo-64.edit`: `internal/c/libqb/include/bitops.h`
- `tools/libqb/edits/d056-screenclick-display-scale.edit`: `internal/c/libqb.cpp`

Licence: MIT for QB64pe (`COPYING.txt`, `licenses/license_qb64.txt`); the parts under `internal/c/parts` keep
their own licences, listed in `licenses/`.

Made by `tools/libqb/copy_libqb.py`; `--check` says whether the copy still equals the commit with its edits. Do not
edit the copied files by hand: add an edit file and run the tool.
