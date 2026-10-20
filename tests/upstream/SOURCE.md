# Source of `tests/upstream/`

Commit: `16f629784e` (2026-09-30) of QB64pe (`github.com/QB64-Phoenix-Edition/QB64pe`), as cloned in `..\QB64pe`.

`compile_tests/` holds the text files of the clone's `tests/compile_tests`, byte for byte, under the same
relative paths. Binary assets (images, fonts, sound, libraries) and `.gitignore` files are not copied; the
upstream tests that need them are run from the clone (`tools/legacy_tests/run_legacy_tests.py --suite compile`).

`root/tests/compile_tests/extra/` mirrors `compile_tests/extra/`: it is the compiler root for these tests
(`--include-root tests/upstream/root`), where `include_paths/include_fixed_compile_location` and
`include_multiple` find `'tests/compile_tests/extra/…'`, as the old compiler finds it under its own folder. It
also holds, under the same paths as in the clone, the files upstream programs name that way in `$EXEICON` and
`$EMBED` (`icon/test`, `winresource/test`, `embed/test`), byte for byte; the two `.ico` files are the only binary
files of the copy (`--check` checks them):

- `tests/compile_tests/embed/test.output`
- `tests/compile_tests/icon/qb64pe.ico`
- `tests/compile_tests/winresource/qb64pe.ico`

Licence: MIT, QB64pe's `licenses/license_qb64.txt`, copied as `LICENSE-QB64pe.txt` (decision of 2026-10-04 in
`DECISIONS.md`; `study/archive/22` §4).

Made by `tools/upstream/copy_upstream_tests.py`; rerun it after updating the clone (with `--commit-ok`). Do not
edit the copied files by hand.

| Extension | Files |
|---|---|
| `.bas` | 404 |
| `.bi` | 6 |
| `.bm` | 6 |
| `.h` | 5 |
| `.c` | 3 |
| `.output` | 331 |
| `.err` | 56 |
| `.license` | 18 |
| `.noprompt` | 2 |
| `.compile-from-base` | 4 |
| `.md` | 1 |

836 files, 2,468,684 bytes.
