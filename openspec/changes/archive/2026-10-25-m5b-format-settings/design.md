# Design

## Context

The oracle of `m5b-formatter` (its design D6) records `qb64pe -y -m` with the six defaults
(`FORMAT_DEFAULTS` in `tools\legacy_tests\run_legacy_tests.py`) into `tests\format\<set>\<path>`, for four
sets: `corpus`, `upstream`, `snippets` and `programs`. Tier 1 (`crates\driver\tests\format_oracle.rs`,
`recordings()`) walks those four sets and formats with `Settings::default()`. The runner's compare walks every
folder under `tests\format\` (`format_recordings`) and maps a recording to its set by the first folder
(`format_set`). A `settings\` folder would be taken for a fifth set and fail. `counts.py`'s `format_membership`
counts each folder under `tests\format\` as a set. `close.py` already asks for the format runs when anything under
`tests\format\` changes (`FORMAT_INPUTS`).

The formatter takes the settings as `qb64rust_base::layout::Settings`, filled by `Settings::set(name, value)` from
`-f:` settings. The five other settings are checked only by QB64pe's `format_tests` (`--suite format`, 24 cases).

## Goals / Non-Goals

**Goals:** each non-default setting is held to `-y` on programs that reach most statement forms; there is one place
where a variant is named, the folder name; and tier 1 time stays small (about 30 programs times 6 variants, beside
1,180).

**Non-Goals:** combinations of settings; a new comparison rule (the variants are compared exactly as the defaults
are).

## Decisions

### D1. A variant is a folder named by its `-f:` setting

`tests\format\settings\<name>=<value>\<set>\<path>`, for example `settings\autoindentsize=2\corpus\...`. The
runner passes the defaults with the one named by the folder replaced. Tier 1 starts from `Settings::default()` and
calls `Settings::set(name, value)`. So neither side keeps a table of the variants. The variants that exist are the
folders, and the recording writes the six of the proposal from one list, `FORMAT_VARIANTS`, beside
`FORMAT_DEFAULTS`. The alternative was a sidecar file per variant listing its flags. It was rejected because it is
one more file to keep in step, and every variant here is a single setting. `=` is a valid character in a Windows
or Git path.

Code: `run_legacy_tests.py` (`FORMAT_VARIANTS`, `format_flags(variant)`, and `format_set` and `format_corpus_test`
reading `settings\<variant>\` as the variant and then the set), and `format_oracle.rs` (`recordings()` walks
`settings\*\<set>` as well, and `Program` gains its `Settings` and a name `settings/<variant>/<set>/<path>`).

### D2. The list: chosen by measurement, kept by hand

`tests\format\settings.list` names `<set>/<path>` of default recordings, with a comment after `#` saying what each
reaches. It is chosen once, in task 1.2, by a script in the scratchpad. For each recorded program, it formats with
`qb64rust -y` under each variant and counts the lines that differ from the default output, which shows where a
setting acts. It also takes the program's statement forms, as tier 1 prints them. It then picks programs greedily
until every variant has many differing lines in the set and the forms are covered. The script is not kept: the list
is short, and the reason for each entry is in its comment. Civil War Strategy (`programs`) is left out because one
file of about 9,000 lines under six variants is more disk than all the rest. Its forms are reached elsewhere.

### D3. What no accepted program has: new slice programs

Some forms are not in any accepted program: an unclosed string at the end of a line, `'$FORMAT:OFF` and
`'$FORMAT:ON`, indentation and spacing written by hand (what `autoindent=0` and `autolayout=0` keep), and a
comment after each kind of block line. For these, new programs `tests\corpus\slice\s160_layout_*.bas` are written.
They are ordinary corpus programs: headless, with a recorded `.output`, and added to `slice.list`. So they join the
default oracle, and the list then names them. Putting them in the corpus rather than a folder of their own keeps the
four sets and gives them a run as well. The cost is a few seconds per slice run.

### D4. Known differences under a variant's name

A variant that differs is listed as `settings/<variant>/<set>/<path>` in the same
`tests\format\known_differences.list`. A difference that the formatter fixes in this change is fixed in
`crates\format` or `crates\base\src\layout.rs`, not listed. The list stays shrink-only. If `-y` itself goes wrong
under a setting, for example by changing the program, that is a case for D8 of `m5b-formatter` (a
`DIVERGENCES.md` row and an `UPSTREAM.md` row), and the sidecar goes beside the variant's recording.

### D5. Counts

`format_membership` counts `settings\` apart from the sets: it adds the line "under the other settings: N
recordings of M listed programs, 6 variants". This keeps the table by set as it is.

## Risks / Trade-offs

- [`autolayout=0` or `autoindent=0` keeps the user's spacing, so most accepted programs, already laid out by
  QB64pe's IDE, show little under them] → D3's hand-written programs carry the spacing and indentation that
  these settings keep.
- [Under one variant the formatter may differ in many places, more than this small change means to fix] → each is
  fixed or listed (D4). If they are many, the list is reported at the close as work for `m5b-language-features` or
  for a change of its own. They are not fixed here without bound.
- [A recording under a variant fails, because `-y` rejects or crashes with a setting] → the program leaves the list
  with a comment, and an `UPSTREAM.md` row is added if it is QB64pe's bug.
