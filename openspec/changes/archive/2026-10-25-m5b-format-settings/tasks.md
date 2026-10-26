# Tasks

Standing rules: CLAUDE.md. What this change does differently:

- All recordings with `qb64pe.exe` (the new slice programs' outputs and every variant) are made in one batch,
  in group 2, before the closing runs.

## 1. The inputs (design D2, D3)

- [x] 1.1 Write `tests\corpus\slice\s160_layout_*.bas` (about three programs, headless). Between them they should
  have: indentation and spacing written by hand, in blocks of every kind; SUBs and FUNCTIONs; an unclosed string at
  the end of a line; `'$FORMAT:OFF` and `'$FORMAT:ON`; `?`; a joined line; and a comment after each kind of block
  line. Record their outputs with the corpus suite's `--record`, and add them to `slice.list`.
  Verify: the corpus suite passes them with `qb64rust` (`--glob "s160_layout_*"`).
- [x] 1.2 Choose the list with a scratchpad script, as design D2 describes, over the default recordings (the new
  programs included). Write `tests\format\settings.list` with about 30 entries, each with a `#` comment on what it
  reaches.
  Verify: each entry has a default recording; every variant shows differing lines in at least five listed programs.

## 2. Recording and comparing (design D1, D5; spec `testing/format-oracle`)

- [x] 2.1 In `run_legacy_tests.py`, add `FORMAT_VARIANTS` and the variants to `--suite format-corpus`: record each
  listed program under each variant with `qb64pe`, and compare `settings\<variant>\` with any other compiler. A
  listed program with no default recording fails, naming the line of the list. `--category <set>` takes that set's
  variants as well. Add tests of the folder naming, the flags, and the list to `test_run_legacy_tests.py`. Update
  "The format oracle" in `tools\legacy_tests\README.md` and the runner's docstring.
  Verify: `python -m unittest discover -s tools/legacy_tests -p "test_*.py"` passes.
- [x] 2.2 Record the variants with `qb64pe` (`--suite format-corpus`). Record again and expect no change.
  Verify: the second recording changes no file; `tools\repo_check\check_repo.py --untracked` passes over
  `tests\format\`.
- [x] 2.3 `counts.py`: count `settings\` apart from the sets (design D5); add a test to `test_counts.py`. In
  `tests\format\README.md`, describe the variants, the list, and how to record them.
  Verify: `python -m unittest tools/legacy_tests/test_counts.py`; `counts.py` writes the block.

## 3. Tier 1 and the formatter (design D1, D4)

- [x] 3.1 In `format_oracle.rs`, add the variants: each with its `Settings` from the folder name, named
  `settings/<variant>/<set>/<path>`, under the three checks. Update the test's header comment, including the time
  line.
  Verify: `cargo test -p qb64rust-driver --test format_oracle` runs the variants (the count it prints includes
  them).
- [x] 3.2 For each differing variant: fix it in `crates\format` or `crates\base\src\layout.rs`, or list it in
  `known_differences.list` with its reason (design D4). If the differences turn out to be many, stop and report
  them, as the design's "Risks" says.
  Verify: `cargo test` passes; the compare of task 2.1 with `target\release\qb64rust.exe` passes every variant
  that is not listed.

## 4. Close

- [x] 4.1 Do the closing runs: `python tools\close\close.py --plan`, running each command on its own, then
  `tools\close\close.py --check` passes. Then update `STATUS.md`'s "Next", write the counts with `counts.py`, and
  run `python tools\testtime\test_time.py`.
  Verify: `--check` passes.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage, and suggest a commit message.
