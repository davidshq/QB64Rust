# Verification programs

Small programs that measure what the old compiler (`qb64pe.exe` at the reference clone's commit) does, each
recorded with its output. The facts the families up to `v45` establish are written in `study\00` §5, which takes no
new facts since 2026-10-18 (`CLAUDE.md` working rule 11): a later family's facts are its recorded output, and its
line here says what it shows. This file
is the catalogue of the families, moved here from `CLAUDE.md`'s layout table on 2026-10-16 (`study\archive\30` §2).
A new family, or new numbers in one, gets a line here, not in `CLAUDE.md`.

The new compiler is compared against these recordings by the same runner (`--suite verification` with
`qb64rust`); what it is held to, the pass list and the known differences, is in `tests\verification\` (change
`verification-compare`). A new family's programs go on one of its lists.

Small programs behind `study\09`, `study\10` and later measurements:

- `v13*`: type suffixes on DIMmed names
- `v14*`, `v15*`: procedures, scopes, reserved names, errors
- `v16*`: comment metacommands, `DATA`, line numbers, blocks, templates, `$IF`, `$INCLUDE`, member access, with include files in `v16_inc\`
- `v17*`: control flow, constants, operators
- `v18*`: arrays, `TYPE`, store rules per place
- `v19*`: procedure names (`v19_proc_names.py` and its list), blanks around a dot, `OPTION _EXPLICIT` and runtime errors in included files, with include files in `v19_inc\`
- `v20*`: core built-ins, `SELECT CASE`, `ON … GOTO/GOSUB`, `v20_x*` one rejection each
- `v21*`: the new numeric types, `_BIT`, fixed-length strings, the numeric-types spec scenarios, `v21_x*` one rejection each
- `v22*`: built-in statements (`v22_a` statement calls, `v22_b` files, `v22_c` data, `v22_d` `SWAP`, `MID$`, `RANDOMIZE`, `SHELL`, `ENVIRON$`, `v22_e` console input with `.stdin` files, `v22_f` functions added by demand, `v22_x*` one rejection or accepted form each, include files in `v22_inc\`)
- `v23_post_keys.py`: posts keys to a program's window, the method measured and not taken for the screen-state check; its programs are in `tests\corpus\screen\`
- `v24*`: step 10, measured before its design: `v24_a` default types, `v24_b` dynamic arrays and `$DYNAMIC`, `v24_c` `REDIM` and `_PRESERVE`, `v24_d` `ERASE`, `v24_e` `OPTION BASE`, `v24_f` arrays used without `DIM`, `v24_g` arrays in procedures, array parameters and `COMMON`, `v24_h` `TYPE` member arrays, `STRING` members, whole values and `TYPE` parameters, `v24_i` the type before the names, `v24_j` a minus right of `^`, `v24_x*` one accepted or rejected form each (`x244`–`x255`: measured while group 4 was built and reviewed; `x256`–`x263`: before group 5, arrays used without `DIM`; `x264`–`x268`: by the review of group 5, where their allocation runs inside a statement; `x269`–`x282`: while group 6 was built, arrays in procedures, `STATIC` arrays and procedures, `SHARED a()`; `x283`–`x285`: by the review of group 6, `DIM` in a procedure with `STATIC` after its header and after `STATIC c(3)`; `x286`–`x296`: before group 7, array parameters; `x297`–`x302`: while group 7 was built, what gives an array listed by `COMMON a()` or `STATIC a()` its number of dimensions; `x303`–`x305`: by the review of group 7, what may stand between an array parameter's parentheses; `x306`–`x308`: before group 8, the order of a whole-value copy and more forms of a user-type parameter; `x309`: by the review of group 8, `STRING` members in arrays, parameters and procedures; `x310`–`x326`: before and while group 9 was built, member arrays: what finds the place of one, the order of a store through a member, whole values, arguments, and ten forms settled one program each)
- `v25*`: the signs the old compiler drops from an expression (`v25_signs`, `v25_signs_const`, `v25_x*` one rejection each)
- `v26*`: step 11, measured before its design: `$CONSOLE`, `$SCREENHIDE`, `_DEST _CONSOLE`, `_ECHO`, the flag `_CONSOLE_` (`v26_a`–`v26_h2`, `v26_x*` one accepted or rejected form each; `x40`–`x56`: measured as the step's first task group, the line rules of every metacommand and more on the flags, include files in `v26_inc\`; `x57`, `x58`: while its group 2 was built, a colon in a branch that is not taken, a tab after a metacommand; `x59`–`x61`: by the review of group 2, a tab or a word after `$INCLUDEONCE`, a letter after `$CONSOLE:ONLY`; `x62`–`x73`: while its group 3 was built, more forms of `_DEST`, `_SOURCE`, `_CONSOLE` and `_ECHO`, accepted and rejected; `x74`–`x113`: while its group 4 was built, the window built-ins, one accepted program and one rejected form each; `x114`–`x126`: while its group 5 was built, the constants of the auto-included file; `x127`: while its group 6 was built, precompiler flags across an included file; `x128`: by the review of groups 4 to 6, `SWAP` after `CALL`)
- `v27*`: the first step of M3, a program with a window and the screen statements: `v27_a` `INKEY$`, `CSRLIN` and `POS` in a console program, `v27_x*` one accepted or rejected form each (`x01`–`x50`: measured before its design, the forms of every screen statement and function, and after `CALL`; `x51`–`x57`: while its group 3 was built, a statement's name, parentheses and `=`; `x58`–`x63`: while its group 5 was built, a statement's name and a colon at the start of a line; `x64`: by the review of group 4, `PALETTE USING`)
- `v28*`: the second step of M3, measured before its design: `v28_a` `TAB` and `SPC`, `v28_b` `DEF SEG`, `VARSEG`, `VARPTR`, `PEEK`, `POKE`, `BSAVE` and `BLOAD`, `v28_c` a line that is a SUB's name and a colon, `v28_x*` one accepted or rejected form each (`x71`–`x81`: labels with the name of a SUB or FUNCTION), and as the step's first task group `v28_d` the index inside `VARPTR` and `VARSEG`, `v28_e` arrays in conventional memory used as any array is, `v28_f` one that is too large, `x82`–`x92` which statements make a SUB's name a label, `x93`–`x95` `GET` and `PUT` with a parenthesised first argument, `VARPTR` in `HEX$`
- `v29*`: the third step of M3, sound and `$EXEICON`, measured before its design: `v29_a` what `PLAY(n)` gives for queued sound, `v29_b` which statements hold the program, `v29_c` the run-time errors, `v29_x*` one accepted or rejected form of `SOUND`, `PLAY` and `BEEP` each (numbers 33–39 not used), `v29_e*` one form of `$EXEICON` each, files in `v29_inc\`, and as the step's first task group `v29_d` volume 0, `v29_f` the queue read as a slice program reads it, `v29_g` voices, `v29_h` the ranges left open, `e29`–`e31` a second `$EXEICON` across an included file and a `$IF` branch; `e32`: by the review of the change, two slashes after the dot of an `$EXEICON` name; `x87`–`x98`: while its group 2 was built, a statement with a template after `CALL`
- `v30*`: the fourth step of M3, the console programs of the corpus that still wait, measured before its design: `v30_a`–`v30_c` `PRINT USING` (numbers, strings and several values, to a file), `v30_d` `GET` and `PUT` on a binary file, `v30_e` on a random file, `v30_f` `EXIT SELECT`, `v30_g` `_IIF`, `_MIN`, `_MAX`, `_CLAMP`, `v30_h` `_SHL`, `_SHR`, `_ROL`, `_ROR`, `v30_i` `LOCK` and `UNLOCK`, `v30_x*` one accepted or rejected form each (also of `CHAIN` and `LPRINT`; numbers 49, 57–59, 68 and 69 not used; `x66`: with task 3.1 of the change, `EXIT CASE` before a `SELECT` in the same `CASE`; `x67`: with its task 5.1, an array's name alone as the variable of `PUT`; `x171`–`x178`: by the review of the change, a FUNCTION's name with empty parentheses and an array that is not declared in `GET`, `PUT`, `FIELD`, `LSET` and `RSET`), and as the step's first task group `v30_j` `FIELD`, `v30_k` its run-time errors and those of `GET` and `PUT` with no variable, `v30_l` `LSET` and `RSET`, `x100`–`x132` one form of `FIELD` each, `x140`–`x170` one form of `LSET` or `RSET` each (numbers 133–139 not used), `v30_result_types.py` and its table `v30_result_types.txt`: the type of the result of `_MIN`, `_IIF` and `_CLAMP` for every pair of numeric types and of `_ROL` and `_SHL` for every type, read from the old compiler's C++)
- `v31*`: the sixth step of M3, the image, colour, font and mouse built-ins, measured before its code: `v31_a` the colour functions, `v31_b` image handles, sizes, the clear colour, a palette entry, blending, `PMAP` and `_AUTODISPLAY`, `v31_c` images saved in six formats and loaded again, `v31_d` fonts built in and loaded from Windows, `_UCHARPOS`, `_PRINTMODE`, `_MAPUNICODE`, `v31_x01` every accepted form as code that is not reached, `v31_x02`–`x109` one rejected form each)
- `v32*`: the seventh step of M3, `_MEM`, measured before its code: `v32_a` the members of the block of every kind of place, `v32_b` reading, writing, filling and copying through a block by each type, an image, when a block stops being valid, `v32_c` `MKI$` … `_CV`, `v32_d01`–`d34` one runtime error or fact about a block's life each (each error 300 to 313, `d33` a double free in libqb, `d34` the lock of the body where `_MEM` stands), `v32_x01` every accepted form as code that is not reached, `v32_x02`–`x116` one other form each (all rejected but `x80`, whose C++ fails to build), `x117`–`x120`: by the review of the change, `READ` and `INPUT #` into a member of a block, `_MEMELEMENT` of members of an element, `_UNSIGNED STRING * n`)
- `v33*`: the eighth step of M3, measured before its code: `v33_a*` `$ASSERTS` and `_ASSERT` (on, off, a message, a console, where the metacommand stands, an included file in `v33_inc\`), `v33_b*` the checksum, compression and base64 functions, `v33_c*` the exit code of `SYSTEM n`, `v33_d*` what `$DEBUG` changes outside the IDE, `v33_x01` every accepted form as code that is not reached, `v33_x02`–`x55` one other form each)
- `v34*`: `REDIM _RETAIN`, measured before its change: `v34_a` numbers, `v34_b` the other kinds of element, `v34_c` the places of an array, `v34_d` a FUNCTION in a bound that gives the same array new bounds (`v34_d2` with plain `REDIM`, `v34_d3` with `_PRESERVE`, recorded as the change closed), `v34_x01`–`x24` one accepted or rejected form each)
- `v35*`: `$UNSTABLE:TYPEFIELDS` and the member markers, measured before its change: `v35_a` a `_DYNAMIC` member of a variable, `v35_b` of array elements and the parent's `REDIM`, `ERASE`, `SWAP` and copy, `v35_c` every kind of element, `v35_d` procedures, `v35_e` statements and whole-array files, `v35_f` `_STATIC` and unmarked members beside the flag, `v35_g`–`g3` runtime errors, `v35_h` the metacommand, `v35_x01`–`x59` one accepted or rejected form each, `x60`: by the review of group 2, a marker before an `AS type` list; `x61`: with task 4.3, the evaluations of a copy of nested owner types and a source member that is not allocated; `x62`: with task 4.2, `READ`, `INPUT`, `LINE INPUT` and `MID$` into a place through a member, with a `.stdin`; `x63`–`x66`: with group 5, `COMMON` of an array and `LEN` of an element of an owner type, files of nested owner types, `_MEM` of a `_DYNAMIC` member; `x67`: while `s118` was recorded, `GET` into a released array; `x68`: with task 6.2, `_OFFSET` of an owner value)
- `v36*`: whole-array assignment, measured before its change: `v36_a` numbers, `v36_b` the other kinds of element, `v36_c` the places of an array, `v36_d` member arrays, `v36_e` owner types, `v36_f` the run-time errors, `v36_x01`–`x52` one accepted or rejected form each, `x53`–`x55`: by the review of the change, the value in parentheses and a SUB as the source)
- `v37*`: `DECLARE LIBRARY`, measured before its change: `v37_a` no library name, `v37_b` every numeric type by value and by reference, `v37_c` what each kind of argument passes, `v37_d` strings, `v37_e` a TYPE and `ALIAS` of C++ names, `v37_f` a `.a`, `STATIC`, a `.o` and a C++ `.a`, `v37_g` `DYNAMIC`, `v37_h` `CUSTOMTYPE`, `v37_i` where a block may stand, `v37_j` error 260, `v37_x01`–`x55` one accepted or rejected form each (`x56`–`x66`: by the review of the proposal, which `BYVAL` 0 is `NULL`, the default types told apart, a `STRING * n` member's missing NUL; `x67`, by task 4.4: two `DYNAMIC` blocks of one DLL; `x68`, `x69`, by the review of group 4: a header without a guard that only declares named twice, a block whose last name is a folder only; `x70`, a library beside the program in a folder whose name holds a `$`, recorded from such a folder with `--verification-root`, not from here: in `verification\` it compiles); the C sources and headers in `v37_lib\` with `build_libs.py`, which builds the libraries into `verification\` (git-ignored; run it before recording), an included file in `v37_inc\`
- `v38*`: what types with `_DYNAMIC` member arrays copy after an error, measured before `m3-measured-copies`: `v38_a` a `GET` of a whole array whose read raised an error, `v38_b` `PUT` and `GET` with a member of other bounds or another number of dimensions, `v38_c` a whole-array copy of a member of another number of dimensions, `v38_d` a whole-array copy of arrays of such a type with other bounds
- `v39*`: sound handles, measured before the code of `m3-sound-handles`: `v39_a` handles and their state (open, length, position, copy, play, pause, stop, loop, limit, a handle that is not open, a raw handle), `v39_b` `_SNDNEW` and `_MEMSOUND` (a new sound, one decoded from memory, the arguments refused), `v39_c` a WAV file the program writes, `_SNDPLAYFILE`, `_SNDPLAYCOPY`, raw sound, `_WAVE` and MIDI from memory (each builds its WAV and MIDI data in a string), `v39_x01` every accepted form as code that is not reached, `v39_x02`–`x72` one other form each (the arrays of `_WAVE` and `_SNDRAWBATCH` among them), `x73`–`x81` one form of `$MIDISOUNDFONT` each
- `v40*`: `$VERSIONINFO`, measured before the proposal of `m3-versioninfo`: `v40_a`–`v40_l` accepted forms (every key, numeric keys alone and in order, letter case, quotes and blanks, a repeated key, with `$EXEICON`, the places of a line, the `$VERSIONINFOX` prefix), `v40_g*` special characters (those whose executable does not start among them), `v40_h*` numeric parts (those whose build fails among them), `v40_x01`–`x22` one rejected form each (`x20` accepted); each accepted program prints its own version resource with `v40_inc\show_version.ps1`. `v40_rc.py` records the files the old compiler writes for the resource compiler into `v40_rc\` (`icon.rc`, `manifest.h`, the manifest, and the console output of `-z -w`), which the runner does not
- `v41*`: `_ARRAYCOPY`, measured before the proposal of `m3-arraycopy`: `v41_a` the four forms of one dimension, overlap and an array copied to itself, `v41_b` every element type, `v41_c` several dimensions, numbers and strings, `v41_d` member arrays of a plain `TYPE` and the order of their indexes, `v41_e` `_DYNAMIC` member arrays, `v41_f` the places of an array, `v41_g` the run-time errors, `v41_h` the order of evaluation and the rounding of bounds, `v41_x01`–`x76` one accepted or rejected form each; while the proposal was written: `v41_i` `_FLOAT` arrays, `v41_j` an index outside its bounds on the way to a member array, `v41_k` `STRING` member arrays of a plain `TYPE`; after the change's review: `v41_l` `_DYNAMIC` member arrays that a `REDIM` gave another number of dimensions
- `v42*`: event trapping, measured as the first task of `m3-events`: `v42_a` timer 0 (off, on, `STOP`, `OFF`, during `_DELAY` and a loop, a slow handler), `v42_b` numbered timers, `_FREETIMER`, `FREE` and the run-time errors, `v42_c` SUB handlers and their argument, a handler set up in a SUB, `v42_d` `KEY` in a console program (its own forms, the switches, the errors, a float key number), `v42_e` when an event is serviced (`SLEEP`, a SUB, a FUNCTION, an error handler), `v42_f` an event that comes during an error handler, `v42_x01`–`x60` one accepted or rejected form each
- `v43*`: `CHR$` in a `CONST` and the error functions, measured before the proposal of `m3-errorline-chr-const`: `v43_a` `_ERRORLINE`, `_INCLERRORLINE`, `_INCLERRORFILE$` and `_ERRORMESSAGE$` in the main file, `v43_b` in included files (`v43_inc\`), `v43_c` the accepted forms of `CHR$` in a `CONST`, `v43_d` lines joined by a continuation, `v43_e` the message of an untrapped error on one, `v43_x01`–`x28` one form of `CHR$` in a `CONST` each, `x29`–`x44` one form of the error functions each, `x45`, `x46` a FUNCTION in a single-line `IF`'s condition, on one line and continued
- `v44*`: `$EMBED`, `_EMBEDDED$`, the network functions and `afterlastline.bm`, measured before the proposal of `m3-network-embed`: `v44_a` the accepted forms of `$EMBED` and `_EMBEDDED$` (every byte value, an empty file, 300,000 bytes, blanks, `.\`, `..`), `v44_b` the network functions where nothing listens, `v44_c` `_SOCKETS_` from `_OPENCLIENT` in a SUB never called, `v44_d` `_ENCODEURL$` and `_DECODEURL$`, `v44_e` (with the change's task 1.2) a TCP/IP exchange over the loopback address on port 47391 (it listens: Windows may ask once whether it may), `v44_x01`–`x68` one accepted or rejected form each (`x66` not used; `x59` ends with an access violation; `x69`–`x71`: by the review of the change, a file as `./` and as `.\`, another separator after `./`, a string for a number parameter of a user FUNCTION); the files they embed in `v44_files\` (byte for byte, `.gitattributes`), an included file in `v44_inc\`
- `v45*`: the `CONST` evaluator in full and `$COLOR`, for `m3-constant-evaluation`: `v45_a` the measurements
  before its proposal (precision, typing by text, the lanes, the clamp, the auto-included limits), and as its task
  1.2 `v45_b_conv` floats in the integer lanes, `v45_b_funcs` every function as a `_FLOAT` constant,
  `v45_b_funcs_plain` some without a suffix, `v45_b_colors32` and `v45_b_colors_m<mode>` the colour functions per
  screen mode, `v45_b_asc_suffix` `ASC` and suffixed constants inside a `CONST`, `v45_b_color0` and
  `v45_b_color32` the colour constants, `v45_x01`–`x66` one accepted or rejected form each (functions, colours,
  `$COLOR`), an included file in `v45_inc\`; as its task 1.3 `v45_sample` the library results behind the
  certification of the functions
- `v46*`: Civil War Strategy as a whole game (`cws-whole-game`): `v46_a_play_v0` the game's own `PLAY` strings and
  `SOUND`s after one `PLAY "V0"`, the edit of its copy: the foreground ones take their time, the background ones
  queue their length and play out in it, as `v29_d` found for plain strings. That the volume holds for later
  statements (no game string holds a `V`, no `SOUND` gives a volume) and nothing is heard is read in libqb's
  `audio.cpp`: one amplitude per voice, set only by `V` and `SOUND`'s third argument
- `v47*`: the four parts of `m4-devices-continue-paths`, measured as its task 1.1: `v47_a_continue` `_CONTINUE` in
  every kind of loop (through `IF` and `SELECT`, the innermost loop, in a SUB and a FUNCTION, the tests of `LOOP
  WHILE`/`UNTIL` still run), `v47_b_devices` the device functions, `STICK` and `STRIG` with no game controller (the
  keyboard and the mouse are devices 1 and 2 with libqb's fixed counts; the error 5 cases; nothing that counts
  controllers, `CLAUDE.md` working rule 12), `v47_c_dot_blanks` blanks around a dot after a plain name (a member
  array's element and bounds, a by-reference argument, nested members, plain dotted names), `v47_e01`, `e02` a
  relative `$EXEICON` and `$EMBED` name found under the compiler's folder (each names its file in a `.root`
  sidecar, which the runner puts there for the compile, `tools\legacy_tests\README.md`; the files in
  `v47_files\`), `v47_x01`–`x31` one accepted or rejected form each (`_CONTINUE` outside a loop, with an
  argument, as a variable; `ON STRIG`'s operand and handler errors; an odd button number of `ON STRIG` or the
  switch, error 5 at run time in libqb, `x13`, `x27`; the device functions' argument errors; `DIM a . b` and
  `SUB a . b`, both accepted; `s$ . x`, rejected)
- `v48*`: the four parts of `m4-names-arrays`, measured as its task 1.1: `v48_a_scalar_array` a scalar and an
  array of one name, one typed by `AS` (the plain scalar is of the default type; between two arrays the one typed
  by `AS` keeps the plain name; in a SUB, beside a shared one, `PUT` and `GET`), `v48_b_redim_scalar` `REDIM name
  [AS type]` in a SUB, a FUNCTION and a `SUB … STATIC` (a second run keeps the value), `v48_c_type_anywhere`
  `TYPE` blocks in a SUB, a FUNCTION, an `IF`, a `FOR` and a `SELECT CASE`, each a type of the whole program,
  `v48_d_len_whole` `LEN` of a whole array and a whole member array (0 after `ERASE`), `v48_x01`–`x26` one
  accepted or rejected form each (the `LEN` errors; `_MEM` of an array of a `TYPE` with a `STRING` member, no
  error; a second `TYPE` block of a name; a parameter typed by a block below its SUB, rejected; `x16`–`x24` which
  array a plain name names, after `qb64pe.bas` 9260: a list `STATIC c() AS type` makes no array until reached);
  `x27`–`x30` from the change's review: a `TYPE` block in a SUB bounds its `_DYNAMIC` member array by the SUB's
  own `CONST`, also where the main module has one of the name; `LEN` of a whole array with a dotted name; `LEN` of
  a whole member array of variable-length strings, no error (32 for four elements)
- `v49*`: the built-ins and the parameter types of `m4-builtins-param-types`, measured as its task 1.1:
  `v49_a_cast` `_CAST` to every numeric type (a float truncated toward zero; the result seen as the type named),
  `v49_b_files` `_FILES$` of a folder, a pattern and no match (a folder's name ends in a backslash; a bare call with
  no listing is error 5), `v49_c_md5_os` `_MD5$` and `_OS$`, `v49_d_clipboard` `_CLIPBOARD$` (the text up to its
  first NUL), `v49_e_clear` what `CLEAR` resets (every kind of variable, `STATIC`, `DATA`, files, dynamic arrays
  released) and `v49_e2_clear_gosub` `RETURN` after it (error 3), `v49_f_param_type` a parameter typed by a
  `TYPE` above (in a SUB, after a `DECLARE`, in an included file in `v49_inc\`), `v49_x01`–`x66` one accepted or
  rejected form each (`x18`: a float constant out of range in `_CAST`, undefined in C++, a different value on each
  build; `x30`–`x38` `CLEAR`'s forms, `CALL CLEAR` accepted; `x60`–`x66` where a parameter's `TYPE` may stand)
- `v50*`: the handler chain, the log statements and `$CHECKING` of `m4-handlers-logging-checking`, measured as its
  task 1.2: `v50_a_chain` `ON ERROR GOTO _NEWHANDLER` and `_LASTHANDLER` (three deep and back, nothing kept, after
  `ON ERROR GOTO 0`, a plain label between, in a SUB and a FUNCTION; error 11 is left out, a critical error no
  handler catches), `v50_b_log` the four log statements and `_LOGMINLEVEL` (2 with the runner's log file), its
  entries in `v50_b_log.log.txt` (the file as a full path, an included one as its includer's folder and the path
  as written; the procedure's C name; a message cut at 199 bytes; include files in `v50_inc\`), `v50_c_checking`
  an error raised below `$CHECKING:OFF` (nothing of libqb runs while it is pending; it is serviced at the next
  checked statement, and `RESUME` runs that statement again), a SUB and an included file below OFF, an `ON
  TIMER` that comes due there, `v50_d_mem` `_MEMGET`, `_MEMPUT` and `_MEMFILL` below OFF (`_FLOAT` left out: the
  old compiler's build crashed on it with checking on too), `v50_e_index` array, member-array and dynamic
  member-array elements below OFF, `v50_x01`–`x28` one accepted or rejected form each (`x12` `$CHECKING:OFF` in an
  `IF` block accepted, `x13` a line number after `_NEWHANDLER`, `x16` `_LOGINFO ("a")` with its log; `x27`, `x28`
  a label with a leading `_` after `_NEWHANDLER`: not defined, and "Invalid name" where it is defined, added on the
  change's review)
- `v51*`: the forms next to the upstream programs that `m4-upstream-rejections` turned into errors, measured as its
  task 1.1, one accepted or rejected form each: `x01`–`x15` a member array's bound (a variable, an unknown name, a
  `CONST` below the block or in a SUB, `LEN`, an element, a FUNCTION: rejected; a `CONST` above with a suffix,
  `_PI`, `ABS(…)`: accepted), `x20`–`x27` a member's type (a later block's type, near words, `* n`: "Undefined
  type"; an `AS type` list its own error), `x28`–`x39` `LBOUND` and `UBOUND` of an element, a value and an
  expression (`LBOUND((a))` and `LBOUND(n)` of a scalar accepted), `x40`–`x54` operators and names in a `CONST`
  (`x50`: `x = OR` outside one), `x60`–`x89` type names by place (`DIM`, `REDIM`, `STATIC`, `COMMON`, `SHARED`,
  parameters, `DECLARE LIBRARY`, `_DEFINE`; `STRING` with a word glued to it; `* n` after other words;
  `_UNSIGNED`), `x90` a `TYPE` named `StringList` (a name that starts with `STRING`) in `DIM`, `DIM SHARED`,
  `REDIM`, `STATIC` and a parameter: accepted; numbers 16–19, 55–59 not used
- `v52*`: what the ten `qb64pe/*` upstream programs of `m4-qb64pe-sources` need, measured as its tasks 1.1 and 1.2:
  `v52_a_instrrev` `_INSTRREV` with and without a start (an empty string or search string, a start of 0, below 0,
  beyond the length, of other types), `v52_b_math` `_ARCSEC`, `_ARCCSC`, `_ARCCOT`, `_SECH`, `_CSCH` of each type
  and at the edges of their domains (outside one: error 5 and 0), `v52_c_files` `_READFILE$`, `_WRITEFILE` and
  `_FULLPATH$` (errors 53 and 76; `_FULLPATH$(".")` is not `_CWD$ + "\"`), `v52_d_asc` the `ASC` statement
  (positions, an empty string: error 5, values converted to LONG and cut to a byte, every kind of string place,
  the place, position and value evaluated in that order); `x01`–`x09` the `ASC` statement's rejected forms,
  `x10`–`x19` `STOP` (ends as `SYSTEM` without a prompt, in a window too; its expression checked as a number, not
  evaluated), `x20`–`x37` `key$`, `exp$`, `base$` in each place (`FUNCTION exp$` and `CONST key$` rejected, `DIM
  key$ AS STRING` its own error), `x39`–`x49` the `$` name of a `STRING * n` array or variable (the same array or
  variable; `DIM a$(3)` beside the array "Type mismatch", `DIM s$` beside the variable the same variable),
  `x50`–`x64` the new built-ins' rejected forms, `x65`, `x66` a dotted name in a procedure beside a main-module
  `TYPE` variable it does not share (a variable of its own), `x67`, `x68` `CONST exp$` and `CONST key!` ("Name
  already in use", as `x32`), `x69`–`x72` the other declarers of `s$` beside a `STRING * n` variable `s` (`REDIM
  s$` and, in a SUB, `STATIC s$` the same variable; `SHARED s$` in a SUB a variable-length one of its own; `STATIC`
  in the main module rejected), `x73`, `x74` `DIM` and `STATIC` mixed in a SUB (`s$` the fixed-length variable
  either way); `v52_builtin_name_suffixes.py` and its `.txt`, the sibling of
  `v15_builtin_names`: every keyword and built-in name with each other suffix, `variable`, `statement` or
  `rejected`; numbers 38 not used
- `v53*`: the built-ins and `$RESIZE` that `qb64pe.bas` needs, measured as task 2.1 of `m4-qb64pe-builtins`:
  `v53_a_window` (`$CONSOLE`) `_CONSOLE OFF` and `ON` with the console a pipe (output goes on while off), `_EXIT`
  with no request (0), each form of the statement `_RESIZE`, the drop built-ins with nothing dropped
  (`_DROPPEDFILE` with an index of 0 or below: empty, no error); `v53_b_colours` `_DEFAULTCOLOR` and
  `_BACKGROUNDCOLOR` after `COLOR` in each kind of screen, of images of each depth, of pages (one that does not
  exist: no error), of a handle of no image or a freed one (error 258, the value 0); `v53_c_system` `_DIR$` of each
  name (the last character, whether it exists, whether it is the desktop's; never the path; the same on any
  Windows machine: libqb asks for each folder with `CSIDL_FLAG_CREATE` and falls back to the user's profile, so
  each exists, `filesystem.cpp` `FS_GetKnownDirectory`), `_SHELLHIDE` exit
  codes (an empty command: error 5 and the value 1), `_COMMANDCOUNT`, the shapes of `_COMPILEDATE$` and
  `_COMPILETIME$`, `_READBIT` of each type; `v53_d_keys` `_KEYHIT`, `_KEYDOWN` (0 and below: error 5) and
  `_KEYCLEAR` (0, 4, -1: error 5) with no key; `x01`–`x64` one wrong form each of the new built-ins (all rejected
  but `_MESSAGEBOX` and `_SELECTFOLDERDIALOG$` with no argument, `_MESSAGEBOX` statement and function with slots
  left out, and `_SCREENMOVE` alone), `x70`–`x83` the forms of `$RESIZE` (any case, in `$CONSOLE:ONLY`, in a SUB,
  two of them: accepted; alone, another word, a blank or a tab by the colon, anything after: "Syntax error"; an
  equals sign for the colon: "Invalid expression"); numbers
  65–69 not used. The programs with a window and keys are screen programs of the corpus (`k02`–`k04`,
  `c15`–`c17`, `tests\corpus\screen\SOURCE.md`)
- `v54*`: line numbers, unsigned literals and a whole array as a value, measured as task 1.1 of
  `m4-line-numbers-whole-arrays`: `v54_a_line_numbers` `ERL` before any numbered line (0), after one, after an
  unnumbered one (the last numbered line run, a numbered handler's too), for `10.5`, `20&`, `4294967296`, in a SUB
  and after it, in an included file (`v54_inc\`), after `RESUME n`; `GOSUB` and `RETURN n`, `ON n GOSUB`, `ON ERROR
  GOTO n` with numbers; `v54_b_unsigned_literals` an unsuffixed literal above 9223372036854775807 printed, in an
  operation (`18446744073709551614 + 1` is -1), stored into `~&&`, `&&`, `DOUBLE`, in `HEX$` and `STR$` (in a
  `CONST`, `x38`, it prints 9223372036854775807, D-027); `v54_c_whole_values` the element a whole array stands for (the first in
  memory, `a(5)` of `a(5 TO 7)`), and `READ`, `INPUT #` and `SWAP` of whole arrays (one item, one element);
  `v54_d_whole_erased` an erased array (0, no error); `x01`–`x38` one form each: the spellings of a line number
  (`010` is `10`, a duplicate; `10.0`, `1.10`, `30#` their own; `010&` gives `ERL` 8, C++'s octal; 41 digits
  "Invalid label!"), a number of another body and `RETURN n` in a SUB (errors), `IF c GOTO n`, `a()` in any
  expression (accepted) and the forms rejected (an assignment's whole value, `SWAP` of two types, a member array,
  a scalar parameter, `LEN` of strings), `-18446744073709551615` (1); `x39`–`x45`, with the review of the change:
  `VARPTR(a())` (the first element's), `08&` and a number of 25 digits (the C++ build fails), an erased array of
  `STRING` printed (empty) and read into (the text shows after later output), of `STRING * 20` printed (blanks),
  `t.m&() = u.m&()` ("Expected array index")
- `v55*`: names and implicit arrays, measured as task 1.1 of `m4-names-implicit-arrays`:
  `v55_a_implicit_headers` below `$DYNAMIC`, an array first named in the line of each kind of block, run twice
  under `RESUME NEXT` (the second allocation raises error 10: the `IF`, a `CASE` and a loop tested at the top run
  their body though the test is false, `LOOP UNTIL` leaves, an `ELSEIF` skips its branch's first statement, a
  `FOR` runs its body with the variable as it was; a `CASE` after one that matched allocates nothing);
  `v55_b_implicit_whole`, `v55_b2_implicit_whole_base1` the whole array `x()` of a name that is no array (`LEN`
  44 for `SINGLE`, 40 under `OPTION BASE 1`; in a SUB 0 and no error); `v55_c_static_bounds` `STATIC` bounds that
  are expressions of constants (taken with their value); `v55_d1`–`d4` `STATIC a(5)` beside a shared array (a
  dynamic one is allocated by it, error 10 at the main `DIM`; one of another type is another array; "Duplicate
  definition" also when the SUB is never called); `x01`–`x75` one form each: built-in names as a place or a target
  (all rejected, "Expected variable =", "Expected variable" or the statement's own error), statements with a
  suffix, names that are no SUB as a statement, after `CALL` or `THEN`, functions as statements ("Syntax error",
  "Expected CALL sub-name"), jumps to built-in names ("Invalid label!", "Invalid label") and lines named like a
  built-in function ("Syntax error"), FUNCTIONs named like built-ins with a suffix ("Name already in use"; `chr&`
  accepted), one leading underscore ("Invalid array name", "Invalid name"; `_CONNECTIONADDRESS$` and `__foo(1)`
  accepted), declared names with blanks around a dot (accepted; beside a `TYPE` variable "DIM: Expected ,"),
  `WIDTH (3) = 5` (an array), `x()` then `x(1, 2)` and `LEN(xs$())` (rejected), `STATIC` bounds of `LEN("abc")`
  (3), a shared variable (its value when the start code runs, 0) and a FUNCTION (what it prints is lost); by the
  change's review, `x85` the whole array `x()` in an index of a store and of a `READ` target (a value: `LEN` 44 and
  88)
- `v56*`: memory and record forms, measured as task 1.1 of `m4-memory-records`: `v56_a1`–`a7` the offsets
  `VARPTR` gives in conventional memory (taken from 65520 down, in the order of declaration, no alignment;
  `COMMON SHARED` first; every `_BIT` scalar takes 4 bytes there, named or not; a string's descriptor 4 bytes, a
  fixed-length string its bytes then a descriptor; a static array of strings a descriptor per element, last first,
  a dynamic one when `REDIM` runs; arrays of numbers, fixed-length strings and a `TYPE` elsewhere), a string's
  descriptor through every kind of assignment (its `VARPTR` never changes), `PEEK` and `POKE` through them, a
  whole array of strings, fixed-length strings or a `TYPE` named without an index (the implicit `SINGLE` scalar);
  `a5` a `TYPE` with a `STRING` member in conventional memory (its strings are never made: the program crashes,
  exit 139); `b1`–`b5` `VARPTR$` (a type byte, then the offset; of a fixed-length string the descriptor's offset)
  and `SADD`; `c` `HEX$`, `OCT$`, `_BIN$` of `VARPTR`, `VARSEG` and `_SCREENEXISTS` (width 0: `HEX$` of -1 is
  empty); `d` graphics `GET`/`PUT` of a plain variable, a string (its characters), a member (from it to the end of
  the variable), an undeclared name, an element of an array of strings (its characters) and of fixed-length
  strings or a `TYPE` (to the array's end), error 5 when too small; `e`, `e2` file `GET` of `und()`, `unl&()`,
  `und$()` (element 0, as a string) and `_UCHARPOS` of `nd&()`, `PUT` of `uns$()` rejected; `f1`–`f4` `FOR` over
  a member (`r.v`, `ro.n.v`; `r.m(i)` evaluates its index at every pass); `g1`, `g2` `LEN` of a whole member array
  of strings (8 bytes an element) and `_MEM` of a `TYPE` with a `STRING` member; `t01`–`t09` `VARPTR`, `VARSEG` and
  `VARPTR$` of `TYPE` variables, members and elements (`VARPTR(r)` of a `TYPE` variable is the old compiler's
  internal error; `VARPTR$(r.v)` gives the variable's offset, not the member's); `x01`–`x30` one form each:
  `VARPTR$` of an element of fixed-length strings or a `TYPE`, `SADD` of a number, a member, an element of
  fixed-length strings or a `TYPE` (rejected), `VARPTR` of a `_BIT` scalar (accepted) and of a `_BIT` array,
  `VARPTR$` and `SADD` of `sa()` (element 0), `FOR` over a whole member array or a fixed-length string member,
  `STRING * n` with a float `CONST` (rounded half to even), a string `CONST`, an unknown name or a variable, the
  name `fs$5` as a SUB, a `CONST` or a member (rejected) and as a parameter (accepted), `LEN` of a `TYPE` with a
  `STRING` member ("UDT must have fixed size"), `VARPTR` of members of such a `TYPE`, `STRING * n` of a member
  with a `CONST`
- `v57*`: statements and metacommands, measured as task 1.1 of `m4-statements-metacommands` (its `design.md`, D1,
  lists what each group found): `m01`–`m09` unknown and bare metacommands ("Syntax error", also in an included
  file; nothing in a branch not taken); `s01`–`s20` the old compiler's stubs ("Command not implemented", also
  after `CALL` and when not reached; `RESET`, `CALL RESET` and `CALL CLOSE` compile); `r01`–`r19` `RUN` with no
  argument, a label, a line number, from a SUB and a FUNCTION, of a file (error 53), with the screen, the timer,
  `COMMON SHARED` and `STATIC` after it, and its rejected forms; `r20`–`r24` (review of the change) `RUN` of a
  string `CONST`, with or without its suffix, and of a name under `DEFSTR`, assigned or not (a file, error 53), and
  of a numeric `CONST` (a label, "Label not defined"); `x01`–`x12` `RESUME` in a SUB reached from a
  main-module handler (the SUB is left; with no error, error 20), in a FUNCTION (the C++ build fails) and its
  labels; `p01`–`p16` places in parentheses in `SWAP`, `READ`, `INPUT`, `LINE INPUT`, `LBOUND` and `UBOUND` (all
  the place itself); `o01`–`o21` an argument left out of a function ("Invalid expression" the last, "Incorrect
  number of arguments" or "Syntax error" another; `INSTR(, a$, b$)` compiles); `f01`–`f11` a program FUNCTION as
  a statement; `l01`–`l04` a library and a program procedure of one name (the header in `v57_lib\`); `a01`–`a04`
  `ALIAS` outside a library; `c01`–`c03`, `c05`–`c08` `_RED` beyond the palette in a `CONST` (0 in mode 0; in mode
  13 "UNEXPECTED INTERNAL COMPILER ERROR!"; `c04`, `c09`, `c10` not kept: their value changed from build to build,
  0, 1, 32 or 179, and `_RED(65535, 0)` crashed `qb64pe.exe`); `g01`–`g12` a `$IF` between two settings of a flag
  (no branch compiled; a `CONST` of the branch the line's value chooses is "Name already in use" where used);
  `j01`–`j06` (review of the change) a label or line number in such a `$IF`'s branches ("Label not defined" for a
  `GOTO`, `GOSUB` or line number naming it, also in a SUB; no jump compiles), and a `DO` opened in a `$IF` and
  closed after its `$END IF` ("PROGRAM FLOW ERROR!", also where the two passes agree);
  `b01`–`b20` blocks, SUBs and FUNCTIONs opened in one file and closed in another, both ways (all compile), with
  the files in `v57_inc\`; `b21`–`b25` (review of the change) a `NEXT` inside a `$IF` of an included file for a
  `FOR` of the main file ("NEXT without FOR"), and a `CONST` in the main-file part of a SUB opened in an included
  file: the SUB's (0 in the main module, "Name already in use" for a main-module `CONST` of the name after it, a
  `TYPE` in the SUB sized by it compiles, one of the main module after it "Invalid number/constant after STRING *
  type"); `h01`–`h04` the function `_SCREENHIDE`; `e01`–`e13` neighbours of the forms the parser
  marked; `u01`–`u06` `PALETTE USING`; `t01`–`t03` `STRING * n` members with a `CONST`
- `v58*`: the built-ins no program used, `$ERRORLOCATION` and `$USELIBRARY`, measured as task 2.1 of
  `m4-builtins-remaining` (its `design.md`, D1): `f01` the hyperbolic functions of each type and out of their
  domain (the C library's `nan` and `inf`, no error), `f02`–`f04` their values stored into each type (the `CONST`
  evaluator rejects them, `x107`), `f05`–`f08` the gradian conversions beside the `CONST` of the same literal (the
  `CONST` evaluator's `_D2G(90)` is 99.99999999900001, the expression's 100: they need not agree), `f09` the bit
  functions of each integer type, a bit of 64 or -1 (the x86 shift mask), floats, `f10` the MBF conversions' bytes
  and values, their errors (a value too large for the format, a string too short), `f11` `_COMPILERVERSION$`
  (`QB64-PE v4.7.0-GLFW-UNKNOWN` for the clone), `_ENVIRONCOUNT` beside a count of `ENVIRON$(i)`, `_UPTIME` as
  comparisons, `LPOS` (1, error 5 above 3 or below 0), `f12` the lock keys in a console program (0, no window to
  read), `f13` `_CONSOLEINPUT` and `_CINP` with nothing waiting (0), `f14` `_DISPLAY` (-2 in screen 0, `_DEST`
  after `SCREEN _NEWIMAGE`), `_SCALEDWIDTH` and `_SCALEDHEIGHT` (640 and 400 for 320 by 200: the window is
  doubled), `_SMOOTH`, `_DESKTOPREFRESHRATE` and the lock keys as comparisons in a hidden window, `f15` the
  dialogs compiled and never run, `f16` `INPUT$` on a file (with and without `#`, past the end: error 62, a negative
  count: 52, a file open for output: 75, not open: 52; `x112`–`x116` its rejected forms, `INPUT$(1, #)` "Invalid
  expression"); `s01` the emulated ports in screen 13 (the palette through `OUT`/`INP`, port
  `&H60`, a port not emulated, `_BLINK` -8 after `OUT &H3C0, 8`, error 6 for `INP` outside ±65535 and nothing for
  `OUT`), `s01b` `WAIT` (returns, also for a port not emulated; error 6 out of range), `s02` `FILES` in `v58_fil\`
  (the 8.3 path, entries padded 8.3-style, `.` and `..` of a folder, the free space on the entries' line when the
  console's position is 1, error 53 for no match; masked by `tests\verification\normalize\v58_s02_files.normalize`),
  `s03` `DATE$ =` and `TIME$ =` (nothing changes), `s04` the console statements (`_CONSOLECURSOR` bare accepted,
  `x33`), `s05` the display statements (`_DISPLAYORDER` bare and with a word twice, `_FPS` bare: accepted,
  `x40`, `x41`, `x44`), `s06`, `s06b` `_BLINK`, `s07`, `s08` the lock-key and desktop statements compiled only;
  `i01` `INSTR` with its start left out (the search starts at 1); `e01`–`e29` `$ERRORLOCATION` (the critical
  error's message names the line of the recursive call, of the division, and the included file's name; a joined
  line by its last physical line; `ON` and `OFF` in one program are the error at the first `$ERRORLOCATION` line,
  also across an included file and a `$IF` taken; `ON` twice and a setting in an included file alone hold; `ERL`,
  `_ERRORLINE` and other errors unchanged; `$ERRORLOCATION` bare, `:MAYBE`, with a blank or a comment: "Syntax
  error"; `e27` a `DYNAMIC` library that does not exist, rejected when compiled ("DYNAMIC LIBRARY not found"),
  `e28` a function the library lacks, error 260 at the `FUNCTION` line, `e29` an error in a `$USELIBRARY` top
  file, at its line and name, with `v58_root`'s `test/eloc`), include files in `v58_inc\`; `u00`–`u26` `$USELIBRARY` with the test libraries of `v58_root\libraries\`
  (each program's `.root` sidecar names the folder, recorded with the runner's copy of the release,
  `tools\legacy_tests\README.md`): each kind of file, two libraries (top files last-first, bottom files in
  order), one named twice or by a library's top file too ("Name already in use": included once per line), a
  library that uses another (its files first), the descriptor's sections, quotes, `;` and `'` comments, key case
  and blanks, an empty value and no section (nothing included), `$USELIBRARY` in an included file, in lower case,
  without the colon, as `$USELIBRARYX`, with a tab after the name (all accepted), the three errors, a `$IF`
  branch not taken, after a colon, and `u17` without the sidecar (the release's own "descriptor not found"); `n01`–`n05`
  a built-in with its result's suffix (`_UFONTHEIGHT&`: "Invalid variable name", "Invalid array name"); `x01`–`x111`
  one accepted or rejected form each (the five accepted above and `_SETBIT(1, 1.5)`; the hyperbolics,
  `_SETBIT`, `MKSMBF$`, `_COMPILERVERSION$` and `_ENVIRONCOUNT` in a `CONST` "Unexpected element"; a function as a
  statement "Syntax error", a statement as a function "Name already in use" or "Invalid array name")
- `v59*`: `SUB _GL` and the OpenGL names, measured as task 1.1 of `m4-sub-gl` (its `design.md`, D1). OpenGL gets a
  hardware context on the runner's hidden desktop (the machine's driver; the renderer is printed only as named).
  `a1`–`a6` the forms accepted: plain, `STATIC` (a local kept between calls), in an included file (`v59_inc\`), in
  lower case with `()` and the OpenGL names in any letter case, a SUB defined before it using a routine; under
  `$CONSOLE:ONLY` and `$SCREENHIDE` it compiles and is never called; `b1`–`b4` the names without `SUB _GL` (a
  statement "Syntax error", a function and a constant "Invalid variable name"); `c` the constants (585 of them:
  values, a `#define` naming an earlier one, `_INTEGER64` arithmetic, the suffixes as for a `CONST`); `d` the
  conversions read back (a float rounded half to even into `GLint`, -1 into `GLuint` as all ones, 256 into
  `GLboolean` as 0, an out-of-range clear colour clamped, -1 into `GLenum`: `GL_INVALID_ENUM`); `e`, `e2` the
  functions (`_glGetString` a `STRING`, empty for a name OpenGL does not know; `_glGetError` an `_UNSIGNED LONG`);
  `f1`–`f9` the errors: 270 for a routine with a helper outside `SUB _GL` (in the main module or a SUB it calls,
  `_glGetString` too) and 271 for `END` or `SYSTEM` inside it are libqb's critical errors (a message, exit 0, no
  handler runs; each program waits after the call, since lines printed just after it race the program's end), the
  routines without a helper (`_glFlush`, `_glGetError`) and `_gluPerspective` raise nothing, `STOP` inside it hangs
  (timed out); `g` reading back (a cleared pixel, the viewport, `_glIsEnabled`, `_glGetError`); `h1`–`h3` the SUB
  called under `_GLRENDER _BEHIND`, `_ONLY`, `_ONTOP`; `x01`–`x47` one accepted or rejected form each (`SUB _GL`
  with a parameter, as a FUNCTION, twice or with a suffix "Invalid name" or "Type symbols after a SUB name are
  invalid", a call "Cannot call SUB _GL directly", a program's name or `CONST` of an OpenGL name "Invalid name", a
  `CONST` naming an OpenGL constant "Unexpected element"; accepted: `CALL _glViewport(…)`, `_GL_TRUE&`, a `LONG` or
  a whole array where an `_OFFSET` goes, `DECLARE SUB _GL ()`, a constant as a `DIM` bound and a `CASE`; `x48`–`x54`:
  by the review of the change, a routine with a suffix as a statement "Syntax error" and after `CALL` "Expected CALL
  sub-name", a count error of a one-argument routine, the third argument a string "3rd sub argument requires a
  number", a constant as a member's bound or `STRING * n` rejected and as a variable's `STRING * n` accepted;
  `x55`–`x57`: by the review of those fixes, a suffix with empty parentheses in an expression "Invalid array name"
  and as a statement "Syntax error", after `CALL` without arguments "Expected CALL sub-name")
- `v60*`: the forms steps 2 and 3 of M4's plan left marked, measured as task 1.1 of `m4-left-marked` (its
  `design.md`, D1–D5): `a1`–`a7` `STATIC v(bounds)` whose bound reads a shared variable, an element of a shared
  array, a main-module `CONST` or calls a FUNCTION (read when the start code allocates the array: at the start,
  also for a SUB never called, and when a handler is entered while `ERASE` has released it, with the values of that
  moment; a FUNCTION does not run while the array is allocated, what it prints at the start is lost and later
  seen; several bounds in file order); `b1`–`b10` in a procedure, `REDIM` of a shared array whose bound first names
  an array used without `DIM` (the name is the procedure's array in the whole procedure, above the `REDIM` too and
  below `SHARED sd()`, new on each call also in `SUB … STATIC` and with `_PRESERVE`; other procedures and the main
  module keep the shared one; in a list only that item; `b7`: named above the `REDIM`, the shared array is given
  the bounds); `c1`–`c6` `FOR` over an element of a member array (steps, `EXIT FOR`, a nested member, an index out
  of range at `NEXT` under a handler, every numeric type, `_OFFSET`, two dimensions); `d1`–`d8` a `TYPE` that holds
  a `STRING` in conventional memory (offsets of members, nested and two strings, among scalars, `VARPTR$` of a
  number member; `d5` an element's string used, no crash; `d6` a variable's string used, exit 139; `d7` `VARPTR`
  of an element's `STRING` member and `d8` `VARPTR$` of a `STRING` member: the C++ build fails)
- `v61*`: the forms QB64pe's `qbasic_testcases` need, measured as task 1.1 of `qbasic-testcases` (its `design.md`,
  D1–D4): `a1`–`a10` `VARPTR`, `VARSEG` and `SADD` of a procedure's variables (a local allocated from the top of
  `dblock` on each call and given back, the second call's at the same offset; a `STATIC` one, also those of `SUB …
  STATIC`, allocated at the start after the main module's, so a later call's locals lie below it; every numeric
  type, strings, a fixed-length string, a `TYPE` and its members, `STRING * n` members at their offset, local and
  `STATIC` arrays, a `REDIM` there, implicit variables, recursion 2 bytes a level; `a10` the name of a FUNCTION
  without parameters is a call there, "Expected reference to a variable/array"); `p1`–`p8` parameters whose
  address is taken (a `TYPE` through a chain of three and from a SUB's variable, an array parameter with static,
  dynamic, `COMMON` and a SUB's arrays, a `REDIM` of one, a scalar called with a literal, a variable in
  parentheses and an expression, each call site a slot of its own placed among the variables in source order
  (`p8`), an element and a member, a member of a `TYPE` parameter and an element of an array parameter passed on;
  `VARPTR` and `VARSEG` of a scalar parameter are those of where its argument lies, an array's segment too; a
  string expression's copy is not in conventional memory, `VARPTR` 0); `p9` `VARSEG` and `VARPTR` of an element of
  a member array of a `TYPE` parameter, called with an element of a `TYPE` array (`VARSEG` the nearest segment,
  4177, `VARPTR` the offset from `dblock` cut to 16 bits, 20, so a `POKE` through the two misses the element);
  `p10` `VARPTR$` of a scalar parameter called with an array element, a member and a value (its offset taken from
  `dblock` as for a variable, which for the element equals `VARPTR`'s only because the array's segment is 4096
  above `dblock`'s); `px1`–`px8` a plain variable passed there (main, shared, a SUB's, a string, through a
  parameter, implicit, a scalar parameter passed on, a fixed-length string): "UNEXPECTED INTERNAL COMPILER
  ERROR", as is `px9`, `VARPTR` of a whole `TYPE` parameter; `c1`–`c9` `CALL ABSOLUTE` (a far return, a store through an
  argument, a string's descriptor offset and a `SINGLE` slot, in a SUB with a literal, a `DOUBLE` and shared
  variables and `SADD` of a `STATIC` string as offset, `INT 33h` answered by libqb, the program's own `SUB
  ABSOLUTE` called only without `CALL`; `ABSOLUTE` without `CALL` "Syntax error", `CALL ABSOLUTE` without
  parentheses "Expected CALL sub-name (...)"); `m_<name>` a `TYPE` member named like each name of the old
  compiler's reserved table and some built-ins, written by `v61_members.py` (refused, "Name already in use", the
  names with `HASHFLAG_TYPE`, `_OPERATOR`, `_CUSTOMSYNTAX` or `_XELEMENTNAME`; `AS`, `DATA`, `ELSE`, `REM` and `END`
  with other messages; a single leading underscore "Invalid name"; every other name taken)

`qb45\` holds programs measured with QuickBASIC 4.5, not QB64pe (`q45_*.bas`, each with `<name>.qb45.txt`,
recorded by `tools\qb45\qb45.py`; `--suite verification` reads only this folder's top level, so it does not see
them), each behind a row of the crash-class rulings (`study\39`): `q45_static_local`,
`q45_static_dims` `STATIC` in a SUB beside a shared array of that name (row 4); `q45_whole_array_value` a whole
array `a()` as a value (row 6); `q45_whole_array_erased` `READ a()` and `SWAP a(), b()`, and an element of an
erased array (row 7); `q45_div_zero` `\`, `MOD`, `ERROR 11` and a float `/` by zero under `ON ERROR` (row 12,
`DIVERGENCES-QB45.md` Q-003); `q45_resume_in_sub` `RESUME` in a SUB (row 13); `q45_run_from_sub` `RUN` and `RUN
100` in a SUB (row 14).

Each program's recorded output lies beside it (`<name>.out.txt` and the like). `run.sh` starts the runner's
`--suite verification`, which records them (`tools\legacy_tests\README.md`).

`m1-extension.md` is the walkthrough record of the M1 extension (`m1-vscode-extension-v0`, tasks 8.1 and 8.2).
