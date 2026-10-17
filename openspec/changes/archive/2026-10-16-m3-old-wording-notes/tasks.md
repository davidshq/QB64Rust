# Tasks

Standing rules: `CLAUDE.md` (no subagents, nothing staged, backslashes through the tools, test cadence rule 9).

## 1. The table

- [x] 1.1 `tools\diagnostics\old_wording.py` (design D1, D2): 1,336 pairs on the same line (16 on another),
  643 patterns, 431 kept (84 whose pairs disagree, 41 naming a file, 79 with text of one program, 8 worded the
  same).
- [x] 1.2 `--check`: 696 notes on recorded rejections, all the recorded message. Found by it on the way and fixed: a
  name found inside an earlier mark (`{{3}}`), and a lookup that let a wider pattern answer for a span the script
  keeps literal (now exact keys with the script's word list), and a name without a suffix that fixed
  its form for every name (`count&` gave `'count&'`, measured: `qb64pe.exe` writes `'count'`; now each
  placeholder lists the forms all its pairs allow, and no note is given where they disagree for a name).

## 2. The note

- [x] 2.1 `qb64rust_base::old_wording`, `Diagnostic::render`'s note line, the language server's message (design
  D3); unit tests of keys, the lookup and the transforms.
- [x] 2.2 Tier 1: 95 `check-fail` snapshots, each only gaining note lines (758, checked by a script), accepted (7 changed again with the forms, 19 note lines); 328
  pass; fmt, clippy; the CLI build tests 32 of 32.

## 3. Close

- [x] 3.1 Documents: `CLAUDE.md`'s layout table, `tools\diagnostics\README.md`, `crates\README.md`, `STATUS.md`,
  `DECISIONS.md`. Not rerun for this change: tier 2 (no executable changes: only the text of a rejection), the
  call-site check (no call changes).
