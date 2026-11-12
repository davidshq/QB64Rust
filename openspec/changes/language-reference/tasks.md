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
- [ ] 3.2 Batch 2: types, literals, operators, conversions (`numeric-semantics`, `evaluation-order`,
      `default-types`). First session (15 pages, 15 examples): the operators `AND`, `OR`, `XOR`, `NOT`, `EQV`,
      `IMP`, `MOD`, `_ANDALSO`, `_ORELSE`, `_NEGATE` and the conversions `CINT`, `CLNG`, `CSNG`, `CDBL`, `_CAST`.
      Left: the types (`INTEGER`, `LONG`, `SINGLE`, `DOUBLE`, `STRING`, `_BIT`, `_BYTE`, `_INTEGER64`, `_FLOAT`,
      `_OFFSET`, `_UNSIGNED`), `DEFINT` to `DEFSTR`, and `_DEFINE`.
- [ ] 3.3 Batch 3: variables, scope, constants (`procedures`, `constants`)
- [ ] 3.4 Batch 4: control flow (`control-flow`; Sonnet may take it)
- [ ] 3.5 Batch 5: procedures (`procedures`)
- [ ] 3.6 Batch 6: arrays and `TYPE` (`arrays-and-types`, `unstable-features`)

## 4. Batches 7 to 17 (Sonnet, but 11 by Opus)

- [ ] 4.1 Batch 7: strings (`fixed-length-strings`, the string functions of `builtin-functions`)
- [ ] 4.2 Batch 8: math and number functions (`builtin-functions`)
- [ ] 4.3 Batch 9: `PRINT`, `PRINT USING`, console and text output (`print-using`, `console-and-window`)
- [ ] 4.4 Batch 10: keyboard and console input (`keyboard`, `builtin-statements`)
- [ ] 4.5 Batch 11: errors (`error-handling`, `run-time-checks`, `asserts-and-debug`; Opus)
- [ ] 4.6 Batch 12: files (`file-io`, `record-files`, `data-read`)
- [ ] 4.7 Batch 13: screen modes and drawing (`screen-output`)
- [ ] 4.8 Batch 14: images and fonts (`images`, `fonts`)
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
