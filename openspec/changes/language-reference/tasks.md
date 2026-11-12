# Tasks

Standing rules: CLAUDE.md. The procedure of every batch: `tools\reference\README.md`, "Writing a page", and
`study\49` §6. One batch is one session; a batch of more than about 15 pages stops after 15, and the next
session goes on with it. Each batch ends by adding its number to `done` in `tools\reference\words.toml`, so that
`--check` then requires all its pages.

## 1. The tool

- [x] 1.1 `tools\reference\reference.py` with `inventory`, `evidence`, `skeleton`, `render`, `--check` and
      `overlap`, and its hand-kept `words.toml` (design D2–D5, D8). Verify: `test_reference.py` passes
      (`python -m unittest discover -s tools/reference -p "test_*.py"`).
- [x] 1.2 The examples' corpus `tests\reference\` with `pass.list`, and the steps `reference` and
      `reference-examples` of `check` (design D6, D7). Verify: `tools\check`'s tests pass, with a test that a page
      plans only the `reference` step.
- [x] 1.3 `tools\reference\README.md` and `tests\reference\README.md`; rows in `docs\LAYOUT.md`.

## 2. The pilot

- [x] 2.1 Six pages, one of each kind: `LEFT$` (a plain function), `LINE` (a statement read by its template),
      `SELECT CASE` (a block of keywords), `$INCLUDE` (a metacommand), `_MEMCOPY` (run-time errors) and `FRE`
      (rejected, as in QB64pe: no word is wholly "not supported yet", design D5). Five examples recorded with
      `qb64pe.exe` and passing with `qb64rust`. Verify: `--check --compiler` and `overlap` report no problem.
- [x] 2.2 What the pilot found: `DIVERGENCES-QB45.md` had no row for `LEFT$` and `RIGHT$` with a negative length,
      which QB 4.5 and PDS 7.1 reject with error 5 (`verification\qb45\q45_left_negative`, measured for the
      Civil War Battleset port). Added as Q-037.
- [x] 2.3 The user reads the six pages. What they change goes into the layout (`tools\reference\README.md`, the
      pages, `reference.py`'s `HEADINGS`) before batch 1 starts. Read with no changes (user, 2026-11-11).

## 3. Batches 1 to 6 (Opus)

Each: run `inventory`, list the batch's words (`docs\reference\inventory.json`, field `batch`), move any word
that belongs elsewhere with `[area]` and cover helper words with `[page]`, then write the pages and examples,
record and compare the examples, run `render --compiler`, `--check --compiler` and `overlap`, and report.

- [x] 3.1 Batch 1: lines, names, comments, labels, `$INCLUDE`, the preprocessor (`compiler/pipeline`). Seven
      pages: `$IF`, `$LET`, `$ERROR`, `$ERRORLOCATION`, `$INCLUDEONCE`, `$NOPREFIX`, `REM`; four examples. The
      library words (`DECLARE LIBRARY`, `ALIAS`, `BYVAL`, `CUSTOMTYPE`, `DYNAMIC`) moved to batch 19. Lines,
      names and labels have no word of their own: they go in the chapter (6.1).
- [x] 3.2 Batch 2: types, literals, operators, conversions (`numeric-semantics`, `evaluation-order`,
      `default-types`). First session (15 pages, 15 examples): the operators `AND`, `OR`, `XOR`, `NOT`, `EQV`,
      `IMP`, `MOD`, `_ANDALSO`, `_ORELSE`, `_NEGATE` and the conversions `CINT`, `CLNG`, `CSNG`, `CDBL`, `_CAST`.
      Second session (13 pages, 13 examples): the types `INTEGER`, `LONG`, `SINGLE`, `DOUBLE`, `STRING`, `_BIT`,
      `_BYTE`, `_INTEGER64`, `_FLOAT`, `_OFFSET`, `_UNSIGNED`, one page `defint` for `DEFINT` to `DEFSTR`
      (`[page]` rows), and `_DEFINE`. Found: `language/numeric-semantics` "_BIT stores" still says a `_BIT` array
      is "not supported yet", where `language/arrays-and-types` "Static arrays" and the compiler accept one.
- [x] 3.3 Batch 3: variables, scope, constants (`procedures`, `constants`). Nine pages and nine examples:
      `$COLOR`, `COMMON`, `CONST`, `DIM` (with `AS`), `LET`, `OPTION` (with `BASE`, `_EXPLICIT`,
      `_EXPLICITARRAY`), `SHARED`, `STATIC`, `SWAP`. Found: `SWAP` is a statement of the built-in table with no
      syntax there, so its page gives the form by hand.
- [x] 3.4 Batch 4: control flow (`control-flow`; Sonnet may take it). Thirteen pages: `IF`, `FOR`, `DO`, `WHILE`,
      `GOTO`, `GOSUB`, `ON`, `EXIT`, `_CONTINUE`, `END`, `STOP`, `SYSTEM`, `RUN`; twelve examples (`RUN` has a
      probe). Found: `FOR 5 = 1 TO 2` is an internal error (a `SOMEDAY.md` row).
- [x] 3.5 Batch 5: procedures (`procedures`). Four pages and four examples: `SUB`, `FUNCTION`, `CALL`, `DECLARE`
      (with `ANY`).
- [x] 3.6 Batch 6: arrays and `TYPE` (`arrays-and-types`, `unstable-features`). Nine pages and nine examples:
      `REDIM` (with `_PRESERVE`, `_RETAIN`), `ERASE`, `LBOUND`, `UBOUND`, `TYPE` (with the `_STATIC` and
      `_DYNAMIC` markers), `_ARRAYCOPY`, `$DYNAMIC`, `$STATIC`, `$UNSTABLE`. Found: `language/arrays-and-types`
      "User types" still says a member array marked `_STATIC` or `_DYNAMIC` is "not supported yet", where
      "Member arrays", "Dynamic member arrays" and the compiler build it.

## 4. Batches 7 to 17 (Sonnet, but 11 by Opus)

- [x] 4.1 Batch 7: strings (`fixed-length-strings`, the string functions of `builtin-functions`). 25 pages and 25
      examples, in two sittings. Grouped pages (`[page]` rows): `cvi` (`CVI`, `CVL`, `CVS`, `CVD`), `mki-str`,
      `cvsmbf`, `mksmbf-str`, `_strcmp` (with `_STRICMP`); `_CV` moved here from batch 8, beside `_MK$`.
- [x] 4.2 Batch 8: math and number functions (`builtin-functions`). 33 pages and 33 examples, the families
      grouped by `[page]` rows (`_asin` with `_ACOS`, `_sinh`, `_asinh`, `_sec`, `_sech`, `_arcsec`, `_d2r` with
      the four gradian conversions, `_setbit` with `_RESETBIT`, `_TOGGLEBIT`, `_READBIT`, `_shl`, `_rol`, `_min`).
      The examples print rounded values where the digits come from the math library (`CLAUDE.md` working rule 8).
      `_COTH` stands in no example: QB64pe's is another function (D-031).
- [x] 4.3 Batch 9: `PRINT`, `PRINT USING`, console and text output (`print-using`, `console-and-window`). 16
      pages, 14 examples (`LPRINT` and `_CONSOLEINPUT` have probes: no printer, no console input in the tests).
      `CSRLIN`, `POS`, `CLS`, `LOCATE` and `TAB` print on a text image and read it back with `SCREEN`, since a
      console whose output is a pipe has no cursor. `LINE INPUT` moved to batch 10.
- [x] 4.4 Batch 10: keyboard and console input (`keyboard`, `builtin-statements`). 8 pages, 6 examples
      (`INPUT` and `LINE INPUT` with `.stdin` files); `_KEYDOWN` and the lock keys have probes, since a test cannot
      hold a key or read the machine's lock state. `_NUMLOCK` and `_SCROLLLOCK` on the `_capslock` page, `_CINP` on
      `_consoleinput`.
- [x] 4.5 Batch 11: errors (`error-handling`, `run-time-checks`, `asserts-and-debug`; Opus). 13 pages, 12
      examples (`TRON` and `TROFF`, rejected as in QB64pe, have a probe). The examples raise errors both compilers
      trap: an integer division by zero is critical in QB64pe (D-052). Found: `language/error-handling` "Errors
      raised inside procedures" still says `RESUME` inside a procedure is "not supported yet", where "RESUME inside
      a procedure" builds it.
- [x] 4.6 Batch 12: files (`file-io`, `record-files`, `data-read`). 29 pages, 26 examples; `FILES` and `_DIR$`
      (their output holds the machine's paths) and the stubs `FILEATTR` and `IOCTL` have probes. The `OPEN` page
      covers the mode and access words; `chdir` covers `MKDIR` and `RMDIR`, `lset` `RSET`, `lock` `UNLOCK`,
      `_fileexists` `_DIREXISTS`, `_readfile-str` `_WRITEFILE`, `_cwd-str` `_STARTDIR$`.
- [x] 4.7 Batch 13: screen modes and drawing (`screen-output`). 28 pages, 25 examples, each drawing on an image
      made with `_NEWIMAGE` and printing `POINT` or `SCREEN` values; `_DISPLAY`, `_AUTODISPLAY` and `_FPS` (a
      window's) have probes. Color families on one page each: `_red`, `_red32`, `_rgb`, `_rgb32`, `_hsb32`,
      `_hue32`, `_defaultcolor`; `PRESET` on `pset`.
- [x] 4.8 Batch 14: images and fonts (`images`, `fonts`). 22 pages, 20 examples; `_LOADFONT` (the machine's
      fonts) and `_PRINTIMAGE` (a printer) have probes. Found: `language/images` "Image handles" says `_WIDTH` and
      `_HEIGHT` with no handle give the destination's size, but in a `$CONSOLE:ONLY` program both compilers give the
      console's while `_DEST` is an image (`tests\reference\examples\_width_1`).
- [ ] 4.9 Batch 15: mouse and devices (`mouse`, `device-input`)
- [ ] 4.10 Batch 16: sound (`sound`)
- [ ] 4.11 Batch 17: events and timers (`events`)

## 5. Batches 18 to 23

- [ ] 5.1 Batch 18: memory (`mem-blocks`, `conventional-memory`; Opus)
- [ ] 5.2 Batch 19: libraries (`external-libraries`; Opus)
- [ ] 5.3 Batch 20: window, clipboard, system, date and time (`console-and-window`, `builtin-statements`)
- [ ] 5.4 Batch 21: data processing, embedded files, logging (`data-processing`, `embedded-files`, `logging`)
- [ ] 5.5 Batch 22: network (`network`). Its examples cannot run on the host (`CLAUDE.md` working rule 6): they
      get `.external` sidecars starting `network:` and run in the suites VM, or are `.norun`.
- [ ] 5.6 Batch 23: OpenGL (`opengl`): one page for `SUB _GL` and a generated table of the routines from
      `crates\sema\src\gl_table.rs`, written by `render`.

## 6. Chapters and appendices (Opus)

- [ ] 6.1 One chapter per area in `docs\reference\chapters\`, from its spec and its pages, with the same `src:`
      rule (`reference.py` to check chapters as it checks pages).
- [ ] 6.2 The generated appendices still missing: the error numbers and messages, the reserved words, the
      metacommands, what is not supported yet (from `SOMEDAY.md`, "Deferred QB64pe features").
- [ ] 6.3 "Coming from QB64pe" and "Coming from QuickBASIC 4.5", from the two divergence registers.

## 7. The review (Opus or Fable)

- [ ] 7.1 Read the index, every chapter and 60 pages picked with a fixed seed: pages of one area agree, "See
      also" leads somewhere useful, and 30 real questions find their answer (from `docs\PORTING-PDS.md` and the
      game ports' lessons). Report what to fix.
- [ ] 7.2 Link the reference from `README.md`, `docs\MANUAL.md` and `docs\QUICKSTART.md`; add to `CLAUDE.md`
      working rule 11 that a change of behavior edits its page with its spec.
- [ ] 7.3 Archive the change and `study\49`.
