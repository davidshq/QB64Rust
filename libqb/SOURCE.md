# Source of `libqb/`

Commit: `16f629784e` (2026-09-30) of QB64pe (`github.com/QB64-Phoenix-Edition/QB64pe`), the reference clone's commit
(`DECISIONS.md` 2026-10-09).

`libqb/` holds QB64pe's `Makefile`, `internal/c/` (without the toolchain `internal/c/c_compiler`, which no git
tracks; the build takes it from the clone), `licenses/` and `COPYING.txt`: 1232 files, 26875913 bytes,
each the commit's file byte for byte under its relative path. The new compiler builds against it with
`make -C libqb` (design of the OpenSpec change `libqb-copy`); the libqb objects it builds here are git-ignored.

Licence: MIT for QB64pe (`COPYING.txt`, `licenses/license_qb64.txt`); the parts under `internal/c/parts` keep
their own licences, listed in `licenses/`.

Made by `tools/libqb/copy_libqb.py`; `--check` says whether the copy still equals the commit. Do not edit the
copied files by hand.
