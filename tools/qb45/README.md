# The QB 4.5 oracle

`qb45.py` builds a program with QuickBASIC 4.5 under DOSBox and records what it writes, so a
`DIVERGENCES-QB45.md` row can say **measured** instead of **believed** (`DECISIONS.md` 2026-10-26).

```
python tools\qb45\qb45.py [--qb45 DIR] [--record] verification\qb45\q45_static_local.bas
```

- **QB 4.5** is not in this repo. It lives in `<qb64contain>\qb45\install`: the files of the five disks of the
  WinWorld archive (`DISK1`–`DISK5`, already unpacked there), copied into one folder. That is what the disks'
  `SETUP.EXE` does for a full install, without its dialogs. The runner needs `BC.EXE`, `LINK.EXE` and
  `BCOM45.LIB` there. Give the folder with `--qb45` or `QB64RUST_QB45_ROOT`.
- **DOSBox** 0.74-3, at its default install path or given with `--dosbox`. It runs with SDL's dummy video driver,
  so no window shows.
- **The build** is `BC P.BAS /O /E /X /D;` (a stand-alone program, `ON ERROR` and `RESUME NEXT` allowed, and
  the run-time checks QB 4.5's editor always makes: without `/D` an index out of bounds raises nothing), `LINK
  P.OBJ;`, then the program, in a fresh temporary folder mounted as `D:`. The source is given CRLF line ends.
  BC still writes an object file after most errors, so a program with a compile error may run too; the record
  shows both.
- **The output** is what the program writes to `OUT.TXT`. QB 4.5's `PRINT` goes to the screen, which DOS cannot
  redirect, so a program opens `OUT.TXT` and uses `PRINT #`. An untrapped run-time error waits for a key, so it
  ends at the timeout (60 s, `--timeout`) and the record says so; trap errors with `ON ERROR` and write `ERR`.
- **The record** (`--record`) is `<name>.qb45.txt` beside the program: BC's messages when it reported an error
  (banner, memory report and bell dropped), the linker's messages when no executable was made, then `OUT.TXT`.

The programs are in `verification\qb45\` (`q45_*.bas`), outside the top level that `--suite verification`
reads, so the QB64pe suites never see them.

Its tests: `python -m unittest discover -s tools/qb45 -p "test_*.py"`.
