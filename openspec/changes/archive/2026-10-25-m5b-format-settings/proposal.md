# Proposal

## Why

The format oracle (`tests\format\`, `m5b-formatter`) holds the formatter to `qb64pe -y` only under QB64pe's
default layout settings. The other settings (`AutoIndent` off, `AutoIndentSize`, `IndentSubs` off, `AutoLayout`
off, keywords in capitals or in lower case) are checked only by QB64pe's own `format_tests`: 24 cases over 5
small programs. The extension now passes its `qb64rust.format.*` settings to the new compiler, so a user who
turns layout off or picks upper-case keywords gets output that almost nothing compares. The review of
`m5b-formatter` (2026-10-25, its `tasks.md` "Workflow follow-up") asked for a small set under each setting. It
comes before `m5b-language-features`, so that the formatter is held under every setting before the server gains
more features.

## What Changes

- **Recordings under each non-default setting**: for a list of about 30 programs, `qb64pe -y`'s output under
  each of six variants. Each variant is the defaults with one setting changed: `autoindent=0`,
  `autoindentsize=2`, `indentsubs=0`, `autolayout=0`, `keywordcapitals=1` and `keywordlowercase=1`. They are kept
  in `tests\format\settings\<variant>\<set>\<path>`, and the folder name is the `-f:` setting itself.
- **The list** `tests\format\settings.list` (hand-kept, one `<set>/<path>` per line) chooses programs of the
  oracle that between them reach blocks of every kind, SUBs and FUNCTIONs, `TYPE`, `DECLARE LIBRARY`, `$IF`,
  `SELECT CASE`, labels and line numbers, comments after statements, joined lines and `?`. For the forms no
  accepted program has (an unclosed string at the end of a line, `'$FORMAT:OFF`, indentation the user wrote by
  hand, odd spacing), a few new programs `tests\corpus\slice\s160_layout_*.bas` are written. They have their
  output recorded and join `slice.list`, so that they are also run.
- **The runner's `--suite format-corpus`** records the variants with `qb64pe` and compares them with any other
  compiler, alongside the default recordings.
- **Tier 1** (`crates\driver\tests\format_oracle.rs`) formats each variant's recordings in process with that
  variant's settings. It applies the same three checks as for the defaults: the text equals the recording, the
  formatted text is the same program, and formatting again changes nothing. A difference goes on
  `known_differences.list` under the variant's name, which only shrinks.

Out of this change: combinations of two or more non-default settings (QB64pe's `format_tests` already have
some), and recording the whole oracle under each setting.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `testing/format-oracle`: three added requirements. The variants are recorded for the listed programs, the runner
  records and compares them, and tier 1 checks them. The existing requirements, which cover the defaults, are
  unchanged.

## Impact

- `tools\legacy_tests\run_legacy_tests.py`: the variants in the recording and the comparison (`FORMAT_DEFAULTS`
  and one changed setting); its tests; `tools\legacy_tests\README.md`, "The format oracle".
- `tools\legacy_tests\counts.py`: the oracle's count block, with the variants.
- `crates\driver\tests\format_oracle.rs`: the variants, each with its `Settings`.
- `tests\format\`: `settings.list`, `settings\`, and `README.md`.
- `tests\corpus\slice\`: the new layout programs with their outputs, and `slice.list`.
- The formatter (`crates\format`, `crates\base\src\layout.rs`) changes only where a variant shows a difference
  from `-y`.
