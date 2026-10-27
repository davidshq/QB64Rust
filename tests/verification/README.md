# The verification programs as a suite of the new compiler

`verification\` holds the measurement programs of the old compiler, each with what `qb64pe.exe` gave
(`<name>.compile.txt`, `<name>.out.txt`; catalogue of the families in `verification\README.md`). This folder holds
what the new compiler is held to against them, so that `verification\` keeps the old compiler's measurements only
(change `verification-compare`, spec `testing/verification-suite`):

| File | Content |
|---|---|
| `pass.list` | the programs the new compiler passes, one name per line (no `.bas`; `#` starts a comment) |
| `known.list` | every other program: its name, a tab, the reason (below) |
| `divergences\<name>.divergences` | the rows of `QB64PE-DIVERGENCES.md` a passing program may take, one `D-0nn` per line, as for the corpus |
| `normalize\<name>.normalize` | rules applied to both the recorded and the actual output before they are compared, as for the corpus (`<regex><TAB><replacement>`); the first is `v58_s02_files`'s, which masks the free space `FILES` prints; `v59_g_readback`'s masks the OpenGL viewport, the window's size as the desktop sets it |

Every program of `verification\` is named exactly once across the two lists; tier 1 checks it
(`verification_lists_are_exact` in `crates\driver\tests\inputs.rs`). A change that adds a family adds its programs
to one of the lists.

## The reasons of `known.list`

Each may be followed by `: <more>`:

| Reason | Meaning |
|---|---|
| `D-0nn` | a row of `QB64PE-DIVERGENCES.md`: the new compiler differs on purpose (the math functions of D-026, say) |
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
device or the `.stdin`, a 60 s timeout, no key presses, its window on a desktop that is never shown. Then:

- recorded `(not compiled)`: passes when the compile fails with at least one error not marked "not supported yet"
  (the message text is not compared);
- recorded output: passes when the build runs and its output, with the folder masked as `<verification>` and the
  `exit=` line as the recording writes it, equals the recording (CRLF as LF, trailing newlines ignored);
- a recorded timeout passes when the run times out too with the same output before it.

A program with a `<name>.root` sidecar (`tools\legacy_tests\README.md`, "Verification programs") is compiled with
`--include-root` of a folder `root\` in its own folder that holds what the sidecar lists: the files it names
(`v47_e01`, an icon) and the contents of the folder it names (`v58_u*`: `v58_root\`, whose `libraries\` holds the
test libraries of `$USELIBRARY`). The recording was made with `qb64pe.exe` in a copy of the release whose own
folder held the same, so both compilers see one compiler root.

The `v37` programs need the libraries `verification\v37_lib\build_libs.py` builds (git-ignored).

```
python tools\legacy_tests\run_legacy_tests.py --suite verification --qb64 target\release\qb64rust.exe ^
    --list tests\verification\pass.list --jobs 8 --build-cache target\build-cache --pass-cache target\pass-cache
```

At a change's close the pass list runs with `--pass-cache` (`tools\close\close.py`); at a merge into `main` the whole
suite runs without it and the list is judged from that run with `--check-list tests\verification\pass.list`.

## Counts

The current numbers, written by `python tools\legacy_tests\counts.py` (not by hand):

<!-- counts:verification-membership -->
Of the 4189 programs of `verification\`, `pass.list` names **4123** and `known.list` 66.

| Reason in `known.list` | Programs |
|---|---|
| `D-003` | 1 |
| `D-004` | 10 |
| `D-005` | 1 |
| `D-006` | 2 |
| `D-007` | 2 |
| `D-008` | 3 |
| `D-009` | 3 |
| `D-014` | 1 |
| `D-016` | 1 |
| `D-017` | 1 |
| `D-018` | 3 |
| `D-019` | 3 |
| `D-020` | 1 |
| `D-021` | 1 |
| `D-022` | 2 |
| `D-025` | 2 |
| `D-026` | 2 |
| `D-027` | 5 |
| `D-028` | 2 |
| `D-029` | 2 |
| `D-031` | 3 |
| `D-032` | 1 |
| `D-034` | 1 |
| `D-036` | 1 |
| `D-037` | 2 |
| `not recorded` | 4 |
| `recorded elsewhere` | 1 |
| `varies` | 5 |
<!-- /counts:verification-membership -->

<!-- counts:verification-results -->
Last full run: 2026-10-26, compiler build `c261b24c2847`, folding on, `--jobs 8`, build cache, 6 min. Of 3956 programs: **3892 pass**, 1 does not (a wrong result, or rejected or with only marked errors where the old compiler accepts the program), 63 known failures (`target\legacy-tests\results-verification.json`).
`pass.list` judged in it (`--check-list`): **3889 of 3890** pass.
- verification:v59_g_readback: changed after the run started: v59_g_readback.normalize
<!-- /counts:verification-results -->
