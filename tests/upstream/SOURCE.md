# Source of `tests/upstream/`

Commit: `16f629784e` (2026-09-30) of QB64pe (`github.com/QB64-Phoenix-Edition/QB64pe`), as cloned in `..\QB64pe`.

`compile_tests/` holds the text files of the clone's `tests/compile_tests`, byte for byte, under the same
relative paths. Binary assets (images, fonts, sound, libraries) and `.gitignore` files are not copied; the
upstream tests that need them are run from the clone (`tools/legacy_tests/run_legacy_tests.py --suite compile`).

Licence: MIT, QB64pe's `licenses/license_qb64.txt`, copied as `LICENSE-QB64pe.txt` (decision of 2026-10-04 in
`DECISIONS.md`; `study/22` §4).

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
