# Roadmap

Milestones from start to finish, what is done, and the significant work remaining. Keep completed entries and
add newly discovered work; mark existing items complete as they land. The remaining items are broad areas, not
a fixed order or an exhaustive list of changes. Keep them current as the gaps become clearer.

No progress numbers and no "now" marker: the current step, immediate next steps, open questions and the numbers
of the last full runs are in `STATUS.md` (`DECISIONS.md` 2026-10-07). The plan behind the milestones is
`study\00-synthesis.md` §2.

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
- [x] The image built-ins that run (`_LOADIMAGE`, `_PUTIMAGE`, `_MAPTRIANGLE`, the colour functions, …), fonts,
      the mouse fed by mouse steps of a keys file, 12 screen programs; `_SCREENIMAGE`, `_CLIPBOARDIMAGE`,
      `_PRINTIMAGE`, `_DEPTHBUFFER` and `_MOUSEMOVE` compiled and never run (`m3-images-fonts-mouse`)
- [x] `_MEM`: the type, the blocks of references, addresses, new memory and images, `_MEMGET`, `_MEMPUT`,
      `_MEMFILL`, `_MEMCOPY`, `_MEMFREE`, `_MEMEXISTS`, `_OFFSET`, every runtime error 300 to 313; `MKI$` …
      `_CV`; `_MEMSOUND` compiled and never run (`m3-mem`). Upstream 153 to 174 of 279
- [x] `$ASSERTS`, `$ASSERTS:CONSOLE`, `_ASSERT`; `$DEBUG` without the IDE's debugger (`DIVERGENCES.md` D-017);
      `_ADLER32`, `_CRC32`, `_DEFLATE$`, `_INFLATE$`, `_BASE64ENCODE$`, `_BASE64DECODE$`; `END n` and `SYSTEM n`;
      `-f:GenerateLicenseFile` (`m3-asserts-debug-zlib`). Upstream 174 to 186 of 279
- [x] `REDIM _RETAIN`, the elements kept by their indexes, for every kind of element and array; a `REDIM` with a
      FUNCTION in a bound that redims the same array no longer crashes (`DIVERGENCES.md` D-018)
      (`m3-redim-retain`). Upstream 186 to 193, now of 286 (the seven left `deferred.list`)
- [x] `$UNSTABLE:TYPEFIELDS`: the `_DYNAMIC`/`_STATIC` member markers; `_DYNAMIC` member arrays of variables,
      elements and nested values, their copies, `_MEM`, and whole arrays of their types in files; `DIVERGENCES.md`
      D-019 to D-021 (`m3-typefields`). Upstream 193 of 286 to 290 of 383 (the programs that needed it left
      `deferred.list`; the 21 left there need whole-array assignment or `_ARRAYCOPY`)
- [x] Whole-array assignment `a() = b()`: arrays and member arrays (plain and `_DYNAMIC`) of every element type and
      place, arrays of owner types, with the old compiler's compile-time and run-time checks; a type suffix on both
      names stays marked; `DIVERGENCES.md` D-004 widened and D-022 (`m3-whole-array-assign`). Upstream 290 of 383 to
      310 of 399 (16 programs left `deferred.list`; the 5 left need `_ARRAYCOPY` or, `t001`, a plain scalar beside
      an array typed by `DIM … AS`)
- [x] `DECLARE LIBRARY` in its four forms on Windows (plain, `STATIC`, `DYNAMIC`, `CUSTOMTYPE`): the old
      compiler's lookup, `nm` linkage, C++, calls and start-up errors 259 and 260; the forms whose C++ fails with
      the old compiler are rejected before the C++ (`m3-declare-library`; D-023 and D-024 proposed and not
      built). Upstream 310 to 318 of 399; CI's upstream list runs from the clone
- [x] The answers of 2026-10-16: what owner types copy after an error, QB64pe's way where no memory is misused
      (`m3-measured-copies`; D-019, D-021, D-022 narrowed); a run-time trace of the divergences that the runner
      checks (`m3-divergence-trace`); the old compiler's wording as a note after an error (`m3-old-wording-notes`)
- [x] Sound handles: `_SNDOPEN`, the `_SND*` family, `_MIDISOUNDBANK`, `_WAVE` (every name of libqb's audio part
      has a row), checked without a sound heard; `_MEMSOUND` run (`m3-sound-handles`). Upstream 318 to 325 of 399
- [x] `$VERSIONINFO`: the version resource and manifest byte for byte, beside `$EXEICON`'s icon; a compile error
      where the old compiler's build fails or its executable cannot start (`m3-versioninfo`). Upstream 325 to 328
      of 399
- [x] `_ARRAYCOPY` in its six forms, for arrays and member arrays (plain and `_DYNAMIC`) of numbers, `_OFFSET` and
      strings, with the old compiler's checks, order and libqb call; `DIVERGENCES.md` D-004 widened and D-025
      (`m3-arraycopy`). Upstream 328 to 332 of 399 (`deferred.list` holds `t001` alone)
- [x] The console programs of the corpus that waited: `PRINT USING`, `GET` and `PUT` on a random or binary
      file, `FIELD`, `LSET`, `RSET`, `LOCK`, `UNLOCK`, `EXIT SELECT`, `_IIF`, `_MIN`, `_MAX`, `_CLAMP`, `_SHL`,
      `_SHR`, `_ROL`, `_ROR`, and `CHAIN` and `LPRINT` compiled and never run (`m3-console-files`). No program
      of the corpus that the old compiler accepts is held back any more

Significant work remaining, from the unsupported report at the close of `m3-arraycopy` and the recorded gaps
in `SOMEDAY.md`. M3 builds the missing capabilities; M4 checks and closes parity across the full inputs.
These areas may take several changes each. Deliberately marked behaviours still need their recorded decisions
or measurements; listing an area does not settle those choices.

- [x] Event handlers and switches (`ON TIMER`, `ON KEY`, `TIMER ON`, `KEY(n) OFF`, and the related event forms),
      including dispatch through the existing runtime: handlers with a label or a SUB, numbered timers and
      `_FREETIMER`, `KEY ON`/`OFF`/`LIST` and `KEY n, text$`; `PEN`, `COM`, `UEVENT` real errors (`m3-events`).
      Upstream 332 to 336 of 399. `STRIG` events go with device input, below
- [x] Networking: clients, hosts, connection addresses and status (`_OPENCLIENT`, `_OPENHOST`,
      `_OPENCONNECTION`, `_CONNECTED`, `_CONNECTIONADDRESS$`, `_STATUSCODE`); the `_SOCKETS_` flag, which `$DEBUG`
      sets too, and QB64pe's `afterlastline.bm` compiled from a copy of the file (`_ENCODEURL$`, `_DECODEURL$`
      and libqb's helper) (`m3-network-embed`, with the next item). Upstream 339 to 347 of 399
- [x] Constant evaluation: `_FLOAT` arithmetic to the bit, the evaluator's three lanes and its typing by text,
      every function of its list (the math functions certified, else correctly rounded, D-026), suffixed constant
      references, the remaining constants of QB64pe's auto-included file, `$COLOR:0` and `$COLOR:32`
      (`m3-constant-evaluation`; `CHR$` with `m3-errorline-chr-const`). Upstream 347 to 356 of 399
- [x] Embedded resources (`$EMBED`, `_EMBEDDED$`), including their use for MIDI sound banks
      (`m3-network-embed`; each file stored as it is)
- [ ] The `$EXEICON` and `$EMBED` names relative to the old compiler's own folder (`icon/test`,
      `winresource/test`, `embed/test`; `SOMEDAY.md`)
- [ ] Runtime error and source-location support: `ON ERROR GOTO _NEWHANDLER` and `_LASTHANDLER`; logging
      built-ins (done: `_ERRORLINE`, `_INCLERRORLINE`, `_INCLERRORFILE$`, `_ERRORMESSAGE$`, and a joined line
      numbered as its last, `m3-errorline-chr-const`; upstream 336 to 339 of 399)
- [ ] `$CHECKING:OFF` and the corresponding unchecked code generation, including memory operations and
      `_ARRAYCOPY` (read in `afterlastline.bm` only since `m3-network-embed`: no event checks there)
- [ ] OpenGL user code (`SUB _GL`) and its integration with the existing runtime
- [ ] Remaining built-in families and statements, including device input (`_DEVICES` and related functions,
      `STICK`, `STRIG()`, and the `STRIG` events `ON STRIG` and `STRIG ON`, user 2026-10-17),
      clipboard text, `_FILES$`, `_MD5$`, `_OS$`, `_CAST`, `_CONTINUE` and `CLEAR`; cover their forms through
      the call-site check and runnable tests where applicable
- [ ] Remaining name, declaration and array rules: a scalar beside an array typed by `DIM ... AS`
      (`arrays/t001_whole_array_core`, the last deferred upstream program), implicit arrays and forward
      procedure references, `REDIM` without bounds in procedures, and the remaining member-array forms
- [ ] Remaining memory and conventional-memory references: string-bearing types, nested and dynamic member
      arrays, `LEN` of a whole member array, and the `VARPTR`/`VARSEG` forms deferred until a program needs them
      (`SOMEDAY.md` records the individual forms and constraints)
- [ ] Plain copy of QB64pe's `internal\c` at the reference clone's commit, when libqb must change

## M4. Parity

Done when: the corpus and the upstream tests in reach match, and `qb64pe.bas` compiles and the result passes the
suite (`study\20`).

- [ ] The corpus and the upstream tests outside `tests\upstream\deferred.list`
- [ ] Close the remaining compatibility gaps exposed by the full suites, including accepted programs still
      marked "not supported yet" and rejected programs receiving only that mark; keep false errors and parse
      gaps at zero (`tests\known_*.list`)
- [ ] Bring the last deferred upstream program into the parity run when its scalar/array naming rule is built;
      finish with no feature-deferred programs left (`tests\upstream\deferred.list`)
- [ ] `qb64pe.bas`, reached through its own include files, smallest first (`study\22` §4.2)
- [ ] Build and run the compiler produced from `qb64pe.bas` against the suite (`study\20`); compiling its source
      alone does not meet M4's exit criterion
- [ ] Review the coverage of features checked only by emitted calls or compiled-but-never-run programs; add
      execution checks where needed to establish parity, using screen state for window output and retaining
      documented limits for hardware-dependent behaviour

## M5. Debugger

- [ ] Debug symbol file and a DAP adapter

## M6. Runtime modernisation

- [ ] The ABI owned by the new compiler; error model, string heap and threading redesigned behind golden tests.
  This starts as a second backend crate beside `codegen-cpp`, which stays pinned to libqb's ABI and to the
  call-site check; `codegen-cpp` is not modernised in place (`study\30` §7, user 2026-10-16)

## Not tied to a milestone yet

- A formatter that matches `-y` (left M2's exit criterion on 2026-10-09, `DECISIONS.md`; the extension keeps
  formatting through the old compiler)
- Publishing the extension as a `.vsix`
- Deferred features and ideas: `SOMEDAY.md`
