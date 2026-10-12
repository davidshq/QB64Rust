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

## M2. Front end: in progress

Done when: a lossless parser with recovery reads the whole language with no false syntax error on any program the
old compiler accepts (corpus, upstream tests, `qbasic_testcases`, its own sources), with resolution, a type
checker and a thin language server.

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
- [ ] `$CONSOLE` with `_DEST _CONSOLE`, and `$SCREENHIDE`

## M3. Code generation to the existing ABI: started early

Done when: the compiler emits the `qbx.cpp` fragments, links with libqb and passes the expected-output and
differential tests; programs without `$CONSOLE:ONLY` are checked by their screen state.

- [x] The slice links with libqb and runs (since M2's first slice)
- [ ] Plain copy of QB64pe's `internal\c` at the reference clone's commit
- [ ] Programs with a window: the drawing statements, each coming in with screen programs that pass
- [ ] Civil War Strategy builds and its screens match (`tests\programs\README.md`)
- [ ] Data files beside a corpus program (`BLOAD` of the game's pictures)

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
