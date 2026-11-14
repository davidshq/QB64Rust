# The QB 4.5 oracle (and PDS 7.1)

`qb45.py` builds a program with QuickBASIC 4.5 (or BASIC PDS 7.1: below) under DOSBox and records what it writes, so a
`DIVERGENCES-QB45.md` row can say **measured** instead of **believed** (`DECISIONS.md` 2026-10-26).

```
python tools\qb45\qb45.py [--qb45 DIR] [--record] verification\qb45\q45_static_local.bas
```

- **QB 4.5** is not in this repo. It lives in `..\qb45\install` from the repo's root: in the repo's parent,
  beside the repo and `pds71` (moved there from `<qb64contain>\qb45` on 2026-11-12). `qb45.py`
  looks there when neither `--qb45` nor `QB64RUST_QB45_ROOT` is given. The files of the five disks of the archive (`DISK1`–`DISK5`, already unpacked there), copied into one folder. That is what the disks'
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
- **A run in which BC did not finish** is not a result. BC finished when `P.CMP` holds its last line, "Severe
  Error(s)". It happened once on 2026-11-02: the record held only "(no OUT.TXT)", and the same program ran in
  full the next time. Such a run is tried once more. If BC still does not finish, nothing is recorded and
  `qb45.py` exits with 1. The message says whether DOSBox ended early or hung until the timeout, and shows
  on stderr what BC wrote before it stopped.
- **A program that was built and ran in time but wrote no `OUT.TXT`** is recorded, with a warning on stderr.
  Every `q45_` program opens `OUT.TXT` first, so the likely cause is DOSBox ending early; run it again.

The programs are in `verification\qb45\` (`q45_*.bas`), outside the top level that `--suite verification`
reads, so the QB64pe suites never see them.

## PDS 7.1

`qb45.py --compiler pds71` runs a program the same way with Microsoft BASIC PDS 7.1, the compiler of programs
ported from PDS (`docs\PORTING-PDS.md`). It was added on 2026-11-05 to measure Q-018, which the World War 2
game's editor hit.

```
python tools\qb45\qb45.py --compiler pds71 [--pds71 DIR] [--record] verification\qb45\q45_gosub_left_in_sub.bas
```

- **The build and the result** are those of QB 4.5 above: `BC P.BAS /O /E /X /D;` with PDS's BC 7.10, then
  `LINK P.OBJ;` with LINK 5.10. The record is `<name>.pds71.txt`, beside the program's `.qb45.txt`.
  `--pds71 DIR` alone also picks PDS 7.1, so it never records a QB 4.5 run by mistake.
- **PDS 7.1** is not in this repo either. It lives in `pds71\install` in the repo's parent folder,
  `..\pds71\install` from the repo's root. `qb45.py` looks there when neither `--pds71` nor
  `QB64RUST_PDS71_ROOT` is given. The runner needs `BC.EXE`, `LINK.EXE` and `BCL71ENR.LIB` there.
- **How it is made:** `pds71_install.py DISKS OUT`. DISKS holds the files of PDS 7.1's five 360K disks,
  one subfolder per disk (`Setup`, `Disk 2` to `Disk 5`; leave out the CodeView disk).
  - The script expands every file, as SETUP would. The disks use the QuickBASIC form of Microsoft's SZDD
    compression. Names come from the disks' `PACKING.LST`.
  - It then builds `BCL71ENR.LIB` with PDS's `LIB.EXE` under DOSBox. That is the library LINK takes for
    `BC /O` with emulator math and near strings in real mode. SETUP would build it, but SETUP only works
    through its dialogs. The parts it joins come from SETUP's own table; the script's comment lists them.
- **Checked on 2026-11-05:** of the 286 files the script makes, 247 have the same name in the folder made
  by hand before it, and those are byte for byte the same, `BCL71ENR.LIB` and the programs among them. The
  other 39 could not be compared: the hand-made folder had kept the `$` in their names, and it was deleted
  before that was noticed. A three-line program and `q45_gosub_left_in_sub` run. A library mixed from the
  stand-alone and the run-time module parts links, but its programs hang before their first line.
- **Not covered:** the QBX environment (`QBX /RUN P.BAS`). It runs too, but only with a `QBX.INI` in the
  program's folder, or it stops at its Welcome dialog; Q-018 was checked there by hand.

Its tests: `python -m unittest discover -s tools/qb45 -p "test_*.py"` (both scripts).
