# What is not supported yet

<!-- Written by tools/reference/reference.py render; do not edit. -->

The forms QB64pe accepts that QB64Rust still rejects with "not supported yet", and the forms it builds
that no test runs, from `docs/SOMEDAY.md` ("Deferred QB64pe features", where each row says why it
waits). 41 rows.

| Group | Form |
|---|---|
|  | `WIDTH #n, columns`, `WIDTH LPRINT columns` and `SCREEN …, _MANUALDISPLAY n` are compiled, and no test runs them |
| sleep | `_LIMIT`, `_DELAY` and `SLEEP` are run by one screen program with small values; how long they wait is not checked |
| sleep | `SLEEP` with no time in a screen program |
| graphics-get-put | Graphics `GET` and `PUT` of a `TYPE` that holds a `STRING` |
| varptr | `VARSEG` and `VARPTR$` of a `TYPE` that owns a dynamic member array, or of a `_MEM` |
| varptr | `VARPTR` and `VARSEG` of a whole array `x()` that is not of numbers |
| varptr | `VARSEG` of a `_BIT` array or element |
| varptr | `VARPTR` of a member of a `TYPE` type |
| varptr | `VARPTR$` of a member of an element (`VARPTR$(a(1).name)`) |
| varptr | `VARPTR$` of an element of a member array |
| sub-colon | A line that is the name of a SUB and a colon, where a line the parser could not read has the word in it |
| embed-paths | An `$EXEICON` or `$EMBED` name with a drive and no slash after it (`C:game.ico`, `$EMBED:'C:f.txt','h'`) |
| sound | The sound programs need a machine with a sound device: `s75_sound_wait`, the second part of `s76_sound_errors`, and the sound handle programs `s127`–`s129` |
|  | `CHAIN` runs only to a program that is not there |
| member-arrays | A reference through an element of a member array that is not its last step (`x(0).n(1).q`), or to a member array of user-type elements |
| varptr, mark-to-error | `_OFFSET` of a variable-length string member of a `TYPE`, or of a whole array of variable-length strings |
| varptr | `_OFFSET` of an element of variable-length strings; `_MEMGET`, `_MEMPUT` and `_MEMFILL` with an element or member of them |
| member-arrays | `_OFFSET(x.v())` of a whole `_DYNAMIC` member array |
| member-arrays | `_MEMGET`, `_MEMPUT` and `_MEMFILL` with a `_DYNAMIC` member array or an element of one as the variable |
| undeclared-array | An array used without `DIM` as the array of `_WAVE` or `_SNDRAWBATCH` |
| declare-library, events | A SUB of a `DECLARE LIBRARY` block as a handler |
| embed-same-file | Two absolute `$EMBED` names of one file written differently (`<drive>/d/f` and `<drive>\d\f`, or with `..`): "was already embedded" in the old compiler, which resolves the folder first (`CHDIR`, `_CWD$`, `qb64pe.bas` 3765–3777); compared as written here, so both are accepted |
|  | Compressing an embedded file as the old compiler does when that saves a fifth |
|  | A successful HTTP download, from a server on the same machine, in the suites VM |
| const-eval | A string argument to a color function in a `CONST` (`_RGB32("a")`), which the evaluator's varying argument count lets through |
| dotted-names | A dotted name written with blanks around the dot: `p . q(1)` where no array `p.q` is declared (an array used without `DIM`), `c . k` where `c.k` is a `CONST`, and a store `f . g = 7` to the name of a FUNCTION `f.g` |
| undeclared-array | `_MEM(x())` of a name that is no array |
| member-arrays | `LEN(x.d())` of a whole `_DYNAMIC` member array |
|  | A log statement in an included file found in the compiler root (`--include-root`) |
| unknown-type | A name that is no type in an `AS` clause: `REDIM` of a member array, `_UNSIGNED` before a name in `SHARED`, a parameter or `_DEFINE` |
| fixedstr-dollar | `REDIM`, `STATIC` or `COMMON` of `a$(…)` beside a `STRING * n` array `a` |
| fixedstr-dollar | `COMMON SHARED s$` beside a `STRING * n` variable `s` |
| paren-place | `ASC((a$)) = 65`: the place in parentheses |
| mark-to-error | `x() = 1`, the whole array of a name that is no array as a place; a SUB written with a type suffix (`s$ 1` beside a `SUB s`); a function of the auto-included files as a place (`_ENCODEURL$(1) = "a"`) |
| undeclared-array | `READ x()` of a name that is no array |
| static-proc-arrays | `STATIC a()` in a SUB beside a shared array `a` of the main module |
| mark-to-error | A statement named with one leading underscore that the table does not hold (not one starting `_GL`: the OpenGL table holds every such name since `m4-sub-gl`) |
|  | `CALL INTERRUPT[X]` (`CALL ABSOLUTE` compiles since `qbasic-testcases`) |
| type-order | A `TYPE` whose member array bounds name a `CONST`, used above its block by `LEN` (`DIM e AS late: PRINT LEN(e)` above `TYPE late`) |
| bit-dblock | A `_BIT` scalar of the main module in a program whose only use of `dblock` is a slot `passN` (a value to `CALL ABSOLUTE`, or to a parameter whose address is taken): compiled, the `_BIT` scalar left out of `dblock` |
| long-names | A word of more than 40 characters in a metacommand line other than a `$LET` name (a `$IF` condition: `$IF abcdefghijabcdefghijabcdefghijabcdefghijk = 1 THEN`) |
