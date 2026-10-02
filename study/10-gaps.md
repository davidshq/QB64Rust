# 10 — Closing the study gaps

The three gaps listed in `00-synthesis.md` (table "not yet read") and in `STATUS.md`. Line numbers refer to
`..\QB64pe\source\qb64pe.bas` unless stated otherwise.

## 1. DIM / REDIM / STATIC / COMMON statements

Storage layout, `dim2` per-type allocation, the array descriptor, static-vs-dynamic allocation and REDIM
`_PRESERVE` are already in `02-expressions-codegen.md` §3.2–3.4. This section covers the statement layer above
them (8767–9673) and the parts of `dim2`/`allocarray` that 02 did not spell out.

Checked by running the old compiler: `verification\v09_dim.bas` and its `.out.txt`.

### 1.1 Statement modes

One block parses all four statements. Flags set from the first word (8769–8786):

| Statement | `dimoption` | other flags |
|---|---|---|
| `DIM` | 1 | |
| `REDIM` | 2 | `redimoption` 1; `REDIM _PRESERVE` 2; `REDIM _RETAIN` 3 (member-array layer, deferred) |
| `STATIC` | 3 | sets `dimstatic = 1` and `AllowLocalName = 1` for the statement |
| `COMMON` | 1 | `commonoption = 1` |

Placement errors: `STATIC` outside a procedure → "STATIC must be used within a SUB/FUNCTION"; `COMMON` inside one →
"COMMON cannot be used within a SUB/FUNCTION"; `DIM/REDIM SHARED` inside one → "DIM/REDIM SHARED invalid within a
SUB/FUNCTION". `SHARED` is accepted after `DIM`, `REDIM [_PRESERVE]` and `COMMON`, not after `STATIC`.

### 1.2 Two syntaxes

**Old syntax** (`DIM a, b$, c(10) AS LONG`): each item carries its own type. For each item (`dimnext`, 8811):

1. name, then an optional `( … )` bounds list collected verbatim as tokens (`elements$`);
2. type from, in order: a suffix (`dimmethod = 1`), `AS type` (`dimmethod = 0`), or the `DEFtype` letter table
   `defineaz` (`dimmethod = 1`, `notype = 1`). A leading `_` uses the 27th entry of the DEF table;
3. anything other than `,` after that → "DIM: Expected ,".

**New syntax** (`DIM AS LONG a, b(10)`, 9565): the type is read once; it ends at the first `,` or `(` and the last
word before it is the first variable name. A suffix on any name → "Cannot use type symbol with DIM AS type
variable-list (…)". The item is then processed by `GOSUB NormalDimBlock` (the old-syntax code, which `RETURN`s
when `newDimSyntax` is set, 9551). The layout text prints `AS type` once, before the first name.

`dimmethod` decides `musthave` (suffix required later: `a%`) versus `mayhave` (suffix optional: `a AS INTEGER`),
see 02 §3.2.

### 1.3 REDIM specifics

- **Type adoption** (9107–9117): `REDIM` (not `DIM`/`STATIC`) of an array without any type information reuses the
  type of an existing array of that name that was declared with `AS` (`id.mayhave` set): `REDIM x(5) AS LONG` then
  `REDIM x(10)` keeps LONG. Verified (v09 test 3).
- `REDIM` of an existing array goes through the same `dim2` → `allocarray` path; `allocarray` finds the descriptor
  (`arraydesc`) and only emits the reallocation. Changing the dimension count → "Cannot change the number of
  elements an array has!"; STRING ↔ STRING * n → "Type mismatch".
- `REDIM` of a static array compiles, then fails at run time with error 10 (Duplicate definition) and the array
  keeps its old bounds. Verified (v09 test 4).
- REDIM with a dotted target (`REDIM p(1).m(5)`) takes the member-array path (8820–9019). Deferred with the rest of
  the member-array work (`SOMEDAY.md`).

### 1.4 When DIM code runs

| Declaration | Allocation | Observable |
|---|---|---|
| main module, constant bounds, no `$DYNAMIC` | once, in the data section at program start | the array exists even if the DIM line is skipped by `GOTO` (v09 test 2); `DIM` cannot run "again" |
| main module, non-constant bounds, or `$DYNAMIC`, or `REDIM` | inline at the statement | `DIM` of an already defined dynamic array → run-time error 10 |
| procedure, `DIM` (not STATIC) | inline at the statement, freed at procedure exit | fresh zeroed array on every call (v09 test 11); executing the same `DIM` twice in one call → error 10 (v09 test 6) |
| procedure, `STATIC a(n)` | dynamic, but the creation code goes into the *main* data section (`dimoption = 3` → `BufInsertBuf DataTxtBuf`, 15541) | created once, values persist across calls (v09 test 7) |
| procedure, `STATIC a()` then `DIM a(n)` ("list array") | see 1.5 | |
| `SUB … STATIC` | `dimstatic = 2` for the whole body (5827): all locals live in main data; constant-bound arrays are static | values persist (v09 test 9) |

`$DYNAMIC`/`$STATIC` toggle `DynamicMode` (3361–3362) for subsequent declarations.

### 1.5 STATIC list arrays (`STATIC a()`)

`STATIC name()` with empty parentheses does not create anything. It appends `name type dimmethod` to
`staticarraylist` (9178–9265); `dim2` is not called. A later `DIM`/`REDIM name(…)` (or an auto-created array) in the
same procedure whose name, full type name and `dimmethod` match sets `dimstatic = 3` (9272–9294, 19617–19637): the
array's descriptor goes to main data, so its contents survive procedure exit, but it is allocated dynamically.
Consequence: a plain `DIM a(3)` on the second call fails with error 10 and the old contents stay (v09 test 8).
Code that wants re-dimensioning must use `REDIM`. The list is cleared at `END SUB/FUNCTION` (5961).

`STATIC a()` twice with the same type → "Name already in use"; `DIM` after it with `dimoption = 3` → "Array already
listed as STATIC".

Name-conflict checks for `STATIC a(n)` and `STATIC a()` (9120–9176, 9204–9258): an existing array of the same name in
the *same* procedure conflicts when both were declared with `AS` (explicit over explicit), or when element type and
size are equal. Globals never conflict with a STATIC local. The check runs twice, once for the bare name and once
for name + type suffix.

### 1.6 COMMON

`COMMON` is a CHAIN feature. For arrays, `elements$` is forced to `"?"` (unknown dimension count) and the array
is appended to `commonarraylist` (`name type dimmethod shared`, 9302–9307). A later module-level `DIM` of an array
with matching name, type and method inherits `SHARED` from the COMMON (9388–9420). The name comparison there is
case-sensitive in the source, but `DIM CS(3)` after `COMMON SHARED cs()` still ended up shared in practice (v09 test
10). Probable reason (not traced): `COMMON SHARED` already registered a shared id, and `dim2` reuses it through
the case-insensitive `findid` lookup, so the list match is not needed.

Code generation writes `#include "chainN.txt"` / `"inpchainN.txt"` lines; scalar COMMON variables emit
`sub_put`/`sub_get` records (`int32` tag 1, `int64` size in bits, then the value) into `chain.txt` (save) and
`inpchain.txt` (load) (9429–9522). This is the CHAIN state file format. Array COMMON writes empty include files
here; the array content is handled elsewhere (not studied, low priority: CHAIN is rare).

Note: "COMMON alone does not imply SHARED" (9297).

### 1.7 Bounds and run-time errors

From `allocarray` (complements 02 §3.4):

- `DIM a(0)` is always `0 TO 0`, even under `OPTION BASE 1` (14970–14980). This is a recent upstream change
  ("keep the classic zero-element bootstrap idiom valid"); QuickBASIC raised an error instead. Verified (v09
  test 1). `DIM a(3)` under `OPTION BASE 1` is `1 TO 3`.
- Constant bounds: explicit `lo TO hi` with `hi < lo`, or a single negative bound → compile error "Invalid array
  bounds" (15004–15015).
- Run time: element count `<= 0` → error 5; allocation overflow or out of memory → error 257; DIM of a defined
  dynamic array or (RE)DIM of a static one → error 10.
- Bounds are evaluated as `_INTEGER64` (`evaluatetotyp(…, 64)`).

### 1.8 ERASE (8529–8705)

| Array | Effect |
|---|---|
| static | contents cleared (numbers zeroed, var-len strings set to length 0, UDT string members cleared); bounds stay (v09 test 5: `UBOUND` 5, element 0) |
| dynamic | strings freed, block freed, descriptor reset to "undefined" (lower bound 2147483647, count 0); the cmem flag is kept. `UBOUND` afterwards → error 9 (v09 test 5) |

`ERASE` takes full expressions so it can address member arrays (`ERASE p(0).m`). "Undefined array passed to ERASE"
for `ERASE ()`.

### 1.9 Notes for the rewrite

- Everything here is a declaration pass plus an "execute at statement" pass. The rewrite should resolve
  static-vs-dynamic in the semantic pass (it needs only constant folding, the enclosing procedure's STATIC flag,
  `$DYNAMIC` state at that line, and the statement kind), then lower to either "allocate at startup" or "allocate
  here".
- `STATIC a()` list arrays and `COMMON` arrays are both "pending declaration" lists keyed by
  `(name, full type, dimmethod)`. One symbol-table mechanism can cover both.
- Error 10 on re-executing `DIM` in a procedure and on `REDIM` of a static array are observable behaviours that test
  programs may rely on; keep them.
- The `DIM a(0)` under `OPTION BASE 1` rule is upstream behaviour; follow it.

## 2. PRINT / INPUT / WRITE / PRINT USING emission

Statement dispatch 11017–11316; emitters `xprint` 27713, `xfileprint` 27258, `xwrite` 27993, `xfilewrite` 27486.
Runtime helpers in `..\QB64pe\internal\c\libqb.cpp`: `makefit` 10426, `tab` 10451, `qbs_print` 10579,
`sub_file_print` 13380, `func_tab` 16421, `func_spc` 16523, `func_pos` 16175. The bodies of `qbs_input` and
`print_using` are still only outlined (03 §9).

Checked by running the old compiler: `verification\v10_print.bas` and its `.out.txt`.

### 2.1 Common shape

Every emitter walks the token list, splits items at depth-0 `,` / `;`, evaluates each item and writes one runtime
call per item, each followed by `if (is_error_pending()) goto skipN;`. The statement ends with the `skipN:` label and
`cleanupstringprocessingcall$` (temp string cleanup). So a run-time error in one item skips the rest of the
statement; with `ON ERROR … RESUME NEXT` execution continues at the next statement, not the next item.

### 2.2 PRINT (screen) and LPRINT

Per item:

| Item | Emitted |
|---|---|
| string expression | `qbs_set(tqbs, e); makefit(tqbs); qbs_print(tqbs, 0);` |
| numeric expression | the item is **re-parsed as the string expression `STR$(e) + " "`** and handled as a string (27889–27894) |
| `,` | `tab();` |
| `;` | nothing |
| end without trailing `;`/`,` | `qbs_print(nothingstring, 1);` (new line) |
| bare `PRINT` | `qbs_print(nothingstring, 1);` |

So a number prints as `STR$` (leading space or `-`) plus one trailing space: `PRINT "["; 5; -3; 1.5; "]"` →
`[ 5 -3  1.5 ]` (v10 test 1). `makefit` moves to a new line first if the whole item does not fit and the cursor
is not in column 1 (QB's "items are not split across lines" rule). `tqbs` is a statement-local temporary
(`TQBSset` is a local of `SUB xprint`, so it is 0 again for every statement; checked in the generated C).

`LPRINT` is the same code with `qbs_lprint`, `tab_LPRINT=1` around it, and `lprint_makefit`, which is a stub (no
wrapping on the printer page).

`tab()` (comma) in a text screen prints one space, then more until the column is a multiple of 14 plus 1, or moves
to the next line when past `width - 13`. Graphics screens use pixel zones (112 px for variable-width fonts).

**Console (`$CONSOLE`) on Windows** (10461–10472) is different: zones are **10** columns wide, and the position comes
from `func_pos`, which asks the console window for its cursor through `CONOUT$`. When stdout is redirected (file or
pipe) that cursor never moves, so `do printf(" "); while (func_pos(0) % 10 != 0);` **never terminates**: `PRINT 1, 2`
in a `$CONSOLE:ONLY` program with redirected output writes spaces forever (seen while writing v10: 4.4 MB of spaces
before the timeout). `func_pos` also opens a new `CONOUT$` handle on every call and never closes it. The rewrite should
track the column itself for console output and must not reproduce this. Consequence for the verification programs:
no screen `PRINT` with a comma under `$CONSOLE` with redirected output.

**Auto-semicolon** (11269–11307): before `xprint`, a `;` is inserted (a) after a string literal followed by anything
except `; , + ) < = >`, and (b) before a string literal preceded by anything except `; , + ( < = >` and `USING`. So
`PRINT "a"1` and `PRINT x "b"` work (v10 test 2), but `PRINT 1 x` (no literal involved) is "Expected operator in
equation". Not applied when the second word is `USING`. Test (a) parses the literal's internal `"text",len` form and
uses a `beginpoint` variable that is never reset between statements; it worked in every case tried, but the rewrite
should simply implement the rule on tokens.

The layout collapses `;;` to `;`.

### 2.3 PRINT USING

`PRINT [items;] USING fmt$; expr [{;|,} expr]… [;|,]`. `USING` may follow ordinary items in the same statement
(`PRINT "["; x; USING "###"; 42` → `[ 7  42`, v10 test 3); after `USING` the rest of the statement is formatted.

- The format must be a string expression followed by `;` ("Expected PRINT USING formatstring ; ..." otherwise,
  including when `,` follows it or nothing follows).
- The format is copied once into `print_using_formatN` (`static qbs*` in main, plain in procedures); `tmp_long` holds
  the current position in the format and is threaded through every call.
- Per item, by type: strings `print_using(fmt, pos, tqbs, s)`; SINGLE `print_using_single`; DOUBLE
  `print_using_double`; `_FLOAT` `print_using_float`; unsigned `_INTEGER64` `print_using_uinteger64`; every other
  integer type (including `_UNSIGNED LONG`, `_OFFSET`, `_BIT`) `print_using_integer64`. Output accumulates in `tqbs`.
- `,` and `;` between items are equivalent (no tab zones).
- `TAB(n)`/`SPC(n)` items are special-cased by looking at the generated C text (`func_tab(`/`func_spc(` prefix):
  flush `tqbs`, print the TAB/SPC string with `makefit`, clear `tqbs`, keep the format position.
- At the end: on a pending error, print what was built so far (without new line) with the error masked; otherwise
  print `tqbs` with a new line unless the statement ended with `;` or `,`.

### 2.4 PRINT # and PRINT # USING

`xfileprint` sets `tab_spc_cr_size=2` (tells `TAB`/`SPC` to use the file column) and `tab_fileno=tmp_fileno=<n>`.
The file number is evaluated as `_INTEGER64`. Per item: `sub_file_print(fileno, s, extraspace, tab, newline)`.

| Item | Call |
|---|---|
| string | `sub_file_print(f, s, 0, usetab, last)` |
| number | `STR$(e)` with `extraspace = 1` (the runtime appends the space): same text as on screen |
| `,` with no item before it | `sub_file_print(f, nothingstring, 0, 1, 0)` |
| `PRINT #f,` | `sub_file_print(f, nothingstring, 0, 0, 1)` |

`sub_file_print` tracks `gfs->column` (reset by CR) and pads comma zones to **14** columns, always at least one space
(v10 test 7: `[x             y 3            4 ]`). No `makefit` (no wrapping in files). For a file opened on `SCRN:` it
calls `qbs_print(str, newline)` and ignores `extraspace` and `tab` (13400–13403; read, not run). Errors: 52 bad file
number, 54 not an output file, 75/70/5/258 for write failures.

`PRINT #f, USING` mirrors 2.3 with `sub_file_print` instead of `qbs_print`; mixing works: `PRINT #1, "b"; USING "#";
7` → `b7` (v10 test 7).

### 2.5 WRITE and WRITE #

Each item is rewritten to a string expression before evaluation (27540–27550, 28022–28033):

- number → `LTRIM$(STR$(e))`, plus `","` if not last;
- string → `CHR$(34) + e + CHR$(34)`, plus `","` if not last.

Then one `qbs_print(s, last)` (screen, no `makefit`) or `sub_file_print(f, s, 0, 0, last)` per item.
Consequences (v10 tests 5 and 7):

- Embedded quotes are not escaped: `WRITE "q" + CHR$(34) + "x"` → `"q"x"`. (QB64 has no `""` escape in string
  literals either; `"q""x"` is a compile error.)
- A trailing comma suppresses the new line *and* leaves the separator: `WRITE #1, 1, 2,` then `WRITE #1, "after"`
  gives the single line `1,2,"after"`.
- Bare `WRITE` and `WRITE #f,` print an empty line.

### 2.6 INPUT (keyboard) and LINE INPUT

`INPUT [;] ["prompt"{;|,}] var[, var]…` (11123–11236):

- `;` right after `INPUT` → `newline = 0` (cursor stays on the line after Enter).
- The prompt must be a **string literal** (detected by a leading `"`); an expression prompt is not accepted. After
  the prompt, `;` prints `"? "` (not for LINE INPUT), `,` prints nothing. Without a prompt, INPUT prints `"? "` and
  LINE INPUT prints nothing. Anything else after the literal → the long "Syntax error - Reference: INPUT …" message.
- Each variable registers a slot: `qbs_input_variabletypes[k] = <type>` (`ISSTRING`, `ISSTRING+512` for LINE INPUT,
  else the numeric type with pointer/cmem/reference flags stripped) and `qbs_input_variableoffsets[k] = <address>`.
  Then one call `qbs_input(count, newline)`, followed by `if (stop_program) end();`. All parsing, validation and the
  "Redo from start" loop are in the runtime.
- Errors: non-variable → "Expected variable"; `_BIT` array element → "INPUT cannot handle _BIT array elements";
  LINE INPUT with a numeric → "Expected string variable", with more than one variable → "Too many variables".
- With `$DEBUG` (vWatch) the call is bracketed by line-number markers −4/−5 so the debugger can show "waiting for
  input".

### 2.7 INPUT # and LINE INPUT #

`INPUT #f, var, …` (11024–11120): `tmp_fileno = <f>` then one call per variable:

| Variable | Call |
|---|---|
| string | `sub_file_input_string(f, ref)` (`sub_file_line_input_string` for LINE INPUT #) |
| SINGLE/DOUBLE/_FLOAT, or any integer narrower than 64 bits | `setrefer(var, func_file_input_float(f, typ))` (`(int64)` cast first for `_BIT`) |
| `_INTEGER64` / `_UNSIGNED _INTEGER64` | `func_file_input_int64(f)` / `func_file_input_uint64(f)` |

So integer variables are read through a floating-point reader and converted on assignment; only 64-bit integers keep
full precision. Item parsing (v10 test 8): surrounding spaces are trimmed, quoted strings may contain commas, `3e2` →
300, `&H10` → 16.

### 2.8 Notes for the rewrite

- Lower PRINT/WRITE to "convert each item to a string, then call one of three runtime primitives", as the old
  compiler does. The visible text depends on `STR$` and on where the trailing space comes from (appended string on
  screen, `extraspace` in files; same result).
- `PRINT USING` needs the type → formatter mapping above; the formatter itself (`print_using`) should be lifted
  verbatim (03 §9.3).
- Track the output column in the runtime for every destination (screen, console, file, printer) instead of asking the
  OS. This removes the console hang and makes comma zones deterministic. Whether console zones stay 10 wide
  (compatible) or become 14 (QB-like) is an open bug-compatibility choice (listed in `09-verification.md`).
- `INPUT` prompts: accept only literals (compatible; QuickBASIC 4.5 also allowed only a literal), or allow
  expressions as an extension.

## 3. Built-in table: argument types and categories

The registration mechanism, the id record, overall counts and the `specialformat` grammar are in
`02-expressions-codegen.md` §4.3 and §5. What was missing was the per-entry data. Following panel R9 it is now
extracted mechanically instead of read by hand:

- Script: `tools\builtins\extract_builtins.py` (no arguments; reads `..\QB64pe\source\subs_functions\
  subs_functions.bas` and `qb64pe.bas`).
- Output: `tools\builtins\builtins.json`, one record per `clearid … regid` block: `name`, `kind` (sub/function),
  `callname` (C entry point), `args`, `minargs`, `arg_types` (decoded names), `ret`, `musthave`/`mayhave`,
  `specialformat`, `hr_syntax`, `secondargmustbe`/`cantbe`, `overloaded`, `dependency`, source `line`, trailing
  comments, a derived `category`, and `compiler_mentions` (see 3.3).

Rerun the script whenever the reference clone is updated; it prints the summary below and flags any type expression
it cannot decode (none today).

### 3.1 Summary (QB64pe 4.7.0)

455 entries, 404 distinct names, ignoring case (46 names have several entries: statement + function forms, or alternative
statement syntaxes such as `GET`/`PUT`/`OPEN`/`SCREEN`). 289 functions, 166 subs.

| Category (derived) | Count | Meaning |
|---|---|---|
| plain function | 210 | fixed argument list, types in `arg_types` |
| special-format statement | 172 | parsed by `seperateargs` against `specialformat` (98 subs, 74 functions; functions only use `?`, `[`, `]`) |
| plain sub | 44 | fixed argument list, no format |
| stub | 28 | `sub_stub`/`func_stub`: reserved name, real handling hard-coded or "not implemented" (02 §5.2) |
| overloaded | 1 | `_RGB32` (`minargs` 1) |

02 §5.2 counts 179 `specialformat`s; that grep includes commented-out lines. 172 entries actually carry one.

Argument types over all entries: LONG 299, STRING 135, `_FLOAT` 67, `_UNSIGNED LONG` 39, SINGLE 36, DOUBLE 29,
any-numeric (−1) 28, `_OFFSET` 8, `_UNSIGNED _INTEGER64` 8, special −3 6, `_MEM` 5, `_INTEGER64` 5, INTEGER 3,
special −4 2, −8 1, −2 1. No built-in takes `_BYTE` or `_UNSIGNED INTEGER`; narrower values are passed widened.

Return types match 02 §5.2, plus two functions with **no** return type (3.2).

### 3.2 Anomalies found by the extraction

- `_LogMinLevel` (`subs_functions.bas` 4383) sets `ideret = LONGTYPE - ISPOINTER` instead of `id.ret`. `ideret` is an
  unrelated, otherwise unused variable, so the function is registered with return type 0.
- `_ScreenExists` (3838) has no `id.ret` line at all.
- Both still work (`x = _SCREENEXISTS` → −1, `PRINT _LOGMINLEVEL + 1` → 6; checked with the old compiler), because a
  type code of 0 falls through to the integer paths (`(int64)` casts). It works by accident; the JSON records them
  without `ret`. A new compiler must give them a real type (LONG for `_LogMinLevel`, per the C prototype
  `uint32_t func__logminlevel()`, ideally `_UNSIGNED LONG`; LONG for `_ScreenExists`, C `int32_t`).

### 3.3 Built-ins with hand-written handling

The data table is not the whole story: 86 names are compared by name somewhere in `qb64pe.bas` (patterns like
`RTRIM$(id2.n) = "NAME"`, `n$ = "NAME"`, `firstelement$ = "NAME"`; a heuristic, so a lower bound). These need code
in the new compiler, not just a table row:

Abs, Asc, Atn, CDbl, Chain, CInt, Clear, CLng, Close, Cos, CSng, CVD, CVI, CVL, CVS, End, Environ, Exp, Fix, Get,
Hex, Input, Int, Key, LBound, Len, Line, List, Log, LPrint, LSet, MKD, MKI, MKL, MKS, Oct, Open, Option, Palette,
Print, Put, Read, Reset, RSet, SAdd, Sin, Sleep, Sqr, Strig, String, Swap, System, Tan, Timer, UBound, Val, VarPtr,
VarSeg, Width, Write, _Bin, _Cast, _Clamp, _Continue, _CV, _Echo, _Embedded, _IIf, _LogError, _LogInfo, _LogTrace,
_LogWarn, _Max, _Mem, _MemFill, _MemGet, _MemPut, _Min, _MK, _Offset, _RoL, _RoR, _Round, _SndRawBatch, _UCharPos,
_Wave.

Groups: type-generic math and conversion (the 25 users of the any-numeric code −1, e.g. `ABS`, `INT`, `FIX`, `CINT`,
`HEX$`, `_ROUND`, `_CAST`, `_CV`/`_MK`; their return type is overridden in `evaluatefunc` for 6 of them: `_Cast`,
`_MemGet`, `Exp`, `Fix`, `Int`, `_CV`), variadic functions (`_MIN`, `_MAX`, `_CLAMP`, `_IIF`), memory and address
functions (`_MEM*`, `VARPTR`, `VARSEG`, `SADD`, `_OFFSET`), array bounds (`LBOUND`/`UBOUND`), and statements with
their own parsers (PRINT, INPUT, WRITE, READ, OPEN, CLOSE, GET/PUT, LSET/RSET, SWAP, END/SYSTEM, CHAIN, CLEAR,
OPTION, KEY, STRIG, TIMER, PALETTE, WIDTH, LINE).

### 3.4 Notes for the rewrite

- Use `builtins.json` (or a Rust table generated from it) as the starting built-in table, with `category` and
  `compiler_mentions` deciding which entries need hand-written semantic code.
- Parse each `specialformat` once into a small grammar value (02 §4.3) instead of interpreting the string per call.
- Fix the two missing return types (3.2).
- `callname` gives the runtime entry point per built-in; it is the list of runtime functions the generated code
  must be able to call if the runtime ABI is kept (00-synthesis decision 2).
