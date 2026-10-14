# Roadmap

Milestones from start to finish, and what is done. No progress numbers and no "now" marker: the current step, the
open items and the numbers of the last full runs are in `STATUS.md` (`DECISIONS.md` 2026-10-07). The plan behind
the milestones is `study\00-synthesis.md` §2.

This file was written again on 2026-10-12 from `study\00` §2, `STATUS.md` and `DECISIONS.md`: the earlier file
was never committed and was no longer on disk.

## M0. Baseline: done

- [x] The old compiler builds on Windows
- [x] Windows runner for the QB64pe test suites; baseline recorded (`baselines\`)
- [x] Dialect settled; study gaps closed (`study\09`, `study\10`)

## M1. VS Code extension on the old compiler: done

- [x] Highlighting, build and run, diagnostics from `-z`, formatting through `-y`, CP437 by default (`vscode\`)

## M2. Front end: done

Done when: a lossless parser with recovery reads the whole language with no false syntax error on any program the
old compiler accepts (corpus, upstream tests, `qbasic_testcases`, its own sources), with resolution, a type
checker and a thin language server.

The criterion is met (2026-10-12): tier 1 runs the front end over all four input sets with an exact round trip,
and its shrink-only lists hold no false error and no parse gap (`tests\known_false_errors.list`,
`tests\known_parse_gaps.list`: both empty). What the checker and the code generator do not cover yet is marked
"not supported yet", never a false error; those constructs are work of M3 and M4.

- [x] Golden corpus (`tests\corpus\`)
- [x] Rust workspace and an end-to-end slice (`crates\`)
- [x] Procedures and error handling
- [x] Upstream tests as inputs (`tests\upstream\`)
- [x] Control flow, operators and constants
- [x] Static arrays and `TYPE`
- [x] Parser breadth: every accepted program parses, no false error
- [x] Core built-ins, `SELECT CASE`, `ON … GOTO/GOSUB`
- [x] Thin language server (`qb64rust lsp`)
- [x] The numeric types, `_BIT`, fixed-length strings; the differential tester (`tests\differential\`)
- [x] Built-in statements and functions by demand; the call-site check (`tests\callsite\`)
- [x] The demand report (`qb64rust --report unsupported`) and tests of the test runner
- [x] The screen-state check (`tools\screen\`)
- [x] Default types, `COMMON`, dynamic arrays, arrays in procedures, array parameters, the rest of `TYPE`, member
      arrays
- [x] `$CONSOLE` (a program built with a console and a window), `$SCREENHIDE`, `$SCREENSHOW`; `_DEST`, `_SOURCE`,
      `_CONSOLE`, `_ECHO`
- [x] The rest of the same step, in the wider scope of `DECISIONS.md` 2026-10-12: the window built-ins of the
      upstream `glut` tests as compiled, never-run code; the constants of the auto-included file (`_TRUE`,
      `_FALSE`); the precompiler flags in `$IF`
- [x] The step's closing runs (the two full suites with the new compiler), its documents and CI after the push
      (group 7 of `m2-console-screenhide`)

## M3. Code generation to the existing ABI: under way

Done when: the compiler emits the `qbx.cpp` fragments, links with libqb and passes the expected-output and
differential tests; programs without `$CONSOLE:ONLY` are checked by their screen state.

- [x] The slice links with libqb and runs (since M2's first slice)
- [x] A program with a window is built (`$CONSOLE` without `:ONLY`, since M2's last step)
- [x] A program with a window as its only output (no `$CONSOLE` at all); the text-screen statements (`SCREEN`,
      `CLS`, `COLOR`, `LOCATE`, `WIDTH`, `VIEW PRINT`), the drawing statements (`PSET`, `PRESET`, `LINE`,
      `CIRCLE`, `PAINT`, `DRAW`, `VIEW`, `WINDOW`, `PALETTE`, `PCOPY`), pacing (`_LIMIT`, `_DELAY`, `SLEEP`,
      `_DISPLAY`) and the functions `INKEY$`, `CSRLIN`, `POS`, `POINT`, `SCREEN`, with screen programs that pass
      on `tests\corpus\slice.list` (`m3-window-output`)
- [x] Graphics `GET` and `PUT`, `DEF SEG` with `BLOAD`, `BSAVE`, `PEEK`, `POKE`, `VARPTR` and `VARSEG` on arrays
      in conventional memory, `TAB` and `SPC`, labels named like a SUB, `_RGB32`, `_RGBA32`, `_FULLSCREEN`,
      `DATE$`, `TIME$`: what Civil War Strategy needs to build apart from sound (`m3-get-put-bload`)
- [x] Data files beside a corpus program (`<name>.files\`); five more programs cut from the game, three of them
      loading its pictures with `BLOAD`
- [x] Sound (`SOUND`, `PLAY` as statement and function, `BEEP`) with the audio part of libqb, checked without
      noise by what a program can observe (the queue, the waits, the errors); `$EXEICON`; a built-in statement
      with a template after `CALL` (`m3-sound`)
- [x] Civil War Strategy compiles with no diagnostic and builds (`m3-sound`; `tests\programs\README.md`, stage 2)
- [x] Civil War Strategy's screens match in a scripted game (`tests\programs\README.md`, stage 3): a copy with
      listed edits plays two turns from a keys file, 23 states compared; the screen-state check's keys file
      gained `prompt` and `state` (`m3-scripted-game`)
- [ ] The image built-ins that run (`_LOADIMAGE`, `_PUTIMAGE`, …; `_RGB32` and `_RGBA32` are done), fonts, the
      mouse
- [x] The console programs of the corpus that waited: `PRINT USING`, `GET` and `PUT` on a random or binary
      file, `FIELD`, `LSET`, `RSET`, `LOCK`, `UNLOCK`, `EXIT SELECT`, `_IIF`, `_MIN`, `_MAX`, `_CLAMP`, `_SHL`,
      `_SHR`, `_ROL`, `_ROR`, and `CHAIN` and `LPRINT` compiled and never run (`m3-console-files`). No program
      of the corpus that the old compiler accepts is held back any more
- [ ] Plain copy of QB64pe's `internal\c` at the reference clone's commit, when libqb must change

## M4. Parity

Done when: the corpus and the upstream tests in reach match, and `qb64pe.bas` compiles and the result passes the
suite (`study\20`).

- [ ] The corpus and the upstream tests outside `tests\upstream\deferred.list`
- [ ] `qb64pe.bas`, reached through its own include files, smallest first (`study\22` §4.2)

## M5. Debugger

- [ ] Debug symbol file and a DAP adapter

## M6. Runtime modernisation

- [ ] The ABI owned by the new compiler; error model, string heap and threading redesigned behind golden tests

## Not tied to a milestone yet

- A formatter that matches `-y` (left M2's exit criterion on 2026-10-09, `DECISIONS.md`; the extension keeps
  formatting through the old compiler)
- Publishing the extension as a `.vsix`
- Deferred features and ideas: `SOMEDAY.md`
