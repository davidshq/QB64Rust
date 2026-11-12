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
- [x] 4.9 Batch 15: mouse and devices (`mouse`, `device-input`). 18 pages, 15 examples, all in `$CONSOLE:ONLY`
      programs: the device functions with no game controller (the keyboard and the mouse as devices 1 and 2), and
      the mouse functions, which there read the console's mouse and give 0. `_MOUSESHOW`, `_MOUSECURSOR` and
      `_MOUSEMOVE` have probes: their effect is the machine's cursor. Grouped: `_mousex` (`_MOUSEY`),
      `_mousemovementx`, `_mouseshow` (`_MOUSEHIDE`), `_lastbutton` (`_LASTAXIS`, `_LASTWHEEL`), `_button`
      (`_BUTTONCHANGE`), `_axis` (`_WHEEL`).
- [x] 4.10 Batch 16: sound (`sound`). 28 pages, 26 examples, each with a `.external` sidecar (`sound:`) like the
      slice programs they follow (`s74`–`s76`, `s127`–`s129`). Every sound is silence or at volume 0. `BEEP` (the
      bell would be heard) and `$MIDISOUNDFONT` (rejected, as in QB64pe) have probes. `_sndplaying` covers
      `_SNDPAUSED`, `_sndgetpos` `_SNDSETPOS`. Found: `_SNDRAWDONE` does nothing in this libqb (an empty inline
      function in `libqb\include\audio.h`), and `_SNDPLAYFILE`'s second argument is ignored.
- [x] 4.11 Batch 17: events and timers (`events`). 9 pages, 5 examples (`timer`, `_freetimer`, `key`, `_delay`,
      `_limit`); the three that wait have a `.external` sidecar (`clock:`). `COM`, `PEN` and `UEVENT` (rejected, as
      in QB64pe) and `SLEEP` have probes: under the test runner, whose input is no console, `SLEEP 10` returns
      after about 0.1 second in QB64pe's build, so a test cannot time it. `ON TIMER` is on the `timer`
      page, `ON KEY` and `KEY LIST` on `key`.

## 5. Batches 18 to 23

- [x] 5.1 Batch 18: memory (`mem-blocks`, `conventional-memory`; Opus). 23 pages (with the pilot's `_MEMCOPY`,
      the 24 of the batch), 23 examples. Found: `language/conventional-memory` "INP, OUT and WAIT" says a port out
      of range raises error 6, but `OUT` checks no range (`OUT 70000, 1` raises nothing,
      `tests\reference\examples\out_1`; libqb's `sub_out`); `INP` and `WAIT` raise it (`inp_1`, `wait_1`). The
      `v58_s01` programs do not show it: their handler prints to the hidden window.
- [x] 5.2 Batch 19: libraries (`external-libraries`; Opus). Two pages: `declare-library` (with `ALIAS`, `BYVAL`,
      `CUSTOMTYPE`, `DYNAMIC`, `LIBRARY`), whose example calls the C runtime through a block with no name, and
      `$USELIBRARY`, which has no example or probe: it reads the libraries installed under the compiler's folder
      (`verification\v58_u*` give one with `.root`, which the examples' runner does not take). Its availability is
      "not checked".
- [x] 5.3 Batch 20: window, clipboard, system, date and time (`console-and-window`, `builtin-statements`). 48
      pages for 65 words (with the pilot's `FRE`), 18 examples, all `$CONSOLE:ONLY`, printing only what is the same
      on any machine (forms of `DATE$` and `TIME$`, comparisons of `_UPTIME` and `_OS$`). The window, dialog,
      clipboard and `SHELL` words have probes: no test may open a window or dialog, or start a shell on the host.
      `$EXEICON` and `$FORMAT` are "not checked". Found: QB64pe rejects `PRINT "x:"; LEFT$(t$, 1) <> "b" OR …`,
      which qb64rust compiles (a `SOMEDAY.md` row); `_SCREENEXISTS` is -1 in a `$CONSOLE:ONLY` program.
- [x] 5.4 Batch 21: data processing, embedded files, logging (`data-processing`, `embedded-files`, `logging`).
      11 pages, 11 examples; `$EMBED` and `_EMBEDDED$` have `.files` folders with the file built in. `_loginfo`
      covers `_LOGTRACE`, `_LOGWARN` and `_LOGERROR`; the log goes to the runner's log file, not the output.
- [x] 5.5 Batch 22: network (`network`). Its examples cannot run on the host (`CLAUDE.md` working rule 6): they
      get `.external` sidecars starting `network:` and run in the suites VM, or are `.norun`. Built with 7 pages
      and a probe each, no example: recording one with `qb64pe.exe` would run it on the host, which needs
      `QB64RUST_ALLOW_NETWORK`, and only the user sets it. The pages cite `s139_net_errors` and `s141_tcp_loopback`,
      which run in the VM. Examples with `network:` sidecars can follow once the user records them.
- [x] 5.6 Batch 23: OpenGL (`opengl`): one page for `SUB _GL` and a generated table of the routines from
      `crates\sema\src\gl_table.rs`, written by `render`. Built: `_gl` (the word `_GL` added in `[extra]`), whose
      block `opengl` lists the 337 routines with their types and whether they raise error 270 outside the SUB
      (`reference.py` `gl_block`, its heading "The OpenGL routines" in `HEADINGS`, two tests), and `_GLRENDER`,
      `_DISPLAYORDER`, `_DEPTHBUFFER` with probes. One example, `_gl_1`. Found: in a `$CONSOLE:ONLY` program a SUB
      `_GL` that calls a routine (`_glClear`) fails the C++ build with both compilers ("use of undeclared
      identifier 'call_glClear'"), where `language/opengl` "SUB _GL without a shown window" says such a program
      SHALL build; `v59_a5` uses only a constant. qb64rust's message calls the routine "of `DECLARE LIBRARY`" (a
      `SOMEDAY.md` row).

## 6. Chapters and appendices (Opus)

- [x] 6.1 One chapter per area in `docs\reference\chapters\`, from its spec and its pages, with the same `src:`
      rule (`reference.py` to check chapters as it checks pages). 36 chapters: one per spec under
      `openspec\specs\language\`, and `pipeline` (the program's text: lines, names, comments, the preprocessor,
      included files, from `compiler/pipeline`). Each is a summary, the links to its pages (every page is linked
      from one), and sections whose every paragraph names its source. `reference.py`: `check_chapter` (the sources, the links, embedded examples, a `TODO`, a spec of
      the chapter's name), `refresh_chapter` (embedded examples only), the index's list of chapters; two tests;
      "Writing a chapter" in `tools\reference\README.md`. Lines, names and labels, which have no word of their own
      (3.1), are in `pipeline` and `control-flow`.
- [x] 6.2 The generated appendices still missing: the error numbers and messages, the reserved words, the
      metacommands, what is not supported yet (from `SOMEDAY.md`, "Deferred QB64pe features"). Written by
      `render`: `appendix-errors.md` (libqb's `error_code_to_text`), `appendix-reserved.md` (the 102 keywords of
      `keywords.rs`, linked to their pages), `appendix-metacommands.md` (27), `appendix-unsupported.md` (41 rows,
      the "Form" column); three tests. An edit of those rows of `SOMEDAY.md` now needs a `render`.
- [x] 6.3 "Coming from QB64pe" and "Coming from QuickBASIC 4.5", from the two divergence registers. Generated by
      `render` (`coming-from-qb64pe.md`, 62 rows; `coming-from-qb45.md`, 37), each row's case and the two
      compilers' columns, so they follow the registers; the reasons and tests stay in the registers.

## 7. The review (Opus or Fable)

- [x] 7.1 Read the index, every chapter and 60 pages picked with a fixed seed: pages of one area agree, "See
      also" leads somewhere useful, and 30 real questions find their answer (from `docs\PORTING-PDS.md` and the
      game ports' lessons). Report what to fix. Done (2026-11-12): 60 pages by seed 49, 30 questions of
      `PORTING-PDS.md` (29 answered). Fixed in the review: the pages of `ATN`, `COS`, `EXP`, `LOG`, `SIN` and `TAN`
      gave `SQR`'s example of the result type; 103 pages no chapter linked (now every page is linked, and the
      chapter `pipeline` holds the metacommands and the program's text); `_CONSOLEINPUT` named a recording instead
      of its values; `_FREEIMAGE`'s error had no number. Left to decide: (a) "See also" names words as code, not
      as links, on every page, so a reader cannot follow it; `render` could make them links. (b) Nothing answers
      "is `CURDIR$` available?", which `PORTING-PDS.md` section 3 warns builds silently as a variable; a section
      on PDS-only names in "Coming from QuickBASIC 4.5" or pages for them would. (c) The window and dialog pages
      say little more than "compiles to QB64pe's call", as the test rules allow.
- [x] 7.2 Link the reference from `README.md`, `docs\MANUAL.md` and `docs\QUICKSTART.md`; add to `CLAUDE.md`
      working rule 11 that a change of behavior edits its page with its spec. `MANUAL.md` links it by its GitHub
      address, as it does `DIVERGENCES-QB64PE.md`, since the manual ships in the download.
- [x] 7.3 A second review (2026-11-12), by scripts over every page. Fixed: graphics `GET` and `PUT` were not on
      their pages (now their syntax, behavior, errors and D-061, and the example `put_2`); hand-written syntax left
      out words of the table's form on `_MAPTRIANGLE` (`_CLOCKWISE`, `_ANTICLOCKWISE`, `_SEAMLESS`, the `_SMOOTH`
      words, the z of the 3-D form), `_PUTIMAGE` (`STEP`, `_SMOOTH`), `_RED`, `_CAPSLOCK` and `_CONSOLEINPUT`, and
      `--check` now fails on such a page; the generated syntax wrote the table's mixed case (`_Middle`,
      `{On|Off|Stop}`) and joined two forms with "or" on one line; `MID$` with a start of 0 was unclear. Left to
      decide: (d) about 85 pages give "A form QB64pe rejects is a compile error" as their only error, with no
      message; (e) about 54 pages say "as QB64pe does", testable ones included (`SCREEN` has no table of its modes,
      `OPEN` gives only error 53, and its spec measures no other); (f) 80 of 444 requirements are cited nowhere,
      among them whole-array assignment's error 9 and the fixes (no page says what a fix is; `RESUME` and
      `$CHECKING` leave theirs out; `compiler/fixes` and `compiler/lints` have no chapter); (g) 38 divergence rows
      are on no word's page (Q-021, Q-022, D-026, D-027, D-029, D-035, Q-020, Q-036 among them); (h) 329 outputs end
      in "Press any key to continue", which nothing explains.
- [ ] 7.4 Archive the change and `study\49`.
