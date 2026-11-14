# The verification programs as a suite of the new compiler

`verification\` holds the measurement programs of the old compiler, each with what `qb64pe.exe` gave
(`<name>.compile.txt`, `<name>.out.txt`; catalogue of the families in `verification\README.md`). This folder holds
what the new compiler is held to against them, so that `verification\` keeps the old compiler's measurements only
(change `verification-compare`, spec `testing/verification-suite`):

| File | Content |
|---|---|
| `pass.list` | the programs the new compiler passes, one name per line (no `.bas`; `#` starts a comment) |
| `known.list` | every other program: its name, a tab, the reason (below) |
| `lines.list` | the programs QB64pe rejected that the new compiler rejects on another line than QB64pe's `LINE n:`: the name, a tab, the line the rejection is pinned to instead, a tab, the reason (mostly a block error, named where the block opens). That line stands for QB64pe's (below, "How the compare works"); a row that a run finds unneeded is named at the end of the run, to remove (OpenSpec change `pin-rejection-line`). A program whose `diverged\` sidecar holds `(not compiled)` needs a row too: QB64pe built it, so the row is the only line compared (OpenSpec change `known-output`) |
| `divergences\<name>.divergences` | the rows of `DIVERGENCES-QB64PE.md` a passing program may take, one `D-0nn` per line, as for the corpus |
| `fixed\<name>.out.txt` | the program's output with every fix on (spec `compiler/fixes`), written by hand from the fixes' specified behaviour, never recorded by `qb64pe.exe`: the runner builds such a program a second time with `--fixes on` and compares that run with it, as the result `<name> [fixes on]`; the program's line on `pass.list` passes only when both halves pass (OpenSpec change `fixes-switch`, design D4). Each fix behind the switch has at least one |
| `diverged\<name>.out.txt` | the output our compiler gives for a program that differs from QB64pe on purpose, by a row `D-0nn` of `DIVERGENCES-QB64PE.md`. It is our run's output, written only after a review: each line that differs from QB64pe's recording must be one the program's D row explains. The compare judges the program by it in place of QB64pe's recording. The program is on `pass.list`, and its `divergences\<name>.divergences` names its rows. An optional `diverged\<name>.log.txt` stands for the recorded log. An optional `diverged\<name>.linux.out.txt` judges the program on Linux, where our output differs from Windows's. There are six: whose partial last line of output comes before libqb's error message on Linux and after it on Windows, or whose `$IF WIN` takes the other branch. A sidecar of `(not compiled)` needs a `lines.list` row, because QB64pe built the program and names no line (OpenSpec change `known-output`) |
| `normalize\<name>.normalize` | rules applied to both the recorded and the actual output before they are compared, as for the corpus (`<regex><TAB><replacement>`); the first is `v58_s02_files`'s, which masks the free space `FILES` prints; `v59_g_readback`'s masks the OpenGL viewport, the window's size as the desktop sets it |

Every program of `verification\` is named exactly once across the two lists; tier 1 checks it
(`verification_lists_are_exact` in `crates\driver\tests\it\inputs.rs`). A change that adds a family adds its programs
to one of the lists.

## The reasons of `known.list`

Each may be followed by `: <more>`:

| Reason | Meaning |
|---|---|
| `D-0nn` | a row of `DIVERGENCES-QB64PE.md`: the new compiler differs on purpose (the math functions of D-026, say) |
| `not supported yet` | the old compiler accepts the program, the new one rejects it with errors marked "not supported yet" |
| `varies` | the output changes from run to run (time, addresses, handles), and no `.normalize` rule makes both sides equal |
| `not recorded` | no `<name>.out.txt`: the old compiler's executable did not start |
| `recorded elsewhere` | recorded with another `--verification-root` |
| `bug: <what>` | a bug of the new compiler, with a line in `STATUS.md`'s known bugs |

A failure of a program of `known.list` counts as a known failure (`KFAIL`) in a run; a program of `known.list` that
passes is named at the end of the run, to move to `pass.list`, unless its reason is `varies` (it passes on some runs).

## How the compare works

`run_legacy_tests.py --suite verification` with `qb64pe` records (unchanged); with any other compiler it compares
(`tools\legacy_tests\README.md`). Each program is compiled with `-x -q -m` in a fresh folder
`target\legacy-tests\verification\<name>\` that holds the program, its `.stdin` and `.noprompt`, and the support files
of its family (the files and folders of `verification\` whose name begins with its family number, such as
`v16_inc\`, `v37c.a` or `v37_lib\`, and those with no family number). It runs as it was recorded: stdin from the null
device or the `.stdin`, a 60 s timeout, no key presses, its window on a desktop that is never shown. It is
judged by `diverged\<name>.out.txt` when that exists, else by QB64pe's recording. Then:

- recorded `(not compiled)`: passes when the compile fails with at least one error not marked "not supported yet",
  and the earliest line of the program's own file with such an error, in whatever order the errors are printed, is
  the first `LINE n:` of `<name>.compile.txt` (or the line `lines.list` gives instead); else it fails at the stage
  `error line`. No line is compared when the recording names none (or `LINE 0:`), or when QB64pe's message places
  the error in an included file ("in line … of … included"). The message text is not compared;
- recorded output: passes when the build runs and its output, with the folder masked as `<verification>` and the
  `exit=` line as the recording writes it, equals the recording (CRLF as LF, trailing newlines ignored);
- a recorded timeout passes when the run times out too with the same output before it.

A program with a `<name>.root` sidecar (`tools\legacy_tests\README.md`, "Verification programs") is compiled with
`--include-root` of a folder `root\` in its own folder that holds what the sidecar lists: the files it names
(`v47_e01`, an icon) and the contents of the folder it names (`v58_u*`: `v58_root\`, whose `libraries\` holds the
test libraries of `$USELIBRARY`). The recording was made with `qb64pe.exe` in a copy of the release whose own
folder held the same, so both compilers see one compiler root.

The `v37` programs need the libraries `verification\v37_lib\build_libs.py` builds (git-ignored); `check`'s step
`v37-libs` builds them before the verification programs run (`tools\check\README.md`).

```
python tools\legacy_tests\run_legacy_tests.py --suite verification --qb64 target\release\qb64rust.exe ^
    --list tests\verification\pass.list --jobs 8 --build-cache target\build-cache --pass-cache target\pass-cache
```

`just full` runs the pass list with `--pass-cache` (`tools\check\README.md`), and `just quick` runs the programs a
change adds or changes alone; at a merge into `main` `just merge` runs the whole suite without the cache and holds the pass list to it.

## Counts

The current numbers, written by `python tools\legacy_tests\counts.py` (not by hand):

<!-- counts:verification-membership -->
Of the 4458 programs of `verification\`, `pass.list` names **4384** and `known.list` 74.

Of `pass.list`, 128 differ from QB64pe on purpose and are judged by their `diverged\` output.

| Reason in `known.list` | Programs |
|---|---|
| `D-004` | 1 |
| `D-016` | 1 |
| `D-019` | 1 |
| `D-020` | 1 |
| `D-031` | 1 |
| `D-037` | 1 |
| `D-050` | 2 |
| `D-061` | 1 |
| `D-063` | 1 |
| `bug` | 2 |
| `not recorded` | 4 |
| `not supported yet` | 53 |
| `recorded elsewhere` | 1 |
| `varies` | 4 |
<!-- /counts:verification-membership -->

<!-- counts:verification-results -->
Last full run: 2026-11-13, compiler build `06821b59a04d`, folding on, `--jobs 8`, 38 min. Of 4465 programs: **4393 pass**, 0 do not (a wrong result, or rejected or with only marked errors where the old compiler accepts the program), 72 known failures (`target\legacy-tests\results-verification.json`).
`pass.list` judged in it (`--check-list`): **4384 of 4384** pass.
<!-- /counts:verification-results -->
