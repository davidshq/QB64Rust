# The format oracle

The old compiler's `-y` output of real programs, which the new compiler's formatter is held to (spec
`testing/format-oracle`; `openspec\changes\archive\2026-10-25-m5b-formatter\design.md` D6). Never edited by hand.

## What is recorded

`<set>\<path within the set>` holds `qb64pe -y`'s output for each program the old compiler accepts in these sets:

| Set | Programs |
|---|---|
| `corpus` | `tests\corpus\` |
| `upstream` | `tests\upstream\compile_tests\` (formatted in the clone's `tests\compile_tests`, which has the binary files and QB64pe's sources the copy leaves out) |
| `snippets` | `tests\snippets\` |
| `programs` | `tests\programs\` (Civil War Strategy) |

A program with an `.err` beside it is rejected by the old compiler and has no recording. The verification
families are left out (about 4,000 programs and 5 MB, mostly the same statement forms again; design D6).

Each output was written with `-y -m` and QB64pe's built-in layout defaults given explicitly, so that QB64pe's
`config.ini` plays no part:

```
-f:autoindent=1 -f:autoindentsize=4 -f:indentsubs=1 -f:autolayout=1 -f:keywordcapitals=0 -f:keywordlowercase=0
```

`qb64pe` writes CRLF after every line; a recording is compared with its line endings taken as LF.

<!-- counts:format-membership -->
**1180** programs recorded; `known_differences.list` names 1. By set:

| Set | Recorded |
|---|---|
| `corpus` | 489 |
| `programs` | 1 |
| `snippets` | 342 |
| `upstream` | 348 |
<!-- /counts:format-membership -->

## Recording again

```
python tools\legacy_tests\run_legacy_tests.py --suite format-corpus                    # every set
python tools\legacy_tests\run_legacy_tests.py --suite format-corpus --category corpus  # one set
```

The runner formats each source with the clone's `qb64pe.exe`, holding the clone's lock, and writes the output only
when it differs from the recording other than by line endings (`tools\legacy_tests\README.md`, "The format
oracle"). A program the old compiler fails to format fails the run and gets nothing. Take the counts above from
`python tools\legacy_tests\counts.py`.

## What holds the new compiler to it

- **Tier 1**, `crates\driver\tests\format_oracle.rs`: each recorded program formatted in process with the
  defaults must equal its recording; the formatted text must give the same typed tree as the source (spans
  removed, names compared without regard to case), and formatting it again must change nothing.
- **Tier 2**, `run_legacy_tests.py --suite format-corpus --qb64 target\release\qb64rust.exe`: the same
  equality through the command line.
- `known_differences.list`: the programs whose formatted text differs from the recording, one
  `<set>/<path without .bas>` per line, a tab, and the reason. It only shrinks: a program on it that comes to
  equal its recording fails tier 1 until it is taken off.
- `<name>.divergences` beside a recording: the rows of `DIVERGENCES.md` the program takes, where the formatter
  does not copy `-y` on purpose (design D8). None so far.
