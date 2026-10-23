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
| A dynamic member array of an array element beside other targets of one `ERASE` (`ERASE a(1).d, t.v`) | Not measured; a static one is a part of one statement with the other targets since `m4-memory-records` (design D9) | `check\places.rs` `clear_member_array` |
| `REDIM t.v(0) AS type` under `OPTION BASE 1` | Not measured: whether the number 0 keeps the rule it has in a `DIM` | `tests\frontend\member_arrays_errors.bas` |

(An element of a member array, and a member of an array element, as a target of `INPUT`, `READ` or the `MID$`
statement compiles since `m3-typefields` task 4.2, measured by `verification\v35_x62`.)

**Left marked by `m2-console-screenhide`** (step 11, 2026-10-12: a console and a window, the window built-ins,
the constants of the auto-included file, the precompiler flags). What the old compiler accepts and the new one
still marks "not supported yet":

| Form | Why it waits | Details |
|---|---|---|
| The window built-ins are compiled and never run (`_TITLE`, `_ICON`, `_MOUSESHOW`, `_SCREENX`, `_NEWIMAGE`, … 20 of them) | What they do shows only in a window; M3 gives programs with a window a check (`tools\screen`) | `study\archive\support-record.md` ("Window built-ins, compiled and never run"); `sema\src\builtins.rs`, `WINDOW_BUILTINS`; call-site programs in `tests\callsite` |
| `SWAP`, `CLOSE` and the `MID$` statement after `CALL` (`CALL SWAP(a, b)`) | Each has a rule of its own, which reads the statement without `CALL` only. Until the review of groups 4 to 6 `CALL SWAP(a, b)` was a false error ("`SWAP` takes 0 arguments") | `verification\v26_x128`; `tests\frontend\swap_mid_marked.bas` |
| The function `_SCREENHIDE` | Not in the change's scope (no upstream test names it) | `verification\v26_x74` |
| `_SCREENEXISTS` as the direct argument of `HEX$`, `OCT$` or `_BIN$` | The table gives the function no return type, and the old compiler passes the width 0 there, not a LONG's 8 or 32; the function runs from M3 on, where the width can be given | Design D6 of the change |
| A `$IF` that names `_CONSOLE_` between `$CONSOLE:ONLY` and a later `$CONSOLE` (or the other way round) | The old compiler's two passes read the line with different values (its program prints nothing from the branches; a `CONST` in them is "Name already in use"); no value here would be right | `verification\v26_x27`, `x34`; `tests\frontend\flags_between_errors.bas` |

**Left marked by `m3-window-output`** (the first step of M3, 2026-10-12: a program with a window as its
output, the text-screen, drawing and pacing statements, the screen functions). What the old compiler accepts and
the new one still marks "not supported yet", and what is compiled but checked by its call alone:

| Form | Why it waits | Details |
|---|---|---|
| An argument left out of a function call (`POINT(, 2)`, `SCREEN(1, , 1)`) | The old compiler rejects both ("Incorrect number of arguments"); here they get the general mark "omitted arguments", which is not an error of the program | `verification\v27_x50`; `sema\src\check\builtins.rs` |
| A label named like a built-in statement that is not compiled yet (`FILES: x = 1`) | It becomes the statement when the built-in comes in; a jump to a built-in's name is the old compiler's error since `m4-names-implicit-arrays` | `verification\v27_x58`; `tests\frontend\stmt_name_colon_errors.bas` |
| `PALETTE USING array(index)` | Not in the change's scope: the old compiler writes it by hand (`qb64pe.bas` 7238), outside `PALETTE`'s template. Until the review of group 4 the parser gave it a syntax error ("matches none of its forms"), a false error for a line the old compiler accepts | `verification\v27_x64`; `tests\frontend\palette_using_marked.bas` |
| `WIDTH #n, columns`, `WIDTH LPRINT columns` and `SCREEN …, _MANUALDISPLAY n` are compiled, and no test runs them | Each is the plain call of its template; what it does is no part of a screen's state (a file's line width, the printer, when the window is updated) | Call-site programs `width_file*`, `width_lprint*`, `screen_manualdisplay*` |
| `_LIMIT`, `_DELAY` and `SLEEP` are run by one screen program with small values; how long they wait is not checked | Nothing a test records may depend on time | `tests\corpus\screen\d15_pacing` |
| `SLEEP` with no time in a screen program | The keys file cannot end it once a key is in the buffer (it gives the next key only to an empty buffer, and a key ends `SLEEP` only while it waits). A `repeat` step that presses whatever the buffer holds would do it; that is a change to `tools\screen\screen_state.cpp` | `tools\screen\README.md`, "Writing a screen program" |

**Left marked by `m3-get-put-bload`** (the second step of M3, 2026-10-13: labels with the name of a SUB, `TAB` and
`SPC`, conventional memory, graphics `GET` and `PUT`). What the old compiler accepts and the new one still marks
"not supported yet", and what is compiled but checked by its call alone:

| Form | Why it waits | Details |
|---|---|---|
| `VARPTR`, `VARSEG`, `SADD` and `VARPTR$` of a variable, member or element of a `TYPE` that holds a `STRING` (or a dynamic member array), and graphics `GET` and `PUT` of one | In conventional memory the old compiler's allocation leaves out the value's strings, and its program crashes at the first use of one (`verification\v56_a5`: exit 139); `GET` writes over the string pointers. Every other variable of the main module is in conventional memory since `m4-memory-records` | `verification\v56_a5`, `x08`, `x26`, `x27`; `tests\frontend\sadd_varptr_str_errors.bas` |
| `VARPTR` and `VARSEG` of an array of a SUB or FUNCTION (`DIM` or `STATIC` there) and of an array parameter | A local array starts with flag 4 and is allocated per call from conventional memory; a parameter named so makes the callers' arguments conventional (`qb64pe.bas` 12081, 12810), a fact carried through calls. The text is the same as for an array of the main module | Same |
| `VARPTR` and `VARSEG` of a whole array `x()` that is not of numbers, `VARSEG` of a `_BIT` array, `VARPTR` of a member of a fixed-length string or `TYPE` type, `VARPTR$` of a member of an element or of a member array's element | Not measured by `verification\v56` (the measured forms compile since `m4-memory-records`) | `tests\frontend\sadd_varptr_str_errors.bas` |
| In the array's place of graphics `GET` and `PUT`: an array of `_BIT` | Not measured; every other place compiles since `m4-memory-records` (design D5) | `tests\frontend\m3_step2_marked.bas` |
| An index that calls a FUNCTION in the array of `GET` or `PUT` (`GET (1, 2)-(3, 4), a(f(1))`) | The old compiler writes the index twice, so the FUNCTION runs twice; writing that the same way needs a second evaluation the emitter does not have | Read from the C++ (`study\00` §5) |
| The file forms `GET #n, …` and `PUT #n, …` | Step 4 of M3's order (random and binary files) | The mark is ` `GET` on a file ` |
| A line that is the name of a SUB and a colon, where a line the parser could not read has the word in it | Whether the line is a call or a label depends on every jump of the program, and that line may be one | `tests\frontend\sub_name_colon_errors.bas` (`five:`) |
| `_FULLSCREEN` (statement and function) is compiled, and no test runs it | A test that takes the whole desktop is not wanted | Call-site programs `fullscreen_*` |
| `DATE$` and `TIME$` are run for their length and separators only; the statements `DATE$ = …` and `TIME$ = …` are not compiled | Nothing a test records may depend on the clock; setting the clock is no test's business | `tests\corpus\slice\s71_tab_spc` |

**Left marked by `m3-sound`** (the third step of M3, 2026-10-13: `SOUND`, `PLAY`, `BEEP`, `$EXEICON`, and a
statement with a template after `CALL`, which closed three rows of the tables above). What the old compiler
accepts and the new one still marks "not supported yet", and what is compiled but not checked by a run:

| Form | Why it waits | Details |
|---|---|---|
| An `$EXEICON` name with a drive and no slash after it (`C:game.ico`) | The old compiler looks for it in the folder it stands in on that drive (read in `qb64pe.bas` 3854–3873, not measured), which has no counterpart here. The other relative names are looked for under the compiler root since `m4-devices-continue-paths` (`DECISIONS.md` 2026-10-20) | `sema\src\check\mod.rs` `exe_icon_meta` |
| `VARPTR$` in a `PLAY` or `DRAW` string (`PLAY "X" + VARPTR$(a$)`) | `VARPTR$` compiles since `m4-memory-records`; what `PLAY` and `DRAW` do with the `X` command was not measured | Not measured |
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
| `LPRINT` and `CHAIN` are compiled, and no test runs them | `LPRINT` prints a page on the default printer; `CHAIN` starts `qb64pe.exe -c` beside the program through `cmd` (user, `DECISIONS.md` 2026-10-13). `238_chain` on `slice.list` reaches `sub_chain` with a name that is not there; no test chains to a program that exists (gap noted by the user, `DECISIONS.md` 2026-10-16). Their calls are compared by `tests\callsite\lprint_*.bas` and `chain_*.bas`; `s82_print_using_file` and `s87_chain_not_reached` hold them as code that is not reached | `NEVER_RUN` in `sema\src\builtins.rs` |
| What a format of `PRINT USING` prints, and what a file holds after `GET` and `PUT`, are shown once per topic | Both compilers' programs call the same libqb with the same arguments, which the call-site check compares form by form; more lines would test libqb | Design, Non-Goals |

**Left marked by `m3-images-fonts-mouse`** (the sixth step of M3, 2026-10-14: images, colours, fonts, the mouse).
None is left: `_UCHARPOS` of an array that is not declared (`nd&()`) compiles since `m4-memory-records`.

**Left marked by `m3-mem`** (the seventh step of M3, 2026-10-14: `_MEM`, its functions and statements, `_OFFSET`,
`MKI$` … `_CV`). What the old compiler accepts and the new one still marks "not supported yet", each not measured:

| Form | Why it waits | Details |
|---|---|---|
| `_MEMGET`, `_MEMPUT` and `_MEMFILL` of a value of a `TYPE` that holds a `STRING` | Such a value holds the pointer of its string; what the old compiler's helpers do with it was not measured (`_MEM` and `_MEMELEMENT` of it compile since `m4-memory-records`, `verification\v56_g2`) | `sema\src\check\mem.rs`, `bytes_reference` |
| A reference through an element of a member array that is not its last step (`x(0).n(1).q`), or to a member array of user-type elements | The old compiler's rules for such paths were not measured | `unmeasured_path` |
| `_MEM(nx())` of a name that is no array; `_OFFSET` of an element, member or array of variable-length strings; `_MEMGET`, `_MEMPUT` and `_MEMFILL` with an element or member of them | Not measured (what `_MEM` and `_MEMPUT` do with such strings and arrays of them is: errors, `verification\v32_x15`, `x16`, `x39`) | `check\mem.rs` |

(`_INFLATE$`, `_DEFLATE$`, `_BASE64ENCODE$` and `_BASE64DECODE$`, the last row here, came in with
`m3-asserts-debug-zlib`.)

**Left marked by `m3-asserts-debug-zlib`** (the eighth step of M3, 2026-10-14: `$ASSERTS`, `_ASSERT`, `$DEBUG`,
the data-processing functions, `END n` and `SYSTEM n`, the licence file):

| Form | Why it waits | Details |
|---|---|---|

**Left marked by `m3-typefields`** (the tenth step of M3, 2026-10-15: `$UNSTABLE:TYPEFIELDS`, the member markers,
`_DYNAMIC` member arrays). What the old compiler accepts and the new one still marks "not supported yet", each not
measured:

| Form | Why it waits | Details |
|---|---|---|
| `_OFFSET(x.v())` of a whole `_DYNAMIC` member array | Not measured (its `_MEM` is, `verification\v35_x66`) | `sema\src\check\mem.rs`, `unmeasured_path` |
| `_MEMGET`, `_MEMPUT` and `_MEMFILL` with a `_DYNAMIC` member array or an element of one as the variable | Outside the change (its design, Non-Goals) | `check\mem.rs`, `bytes_reference` |

**Left marked by `m3-declare-library`** (the twelfth step of M3, 2026-10-16: `DECLARE LIBRARY` in its four forms on
Windows). What the old compiler accepts and the new one still marks "not supported yet", or does otherwise:

| Form | Why it waits | Details |
|---|---|---|
| A library FUNCTION beside a program SUB of the same name (`x22`) | The old compiler accepts both; which one a use names was not measured | `verification\v37_x22`; `sema\src\check\library.rs` |
| `ALIAS` in a `DECLARE SUB` outside a library block (`DECLARE SUB s ALIAS t ()`, `x08`) | The old compiler accepts it and ignores it; this compiler marks it "not supported yet" (user, `verification-compare`; until then the parser's error "expected the end of the statement", `m3-declare-library` design D1, task 2.2) | `verification\v37_x08`; `tests\frontend\declare_alias_marked.bas` |
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
| `$VERSIONINFO` on other systems than Windows | The old compiler writes the files on Windows only and ignores the lines elsewhere | design Non-Goals |
| `s130_versioninfo` on a machine without PowerShell | It reads the version resource back with a PowerShell script; every Windows machine and GitHub's runner have one | design, risks |

**Left marked by `m3-arraycopy`** (the eighteenth step of M3, 2026-10-17: `_ARRAYCOPY`). No form of `_ARRAYCOPY` is
marked; left out:

| Form | Why it waits | Details |
|---|---|---|

**Left marked by `m3-events`** (the nineteenth step of M3, 2026-10-17: `ON TIMER`, `ON KEY`, the switches,
`_FREETIMER`, `KEY`'s own forms). What the old compiler accepts and the new one still marks "not supported yet":

| Form | Why it waits | Details |
|---|---|---|
| A SUB of a `DECLARE LIBRARY` block as a handler | Not measured | `sema\src\check\events.rs` |

**Left marked by `m3-network-embed`** (2026-10-17: `$EMBED`, `_EMBEDDED$`, the network functions, `_SOCKETS_`,
`afterlastline.bm`). What the old compiler accepts and the new one still marks "not supported yet", and what is
left out:

| Form | Why it waits | Details |
|---|---|---|
| A `$EMBED` name with a drive and no slash after it (`$EMBED:'C:f.txt','h'`) | As for `$EXEICON` (the row of `m3-sound` above); the other relative names are looked for under the compiler root since `m4-devices-continue-paths` | `sema\src\check\embed.rs` `embed_meta` |
| `_EMBEDDED$` with its argument in parentheses (`_EMBEDDED$(("h"))`) | Not measured | `sema\src\check\embed.rs` |
| Two absolute `$EMBED` names of one file written differently (`<drive>/d/f` and `<drive>\d\f`, or with `..`): "was already embedded" in the old compiler, which resolves the folder first (`CHDIR`, `_CWD$`, `qb64pe.bas` 3765–3777); compared as written here, so both are accepted | Not measured: a recording would hold a local folder (`CLAUDE.md` rule 2); `sema` reads no folder | `sema\src\check\embed.rs` `same_file` |
| Compressing an embedded file as the old compiler does when that saves a fifth | The program sees the same bytes; only the executable's size differs (design D4) | design D4 |
| An HTTP download from a real server (`http/read_example`) | The page it downloads changed; it compiles and stays a known failure | `tools\legacy_tests\known_failures.txt` |

**Left marked by `m3-constant-evaluation`** (2026-10-18: the `CONST` evaluator in full, `$COLOR`). What the
old compiler accepts and the new one still marks "not supported yet":

| Form | Why it waits | Details |
|---|---|---|
| `_RED`, `_GREEN`, `_BLUE` of a colour from 256 to 65535 in screen mode 0 in a `CONST` (`_RED(2222, 0)`) | A text image's mask lets the index through, and libqb reads past its 256-entry palette (the old compiler gave 0 once) | `verification\v45_x64`; `sema\src\colors.rs` |
| A string argument to a colour function in a `CONST` (`_RGB32("a")`), which the evaluator's varying argument count lets through | Not measured | `sema\src\colors.rs` |
| A math function or a power of an infinite value in a `CONST` (`SIN(1F+4000 * 1F+4000)` inside an expression is the old compiler's error already; as a function argument not measured) | Not measured | `sema\src\certify.rs` |

**Left marked by `m4-devices-continue-paths`** (2026-10-20: `_CONTINUE`, blanks around `.` after a name, relative
`$EXEICON` and `$EMBED` names, device input and `ON STRIG`). What the old compiler accepts and the new one still
marks "not supported yet", and what is left out:

| Form | Why it waits | Details |
|---|---|---|
| `p . q(1)` written with blanks where no array `p.q` is declared (an array used without `DIM`) | Not measured | `sema\src\check\places.rs` `field_place` |
| `c . k` written with blanks where `c.k` is a `CONST`, and a store `f . g = 7` to the name of a FUNCTION `f.g` (the old compiler drops the blanks before it reads the statement, `qb64pe.bas` 4196–4241, so it would be the constant or the FUNCTION's result) | Not measured; found in the change's review, where such a name was read as a new variable. A call `f . g` or `f . g(4)` is built since `m4-names-implicit-arrays` (`verification\v55_x60`) | `sema\src\check\places.rs` `plain_spelled`; `tests\frontend\dot_blanks_errors.bas` |
| `ON STRIG(1 GOSUB h` and `ON TIMER(1 GOSUB h`: a keyword where `,` or `)` is expected in an argument list is "not supported yet" (the old compiler: "Expected )") | The parser's general mark for a word inside an argument list, not `ON STRIG`'s | `verification\v47_x08` |
| `C:name` in `$EXEICON` and `$EMBED` | The rows of `m3-sound` and `m3-network-embed` above | design D3 |
| A test with a game controller attached | The tests run with none; the call-site check holds every call (`event_strig_*`, `stick_*`, `strig_*`, `device*`, …) | design Non-Goals |

**Left marked by `m4-names-arrays`** (2026-10-21: a scalar and an array of one name, `REDIM name [AS type]` in a
procedure, `TYPE` blocks anywhere, `LEN` of a whole array). What the old compiler accepts and the new one still
marks "not supported yet", what it accepts and the old compiler rejects, and what is left out:

| Form | Why it waits | Details |
|---|---|---|
| An array without `AS` beside one listed by `COMMON name() AS type` (`COMMON SHARED g() AS DOUBLE: DIM g(3)`) | Not measured; the old compiler keeps a list of its own for `COMMON` (`qb64pe.bas` 9296) | design D2; `tests\frontend\names_arrays_errors.bas` |
| `_MEM(x())` of a name that is no array | The old compiler makes `x` an array used without `DIM` there, as for `LEN(x())`, which is built since `m4-names-implicit-arrays` (`LEN` 44: 11 SINGLEs); not measured for `_MEM` | `verification\v48_x04`, `x14` |
| `LEN` of a whole array of a `TYPE` with a `_DYNAMIC` member array, and of a whole `_DYNAMIC` member array | Not measured (`LEN` of such a value is an error, `v35_x20`) | `check\builtins.rs` `len_whole` |

**Left marked by `m4-builtins-param-types`** (2026-10-21: `_CAST`, `_CLIPBOARD$`, `_FILES$`, `_MD5$`, `_OS$`,
`CLEAR`, a parameter's `TYPE` above its procedure):

| Form | Why it waits | Details |
|---|---|---|

**Left marked by `m4-handlers-logging-checking`** (2026-10-21: the handler chain, the log statements and
`_LOGMINLEVEL`, `$CHECKING` anywhere):

| Form | Why it waits | Details |
|---|---|---|
| `ON ERROR GOTO foo h`: another word before the label | Rejected here as a parse error; the old compiler's message is "Expected: ON ERROR GOTO [_NEWHANDLER] label". Taking any two words would read a keyword after a label as the label | `verification\v50_x26`; design D1 of the change |
| A log statement in an included file found in the compiler root (`--include-root`) | Its entry names the file by the working folder and the name relative to the compiler root, a path that does not exist; the old compiler names it by the path as written. No program logs from such a file | Design D2 of the change (`codegen-cpp` `lib.rs`, the log folder) |

**Left marked by `m4-upstream-rejections`** (2026-10-21: the old compiler's errors for member bounds and types,
`LBOUND` of an element, `CONST` operators and names, type names by place):

| Form | Why it waits | Details |
|---|---|---|
| `LBOUND((a))`: an array's name in parentheses | Accepted by the old compiler as the array `a`; read here as a value, which it is everywhere else | `verification\v51_x34` |
| An operator where a value is expected outside a `CONST` (`x = OR`) | The old compiler: "Expected variable/value before 'OR'". The parser's mark covers forms of its grammar not parsed here, so only a `CONST` turns it into an error | `verification\v51_x50`; design D5 of the change |
| A name that is no type in an `AS` clause where no form was measured: `REDIM` of a member array, `_UNSIGNED` before a name in `SHARED`, a parameter or `_DEFINE` | Each place gives its own error in the old compiler (`v51`), so a place not measured keeps its mark | `check\decl.rs` `no_type` |

**Left marked by `m4-qb64pe-sources`** (2026-10-21: `_INSTRREV`, the whole-file functions, `_FULLPATH$`, the five
functions, the `ASC` statement, `STOP`, reserved words with a suffix, the `$` name of a `STRING * n` array or
variable):

| Form | Why it waits | Details |
|---|---|---|
| A procedure named after a reserved word with a suffix other than none and `&` (`FUNCTION exp$`) | `v52_builtin_name_suffixes` measured variables only; the old compiler rejects the form recorded. A sweep like it, of `FUNCTION` and `SUB`, would decide the rest (a `CONST` is decided: "Name already in use", `x32`, `x67`, `x68`) | `verification\v52_x28` (on `tests\verification\known.list`); design D6 of the change |
| `REDIM`, `STATIC` or `COMMON` of `a$(…)` beside a `STRING * n` array `a`, and `COMMON SHARED s$` beside a `STRING * n` variable `s` | Only `DIM` was measured for the array ("Type mismatch", `v52_x45`), and every declarer but `COMMON` for the variable (`x49`, `x69`–`x72`) | Design D7 of the change |
| `ASC((a$)) = 65`: the place in parentheses | Not measured, as for `MID$` | `check\io.rs` `asc_assign` |

**Left marked by `m4-qb64pe-builtins`** (2026-10-22: the keyboard, window, colour and system built-ins and `$RESIZE`
that `qb64pe.bas` needs):

| Form | Why it waits | Details |
|---|---|---|
| `_SETBIT`, `_RESETBIT` and `_TOGGLEBIT`, the siblings of `_READBIT` | Not used by `qb64pe.bas`, so out of the change's scope; each is an inline function of libqb's `bitops.h`, as `_READBIT` is, and would be a plain row | Design, "Non-Goals"; `tests\frontend\unsupported.bas` uses `_SETBIT` and `_TOGGLEBIT` as its examples |

**Left marked by `m4-line-numbers-whole-arrays`** (2026-10-22: line numbers, unsigned literals, a whole array as a
value):

| Form | Why it waits | Details |
|---|---|---|
| A negative integer literal beyond the `_INTEGER64` range (`-9223372036854775809`, `-18446744073709551615`) | "Overflow" here; the old compiler gives it `&&` and its C++ build decides the value (`-18446744073709551615` prints 1). No program needs it (`CLAUDE.md` working rule 12) | Design, "Non-Goals"; `verification\v54_x26` (on `tests\verification\known.list`) |

**Left marked by `m4-names-implicit-arrays`** (2026-10-23: names and implicit arrays; what `verification\v55`
found for each):

| Form | Why it waits | Details |
|---|---|---|
| `STATIC v(n)` with a bound that names a shared main-module variable or calls a FUNCTION | The old compiler reads the bound when its start code runs: at the start (a shared `n` is 0 then) and again whenever a handler is entered, when the value may differ; a FUNCTION there runs before the console is set up, and what it prints is lost | `verification\v55_x74`, `x75`; design D4 |
| In a procedure, `REDIM sd(lc(1) + 4)` of a shared array `sd` whose bound first names an array used without `DIM` | The old compiler then gives a local array `sd` its bounds and leaves the shared one as it was (in the SUB `UBOUND` 4, in the main module the old 6); why it does was not found | `verification\v24_x268` case 5, `v55_x84`; `check\places.rs` |
| The whole array `x()` of a name that is no array as a place (`x() = 1`, `READ x()`), `_MEM(x())` of such a name, a SUB written with a type suffix (`s$ 1` beside a `SUB s`), a function of the auto-included files as a place (`_ENCODEURL$(1) = "a"`), `STATIC a()` beside a shared array, `STATIC a(5, 5)` beside a shared array of one dimension, a `REDIM` bound naming an array used without `DIM` in a procedure with `STATIC` after its header | Not measured | `check\places.rs`, `check\proc.rs` |
| A statement named with one leading underscore that the table does not hold (the OpenGL statements of `SUB _GL`), and `CALL ABSOLUTE`, `CALL INTERRUPT[X]` | The old compiler reads them by its own lists (`qb64pe.bas` 10606–10687); without them a name that is no SUB would be a false error on the qbasic test programs | `check\proc.rs`, the call statement |

**Left marked by `m4-memory-records`** (2026-10-23: memory and record forms; what `verification\v56` found for
each):

| Form | Why it waits | Details |
|---|---|---|
| `FOR` over an element of a member array (`FOR r.m(i) = 1 TO 3`) | The old compiler finds the element again at every pass: an index's FUNCTION runs 7 times for 4 passes, and a change of `i` in the body moves the loop to another element | `verification\v56_f3`; design D8 |
| A `TYPE` whose member array bounds name a `CONST`, used above its block by `LEN` or a `REDIM` of the member (`DIM e AS late: PRINT LEN(e)` above `TYPE late`) | Its layout is read when pass 2 reaches the block | `tests\frontend\memory_records_errors.bas`; design D7 |
| `STRING * n` of a `TYPE` member with a `CONST` as `n` (`f AS STRING * n`) | The `TYPE` blocks are read before the `CONST`s; the old compiler takes it | `verification\v56_x30` |

Test impact: some of the 211 `tests\compile_tests\arrays` tests exercise this layer. When the new compiler is first
measured, list them in `tools\legacy_tests\known_failures.txt` (reported as `KFAIL`) until the features land.

## Strict QuickBASIC 4.5 mode

Not planned (QB64pe has no such mode). What it would take: `study\archive\08-qb45-strict-mode.md`.

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
| Below `$DYNAMIC`, an array used without `DIM` is allocated by the statement that names it first, as part of that statement: run again (in a loop) the statement raises error 10 and its store is skipped; skipped by `GOTO` or standing in an `ELSEIF` that is not reached, every later use raises 9; in a `LOOP UNTIL` line it raises 10 on every round after the first. Inside a statement of items the allocation runs just before the item or target that names it: the items before it are printed or read, the separator of `WRITE` and the prompt of `INPUT` written, then error 10 skips the rest (`READ` reads the later targets as zero). In a block's line it runs before an `IF`, `FOR`, `WHILE`, `DO` and `SELECT CASE` (a `WHILE` and a `DO` tested at the top once, in their first test), and each time an `ELSEIF`, a test at `LOOP` and a `CASE` is tested; its error 10 takes the branch of an `IF`, a `CASE` and a loop tested at the top though the test is false, leaves a `LOOP UNTIL`, and runs a `FOR`'s body with its variable as it was. A `REDIM` whose bound names the array first allocates it in the `REDIM`'s mode, so it raises nothing | Allocate it once, at program start or at the first use that runs | `m2-defxxx-common-arrays` task 5.1 (`verification\v24_x88`, `x101`, `x262`); `m4-names-implicit-arrays` design D1–D3 (`v24_x264`, `x265`, `x266`, `v55_a_implicit_headers`) |
| `READ` of a quoted item into a number is no number: `DATA "5"` read into a LONG raises a syntax error, where the unquoted item is 5 | Read the digits inside the quotes | Same (`v22_c_types`) |
| `DIM a()`, `REDIM a()` and `REDIM a AS LONG` declare the scalar `a` and no array (`a(1)` after it is another array, used without `DIM`) | An error, or the dynamic array without bounds that QuickBASIC programs mean by `DIM a()` | `m2-defxxx-common-arrays` task 1.2 (`verification\v24_b_dim_empty*`, `v24_x43`, `x44`, `x86`) |
| Whether an array is static or dynamic follows the spelling of its bounds, not their value: `DIM a(5)` and `DIM a(k)` (a `CONST`) are static, `DIM a(2 + 3)`, `DIM a(k + 1)` and `DIM a(-k)` dynamic, so the second kind is allocated only when the `DIM` runs, raises error 10 when it runs twice and takes `REDIM` | Static for every bound whose value the compiler knows, as QuickBASIC 4.5 | Task 3.2 (`verification\v24_b_constant_bounds*`, `v24_x249`) |
| A dynamic `DIM` or `REDIM` with no element in a dimension (an upper bound one below the lower bound) raises error 5 and leaves the array allocated with those bounds and no element; one element fewer ends the program with "Out of memory" (error 257), which no handler receives. A bound that raises (`DIM s(ASC(""))`) does not stop the allocation either | Error 5 for every count below 1, and nothing allocated after an error | Task 1.2 (`verification\v24_b_dynamic`, `v24_x89`–`x96`) |
| Under `OPTION BASE 1` a constant bound that is 0 but not written `0` (`0&`, `(0)`, a `CONST`) compiles to a static array 1 to 0, and the program raises error 5 when it starts, before any handler is set | A compile error ("Invalid array bounds"), as for a negative constant | Task 1.2 (`verification\v24_x71`, `x103`) |
| `SHARED` on a later `REDIM` of an array is accepted and ignored: after `REDIM ns(3)`, `REDIM SHARED ns(4)` does not make `ns` visible in procedures | A compile error, or share it from there on | Task 4.1 (`verification\v24_x248`) |
| A procedure with `STATIC` after its header keeps its variables and arrays only down to its first `STATIC` statement: below that statement new variables are made on each call and a `DIM` is a local dynamic array (QuickBASIC 4.5 keeps every variable of such a procedure; `DIVERGENCES-QB45.md` Q-010) | Keep every variable and array of the procedure, wherever a `STATIC` statement stands | `m2-defxxx-common-arrays` task 6.1 (`verification\v24_x279`, slice `s56_proc_arrays` line 21; `qb64pe.bas` 5827, 8805, 9558) |
| `STATIC c(3)` is a dynamic array allocated when the program starts: `ERASE c` releases it (the next use raises 9) and the next error that reaches a handler allocates it again, empty; `REDIM c(5)` is accepted | A static array, as a `DIM` with constant bounds in the main module: `ERASE` clears it and `REDIM` raises 10 | Same (`verification\v24_x278`, `s56_proc_arrays` line 24, the old compiler's C++) |
| `STATIC a(5)` beside a shared main-module array of that name and type names the shared array, and its allocation when the program starts is a second allocation of it: a static shared array ends the program when it starts ("Duplicate definition", before any handler; also when the SUB is never called), a dynamic one gets the `STATIC`'s bounds and the main module's `DIM` of it then raises error 10. Compiles as the old compiler's since `m4-names-implicit-arrays` | A compile error | Same (`verification\v24_x272`, `v55_d1`–`d4`; design D4 of `m4-names-implicit-arrays`) |
| `SHARED a()` below an array of that name and type that the procedure has already (a local `DIM`, or a use without `DIM`) is accepted and changes nothing | A compile error ("Name already in use") | Same (`verification\v24_x275`) |
| Signs are removed from the text of an expression before it is read: a `+` right after a `+` or a `-` (`a + +b`, `-+b`), then two `-` right after an operator, so `"a" + - -s$` joins two strings and `2 * - -i%` never negates, while `+b`, `2 * +b` and `(+b)` are errors and `s$ = - -s$` is "Cannot convert string to number". A `CONST` has another rule: a run of signs is one sign, so `CONST c = +-3` is accepted and `CONST c = 2 * - -3` is an error | A unary plus everywhere, as QuickBASIC 4.5 has (not measured there); or an error for every doubled sign | The known bug of 2026-10-11 (`verification\v25_*`, `study\00` §5) |
| An array made by passing a name that names none to an array parameter (`fill nx()`) has one dimension, 0 to 10, whatever the procedure does with the parameter; inside a SUB or FUNCTION the same call is a compile error ("Expected (...)") | An error in the main module too, or an array in both | Task 7.1 of `m2-defxxx-common-arrays` (`verification\v24_x146`, `x286`, `x292`) |
| `LBOUND` and `UBOUND` of an array listed by `STATIC a()` give it one dimension (a `DIM a(2, 5)` below is an error); of one listed by `COMMON a()` they do not | One rule for both | Task 7.2 (`verification\v24_x282`, `x297`–`x302`) |
| What stands between the parentheses of `COMMON a(3)` and of an array parameter `a(1)` is accepted and not read | A compile error for anything but `()` | Tasks 1.3 and 7.1 (`verification\v24_x157`, `x166`) |
| Nine constants of the auto-included file do not hold the value their names say: `_FLOAT_MAX` and `_FLOAT_MIN` print `INF 7693134862315D+308` and `-INF 7693134862315D+308`, `_FLOAT_MIN_FRAC` is 0; `_DOUBLE_MIN`/`_MAX` are right. Compiled QB64pe's way since `m3-constant-evaluation` (`_FLOAT_MAX` and `_FLOAT_MIN_FRAC` hold their `_FLOAT` values; `PRINT` shows them through `DOUBLE`, `_TOSTR$` gives `1.189731495357231765F+4932`). The four clamped limits (`_INTEGER64_MIN`, `_OFFSET_MIN`, `_UINTEGER64_MAX`, `_UOFFSET_MAX`) are ruled: fixed, `DIVERGENCES.md` D-027, built by `const-rulings` | `PRINT` of the `_FLOAT` limits: libqb's | `m2-console-screenhide` task 5.2 (`verification\v26_x114`), `m3-constant-evaluation` (`verification\v45_a_auto`) |
| A `CONST` that names a `$COLOR` constant above the `$COLOR` line is "Unexpected element" (the first pass meets it before the colour file is included), while a statement there that names one compiles. **Ruled 2026-10-18: kept QB64pe's way** (the program stays rejected), with a clearer message (built by `m3-constant-evaluation`), `DIVERGENCES.md` D-030 | Let the colour constants count from the program's first line, wherever the metacommand stands | `m3-constant-evaluation` (`verification\v45_x59`, `x42`) |
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
| A relative `$EXEICON` or `$EMBED` name without `./` is looked for in the compiler's own folder, not beside the program (`$EXEICON:'game.ico'` with `game.ico` beside the source is "File 'game.ico' not found"); one with a single leading slash on the compiler's drive. Here under the compiler root, its counterpart (`DECISIONS.md` 2026-10-20) | Look beside the main source file, as for a name with `./` | `verification\v29_e02`, `e03`, `e08`, `e18`, `v44_x26`, `v47_e01`, `e02`; `qb64pe.bas` 3852 |
| An odd button number of `ON STRIG` or the `STRIG` switch (`ON STRIG(1) GOSUB h`, a button's "currently down" state) compiles and raises error 5 when it runs, in libqb's `onstrig_setup` and `sub_strig` | A compile error for a constant odd number | `m4-devices-continue-paths` (`verification\v47_x13`, `x27`) |
| `$EXEICON` drops the rest of its line, a statement after a colon included (`$EXEICON:'./a.ico': PRINT 1` prints nothing), and takes any line that starts with the word (`$EXEICONS:'…'`, `$EXEICON '…'` without the colon) | Reject what is not `$EXEICON:'name'` alone on its line, as the console metacommands are | `verification\v29_e08`, `e09`, `e20`, `e24` |
| A `PLAY` with more than one string never holds the program, even for a voice in the foreground, and ends a `SOUND WAIT`; a `PLAY` string takes a tempo and an octave outside QuickBASIC's range (`T31`, `T256`, `O7`: `DIVERGENCES-QB45.md` Q-011) | Wait for the longest foreground voice; raise error 5 for the values QuickBASIC rejects | `verification\v29_g_voice`, `v29_h_sound_ranges`, `v29_c_sound_errors`; `PlayMML` in libqb's `audio.cpp` |
| A sound handle that is not open (0, negative, never opened, closed) raises no error: a statement does nothing and a function gives 0; a missing file or a sound bank that is not there is silently 0 or ignored | Error 5 or 52 for a bad handle, as QuickBASIC raises for a bad file number | `m3-sound-handles` (`verification\v39_a_handles` 5., 6., 13.; `IsHandleValid` in libqb's `audio.cpp`) |
| A sound just opened, paused or stopped has `_SNDPAUSED` -1 (a sound that never played counts as paused) | 0 until the sound has been paused | Same (`v39_a_handles` 2., 7.) |
| `_SNDLEN` of a `_SNDNEW` sound is a SINGLE widened to DOUBLE (`.1000000014901161` for 800 frames at 8000 Hz), and the length and position of a decoded sound are counted in frames of the device's rate (`.125` s reads `.1249886602163315` at 44100 Hz) | Compute in DOUBLE from the sound's own rate | Same (`v39_b_new_mem` 1., `v39_a_handles` 4.; `GetSoundDuration`, `GetSoundPosition`) |
| `_SNDPLAYFILE`'s second argument (`sync`) is read and ignored; `_SNDBAL`'s fifth (the channel) is accepted though its reference shows four, and ignored | An error for an argument that does nothing, or drop it from the forms | Same (`verification\v39_x01_accepted`; `PlaySoundFile`, `SetSoundBalance`) |
| A whole array `a()` in an expression is the array's first element in memory, with no bounds check (`a(5)` of `a(5 TO 7)`): `PRINT a()`, `a() + 1`, `-a()`, `(a())`, `ABS(a())`, `IF a() THEN`, `PRINT USING`, `_IIF`, `WRITE`, `PRINT #`, `x = a() + 1`, `VARPTR(a())`; only an assignment whose whole value is `a()` is an error | An error everywhere, as `x = a()` is | `m4-line-numbers-whole-arrays` design D4 (`verification\v54_c_whole_values`, `v54_x05`–`x34`, `x39`, `v30_x18`, `x99`, `v36_x37`) |
| A whole array as the variable of `READ`, `INPUT`, `INPUT #` and `LINE INPUT #` takes one item into its first element; `SWAP x(), y()` swaps the two first elements (of two element types: "Type mismatch") | An error; or, for `SWAP`, swap the whole arrays | Same (`v54_c_whole_values`, `v22_x49`, `x70`, `x101`, `v54_x09`, `x10`, `x19`) |
| A whole array `e()` where a string variable is expected (`FIELD #1, 4 AS e()`, `LSET e() = "x"`, `RSET`) is the array's first element | An error | Same (`verification\v30_x109`, `x153`, `v54_x14`) |
| A whole array after `ERASE` read as a value gives 0, and a store into it (`READ a()`) raises nothing: the array's data points at libqb's `nothingvalue`, 8 bytes shared by every array that is not allocated, so the store is seen by the next read of any of them. Of strings: `PRINT s()` gives an empty string, of `STRING * 20` 20 blanks, and `READ s()` stores into memory that is no array's (its text then shows at the end of each later `PRINT`, in both compilers) | Error 9, as for an element of an erased array | Same (`v54_d_whole_erased`, `v54_x42`–`x44`) |
| `ERL` is the last numbered line the program ran before the error, a numbered line of the error handler included: after a handler that starts with a numbered line, an error on an unnumbered line gives the handler's number | The last numbered line before the error in the program's text, as QuickBASIC's manual describes `ERL` | `m4-line-numbers-whole-arrays` (`verification\v54_a_line_numbers`) |
| `ERL` is the line number's text read by C++: `010&` gives 8 (an octal literal), `1.10` and `1.1` are two lines that both give 1.1, as `10.0` and `10` give 10; `08&` and a number above 18446744073709551615 fail the old compiler's C++ build, and are an error here | The number's value; `1.10` and `1.1` one line, or an error | Same (`v54_x03`, `x12`, `x25`, `x40`, `x41`) |
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
| `b%() = a%()`, a whole-array assignment with a type suffix on both names, copies element 0 of `a%` into element 0 of `b%` (converting, `a&() = b%()`) and nothing else, with no check of types or bounds; with a suffix on one name only it is an error (built so since `m4-line-numbers-whole-arrays`) | Copy the whole array, as without the suffix | `m3-whole-array-assign` (`verification\v36_x24`, `x38`, `x40`, `x43`, `x44`) |
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
| `ON KEY(k)`'s key is a `DOUBLE` that C++ cuts toward zero into libqb's `int32`: `ON KEY(0.7)` is error 5 and `ON KEY(31.6)` key 31, while `KEY(31.6) ON` rounds to 32 (error 5) and every other integer argument rounds (the user kept QB64pe's way, `DECISIONS.md` 2026-10-17) | Round as the switch does | `m3-events` (`verification\v42_d` 12.–14., 17.) |
| A SUB handler's argument is converted to `_INTEGER64` when the handler is set up, and the SUB gets a pointer to that value cast to its parameter's type: `_BYTE` given 300 sees 44, `_UNSIGNED INTEGER` given -5 sees 65531, with no overflow error; what the SUB stores into its parameter is lost (the user kept QB64pe's way, `DECISIONS.md` 2026-10-17) | Convert the argument to the parameter's type, with its overflow error, at the setup | `m3-events` (`verification\v42_c`) |
| A timer event that comes while an error handler runs is not serviced after `RESUME` either: it waits through plain statements (also a SUB's loop) until the next `TIMER ON` or the next `_DELAY`, where several come at once. libqb's: `evnt` clears `qbevent`, `events()` skips during error handling, and the timer thread sets `qbevent` only when it marks a new event (the user kept QB64pe's way for now, `DECISIONS.md` 2026-10-17: fix it when libqb is copied for another reason) | Service the held event at the first statement boundary after the handler ends. libqb's, so a change is a change to the runtime | `m3-events` (`verification\v42_e` 4.–6., `v42_f`) |
| A SUB with a `TYPE` parameter passes the old compiler's checks as an event handler, and its C++ then fails to build; this compiler rejects it with a compile error (accepted as built by the user, `DECISIONS.md` 2026-10-17) | The error of a parameter that is not an integer | `m3-events` (`verification\v42_x42`) |
| `_ERRORLINE` for an error in an included file is the main file's line of the outermost `$INCLUDE`, the same for every statement of that file and of the files it includes; only `_INCLERRORLINE` tells them apart | None needed if kept: the pair names the place. A change would be to libqb's meaning of `_ERRORLINE` | `m3-errorline-chr-const` (`verification\v43_b_include`) |
| A line joined to the next by a continuation (`_`) is numbered as the join's last line for every statement on it: `ERROR 5: x = 1 + _` reports the line below it, in `_ERRORLINE` and in the message of an untrapped error | The line each statement starts on | `m3-errorline-chr-const` (`verification\v43_d_continued`, `v43_e_continued_untrapped`) |
| `CHR$` in a `CONST` rounds its code half to even through `_UNSIGNED _INTEGER64`, so `CHR$(-0.5)` is `CHR$(0)` while `CHR$(-0.6)` is an error | Range-check the code before rounding | `m3-errorline-chr-const` (`verification\v43_x11`, `x12`) |
| `_STATUSCODE` of a negative handle that was never opened (`_STATUSCODE(-5)`) ends the program with an access violation (exit 139), where `_CONNECTED`, `_CONNECTIONADDRESS$`, `EOF`, `LOF`, `GET` and `PUT` of it raise error 52 (the user kept QB64pe's way for now, `DECISIONS.md` 2026-10-17) | Error 52, as the other functions raise. libqb's (`func__statusCode`), so a change is a change to the runtime | `m3-network-embed` (`verification\v44_x59`, `x60`–`x68`) |
| `$EMBED` takes any line that starts with the word (`$EMBEDX:'…'`, `$EMBED '…''h'` without the colon and the comma) and drops the rest of its line; two files are the same only when their names are the same text with the folder in front (`./a/../a/f` embeds `./a/f` a second time, `./d\f` embeds `./d/f`; only `./` and `.\` count as one) (the user kept QB64pe's way, `DECISIONS.md` 2026-10-17) | Reject what is not `$EMBED:'file','handle'` alone on its line; compare the files themselves | `m3-network-embed` (`verification\v44_a_embed`, `x22`, `x35`, `x39`, `x69`, `x70`) |
| A member's type must be defined in an earlier `TYPE` block ("Undefined type"), though `DIM`, `SHARED` and a SUB's body take a type defined anywhere | Take a type of any block, as `DIM` does | `m4-upstream-rejections` (`verification\v51_x21`, `x73`) |
| A member array's bound may name a `CONST` above the block only in the main module: one in a SUB above it is "TYPE member array bounds must be numeric constant", though a SUB's `CONST` is not visible there anyway and the message says nothing of scope | A message that names the scope | `m4-upstream-rejections` (`verification\v51_x13`) |
| The same misspelt type gives a different error by place: "Unknown type" in `DIM`, "Cannot convert type (…) to symbol" in `SHARED` and `_DEFINE`, "Illegal SUB/FUNCTION parameter" in a parameter, "Undefined type" in a member | One message, "Unknown type", everywhere | `m4-upstream-rejections` (`verification\v51_x20`, `x60`–`x89`) |
| `_DECODEURL$` turns a `%` whose next two characters are not hexadecimal into a byte 0 (`%zz` is one byte 0, `abc%` ends with one) (the user kept QB64pe's way, `DECISIONS.md` 2026-10-17) | Keep such a `%` as it is | `m3-network-embed` (`verification\v44_d_url`) |
| A plain name beside an array typed by `DIM … AS` is silently a scalar of another type: after `DIM a(3) AS INTEGER`, `a = 1.5` stores into a new SINGLE `a`, and `PUT #1, , a` writes its 4 bytes instead of the array | An error or a warning for a plain scalar beside a typed array of its name | `m4-names-arrays` (`verification\v48_a_scalar_array`, `v30_x67`) |
| `DIM c(3)` without `AS` beside `DIM c(2) AS DOUBLE` silently makes a second array, reachable only as `c!`: the plain `c(1)` below it is still the DOUBLE one, and `UBOUND(c)` 2; a `REDIM c(3)` gives the DOUBLE array new bounds instead. In a SUB beside a shared typed array, and beside a `STATIC c() AS DOUBLE` list nothing has reached, the new array takes the plain name | One rule for the plain name, or an error for the second declaration | `m4-names-arrays` (`verification\v48_x16`–`x24`, `v24_x126`) |
| A `TYPE` block in a SUB, a FUNCTION or a block (also one never run, `IF 0 THEN`) defines a type of the whole program, usable above its block and in every procedure; a second block of the name anywhere is "Name already in use" | Types scoped to their procedure, as a local variable is | `m4-names-arrays` (`verification\v48_c_type_anywhere`, `v48_x05`) |
| A parameter's `TYPE` must be defined above its SUB or FUNCTION ("Illegal SUB/FUNCTION parameter"), also one in the procedure's own body, though a `TYPE` block anywhere serves `DIM … AS` above it | One rule for both: any `TYPE` of the program serves a parameter too | `m4-builtins-param-types` (`verification\v49_x60`–`x65`, `v48_x12`, `x25`) |
| `CLEAR` with a parenthesis anywhere in its arguments is "Incorrect number of arguments", also inside an expression (`CLEAR , , LEN("a")`), though its arguments are ignored | Accept any numeric expression | `m4-builtins-param-types` (`verification\v49_x33`) |
| `_CAST` of a float constant outside the integer type's range gives whatever g++ folds it to, another value on each build (`_CAST(LONG, 1E20)`: `1008726880`, then `-360710944`) | A defined value (saturated, or the x86 value `-2147483648` a variable gives), with a `DIVERGENCES.md` row | `m4-builtins-param-types` (`verification\v49_x18`, design D2) |
| `_CLIPBOARD$ = s$` puts `s$` on the clipboard only up to its first NUL | Keep the whole string | `m4-builtins-param-types` (`verification\v49_d_clipboard`) |
| An error raised below `$CHECKING:OFF` waits until the next checked statement, and libqb's routines do nothing meanwhile (a `PRINT` prints nothing); the handler then runs for that checked statement, so `RESUME` runs it again, not the statement that raised the error, and `RESUME NEXT` skips it | None in the compiler if kept: it is what OFF leaves out. A change would service the error at `$CHECKING:ON` | `m4-handlers-logging-checking` (`verification\v50_c_checking`) |
| A reserved word is a variable's name with `$` (`key$`, `exp$`, `cls$`), but "Name already in use" with every other suffix (`key%`, `exp#`), and a word reserved only with `$` (`chr`) is free with every other suffix | One rule for every suffix | `m4-qb64pe-sources` (`verification\v52_builtin_name_suffixes.txt`) |
| `STOP n` checks `n` as a number and never evaluates it: a FUNCTION in it is not called, and the exit code is 0 | Evaluate it, or make it the exit code as `SYSTEM n` does | `m4-qb64pe-sources` (`verification\v52_x12`, `x15`) |
| `name$` is the `STRING * n` variable or array of the name; `DIM s$` beside such a variable declares nothing (`s$` stays the fixed-length one), while `DIM a$(3)` beside such an array is "Type mismatch" | One rule for both: `DIM` of the `$` name an error, or a variable-length string of its own | `m4-qb64pe-sources` (`verification\v52_x45`, `x49`) |
| A dotted name in a SUB or FUNCTION beside a main-module `TYPE` variable it does not share (`num.typ`) is silently a new local variable of the default type; QB64pe's own `const_eval.bas` reads `num.typ` so in `LogicalNot&`, where it is always 0 | A warning, as for a name used before it is set | `m4-qb64pe-sources` (`verification\v52_x65`, `x66`) |
| `_CONSOLE OFF` only hides the console's window when the console is the program's own; output still goes to the console while it is off (with the console a pipe, nothing changes) | Stop the output while off, as the name suggests | `m4-qb64pe-builtins` (`verification\v53_a_window`; libqb `console.cpp`, `sub__console`) |
| `_SHELLHIDE("")` raises error 5 and still gives 1 to an assignment that goes on under `RESUME NEXT` | Give 0 with the error, as the other functions that raise do | `m4-qb64pe-builtins` (`verification\v53_c_system`) |
| `_DEFAULTCOLOR(n)` and `_BACKGROUNDCOLOR(n)` of a page number the screen does not have (`8` in `SCREEN 7`) give a colour and raise no error | Error 5, as a page outside the screen's range is elsewhere | `m4-qb64pe-builtins` (`verification\v53_b_colours`) |
| `_READBIT(a, b)` with `b` outside 0 to 63 is a C++ shift by `b` (`bitops.h`), undefined behaviour: the result depends on the C++ compiler | Error 5, or 0 | `m4-qb64pe-builtins` (read in libqb; not measured, since the value is not defined) |
| `_SCREENMOVE` with nothing after it is accepted (its template's two parts are both optional) | "Syntax error", as `_SCREENMOVE 10` is | `m4-qb64pe-builtins` (`verification\v53_x41`, `x40`) |
| A `$RESIZE` line with a comment after it is "Syntax error", as for `$CONSOLE` | Accept the comment | `m4-qb64pe-builtins` (`verification\v53_x79`) |
| `ERROR 11` (division by zero) is a critical error in this libqb: it ends the program, and no `ON ERROR` handler catches it, as QB 4.5's would | A trappable error 11, as the other errors are. libqb's (`error_handle.cpp`), so a change is a change to the runtime | `m4-handlers-logging-checking` (found writing `verification\v50_a_chain`, design D1) |
| `VARSEG` of any variable, of a member of one and of an element of an array of strings is 80, the segment of libqb's `dblock`, whatever the variable | The segment where each lies | `m4-memory-records` (`verification\v56_a1`–`a3`) |
| Every `_BIT` scalar of the main module lies in conventional memory and takes 4 bytes there, which moves the offsets of the variables after it (here only when the program has another variable there) | Leave `_BIT` scalars out of `dblock` | `m4-memory-records` (`verification\v56_a3`, `x06`) |
| `VARPTR$` of a member gives the offset of its variable, not the member's; of a fixed-length string the offset of its descriptor, where `VARPTR` gives its characters' | The member's and the characters' offsets | `m4-memory-records` (`verification\v56_t05`, `b4`) |
| `GET #1, , und$()` of a name that is no array makes the array and reads into its first element as a string, where a declared array of strings is an error (and `PUT` of `und$()` too) | The error for both | `m4-memory-records` (`verification\v30_x178`, `v56_e`, `e2`) |
| `LEN` of a whole member array of variable-length strings is 8 bytes an element (the pointers), where `LEN` of a whole array of strings and of a `TYPE` that holds one is an error | The error | `m4-memory-records` (`verification\v48_x30`, `v56_g1`) |
| `STRING * n` with a float `CONST` takes it rounded half to even (2.5 is 2, 3.5 is 4), where a float literal is an error | The error | `m4-memory-records` (`verification\v56_x14`, `x29`) |
| A name alone that is an array of strings or of fixed-length strings is the implicit `SINGLE` scalar of that name in `VARPTR` and in graphics `GET` and `PUT` (`GET …, sa`), where an array of numbers or of a `TYPE` is the array | The array | `m4-memory-records` (`verification\v56_a6`, `v56_d`) |

## Other ideas raised during the study

| Idea | Source |
|---|---|
| "Skip lines" debugger feature (no Debug Adapter Protocol equivalent) | `study\06-vscode-parity.md` |
| Accept two `DECLARE LIBRARY` forms whose build fails with the old compiler: a header named by two blocks included once (it is included once per block, and a header without a guard fails in C++; was D-023), and a name looked up in every library of a block (the old compiler looks only in the last; was D-024); and two `DYNAMIC` blocks of one DLL sharing its handle (each declares it, and the C++ fails, `verification\v37_x67`). Kept the old compiler's way for now, so code that compiles here compiles with QB64pe; the unguarded header that defines something (one that only declares builds, `v37_x68`), the doubled handle and a block without a header whose last name finds no file (`v37_x69`) are rejected before the C++ with an error that says why | User, `DECISIONS.md` 2026-10-16 (`m3-declare-library` proposal) |
| A C++ build error that names a C function declared twice in `DECLARE LIBRARY` (two BASIC names, one through `ALIAS`) stands at the first declaration's uses: a call of the second on that line is not found, so the error covers the whole line and names the first BASIC name. Match every procedure of that C name against the uses on the line. No program of the suites declares a C name twice | Review of `cpp-failure-errors` (`crates\driver\src\cpp_errors.rs`, `library_proc`), 2026-10-20 |
| The CI `tier2` job runs `tests\verification\pass.list` too. Left out of `verification-compare`: the job is at its 45-minute limit already, and the list adds about a thousand C++ builds; to be decided at a merge into `main` | `verification-compare`, design "Open Questions" (`DECISIONS.md` 2026-10-19) |
| New compiler compiles the old `qb64pe.bas` as a stress test | `study\07-expert-panel.md` R10 |
| Build the compiler that the new compiler makes from `qb64pe.bas` and run it against the suite. Taken out of M4's exit criterion, which keeps `qb64pe.bas` compiling with no diagnostic: debugging a 30,000-line program nobody ships, as self-hosting and the text-mode IDE are out of scope | User, `DECISIONS.md` 2026-10-18 (tenth review `study\35` §3 item 5) |
| Redesign the runtime ABI (variables as pointers, `qbs` moving heap, `passed` bitmasks) once the old compiler is no longer a producer | `study\07-expert-panel.md` R6 |
| Fuzz the lexer and parser with a coverage-guided fuzzer: "no panic, exact round trip" is an ideal fuzz property. A seeded mutation test over the corpus comes first, in `m2-upstream-tests` (`study\archive\22` §5) | `study\archive\21-rust-review.md` §4 |
| An expression as the `INPUT` prompt (`INPUT p$; x`), a language extension; QB64pe and QB 4.5 accept only a string literal | Bug-compatibility decisions, 2026-10-08 (`study\10` §2.8) |
| The language server notices included files that change on disk while not open (`workspace/didChangeWatchedFiles`, or parsing the includers again on `didSave`); today the change shows at the includer's next edit | Review of `m2-language-server`, 2026-10-08 |
| Screen recordings of QB64pe's `tests\qbasic_testcases` (143 programs by several authors, no licence stated, so not copied; 134 read the keyboard, 46 use `RANDOMIZE TIMER`). Of the 9 that read no key, 6 end with equal states in two runs. They could be run from the clone, where they are, with only the recordings kept here | `m2-screen-state-check`, design D8, 2026-10-10 |
