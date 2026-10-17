# Someday

Features and ideas deliberately left out of the first versions of the rewrite. Each entry says why it was deferred
and where the details are. Nothing here is rejected; it is just not on the roadmap yet.

## Deferred QB64pe features

These are recent upstream QB64pe additions (not the user's own work). They add a lot of complexity to arrays, so they
wait until the core compiler works (roadmap M4 or later, `study\07-expert-panel.md`).

None is left. Whole-array assignment `a() = b()` came with `m3-whole-array-assign` (2026-10-15), and `_ARRAYCOPY`
(Petr, 2026-09-12; `study\01` §8.2, `study\03` §2.3), from the same work, with `m3-arraycopy` (2026-10-17).

Features from the same period that are **kept** in scope: `$USELIBRARY`, `$ERRORLOCATION`, the GLFW runtime.

**Plain member arrays compile** (`m2-defxxx-common-arrays` group 9, 2026-10-11: `v(0 TO 3) AS LONG` in a `TYPE`,
elements through any chain, `LBOUND`/`UBOUND`, `REDIM` and `ERASE` as a clear, a whole member array as an array
argument). What of that layer the old compiler accepts and the new one still marks "not supported yet":

| Form | Why it waits | Details |
|---|---|---|
| A bound of a member array that names a `CONST` (`v(top) AS LONG`, `w(top * 2 - 1) AS LONG`) | The `TYPE` blocks are read before the program's `CONST`s (a type may be used above its block), so a bound must be written with numbers alone. It needs the constants of the main module known in the first pass | `verification\v24_x196`; design D8 of the change, "As built in group 9" |
| A member array of an array element beside other targets of one `ERASE` or `REDIM` (`ERASE a(1).v, t.v`) | The old compiler goes on with the other targets before the error of an index is handled; each clear here is a statement of its own. It needs several operations in one statement of the typed tree, as the allocations of one `DIM` have | `verification\v24_x312` lines 9 and 10 |
| `REDIM t.v(0) AS type` under `OPTION BASE 1` | Not measured: whether the number 0 keeps the rule it has in a `DIM` | `tests\frontend\member_arrays_errors.bas` |

(An element of a member array, and a member of an array element, as a target of `INPUT`, `READ` or the `MID$`
statement compiles since `m3-typefields` task 4.2, measured by `verification\v35_x62`.)

**Left marked by `m2-console-screenhide`** (step 11, 2026-10-12: a console and a window, the window built-ins,
the constants of the auto-included file, the precompiler flags). What the old compiler accepts and the new one
still marks "not supported yet":

| Form | Why it waits | Details |
|---|---|---|
| The window built-ins are compiled and never run (`_TITLE`, `_ICON`, `_MOUSESHOW`, `_SCREENX`, `_NEWIMAGE`, … 20 of them) | What they do shows only in a window; M3 gives programs with a window a check (`tools\screen`) | `study\support-record.md` ("Window built-ins, compiled and never run"); `sema\src\builtins.rs`, `WINDOW_BUILTINS`; call-site programs in `tests\callsite` |
| `SWAP`, `CLOSE` and the `MID$` statement after `CALL` (`CALL SWAP(a, b)`) | Each has a rule of its own, which reads the statement without `CALL` only. Until the review of groups 4 to 6 `CALL SWAP(a, b)` was a false error ("`SWAP` takes 0 arguments") | `verification\v26_x128`; `tests\frontend\swap_mid_marked.bas` |
| The function `_SCREENHIDE` | Not in the change's scope (no upstream test names it) | `verification\v26_x74` |
| `_SCREENEXISTS` as the direct argument of `HEX$`, `OCT$` or `_BIN$` | The table gives the function no return type, and the old compiler passes the width 0 there, not a LONG's 8 or 32; the function runs from M3 on, where the width can be given | Design D6 of the change |
| 10 of the 310 constants of `beforefirstline.bi`: `_E` and the nine of the next row | `_E`'s `CONST` line needs `EXP`, which the `CONST` evaluator does not compute yet (of its functions only `CHR$` is, with the 69 `_CHR_…`/`_STR_…` strings, since `m3-errorline-chr-const`) | `sema\src\check\auto.rs`; `tests\frontend\auto_const_errors.bas` |
| The other nine: `_DOUBLE_MIN`, `_DOUBLE_MAX`, `_FLOAT_MIN`, `_FLOAT_MAX`, `_FLOAT_MIN_FRAC`, `_INTEGER64_MIN`, `_UINTEGER64_MAX`, `_OFFSET_MIN`, `_UOFFSET_MAX` | Numbers the `CONST` evaluator does not read yet (a value at a rounding boundary, beyond `DOUBLE` range, or beyond `_INTEGER64`). The old compiler's values for them are recorded and odd: see "QB64pe behaviours to review" | `verification\v26_x114` |
| The `$COLOR` constants | The files are not included (`afterlastline.bm` is compiled since `m3-network-embed`) | `tools\builtins\builtins.json`, `auto_include` |
| A `$IF` that names `_CONSOLE_` between `$CONSOLE:ONLY` and a later `$CONSOLE` (or the other way round) | The old compiler's two passes read the line with different values (its program prints nothing from the branches; a `CONST` in them is "Name already in use"); no value here would be right | `verification\v26_x27`, `x34`; `tests\frontend\flags_between_errors.bas` |

**Left marked by `m3-window-output`** (the first step of M3, 2026-10-12: a program with a window as its
output, the text-screen, drawing and pacing statements, the screen functions). What the old compiler accepts and
the new one still marks "not supported yet", and what is compiled but checked by its call alone:

| Form | Why it waits | Details |
|---|---|---|
| An argument left out of a function call (`POINT(, 2)`, `SCREEN(1, , 1)`) | The old compiler rejects both ("Incorrect number of arguments"); here they get the general mark "omitted arguments", which is not an error of the program | `verification\v27_x50`; `sema\src\check\builtins.rs` |
| A jump to a built-in statement's name (`GOTO CLS` below a line `CLS: x = 1`), and a label named like a built-in that is not compiled yet (`FILES: x = 1`) | The first is an error in the old compiler ("Invalid label!": `CLS:` is the statement, which is compiled so here); here it keeps the general mark for a label named like a built-in. The second becomes the statement when the built-in comes in | `verification\v27_x58`, `x63`; `tests\frontend\stmt_name_colon_errors.bas` |
| `PALETTE USING array(index)` | Not in the change's scope: the old compiler writes it by hand (`qb64pe.bas` 7238), outside `PALETTE`'s template. Until the review of group 4 the parser gave it a syntax error ("matches none of its forms"), a false error for a line the old compiler accepts | `verification\v27_x64`; `tests\frontend\palette_using_marked.bas` |
| `WIDTH (80) = 80`: an element of an array named `WIDTH` | `WIDTH` is the one statement name that is free as a variable's (`WIDTH = 80` is compiled); an array of that name used without `DIM` is marked. Every other statement name written `name (…) =` is "name already in use", as in the old compiler | Measured 2026-10-12 (`tests\frontend\stmt_name_assigned.bas`) |
| `WIDTH #n, columns`, `WIDTH LPRINT columns` and `SCREEN …, _MANUALDISPLAY n` are compiled, and no test runs them | Each is the plain call of its template; what it does is no part of a screen's state (a file's line width, the printer, when the window is updated) | Call-site programs `width_file*`, `width_lprint*`, `screen_manualdisplay*` |
| `_LIMIT`, `_DELAY` and `SLEEP` are run by one screen program with small values; how long they wait is not checked | Nothing a test records may depend on time | `tests\corpus\screen\d15_pacing` |
| `SLEEP` with no time in a screen program | The keys file cannot end it once a key is in the buffer (it gives the next key only to an empty buffer, and a key ends `SLEEP` only while it waits). A `repeat` step that presses whatever the buffer holds would do it; that is a change to `tools\screen\screen_state.cpp` | `tools\screen\README.md`, "Writing a screen program" |

**Left marked by `m3-get-put-bload`** (the second step of M3, 2026-10-13: labels with the name of a SUB, `TAB` and
`SPC`, conventional memory, graphics `GET` and `PUT`). What the old compiler accepts and the new one still marks
"not supported yet", and what is compiled but checked by its call alone:

| Form | Why it waits | Details |
|---|---|---|
| `VARPTR` and `VARSEG` of a scalar, a string or a fixed-length string (`VARPTR(i)`, `VARSEG(d)`, `VARPTR(t$)`) | The variable would have to live in conventional memory, allocated another way: a number from `dblock+cmem_sp` (`VARPTR` is `((unsigned short)(((uint8*)V)-&cmem[1280]))`, `VARSEG` 80), a string by `qbs_new_cmem` (`VARPTR` its `cmem_descriptor_offset`), a fixed-length one by `qbs_new_fixed` over `dblock` (`V->chr-&cmem[1280]`). Decided 2026-10-13: arrays only until a program needs one | `study\00` §5 (`v28`); `tests\frontend\m3_step2_marked.bas` |
| `VARPTR` and `VARSEG` of an array of a SUB or FUNCTION (`DIM` or `STATIC` there) and of an array parameter | A local array starts with flag 4 and is allocated per call from conventional memory; a parameter named so makes the callers' arguments conventional (`qb64pe.bas` 12081, 12810), a fact carried through calls. The text is the same as for an array of the main module | Same |
| `VARPTR` and `VARSEG` of an array of a `TYPE`, of strings, of fixed-length strings or of `_BIT`, and of a member (`VARPTR(r(2).y)`, `VARPTR(one.y)`) | Other texts: a `TYPE` element `((index)*size+offset)`, a string element its `cmem_descriptor_offset`, a member of a variable `((unsigned short)(((uint8*)V) - &cmem[1280] + offset))` | Same |
| `SADD(t$)` and `VARPTR$(v)` | `SADD` is `((unsigned short)(T->chr-&cmem[1280]))` of a string in conventional memory; `VARPTR$` is `func_varptr_helper(type, offset)` | Same; `verification\v28_x55`–`x57` |
| `VARPTR` or `VARSEG` as the direct argument of `HEX$`, `OCT$` or `_BIN$` | The old compiler passes the width 0 there (`func_hex(…,0)`), not an `_INTEGER64`'s | `verification\v28_x95` |
| In the array's place of graphics `GET` and `PUT`: an array of a `TYPE`, of strings or fixed-length strings, of `_BIT`; a name that is no array (the bytes of that plain variable, also of an undeclared name); a member | Each is another text of the `byte_element` argument (`((char*)A[0])+…` for a `TYPE`, `(uint64)V,size` for a variable, `T->chr,T->len` for a string). No program in hand has one | `verification\v28_x12`, `x13`, `x20`, `x23`, `x59` |
| An index that calls a FUNCTION in the array of `GET` or `PUT` (`GET (1, 2)-(3, 4), a(f(1))`) | The old compiler writes the index twice, so the FUNCTION runs twice; writing that the same way needs a second evaluation the emitter does not have | Read from the C++ (`study\00` §5) |
| The file forms `GET #n, …` and `PUT #n, …` | Step 4 of M3's order (random and binary files) | The mark is ` `GET` on a file ` |
| `TAB(5) = 1` | An error in the old compiler ("Expected variable ="); here the general mark for a built-in function's name as a place | `verification\v28_x53` |
| A line that is the name of a SUB and a colon, where a line the parser could not read has the word in it | Whether the line is a call or a label depends on every jump of the program, and that line may be one | `tests\frontend\sub_name_colon_errors.bas` (`five:`) |
| `_FULLSCREEN` (statement and function) is compiled, and no test runs it | A test that takes the whole desktop is not wanted | Call-site programs `fullscreen_*` |
| `DATE$` and `TIME$` are run for their length and separators only; the statements `DATE$ = …` and `TIME$ = …` are not compiled | Nothing a test records may depend on the clock; setting the clock is no test's business | `tests\corpus\slice\s71_tab_spc` |

**Left marked by `m3-sound`** (the third step of M3, 2026-10-13: `SOUND`, `PLAY`, `BEEP`, `$EXEICON`, and a
statement with a template after `CALL`, which closed three rows of the tables above). What the old compiler
accepts and the new one still marks "not supported yet", and what is compiled but not checked by a run:

| Form | Why it waits | Details |
|---|---|---|
| A relative `$EXEICON` name that does not start with `./` or `.\` (`$EXEICON:'icons/game.ico'`), one with a single leading slash, and one with a drive and no slash after it (`C:game.ico`) | The old compiler looks for the first in its own folder, for the second on its own drive and for the third in the folder it stands in on that drive (read in `qb64pe.bas` 3854–3873, not measured), which has no counterpart here (user, `DECISIONS.md` 2026-10-13). Also under "QB64pe behaviours to review" below | `verification\v29_e02`, `e03`, `e08`, `e18`; the upstream tests `icon` and `winresource` name their icon so |
| The language server does not check that the icon of `$EXEICON` is there | The check is the driver's (`check_exe_icon`), because `sema` reads no file; the server would need the folder of the main file behind its loader | Design D4 of `m3-sound` |
| `ON PLAY(n) GOSUB 10` and any `ON … GOSUB` or `ON … GOTO` whose target is a line number | Line numbers are not compiled yet, whatever the number of the `ON` is; with a label the statement compiles | `verification\v29_x64`; `tests\frontend\sound_typed.bas` |
| `VARPTR$` in a `PLAY` string (`PLAY "X" + VARPTR$(a$)`) | `VARPTR$` is not compiled (the table above) | Not measured |
| `BEEP` is compiled, and no test runs it | It rings the system's bell, which no volume silences (user, `DECISIONS.md` 2026-10-13). Its call is compared by `tests\callsite\beep_*.bas`, and `s74_sound_queue` holds it as code that is not reached | `NEVER_RUN` in `sema\src\builtins.rs` |
| What is heard is checked by no test | Both compilers' programs call the same libqb with the same arguments, which the call-site check compares form by form; the samples would test libqb. The slice programs check what a program can observe: the queue, the waits, the errors | Design D6 of `m3-sound` |
| `SOUND 440, 65535`, the longest accepted duration, is in no slice program | libqb fills a buffer of an hour of samples for it, about 700 MB | `verification\v29_c_sound_errors` line 19 |
| `s75_sound_wait` and the second part of `s76_sound_errors` need a machine with a sound device | Without one libqb queues nothing, holds nothing and checks neither a volume, a pan position, a waveform nor a `PLAY` string (read in `audio.cpp`). Not seen on GitHub's Windows runner yet (CI is looked at when asked) | Design, Risks, of `m3-sound` |
| `CALL SWAP(a, b)`, `CALL CLOSE(1)` and the `MID$` statement after `CALL` | Each has a rule of its own, which reads the statement without `CALL` only (the row of `m2-console-screenhide` above) | `verification\v26_x128` |

**Left marked by `m3-console-files`** (the fourth step of M3, 2026-10-13: `PRINT USING`, `GET` and `PUT` on a
file, `FIELD`, `LSET`, `RSET`, `LOCK`, `UNLOCK`, `EXIT SELECT`, `_IIF`, `_MIN`, `_MAX`, `_CLAMP`, the shifts and
rotations, `CHAIN`, `LPRINT`). What the old compiler accepts and the new one does not take, and what is
compiled but not checked by a run:

| Form | Why it waits | Details |
|---|---|---|
| A whole array `e()` as the string of `FIELD`, `LSET` or `RSET` | The old compiler takes it as the array's first element with no bounds check (of an array that is not allocated: whatever lies there). "Not supported yet"; also under "QB64pe behaviours to review" | `verification\v30_x109`, `x153` |
| A whole array `a()` as a value of `PRINT USING` or as a part of `_IIF` | An error here, as a whole array is in every expression; the old compiler takes it as element 0 | `verification\v30_x18`, `x99` |
| `GET #1, , und()` and `PUT` with an array that is not declared | The old compiler accepts it, also `und$()`, although a declared array of variable-length strings is rejected; "not supported yet" | `verification\v30_x176`–`x178`; `tests\frontend\file_get_put_ir.bas` has the declared forms |
| `GET #1, , a` where only an array `a` declared with `DIM … AS` exists | The old compiler makes a new SINGLE scalar `a` (`verification\v30_x67`); a plain scalar beside an array typed by `DIM … AS` is marked everywhere (the row of `m2-defxxx-common-arrays`) | `verification\v30_x67` |
| An `_UNSIGNED _INTEGER64` literal above the `_INTEGER64` range (`18446744073709551615`) | "Not supported yet: unsigned literals", older than this step; the slice programs make the value by `u = 0: u = u - 1` | `tests\corpus\slice\s78_choice_min_shifts.bas` |
| `LPRINT` and `CHAIN` are compiled, and no test runs them | `LPRINT` prints a page on the default printer; `CHAIN` starts `qb64pe.exe -c` beside the program through `cmd` (user, `DECISIONS.md` 2026-10-13). `238_chain` on `slice.list` reaches `sub_chain` with a name that is not there; no test chains to a program that exists (gap noted by the user, `DECISIONS.md` 2026-10-16). Their calls are compared by `tests\callsite\lprint_*.bas` and `chain_*.bas`; `s82_print_using_file` and `s87_chain_not_reached` hold them as code that is not reached | `NEVER_RUN` in `sema\src\builtins.rs` |
| What a format of `PRINT USING` prints, and what a file holds after `GET` and `PUT`, are shown once per topic | Both compilers' programs call the same libqb with the same arguments, which the call-site check compares form by form; more lines would test libqb | Design, Non-Goals |

**Left marked by `m3-images-fonts-mouse`** (the sixth step of M3, 2026-10-14: images, colours, fonts, the mouse).
What the old compiler accepts and the new one still marks "not supported yet":

| Form | Why it waits | Details |
|---|---|---|
| `_UCHARPOS(t$, nd&())` with an array that is not declared | The array of `_UCHARPOS` is read as the array of a graphics `GET` is, which marks a name that is no declared array (the row of `m3-get-put-bload` above) | `verification\v31_x01_accepted` (last lines) |

**Left marked by `m3-mem`** (the seventh step of M3, 2026-10-14: `_MEM`, its functions and statements, `_OFFSET`,
`MKI$` … `_CV`). What the old compiler accepts and the new one still marks "not supported yet", each not measured:

| Form | Why it waits | Details |
|---|---|---|
| `_MEM`, `_MEMELEMENT`, `_MEMGET`, `_MEMPUT` and `_MEMFILL` of a value of a `TYPE` that holds a `STRING` | Such a value holds the pointer of its string; what the old compiler's helpers give for it was not measured | `sema\src\check\mem.rs`, `block_reference`, `bytes_reference` |
| A reference through an element of a member array that is not its last step (`x(0).n(1).q`), or to a member array of user-type elements | The old compiler's rules for such paths were not measured | `unmeasured_path` |
| `_MEM(nx())` of a name that is no array; `_OFFSET` of an element, member or array of variable-length strings; `_MEMGET`, `_MEMPUT` and `_MEMFILL` with an element or member of them | Not measured (what `_MEM` and `_MEMPUT` do with such strings and arrays of them is: errors, `verification\v32_x15`, `x16`, `x39`) | `check\mem.rs` |
| `LEN(p(0).a())`, `LEN` of a whole member array | The old compiler takes it (`qb64pe.bas` 20652); its size was not measured. Holds `arrays/test_mem_vs_len` | `check\builtins.rs`, `len` |
| `$CHECKING:OFF` and the forms the old compiler writes without checks (`sub__memfill_nochecks`, the inline checks left out) | Decided with the proposal: a step of its own (`DECISIONS.md` 2026-10-14) | design of `m3-mem`, Non-Goals |

(`_INFLATE$`, `_DEFLATE$`, `_BASE64ENCODE$` and `_BASE64DECODE$`, the last row here, came in with
`m3-asserts-debug-zlib`.)

**Left marked by `m3-asserts-debug-zlib`** (the eighth step of M3, 2026-10-14: `$ASSERTS`, `_ASSERT`, `$DEBUG`,
the data-processing functions, `END n` and `SYSTEM n`, the licence file):

| Form | Why it waits | Details |
|---|---|---|
| A built-in that needs `$` named without it (`_DEFLATE("x")`, `_TRIM(" a")`): "not supported yet", where the old compiler says "Invalid array name" (`verification\v33_x25`–`x29`) | The same words name an array used without `DIM` or a FUNCTION not yet seen; settled with them (user, task 3.1 of the change) | `sema\src\check\expr.rs`, the call of an unknown name |

**Left marked by `m3-typefields`** (the tenth step of M3, 2026-10-15: `$UNSTABLE:TYPEFIELDS`, the member markers,
`_DYNAMIC` member arrays). What the old compiler accepts and the new one still marks "not supported yet", each not
measured:

| Form | Why it waits | Details |
|---|---|---|
| `_OFFSET(x.v())` of a whole `_DYNAMIC` member array | Not measured (its `_MEM` is, `verification\v35_x66`) | `sema\src\check\mem.rs`, `unmeasured_path` |
| `_MEMGET`, `_MEMPUT` and `_MEMFILL` with a `_DYNAMIC` member array or an element of one as the variable | Outside the change (its design, Non-Goals) | `check\mem.rs`, `bytes_reference` |

**Left marked by `m3-whole-array-assign`** (2026-10-15: `a() = b()` with arrays and member arrays of every kind).
What the old compiler accepts and the new one still marks "not supported yet":

| Form | Why it waits | Details |
|---|---|---|
| A whole-array assignment with a type suffix on both names (`b%() = a%()`, `a&() = b%()`, `a$() = b$()`, also for arrays declared `AS INTEGER`) | The old compiler does not take the whole-array path for a suffixed name: it copies element 0 into element 0, converting, with no type check (`b%(1)` and `b%(2)` keep their values). Marked rather than copied so (user, 2026-10-15, as for a whole array in `FIELD`); also under "QB64pe behaviours to review" | `verification\v36_x24`, `x38`, `x40`, `x43`, `x44`, `v36_a` 7.–9.; `sema\src\check\whole.rs` |
| A whole array as a value of `PRINT` (`PRINT a()`) | The old compiler prints element 0; a whole array in an expression is marked, as before the change | `verification\v36_x37`; `check\places.rs`, `element` |

**Left marked by `m3-declare-library`** (the twelfth step of M3, 2026-10-16: `DECLARE LIBRARY` in its four forms on
Windows). What the old compiler accepts and the new one still marks "not supported yet", or does otherwise:

| Form | Why it waits | Details |
|---|---|---|
| A library FUNCTION beside a program SUB of the same name (`x22`) | The old compiler accepts both; which one a use names was not measured | `verification\v37_x22`; `sema\src\check\library.rs` |
| `ALIAS` in a `DECLARE SUB` outside a library block (`DECLARE SUB s ALIAS t ()`, `x08`) | The old compiler accepts it and ignores it; this compiler keeps the parser's error it gave before the change ("expected the end of the statement"; design D1, task 2.2) | `verification\v37_x08` |
| `on_error/errtest`, the ninth upstream program with a block | It also needs `ON ERROR GOTO _NEWHANDLER` and `_LASTHANDLER`, which stay "not supported yet" (left out of the change, user, 2026-10-15) | `DECISIONS.md` 2026-10-15 |
| Linux and macOS libraries (`.so`, `.dylib`, `/usr/lib`, `dlopen`) and the IDE's folder in the lookup | The project is Windows only | design Non-Goals |

**Left marked by `m3-sound-handles`** (the sixteenth step of M3, 2026-10-16: `_SNDOPEN`, the `_SND*` family,
`_MIDISOUNDBANK`, `_WAVE`; every name of libqb's audio part has a row now). What the old compiler accepts and the
new one still marks "not supported yet", and what is compiled but not checked by a run:

| Form | Why it waits | Details |
|---|---|---|
| A FUNCTION call in the index of the array of `_WAVE` or `_SNDRAWBATCH` (`_WAVE 0, w(f(1))`), and an array used without `DIM` there | The old compiler writes the index twice (the FUNCTION is called twice); both are marked as for graphics `GET` and `PUT` (`block_arg`) | `verification\v39_x01_accepted` (its C++) |
| `$MIDISOUNDFONT` without its colon (`$MIDISOUNDFONT`, `$MIDISOUNDFONT :x`) | The old compiler's "Syntax error" for any unknown metacommand; every unknown metacommand is "not supported yet" here | `verification\v39_x74`, `x79`–`x81` |
| What is heard, the 3-D position of `_SNDBAL` and `_SNDPLAYCOPY`, `"stream"` and `"nodecode"` beyond what their lengths show | Both compilers' programs call the same libqb with the same arguments, which the call-site check compares form by form | design Non-Goals |
| The sound handle programs on a machine without a sound device | libqb's engine is not started there: every function gives 0 and every statement does nothing, so `s127`–`s129` would fail; not known until the `tier2` job runs them on GitHub | design, risks |

**Left marked by `m3-versioninfo`** (the seventeenth step of M3, 2026-10-16: `$VERSIONINFO`, the executable's
version resource and manifest). Nothing the old compiler builds into a working executable is marked; left out:

| Form | Why it waits | Details |
|---|---|---|
| `winresource/test` (`$VERSIONINFO` with `$EXEICON:'tests/compile_tests/winresource/qb64pe.ico'`) | Held back by its `$EXEICON` name, which neither starts with `./` nor is absolute (the row of `m3-sound` above) | proposal |
| `$VERSIONINFO` on other systems than Windows | The old compiler writes the files on Windows only and ignores the lines elsewhere | design Non-Goals |
| `s130_versioninfo` on a machine without PowerShell | It reads the version resource back with a PowerShell script; every Windows machine and GitHub's runner have one | design, risks |

**Left marked by `m3-arraycopy`** (the eighteenth step of M3, 2026-10-17: `_ARRAYCOPY`). No form of `_ARRAYCOPY` is
marked; left out:

| Form | Why it waits | Details |
|---|---|---|
| `_ARRAYCOPY` under `$CHECKING:OFF` (`verification\v41_x62`, not run with this compiler) | `$CHECKING` is marked "not supported yet" by itself | task 3.4 |

**Left marked by `m3-events`** (the nineteenth step of M3, 2026-10-17: `ON TIMER`, `ON KEY`, the switches,
`_FREETIMER`, `KEY`'s own forms). What the old compiler accepts and the new one still marks "not supported yet":

| Form | Why it waits | Details |
|---|---|---|
| `ON STRIG(…) GOSUB label` or `… sub`, and `STRIG[(…)] ON\|OFF\|STOP` | libqb's device-input part is not built, nor its other names (`STICK`, `STRIG()`, `_DEVICES`, …): a later device-input change (user, `DECISIONS.md` 2026-10-17) | `verification\v42_x11` |
| A line number as a handler (`ON TIMER(1) GOSUB 10`) | Line numbers are not compiled yet (the row of `ON PLAY` above) | `verification\v42_x46` |
| A SUB of a `DECLARE LIBRARY` block as a handler | Not measured | `sema\src\check\events.rs` |

**Left marked by `m3-network-embed`** (2026-10-17: `$EMBED`, `_EMBEDDED$`, the network functions, `_SOCKETS_`,
`afterlastline.bm`). What the old compiler accepts and the new one still marks "not supported yet", and what is
left out:

| Form | Why it waits | Details |
|---|---|---|
| A relative `$EMBED` name that does not start with `./` or `.\` (`$EMBED:'data/f.txt','h'`), so `embed/test` stays held back, beside `icon/test` and `winresource/test` | The old compiler looks for it in its own folder, as for `$EXEICON` (the row of `m3-sound` above, and "QB64pe behaviours to review") | `verification\v44_x26` |
| `_EMBEDDED` without its `$` (`x19`) and `_OPENCLIENT&("x")` (`x46`), "Invalid array name" in the old compiler | Every `_name(...)` that names no array is "not supported yet", as `_DEFLATE("x")` (`v33_x25`–`x29`) | `tests\frontend\network_errors.bas` |
| `_EMBEDDED$` with its argument in parentheses (`_EMBEDDED$(("h"))`) | Not measured | `sema\src\check\embed.rs` |
| Two absolute `$EMBED` names of one file written differently (`<drive>/d/f` and `<drive>\d\f`, or with `..`): "was already embedded" in the old compiler, which resolves the folder first (`CHDIR`, `_CWD$`, `qb64pe.bas` 3765–3777); compared as written here, so both are accepted | Not measured: a recording would hold a local folder (`CLAUDE.md` rule 2); `sema` reads no folder | `sema\src\check\embed.rs` `same_file` |
| `$CHECKING:OFF` and `:ON` in a program's own code | Read in `afterlastline.bm` only, whose procedures have no array, `_MEM` or `ON ERROR`; elsewhere the statements it changes need more than the event check left out | design D8 |
| Compressing an embedded file as the old compiler does when that saves a fifth | The program sees the same bytes; only the executable's size differs (design D4) | design D4 |
| `auto_include/general` | Held back by `_E` (the `CONST` evaluator's `EXP`, the row of `m2-console-screenhide` above), no longer by `_ENCODEURL$` | proposal |
| An HTTP download from a real server (`http/read_example`) | The page it downloads changed; it compiles and stays a known failure | `tools\legacy_tests\known_failures.txt` |

Test impact: some of the 211 `tests\compile_tests\arrays` tests exercise this layer. When the new compiler is first
measured, list them in `tools\legacy_tests\known_failures.txt` (reported as `KFAIL`) until the features land.

## Strict QuickBASIC 4.5 mode

Not planned (QB64pe has no such mode). What it would take: `study\08-qb45-strict-mode.md`.

## QB64pe behaviours to review

The new compiler does what QB64pe does, also where that looks wrong (user, 2026-10-08, `DECISIONS.md`); only crashes,
hangs, memory corruption and failed builds are fixed (`DIVERGENCES.md`). Each questionable behaviour found is listed
here, implemented QB64pe's way, for the user to review later. Changing one is a decision and a `DIVERGENCES.md` row.

| Behaviour (as implemented, as QB64pe) | Possible change | Source |
|---|---|---|
| The result-changing oddities kept on 2026-10-08: INTEGER arithmetic in 32 bits, `RESUME NEXT` in a `WHILE` condition looping forever, the `ELSEIF` placeholder, one `GOSUB` stack, a SUB with a raising argument doing nothing, a raising `CASE` item running its body, a bad index reading element 0, `HEX$` of a non-place 64-bit -1 giving `""`, `STRING$(n, "")`, `CSNG`/`VAL` not narrowed, a DOUBLE `ON n` narrowed to SINGLE, `_ROUND`/`VAL` beyond `_INTEGER64`, the static `SELECT CASE` copy, `CONST` `^` right-associative (with a warning) | Each could follow its definition or QB 4.5 | `study\00` §6 "Kept", `DIVERGENCES-QB45.md` |
| Comma zones on the console: 10 columns on Windows, one space on Linux and macOS | 14 columns everywhere, as on the window, in files and in QB 4.5. To evaluate: how many console programs print with commas, and whether a program that switches `_DEST` between window and console should lay out the same | Bug-compatibility decisions, 2026-10-08 (`study\00` §6, `DIVERGENCES.md` D-011) |
| `_BIT` and `_BIT * n` parameters: a compile error (QB64pe declares the parameter twice and fails its own C++ build, so no program uses them) | Support them. By reference across `_BIT` widths would need a design: QB64pe passes a `_BIT` variable only as a copy | `m2-numeric-types` measurements, 2026-10-08 (`study\00` §5, `verification\v21_x20`–`x24`; `DECISIONS.md`) |
| `STRING * n` reads n as 32 bits: 4294967297 is length 1 | An error for n beyond 2147483647 | Same |
| `VAL(s$, <unsigned type>)` drops the minus sign: `VAL("-1", _UNSIGNED LONG)` is 1 | Wrap, as a store does (4294967295), or an error | `m2-numeric-types` measurements (`verification\v21_d_builtins`) |
| An integer `CASE` item is compared as written, not converted to the selector's type: `CASE -1` misses a `~%` holding 65535 but matches a `~&` holding 4294967295 | Convert the item to the selector's type, as a float item is | Same (`v21_d_select`) |
| `FOR uq~&& = 1 TO 18446744073709551615~&&` runs no pass (the hidden limit is signed 64-bit) | Unsigned hidden copies for unsigned 64-bit variables | Same (`v21_d_for`) |
| A `_BIT * n` of up to 32 bits computes as its `int32`/`uint32` storage: `u3 * 1000000000` with `u3 = 7` is 2705032704, `b3 - u3` is 4294967286 | Compute as the believed `_INTEGER64` | Same (`v21_b_bit_stores`) |
| A `_BIT` value prints as `_INTEGER64`: an `_UNSIGNED _BIT * 64` holding 2^64-1 prints -1 (while the result of an operation on it is believed by its width and signedness: `u64 + 0~&&` prints unsigned) | Print an unsigned 64-bit `_BIT` as `_UNSIGNED _INTEGER64` | Same |
| Comparisons and arithmetic across signedness follow C++: `-1 < u~&` is false (-1 becomes 4294967295) but `-1 < u~%` is true (both become `int`); `` 0~`40 > -1 `` is false (a `_BIT` literal wider than 32 bits is `ull`) | Compare by value, as QB 4.5 would have (it had no unsigned types) | `m2-numeric-types` task 5.3 (differential `ops`, `fold`; `s31_unsigned_ops`) |
| `_ANDALSO`, `_ORELSE` (and `-`, `NOT`) of unsigned 64-bit operands are believed `_UNSIGNED _INTEGER64`, so a true `_ANDALSO` prints 18446744073709551615 and `-v~&&` with `v~&& = 1` prints 18446744073709551615 | Believe `_ANDALSO`/`_ORELSE` LONG, like the comparisons | Same (`ops\andalso`, `unary\unary`) |
| `EQV` and `IMP` complement the left operand in its own width before widening it (`~a^b`): with `u~& = 4294967295` and `q&& = 0`, `u~& EQV q&&` is 0, not the 64-bit complement -4294967296 | Complement after the conversion to the common type | Same (`ops\eqv`, `ops\imp`) |
| An `_OFFSET` operand makes a float operand of `+`, `-`, `\`, `MOD`, the bit-wise operators and the comparisons round first: with `o = 7`, `o < 7.4` is false (7.4 becomes 7) | Compare and add in floating point, as for the other integer types | Same (`s31_unsigned_ops`) |
| A FUNCTION named with a bare `` ` `` or `` ~` `` can be defined but not called ("Name already in use") | Allow the call | Same (`v21_x16`, `x17`) |
| `_UNSIGNED STRING` and `_UNSIGNED STRING * n` are accepted and mean `STRING` | An error, as for `_UNSIGNED SINGLE` | Same (`v21_a_decls`) |
| A suffixed literal or suffixed `CONST` outside its type's range is held as written and converted only by `PRINT`, `STR$` and type-keeping built-ins: `PRINT 300~%%` is 44, `300~%% + 0` and `l& = 300~%%` are 300, `CASE 300~%%` misses 44 | Convert it where it is read (44 everywhere), or a compile error | Same (`v21_a_literals`, `v21_d_const`, `v21_f_literal_uses`) |
| A constant used with another integer suffix is a literal of that suffix with the constant's digits, in range or not: `CONST u~& = 4294967295` makes `u%` print -1 and `u% + 0` 4294967295 | An error when the value does not fit the use's type, or convert it once | Same (`v21_g_const_suffix`) |
| A `CONST` with an `_OFFSET` suffix is an `_INTEGER64` literal, not an `_OFFSET`: `CONST k~%& = -1` makes `k~%& + 0` print -1 where an `_UNSIGNED _OFFSET` variable holding the same value gives 18446744073709551615 | Believe it `_OFFSET`, as a variable | Same (C++ of `v21_d_const`) |
| A `_BIT`-suffixed literal or constant is never narrowed: `` PRINT 9`3 `` is 9, `` CONST j~`3 = -1 `` prints -1 | Narrow it to its width and signedness | Same |
| A parameter declared `STRING * n` (or `t$n`) is a `STRING` parameter: only `LEN(t)` is n; the value, stores and comparisons see the caller's whole string | A real fixed-length parameter (cut and padded), or a compile error | Same (`v21_c_fixed_args`, `v21_f_fixed_param`) |
| Console `INPUT` into a `_BIT` variable compiles, reads the answer and stores nothing (a `_BIT` element is a compile error) | Store the value, or a compile error as for the element | `m2-builtin-statements` measurements (`verification\v22_e_bit`, `v22_e_redo`; `study\00` §5) |
| Console `INPUT` never asks again: a character that does not fit is dropped (`300` into a `_BYTE` is 30, `abc` into a number 0, `-1` into an unsigned type 1), and fewer fields than targets leave the rest 0 | "Redo from start" as in QB 4.5, or an error for a value out of range. A runtime (libqb) change | Same (`v22_e_redo`) |
| `INPUT l,` and `LINE INPUT s,` (one `,` after the last target) compile; the `#` forms and `READ` reject it | A compile error | Same (`v22_x148`) |
| A program whose standard input has run out waits for ever at the next console `INPUT`, and `END` waits for a key although the input is a file | End the program (or raise error 62) at the end of the input; `END` without the wait when the input is no console. A runtime change | Same (`v22_e_eof`) |
| `SWAP` with an element whose index is out of range raises error 9 **and** exchanges with the array's first element (`SWAP l, arr(9)` on `DIM arr(3)` exchanges `l` and `arr(0)`); nothing outside the array is touched | Skip the exchange when an operand's index raised, as `DIVERGENCES.md` D-004 does for a member store. Asked as `questions.md` Q3 of `m2-builtin-statements`; the user kept QB64pe's way (`DECISIONS.md` 2026-10-10) | Same (`v22_d_swap`, `s40_swap_mid`) |
| A number with a fraction of one half read by `INPUT #` or `READ` rounds differently by the target: 2.5 into an `_INTEGER64` is 3, into a LONG or INTEGER 2 | Round half to even for every integer target, as a store does | Same (`v22_b_input`, `v22_c_types`) |
| `OPEN f$ FOR OUTPUT ACCESS READ` raises error 53 ("File not found") | Error 75 or 5, or a compile error | Same (`v22_b_open`) |
| `WRITE` does not escape a quote inside a string: the value is written between quotes as it is, so `INPUT #` reads it back cut at the inner quote | Double the quote, or leave it (to check what QB 4.5 writes) | Same (`study\00` §5, "WRITE"; task 3.2 of the change) |
| A whole `TYPE` value stored with an index outside its bounds copies element 0 instead: `a(9) = p` raises 9 and overwrites `a(0)`; `q = a(9)` raises 9 and copies `a(0)` into `q` (kept as QB64pe for now: user, 2026-10-11) | Store nothing when an error is pending, as `DIVERGENCES.md` D-004 does for a member store (`a(9).m = 5`); the two rules then agree | `m2-defxxx-common-arrays` task 1.4 (`verification\v24_h_whole`, `v24_x210`) |
| A whole `TYPE` value of a type with a `STRING` member is copied part by part, and the indexes of both sides are evaluated again for every part: `a(one(1)) = b(one(2))` calls `one` six times for a type of a LONG, a `STRING` and a `STRING * 2` (twice for a type of numbers only) | Evaluate each side once, then copy. Only a FUNCTION with a side effect in an index shows the difference | `m2-defxxx-common-arrays` task 8.2 (`verification\v24_x306`, `x307`; `study\00` §5); with member arrays, where each element of a `STRING` member array is one more part, group 9 (`v24_x310`) |
| A store through a member with an index on the way evaluates a number before the indexes and a string after them: `a(setz(1)).s = z$` stores what `setz` put into `z$` (also for `z$ + "!"`), while `a(setz(1)).k = n` stores the `n` of before the call. `READ`, `INPUT #` and `LINE INPUT #` into such a place follow the same rule: a number's item is read before the indexes, a string's after them | One order for both, the value first, as an element store of a plain array has the indexes first for both | `m2-defxxx-common-arrays` group 9 (`verification\v24_x313`, `x316`, slice `s61_member_arrays` line 17); the reads: `m3-typefields` task 4.2 (`v35_x62`, `s117_dyn_member_kinds` items 17–22, 29–32, 36, 37) |
| `ERASE` and `REDIM` of a member array of an array element with an index outside its bounds raise error 9 **and** clear the member array of element 0 (`ERASE a(9).v` clears `a(0).v`) | Clear nothing when the index raised, as `DIVERGENCES.md` D-004 does for a store | Same (`v24_x312`, `s61` line 36) |
| `LBOUND`, `UBOUND` and `LEN` of a member array, and `REDIM _PRESERVE` or `REDIM _RETAIN` of one, never evaluate the indexes of the place it is part of: `UBOUND(a(f(1)).v)` does not call `f`, `LBOUND(a(9).v)` raises nothing | Evaluate and check the indexes, as an element does | Same (`v24_x311`, `x312`, `s61` lines 12 and 35; `_RETAIN`: `v34_x09`, `x10`) |
| An array used without `DIM` inside a SUB or FUNCTION is never allocated: every use raises error 9, on every call; only a `SUB … STATIC` gets a working 0 to 10 array (kept as QB64pe: user, 2026-10-11; `DIVERGENCES-QB45.md` Q-009) | Allocate it 0 to 10 on each call, as QuickBASIC 4.5 does | `m2-defxxx-common-arrays` task 1.3 (`verification\v24_f_implicit_sub`, `v18_g_implicit_in_sub`); compiled so since task 5.1 (`v24_x260`, `x263`, slice `s55_implicit`) |
| Below `$DYNAMIC`, an array used without `DIM` is allocated by the statement that names it first, as part of that statement: run again (in a loop) the statement raises error 10 and its store is skipped; skipped by `GOTO` or standing in an `ELSEIF` that is not reached, every later use raises 9; in a `LOOP UNTIL` line it raises 10 on every round after the first | Allocate it once, at program start or at the first use that runs | `m2-defxxx-common-arrays` task 5.1 (`verification\v24_x88`, `x101`, `x262`; inside a statement of several items and in a bound of `REDIM`: `x264`–`x266`, `x268`, marked "not supported yet") |
| `READ` of a quoted item into a number is no number: `DATA "5"` read into a LONG raises a syntax error, where the unquoted item is 5 | Read the digits inside the quotes | Same (`v22_c_types`) |
| `DIM a()`, `REDIM a()` and `REDIM a AS LONG` declare the scalar `a` and no array (`a(1)` after it is another array, used without `DIM`) | An error, or the dynamic array without bounds that QuickBASIC programs mean by `DIM a()` | `m2-defxxx-common-arrays` task 1.2 (`verification\v24_b_dim_empty*`, `v24_x43`, `x44`, `x86`) |
| Whether an array is static or dynamic follows the spelling of its bounds, not their value: `DIM a(5)` and `DIM a(k)` (a `CONST`) are static, `DIM a(2 + 3)`, `DIM a(k + 1)` and `DIM a(-k)` dynamic, so the second kind is allocated only when the `DIM` runs, raises error 10 when it runs twice and takes `REDIM` | Static for every bound whose value the compiler knows, as QuickBASIC 4.5 | Task 3.2 (`verification\v24_b_constant_bounds*`, `v24_x249`) |
| A dynamic `DIM` or `REDIM` with no element in a dimension (an upper bound one below the lower bound) raises error 5 and leaves the array allocated with those bounds and no element; one element fewer ends the program with "Out of memory" (error 257), which no handler receives. A bound that raises (`DIM s(ASC(""))`) does not stop the allocation either | Error 5 for every count below 1, and nothing allocated after an error | Task 1.2 (`verification\v24_b_dynamic`, `v24_x89`–`x96`) |
| Under `OPTION BASE 1` a constant bound that is 0 but not written `0` (`0&`, `(0)`, a `CONST`) compiles to a static array 1 to 0, and the program raises error 5 when it starts, before any handler is set | A compile error ("Invalid array bounds"), as for a negative constant | Task 1.2 (`verification\v24_x71`, `x103`) |
| `SHARED` on a later `REDIM` of an array is accepted and ignored: after `REDIM ns(3)`, `REDIM SHARED ns(4)` does not make `ns` visible in procedures | A compile error, or share it from there on | Task 4.1 (`verification\v24_x248`) |
| A procedure with `STATIC` after its header keeps its variables and arrays only down to its first `STATIC` statement: below that statement new variables are made on each call and a `DIM` is a local dynamic array (QuickBASIC 4.5 keeps every variable of such a procedure; `DIVERGENCES-QB45.md` Q-010) | Keep every variable and array of the procedure, wherever a `STATIC` statement stands | `m2-defxxx-common-arrays` task 6.1 (`verification\v24_x279`, slice `s56_proc_arrays` line 21; `qb64pe.bas` 5827, 8805, 9558) |
| `STATIC c(3)` is a dynamic array allocated when the program starts: `ERASE c` releases it (the next use raises 9) and the next error that reaches a handler allocates it again, empty; `REDIM c(5)` is accepted | A static array, as a `DIM` with constant bounds in the main module: `ERASE` clears it and `REDIM` raises 10 | Same (`verification\v24_x278`, `s56_proc_arrays` line 24, the old compiler's C++) |
| `STATIC a(5)` beside a shared main-module array of that name and type compiles, and the program ends when it starts ("Duplicate definition"); marked "not supported yet" here | A compile error | Same (`verification\v24_x272`) |
| `SHARED a()` below an array of that name and type that the procedure has already (a local `DIM`, or a use without `DIM`) is accepted and changes nothing | A compile error ("Name already in use") | Same (`verification\v24_x275`) |
| Signs are removed from the text of an expression before it is read: a `+` right after a `+` or a `-` (`a + +b`, `-+b`), then two `-` right after an operator, so `"a" + - -s$` joins two strings and `2 * - -i%` never negates, while `+b`, `2 * +b` and `(+b)` are errors and `s$ = - -s$` is "Cannot convert string to number". A `CONST` has another rule: a run of signs is one sign, so `CONST c = +-3` is accepted and `CONST c = 2 * - -3` is an error | A unary plus everywhere, as QuickBASIC 4.5 has (not measured there); or an error for every doubled sign | The known bug of 2026-10-11 (`verification\v25_*`, `study\00` §5) |
| An array made by passing a name that names none to an array parameter (`fill nx()`) has one dimension, 0 to 10, whatever the procedure does with the parameter; inside a SUB or FUNCTION the same call is a compile error ("Expected (...)") | An error in the main module too, or an array in both | Task 7.1 of `m2-defxxx-common-arrays` (`verification\v24_x146`, `x286`, `x292`) |
| `LBOUND` and `UBOUND` of an array listed by `STATIC a()` give it one dimension (a `DIM a(2, 5)` below is an error); of one listed by `COMMON a()` they do not | One rule for both | Task 7.2 (`verification\v24_x282`, `x297`–`x302`) |
| What stands between the parentheses of `COMMON a(3)` and of an array parameter `a(1)` is accepted and not read | A compile error for anything but `()` | Tasks 1.3 and 7.1 (`verification\v24_x157`, `x166`) |
| Nine constants of the auto-included file do not hold the value their names say: `_FLOAT_MAX` and `_FLOAT_MIN` print `INF 7693134862315D+308` and `-INF 7693134862315D+308`, `_FLOAT_MIN_FRAC` is 0, `_INTEGER64_MIN` and `_OFFSET_MIN` are -9223372036854775807, `_UINTEGER64_MAX` and `_UOFFSET_MAX` 9223372036854775807 (QB64pe's `CONST` evaluator does not read their literals); `_DOUBLE_MIN`/`_MAX` are right. Not compiled here yet ("not supported yet"), so nothing is decided | Give them the values of their types' limits when the `CONST` evaluator reads such numbers | `m2-console-screenhide` task 5.2 (`verification\v26_x114`) |
| `$SCREENHIDE` inside a `$IF` branch that its program's own flag switches off (`$IF _CONSOLE_ = 0 THEN` … `$CONSOLE`, `$SCREENHIDE`) is not compiled in the last round, so the window shows; the `$CONSOLE` beside it still counts, by its flag | One rule for the lines of such a branch | `m2-console-screenhide` design D1 (read from the source, `qb64pe.bas` 3617; `verification\v26_x26`) |
| An array argument in parentheses, `fill (x())`, compiles and fails QB64pe's C++ build; a compile error here | Pass the array, as without the parentheses | Task 1.3 (`verification\v24_x145`) |
| Screen statements that are wrong for their screen and raise nothing: `VIEW (1, 1)-(5, 5)` in `SCREEN 0`, `PCOPY 0, 9` with eight pages, `WIDTH 81` (an 81-column text screen), `PALETTE 16, 0` and `CLS 3` in `SCREEN 0`, `SCREEN 0, , 9` (a ninth page is made); the others of `d14_errors` raise error 5 | Error 5, as for `PSET` in `SCREEN 0` or `LOCATE 26, 1`. Each is libqb's, so a change is a change to the runtime | `m3-window-output` task 1.4 (`tests\corpus\screen\d14_errors` and the programs tried while it was written) |
| `COLOR` takes a fourth argument (`COLOR 1, 2, 3, 4`), which its help text does not name, and `SCREEN` a fifth after the word `_MANUALDISPLAY`, "a temp format for transition reasons" in the old compiler's source | Leave them out, or document them | `m3-window-output` task 1.1 (`verification\v27_x01`) |
| Every built-in statement name written `name (…) = …` is an error ("Name already in use"), but `WIDTH (80) = 80` makes an array named `WIDTH` and `WIDTH = 80` a variable; `CLS (1) + 2 = 5` is `CLS` with a comparison | `WIDTH` reserved like the others | `m3-window-output` task 3.2 (measured 2026-10-12, `tests\frontend\stmt_name_assigned.bas`) |
| In a program with a console and no window, `POS(0)` is 1 wherever the output stands and `CSRLIN` does not move as lines are printed (standard output redirected) | The column and row the program has written to, as on a screen | `m3-window-output` task 1.3 (`verification\v27_a_console_functions`; `tests\corpus\runtime_comparison\237_csrlin_pos`) |
| A key pressed before `SLEEP` starts does not end it, and stays in the `INKEY$` buffer | To check against QB 4.5, where `SLEEP` ends at once when a key is waiting | `m3-window-output` task 1.4 (read from `sub_sleep`, `libqb.cpp`; seen as a screen program that never ended) |
| A line that is a SUB's name and a colon is a call of the SUB or a label, by whether any jump of the whole program names the word: a `GOTO name` added in one SUB turns the line `name:` of the main module from a call into a label, without a message | A warning for such a line, or one meaning | `m3-get-put-bload` design D1 (`verification\v28_c_labels`, `v28_x71`–`x92`) |
| The index inside `VARSEG(a(i))` is not evaluated: a FUNCTION in it is not called and an index outside the bounds raises nothing; inside `GET`/`PUT` an element's index is evaluated twice | Evaluate it once | `m3-get-put-bload` task 1.2 (`verification\v28_d_varptr_index`; the C++) |
| `GET (1, 2)-(3, 4), 5` is "UNEXPECTED INTERNAL COMPILER ERROR!"; an error here. `BLOAD` and `POKE` write where the segment and offset say, past the end of the array they were taken from too (inside libqb's block of conventional memory) | A plain error message; the writes are what `BLOAD` and `POKE` are for | `verification\v28_x15`, `v28_b_bload` |
| On the console `TAB(n)` is n - 1 blanks wherever the line stands, because `POS(0)` is always 1 there; in a window and in a file it moves to column n | The column the program has written to | `m3-get-put-bload` task 1.1 (`verification\v28_a_tab_spc`) |
| A plain variable, an undeclared name or a string where graphics `GET` and `PUT` take an array is accepted: the picture is read from or written over that variable's few bytes | An error, as for a `CONST` or an expression | `verification\v28_x23`, `x59` (not compiled here: marked) |
| A relative `$EXEICON` name without `./` is looked for in the compiler's own folder, not beside the program (`$EXEICON:'game.ico'` with `game.ico` beside the source is "File 'game.ico' not found") | Look beside the main source file, as for a name with `./`. Marked "not supported yet" until decided | `verification\v29_e02`, `e03`, `e08`, `e18`; `qb64pe.bas` 3852 |
| `$EXEICON` drops the rest of its line, a statement after a colon included (`$EXEICON:'./a.ico': PRINT 1` prints nothing), and takes any line that starts with the word (`$EXEICONS:'…'`, `$EXEICON '…'` without the colon) | Reject what is not `$EXEICON:'name'` alone on its line, as the console metacommands are | `verification\v29_e08`, `e09`, `e20`, `e24` |
| A `PLAY` with more than one string never holds the program, even for a voice in the foreground, and ends a `SOUND WAIT`; a `PLAY` string takes a tempo and an octave outside QuickBASIC's range (`T31`, `T256`, `O7`: `DIVERGENCES-QB45.md` Q-011) | Wait for the longest foreground voice; raise error 5 for the values QuickBASIC rejects | `verification\v29_g_voice`, `v29_h_sound_ranges`, `v29_c_sound_errors`; `PlayMML` in libqb's `audio.cpp` |
| A sound handle that is not open (0, negative, never opened, closed) raises no error: a statement does nothing and a function gives 0; a missing file or a sound bank that is not there is silently 0 or ignored | Error 5 or 52 for a bad handle, as QuickBASIC raises for a bad file number | `m3-sound-handles` (`verification\v39_a_handles` 5., 6., 13.; `IsHandleValid` in libqb's `audio.cpp`) |
| A sound just opened, paused or stopped has `_SNDPAUSED` -1 (a sound that never played counts as paused) | 0 until the sound has been paused | Same (`v39_a_handles` 2., 7.) |
| `_SNDLEN` of a `_SNDNEW` sound is a SINGLE widened to DOUBLE (`.1000000014901161` for 800 frames at 8000 Hz), and the length and position of a decoded sound are counted in frames of the device's rate (`.125` s reads `.1249886602163315` at 44100 Hz) | Compute in DOUBLE from the sound's own rate | Same (`v39_b_new_mem` 1., `v39_a_handles` 4.; `GetSoundDuration`, `GetSoundPosition`) |
| `_SNDPLAYFILE`'s second argument (`sync`) is read and ignored; `_SNDBAL`'s fifth (the channel) is accepted though its reference shows four, and ignored | An error for an argument that does nothing, or drop it from the forms | Same (`verification\v39_x01_accepted`; `PlaySoundFile`, `SetSoundBalance`) |
| A whole array `e()` where a string variable is expected (`FIELD #1, 4 AS e()`, `LSET e() = "x"`), as a value of `PRINT USING` and as a part of `_IIF` is accepted and is the array's first element, with no bounds check (not compiled here: marked, or an error) | An error, as for any other whole array in an expression | `verification\v30_x18`, `x99`, `x109`, `x153` |
| `_MIN(nan, 1)` gives the value that is not a number and `_MIN(1, nan)` gives 1 (`std::min` compares once); `RSET` cuts a value that is too long at its end, as `LSET` does, where QuickBASIC's manual says nothing else | Document, or return the number in both orders | `verification\v30_g_iif_min_max`, `v30_l_lset_rset`; line 19 of `s78_choice_min_shifts` |
| A field string keeps its field after `=` gives it another length, and gets its width and the buffer's bytes back at the next `LSET` to any field of the file or `GET` with no variable; a second `FIELD` on a file keeps the strings of the first as fields | QuickBASIC ends a string's field at an assignment. A decision for a strict mode | `verification\v30_j_field`; lines 6 to 9 of `s86_field_lset` |
| A number that was never an image handle passed to `_WIDTH` (`_WIDTH(12345)`) raises nothing and gives the screen's width; `_SAVEIMAGE "x.png", 12345` writes a file and raises nothing; a freed handle raises 258 in both | Error 258 for any number that is no handle. libqb's, so a change is a change to the runtime | `m3-images-fonts-mouse` task 1.1 (`verification\v31_b_images`, `v31_c_save_load`) |
| `_CLEARCOLOR` alone and `_CLEARCOLOR _NONE 4` (the word and a colour) are accepted by the template `[{_None}][?][,?]` | A compile error for both | `m3-images-fonts-mouse` task 1.1 (`verification\v31_x01_accepted`) |
| `_MOUSEX` and `_MOUSEY` map an event when they are read, by the frame libqb drew last: read before the window's first frame after `SCREEN`, they give -1, and after a `SCREEN` that changed the size, by the old size until a frame is drawn (read in `mouse.cpp`, not measured) | Keep the position the event had on the screen when it came, as `_MOUSEMOVEMENTX` is kept. libqb's, so a change is a change to the runtime | `m3-images-fonts-mouse` task 4.3 (`study\00` §5; `tools\screen\screen_state.cpp` waits for that frame) |
| `_PRINTSTRING STEP(x, y), t$` without an image counts from neither the last point of `PSET` nor that of the last `_PRINTSTRING`: after `SCREEN 13: PSET (100, 0)` the text starts at (4, 4). libqb takes `write_page_index` (a page) as an image's index there, where the form with an image maps the page through `page[]` (`libqb.cpp` 18291–18309), so it reads another image's point | Count from the destination's last point, as `PSET` and `LINE` do. libqb's, so a change is a change to the runtime | `m3-images-fonts-mouse` task 3.2 (`tests\corpus\screen\f01_builtin_fonts`, where "step" stands over "font 14"; a program built outside the repository) |
| `EXIT CASE` followed, in the same `CASE`, by a `SELECT` with a `CASE` jumps to the end of that inner `SELECT`'s first `CASE`: the statements up to there are skipped (also the inner `SELECT`'s own header, so a copied selector is 0), the program goes on after the inner `END SELECT`, and the rest of the outer `CASE` runs. The old compiler names the jump's label by a count of the `CASE`s closed so far | Jump to the end of the `CASE` the statement stands in. `EXIT CASE` in a plain `SELECT` (it leaves the block) is as the wiki has it | `m3-console-files` task 3.1 (`verification\v30_x66`; `qb64pe.bas` 7511; lines 11 and 12 of `s79_exit_select`) |
| `_MEMEXISTS` of a block of an array after `ERASE` is -1: libqb renews an array's lock only in `REDIM`. Of a dynamic array the elements are released, and `_MEMPUT` through the block then writes into the released memory with no error | `ERASE` renews the lock as `REDIM` does, so the block stops being valid (error 308). In the emitter's `ERASE`, not in libqb | `m3-mem` task 1.2 (`verification\v32_b_get_put`, `v32_d29_erase_dynamic`) |
| A block's lock is that of the SUB, FUNCTION or main module where `_MEM` stands, not of the one its variable lives in: a block of a main-module or `STATIC` variable taken in a SUB stops being valid when the SUB returns | The lock of the variable's own body (the main module's for a shared variable, the program's for a `STATIC` one) | `m3-mem` task 1.2 (`verification\v32_d34_lock_of_the_body`) |
| `_MEMFREE` of a block of `_MEMNEW(0)` taken after another block was freed ends the program (exit code 127): libqb sets no lock type for a block of 0 bytes, so a lock taken again from the freed list keeps type 1 and its old pointer, which is freed a second time | Set the lock's type for a block of 0 bytes. libqb's (`memblock.cpp` `func__memnew`), so a change is a change to the runtime | `m3-mem` task 1.2 (`verification\v32_d33_free_zero_size`) |
| `ERASE` of a static array of a type with `_DYNAMIC` members releases each element's members and leaves them not allocated: every use raises error 9 until a `REDIM` of the member, while a variable's or a dynamic array's members always have their declared bounds when made | Give each member its declared bounds again, as `ERASE` of a static array gives its other members zero | `m3-typefields` (`verification\v35_b` 9., `x51`; `s116_dyn_member_parents` items 9 and 10) |
| A whole-value copy of a type with `_DYNAMIC` members finds both places again for each of its statements (a test that they differ, each member, each element of an inline member array): `a(one(1)) = b(one(2))` calls `one` 8 times for a type of a LONG and a `_DYNAMIC` member, 12 times with a `STRING` beside them, 29 times for a nested owner type with strings | Evaluate each side once, then copy (as for the string case of plain types above) | `m3-typefields` task 4.3 (`verification\v35_x56`, `x61`; `s117_dyn_member_kinds` items 11, 12, 14, 15) |
| A whole-value copy from a value whose `_DYNAMIC` member is not allocated (after `ERASE x.v`) keeps the target's member when the type holds no `STRING`, and leaves it not allocated when it holds one: the old compiler has two copy routines, `AppendDynUDTDescCopy` and `AppendDynUDTOwnCopyEx` | One rule: the target's member as the source's (not allocated) | `m3-typefields` task 4.3 (`verification\v35_x61` 3. and 4.; `s117_dyn_member_kinds` item 16) |
| `b%() = a%()`, a whole-array assignment with a type suffix on both names, copies element 0 of `a%` into element 0 of `b%` (converting, `a&() = b%()`) and nothing else, with no check of types or bounds; with a suffix on one name only it is an error (not implemented here: marked, see "Left marked by `m3-whole-array-assign`") | Copy the whole array, as without the suffix | `m3-whole-array-assign` (`verification\v36_x24`, `x38`, `x40`, `x43`, `x44`) |
| `_MEMELEMENT` of an element of a member array (`_MEMELEMENT(u.v(1))`) is the whole member array's bytes typed as one element (16, 132, 16 for four LONGs), and `_MEM` of a member is typed 32768 with element size 1 whatever the member's type, where `_MEMELEMENT` of it has the member's own type | One element, as for an array variable; the member's type for `_MEM` too | `m3-mem` task 1.1 (`verification\v32_a_blocks`) |
| A `DECLARE LIBRARY` block that finds a header writes no prototype for any of its declarations, also for those of the block's other libraries: a function of a `.a` named beside a header must be declared by the header, or the C++ fails (`DECLARE LIBRARY "v37h", "./v37c"` with `c_half#`, which only `v37c` has) | A prototype for each declaration the header does not declare; that needs the header read, and a prototype could disagree with what it declares | `m3-declare-library` (`verification\v37_x23`, `x30`; design D3) |
| A library FUNCTION's result is cast to its declared type, so a C `double` returned to a FUNCTION typed LONG (no suffix under `DEFLNG`) is truncated: 2.7 gives 2, where an assignment of 2.7 to a LONG gives 3 | Round as every other conversion to an integer does | `m3-declare-library` (`verification\v37_x63`; `slice\s121_library_numbers`, line `expr:`) |
| A `STRING * n` member passed to a library's `char*` has no NUL after its n bytes: C reads on into the next member (`"abcd"` before a LONG of 65 has length 5) | Pass a copy with a NUL, as a variable-length string is passed. A change to the call, not to libqb | `m3-declare-library` (`verification\v37_x66`) |
| A `$VERSIONINFO` string value keeps a trailing comment: `$VERSIONINFO:Comments='a' ' note` is the value `'a' ' note`, with the warning "Missing string bracket delimiters" under `-w` (on a numeric key the comment makes the line an error) | End the value at a `'` that follows the closing quote, as a comment ends any other line | `m3-versioninfo` (`verification\v40_d`, `x18`) |
| Any metacommand that starts with `$VERSIONINFO` is taken as one: `$VERSIONINFOX:CompanyName='a'` sets the company name | "Syntax error", as for an unknown metacommand | `m3-versioninfo` (`verification\v40_j`) |
| `FILEVERSION#` and `PRODUCTVERSION#` accept parts the resource compiler refuses (empty, over 65535, a leading 0 with an 8 or 9), so the old compiler's build fails; this compiler rejects them with a compile error. A part with a leading 0 is octal (`010` is 8), which the old compiler passes on unchanged | Read every part as decimal (writing it without its leading zeros) | `m3-versioninfo` (`verification\v40_h1`–`h7`; design D3) |
| A `<` or `&` (or a byte that is not UTF-8) in the company name, product name or description goes unescaped into the manifest, and Windows does not start the executable; this compiler rejects them with a compile error (user, 2026-10-16) | Escape them in the manifest (`&lt;`, `&amp;`), so the program starts | `m3-versioninfo` (`verification\v40_g2`–`g6`; design D3) |
| `_ARRAYCOPY s() TO d()` keeps the source's indexes for a one-dimensional source (`d` must hold them), but for a source of several dimensions needs the destination's bounds exactly (error 9 for `m(1 TO 2, 1 TO 3)` to `n(0 TO 4, 0 TO 5)`, which holds it) | Keep the indexes in every number of dimensions, as `s(lo TO hi, …) TO d()` does | `m3-arraycopy` (`verification\v41_a` 1., `v41_g` 15., 16., `x57`) |
| `_ARRAYCOPY m(1 TO 2) TO n(0)` for two-dimensional arrays is "_ArrayCopy TYPE-member step supports only one-dimensional arrays", a message about members for arrays that are none (this compiler's message names the dimensions) | A message that names the form and the dimensions | `m3-arraycopy` (`verification\v41_x25`) |
| `ON KEY(k)`'s key is a `DOUBLE` that C++ cuts toward zero into libqb's `int32`: `ON KEY(0.7)` is error 5 and `ON KEY(31.6)` key 31, while `KEY(31.6) ON` rounds to 32 (error 5) and every other integer argument rounds | Round as the switch does | `m3-events` (`verification\v42_d` 12.–14., 17.) |
| A SUB handler's argument is converted to `_INTEGER64` when the handler is set up, and the SUB gets a pointer to that value cast to its parameter's type: `_BYTE` given 300 sees 44, `_UNSIGNED INTEGER` given -5 sees 65531, with no overflow error; what the SUB stores into its parameter is lost | Convert the argument to the parameter's type, with its overflow error, at the setup | `m3-events` (`verification\v42_c`) |
| A timer event that comes while an error handler runs is not serviced after `RESUME` either: it waits through plain statements (also a SUB's loop) until the next `TIMER ON` or the next `_DELAY`, where several come at once. libqb's: `evnt` clears `qbevent`, `events()` skips during error handling, and the timer thread sets `qbevent` only when it marks a new event | Service the held event at the first statement boundary after the handler ends. libqb's, so a change is a change to the runtime | `m3-events` (`verification\v42_e` 4.–6., `v42_f`) |
| A SUB with a `TYPE` parameter passes the old compiler's checks as an event handler, and its C++ then fails to build; this compiler rejects it with a compile error | The error of a parameter that is not an integer | `m3-events` (`verification\v42_x42`) |
| `_ERRORLINE` for an error in an included file is the main file's line of the outermost `$INCLUDE`, the same for every statement of that file and of the files it includes; only `_INCLERRORLINE` tells them apart | None needed if kept: the pair names the place. A change would be to libqb's meaning of `_ERRORLINE` | `m3-errorline-chr-const` (`verification\v43_b_include`) |
| A line joined to the next by a continuation (`_`) is numbered as the join's last line for every statement on it: `ERROR 5: x = 1 + _` reports the line below it, in `_ERRORLINE` and in the message of an untrapped error | The line each statement starts on | `m3-errorline-chr-const` (`verification\v43_d_continued`, `v43_e_continued_untrapped`) |
| `CHR$` in a `CONST` rounds its code half to even through `_UNSIGNED _INTEGER64`, so `CHR$(-0.5)` is `CHR$(0)` while `CHR$(-0.6)` is an error | Range-check the code before rounding | `m3-errorline-chr-const` (`verification\v43_x11`, `x12`) |
| `_STATUSCODE` of a negative handle that was never opened (`_STATUSCODE(-5)`) ends the program with an access violation (exit 139), where `_CONNECTED`, `_CONNECTIONADDRESS$`, `EOF`, `LOF`, `GET` and `PUT` of it raise error 52 | Error 52, as the other functions raise. libqb's (`func__statusCode`), so a change is a change to the runtime | `m3-network-embed` (`verification\v44_x59`, `x60`–`x68`) |
| `$EMBED` takes any line that starts with the word (`$EMBEDX:'…'`, `$EMBED '…''h'` without the colon and the comma) and drops the rest of its line; two files are the same only when their names are the same text with the folder in front (`./a/../a/f` embeds `./a/f` a second time, `./d\f` embeds `./d/f`; only `./` and `.\` count as one) | Reject what is not `$EMBED:'file','handle'` alone on its line; compare the files themselves | `m3-network-embed` (`verification\v44_a_embed`, `x22`, `x35`, `x39`, `x69`, `x70`) |
| `_DECODEURL$` turns a `%` whose next two characters are not hexadecimal into a byte 0 (`%zz` is one byte 0, `abc%` ends with one) | Keep such a `%` as it is | `m3-network-embed` (`verification\v44_d_url`) |

## Other ideas raised during the study

| Idea | Source |
|---|---|
| "Skip lines" debugger feature (no Debug Adapter Protocol equivalent) | `study\06-vscode-parity.md` |
| Accept two `DECLARE LIBRARY` forms whose build fails with the old compiler: a header named by two blocks included once (it is included once per block, and a header without a guard fails in C++; was D-023), and a name looked up in every library of a block (the old compiler looks only in the last; was D-024); and two `DYNAMIC` blocks of one DLL sharing its handle (each declares it, and the C++ fails, `verification\v37_x67`). Kept the old compiler's way for now, so code that compiles here compiles with QB64pe; the unguarded header that defines something (one that only declares builds, `v37_x68`), the doubled handle and a block without a header whose last name finds no file (`v37_x69`) are rejected before the C++ with an error that says why | User, `DECISIONS.md` 2026-10-16 (`m3-declare-library` proposal) |
| New compiler compiles the old `qb64pe.bas` as a stress test | `study\07-expert-panel.md` R10 |
| Redesign the runtime ABI (variables as pointers, `qbs` moving heap, `passed` bitmasks) once the old compiler is no longer a producer | `study\07-expert-panel.md` R6 |
| Fuzz the lexer and parser with a coverage-guided fuzzer: "no panic, exact round trip" is an ideal fuzz property. A seeded mutation test over the corpus comes first, in `m2-upstream-tests` (`study\22` §5) | `study\21-rust-review.md` §4 |
| An expression as the `INPUT` prompt (`INPUT p$; x`), a language extension; QB64pe and QB 4.5 accept only a string literal | Bug-compatibility decisions, 2026-10-08 (`study\10` §2.8) |
| The language server notices included files that change on disk while not open (`workspace/didChangeWatchedFiles`, or parsing the includers again on `didSave`); today the change shows at the includer's next edit | Review of `m2-language-server`, 2026-10-08 |
| Run the screen programs of the corpus on a desktop of their own (`CreateDesktop`), so their windows do not show for a moment on the user's desktop and cannot take a key the user types. Not measured: whether GLFW's OpenGL window works on a desktop that is not shown | `m2-screen-state-check`, design D5, 2026-10-10 |
| Screen recordings of QB64pe's `tests\qbasic_testcases` (143 programs by several authors, no licence stated, so not copied; 134 read the keyboard, 46 use `RANDOMIZE TIMER`). Of the 9 that read no key, 6 end with equal states in two runs. They could be run from the clone, where they are, with only the recordings kept here | `m2-screen-state-check`, design D8, 2026-10-10 |
| A longer scripted game of Civil War Strategy, or a second one: what `g12_game`'s two turns as the Union do not reach (a naval action with `barnacle`, `ironclad` and `schooner`; a retreat; `combine`, `relieve`, Capital, Supply, Detach as the Confederacy; the end of a game with `endit`, `capitol` and the high-score file; loading a saved game; the alternative map; two players; the history file). Each is more keys and states in a keys file written by playing the old compiler's build (`tools\screen\README.md`, "Prompts") | `m3-scripted-game`, task 4.1, 2026-10-14 |
| Sound in a scripted game: the copy runs with `noise` 0. With one more edit (`PLAY "MB V0"` near the start) the game's `SOUND` and `PLAY` lines would run silently, at the cost of the notes' time; whether `SOUND` keeps volume 0 is not measured | `m3-scripted-game`, `DECISIONS.md` 2026-10-14 |
