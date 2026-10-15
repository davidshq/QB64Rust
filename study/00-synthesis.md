# 00 — Synthesis: start here

The one-page entry point to this folder. It summarises every study, the decisions, the plan, what was
measured, and what is still open. Details live in the numbered documents; each section points to them. The reviews
of other repositories (`11`–`14`) are closed and live in `archive\`; their conclusions are in §11 here.

First written 2026-10-02 from `01`–`05`; rewritten 2026-10-02 (session 5) to cover `06`–`14`, `baselines\` and
`verification\`; trimmed 2026-10-03 when `11`–`14` were archived. Tree studied: `..\QB64pe`, HEAD `16f629784e`,
`Version$ = "4.7.0-GLFW"` (upstream QB64pe `main`).
Current phase and next steps: `STATUS.md`. Rules: `CLAUDE.md`. Decision log: `DECISIONS.md`.

## 1. Decisions

All taken 2026-10-02; the authoritative list is in `DECISIONS.md`.

| Decision | Consequence |
|---|---|
| Ground-up rewrite of QB64pe | Understand the old code first (done: `01`–`05`, checked in `09`–`10`) |
| Do not port the text-mode IDE; reach parity with a VS Code extension | Compiler must be a library with an analysis API, a separate formatter, column-accurate diagnostics, debug symbols (`06`) |
| New compiler in **Rust** | Self-hosting is not a goal; compiling the old `qb64pe.bas` remains a large test case |
| **New error messages** (columns, several errors per run) | The 56 `.err` tests compare old texts; use them loosely (must fail, right line). Old texts optional during a transition |
| No strict QuickBASIC 4.5 mode | What one would need is recorded in `08` |
| Target dialect = upstream QB64pe 4.7.0 | The "fork" features turned out to be upstream work. Keep `$USELIBRARY`, `$ERRORLOCATION`, GLFW; defer TYPE member arrays, `_ARRAYCOPY`, whole-array assignment, `REDIM _RETAIN` (`SOMEDAY.md`) |

## 2. Plan: architecture and roadmap

From the expert panel (`07`), whose recommendations R1–R13 the decisions above adopt or leave standing:

- **Strategy (R1):** rewrite the compiler, refactor the runtime (libqb) in place, replace the IDE with VS Code.
- **Correctness (R2, R8):** the target is QB64pe behaviour **as observed** at a pinned commit, plus a divergence
  register for intentional differences. Fix memory-corrupting accidents; keep result-changing ones until decided
  (§7). Numeric rules get a written spec driven by differential testing.
- **Architecture (R5):** lossless syntax tree (keeps every byte: needed for formatter, LSP, include positions) →
  resolved, typed AST → typed IR with explicit conversions and explicit error/resume points → C++ emitter.
- **Back end (R6, R7):** first target is the existing libqb ABI and `qbx.cpp`, so every runtime behaviour and
  `DECLARE LIBRARY` keep working; the IR must not encode that ABI. Change the ABI only once the old compiler no
  longer produces code for it. Keep a C++ back end while C/C++ `DECLARE LIBRARY` and `SUB _GL` are supported.
- **Built-ins (R9):** table as data, extracted mechanically (done, §9); parse `specialformat` once.
- **Encoding (R12):** VS Code opens `.bas` as CP437 by default, per-file override; the compiler reads bytes.
  Round trip checked (`09`, last section): every byte survives except a lone 0x0D (becomes the line ending);
  a file containing 0x00 cannot be opened as text in VS Code at all, even with "Open Anyway".
- **Toolchain (R13):** pin and checksum the Windows C++ toolchain (QB64pe's setup downloads the latest llvm-mingw
  with no checksum; build problems in `baselines\README.md`).

| # | Milestone | Done when | State |
|---|---|---|---|
| M0 | Baseline | Old compiler builds on Windows; Windows runner; baseline recorded; dialect settled; study gaps closed | **done** |
| M1 | VS Code extension v0 on the old compiler | Highlighting, build/run, diagnostics from `-z`, formatting via `-y`, CP437 default | **done** (`vscode\`) |
| M2 | Front end | Lossless parser with recovery over the whole language (no false syntax error on any program the old compiler accepts: corpus, upstream tests, `qbasic_testcases`, its own sources), resolution, type checker, a thin language server (the formatter matching `-y` left this criterion on 2026-10-09, `DECISIONS.md`) | **done 2026-10-12**: golden corpus; Rust workspace and end-to-end slice; procedures and error handling; upstream tests; control flow and constants; arrays and `TYPE`; parser breadth (every accepted program parses, no false error); core built-ins; the language server; the numeric types; built-in statements; the screen-state check; default types, dynamic arrays, arrays in procedures, `COMMON`, the rest of `TYPE` and member arrays; a console and a window (`$CONSOLE`, `$SCREENHIDE`, `_DEST _CONSOLE`), the window built-ins as compiled code, the auto-included constants and the precompiler flags (`crates\`). No step of the order of work is left. Order of work and the numbers of the last full runs: `STATUS.md` |
| M3 | Code generation to the existing ABI | Emits `qbx.cpp` fragments, links with libqb, passes expected-output and differential tests; programs without `$CONSOLE:ONLY` with a screen-state oracle | under way: the slice links with libqb since M2; since `m3-window-output` (2026-10-12) a program whose output is a window compiles, with the text-screen, drawing and pacing statements, and 42 screen programs pass by their screen state |
| M4 | Parity | The corpus (275) and the upstream tests in reach (279 of 404; 125 need the deferred array features) match; `qb64pe.bas` compiles and the result passes the suite (exit criterion, `study\20`) | |
| M5 | Debugger | Debug symbol file + DAP adapter | |
| M6 | Runtime modernisation | ABI owned by the new compiler; error model, string heap, threading redesigned behind golden tests | |

## 3. The old system in one page

```
source text
  └─ lineformat$            lexer: line → CHR$(13)-separated "elements" (string), with side effects
      ├─ PREPASS            TYPE, CONST (own evaluator), DEFxxx, SUB/FUNCTION signatures, DECLARE LIBRARY
      └─ MAIN PASS          per statement: parse + type-check + emit C++ text + build formatter output
            ├─ fixoperationorder → evaluate → refer/setrefer     (expressions, all string-to-string)
            ├─ hard-coded statements (control flow, DIM, PRINT/INPUT, SWAP, …)
            └─ generic sub call: id table + `specialformat` mini-language (455 built-ins)
  (any of six conditions → throw everything away and restart both passes)
  └─ ~30 text fragments in internal\temp  ──#include──▶  internal\c\qbx.cpp  (one C++ TU)
        └─ make: qbx.o + libqb_make_<featurebits>.o + part libs (chosen by DEP_* flags) → exe
```

| Piece | Size | Nature |
|---|---|---|
| Compiler `source\qb64pe.bas` | 28,828 lines | Lines 1–14,352 are one flat GOTO-driven module; 116 SUB/FUNCTIONs after that |
| IDE `source\ide\ide_methods.bas` | 21,424 lines | `$INCLUDE`d into the compiler; one function (`ide2`) is 6,880 lines |
| Runtime `internal\c\libqb.cpp` | 27,119 lines | ~297 functions, 892 static locals, 628 gotos; plus ~24.5k lines already modularised in `libqb\src` |
| Skeleton `internal\c\qbx.cpp` | 1,608 lines | The TU user code is pasted into; owns `QBMAIN`, events, CHAIN |
| Vendored libs `internal\c\parts` | ~1,100 files | GLFW, miniaudio, FreeType, curl, stb, etc. (table in `05` §3) |
| Bootstrap `internal\source` | 2,079 files | Generated C++ of the compiler, committed by CI; usable to check codegen claims without building |
| Tests | 404 + 143 `.bas` | 331 with expected stdout, 56 with expected error text, 17 compile-only; plus 5 formatter sources |

Five properties define the current design (details `01`, `02`):

1. **No AST, no IR.** A line is a delimiter-separated string; expressions are rewritten in place into bracketed
   strings and then into C++ text. Types are one bit-packed LONG. Lvalues are strings like `id␚udt␚element␚offset`.
2. **Parse, check, emit and format in one step.** No formatter separate from the compiler, no analysis without code
   generation.
3. **Fixpoint by restart.** Late facts (a variable needs a 16-bit address, an array parameter's dimension count, a
   metacommand, a label that is really a SUB call) set a flag and re-run both passes.
4. **C++ is the semantic back end.** Arithmetic typing is delegated to C++ promotion rules; the runtime ABI is
   "whatever libqb's C++ overloads accept", including `passed` bitmasks for optional arguments.
5. **The IDE lives inside the compiler** as a coroutine reading compiler globals (`04`; dropped entirely by the
   VS Code decision, `06`).

## 4. Compatibility contract (what user programs depend on)

Full lists: `01` §11.1, `02` §9.1, `03` §9.1. Items marked ✓ were confirmed by running the old compiler (`09`, `10`).

**Language front end**
- Lexical rules: suffix forms, `&H/&O/&B` typed by digit count with signed wrap (✓ `&HFFFF` = −1, `&HFFFF&` = 65535),
  float literals typed by significant digits, periods in names vs. UDT member access, 40-char names, `?` = PRINT,
  DATA captured raw.
- Name resolution: `x%`, `x&`, `x$` are different variables; `musthave`/`mayhave` suffix rules; four-step lookup;
  scalar and array of the same name coexist.
- Implicit declaration of scalars and arrays (upper bound 10); `OPTION _EXPLICIT` turns it off.
- Operator precedence (16 levels): `MOD` below `\` below `* /`; unary minus below `^` (✓ `-2 ^ 2` = −4); `^`
  left-associative (✓ `2 ^ 3 ^ 2` = 64); `NOT` below comparisons.
- Single-line IF forms, position-dependent DEFxxx, `'$DYNAMIC` from the next line, `$IF` evaluated left to right
  without parentheses.
- Three compile-time evaluators with different rules: CONST (✓ right-associative `^`: `CONST c = 2 ^ 3 ^ 2` is
  512), `$IF`, and ordinary constant folding.

**Numeric semantics** (highest risk of silent differences)
- Float → integer is round-half-to-even (✓ `CINT(2.5)` = 2, `CINT(3.5)` = 4), with no range check on store
  (✓ `x% = 70000` → 4464). INTEGER targets round the **single-precision** value (✓ `d# = 2.5000001: x% = d#` → 2,
  but `l& = d#` → 3).
- Integer `+ - *` on ≤32-bit operands is computed in 32-bit C `int`, no overflow error (✓ `32767 + 1` → −32768;
  `c% * d%` with 200 × 200 prints 40000 but stores −25536). Even two LONG **literals** wrap
  (✓ `2147483647 * 2` = −2).
- `int / int` is computed in `long double`, but prints with 16 digits like DOUBLE (✓ `PRINT 1 / 3` →
  ` .3333333333333333`; QB4.5 printed ` .3333333`). `^` goes through `long double` pow.
- SINGLE literals are emitted as C++ doubles (✓ even `d# = 0.1!` gives the exact double 0.1).
- Comparisons yield −1/0; logical operators are bitwise; float-vs-float comparisons narrow to the smaller type
  (✓ `s! = 2.1: IF s! = 2.1` is true).

**Runtime model**
- By-reference arguments, with a silent by-value temporary (no copy-back) on type mismatch, expression, or `(x)`.
- Arrays: column-major, descriptor layout, static-vs-dynamic rule; `REDIM _PRESERVE` keeps the flat position, not
  coordinates, unless only the last dimension changes (✓).
- Errors and events are serviced only at statement boundaries; `RESUME`/`RESUME NEXT` are statement-granular; a
  runtime error in one PRINT item skips the rest of the statement (`10` §2.1).
- Integer division or `MOD` by zero is **fatal** (critical error 11 from `qb_safe_idiv`/`qb_safe_mod`; not trappable,
  unlike QB4.5) (✓). Float division by zero gives IEEE infinity (✓). `CINT(40000)` raises a trappable error 6 (✓).
- One DATA blob for the whole program in source order.
- Emulated DOS memory is observable: SCREEN 13 at `&HA000`, SCREEN 0 at `&HB800`, BIOS keyboard buffer, DGROUP at
  segment `&H50`, `VARPTR`/`VARSEG` values, DAC ports, INT 33h mouse.
- Bit-exact library behaviour: `STR$`/PRINT number formatting, PRINT USING, `VAL`, the RND generator, `TIMER`
  quantisation, MBF conversions, screen-mode table, GET/PUT image format, INKEY$/`_KEYHIT` codes, file semantics.
- Part of the language is implemented in BASIC: the auto-included files (`beforefirstline.bi`, `aftermain.bas`,
  `afterlastline.bm`, color constants, `vwatch`) and `_IKW_` routines. The new front end must compile them.

## 5. Statement semantics worth knowing (measured)

From `10` (checked by `verification\v09_dim.bas`, `v10_print.bas`):

- **DIM in the main module** with constant bounds (no `$DYNAMIC`) is allocated at program start: the array exists
  even if the `DIM` line is skipped by `GOTO`. Executing a `DIM` of an existing dynamic array, or the same `DIM`
  twice in one procedure call → runtime error 10. `REDIM` of a static array compiles and fails at run time with
  error 10.
- `DIM a(0)` is `0 TO 0` even under `OPTION BASE 1` (recent upstream rule; follow it). `REDIM x(10)` without a type
  reuses the type of an earlier `REDIM x(5) AS LONG`. `STATIC a()` followed by `DIM a(n)` creates a persistent but
  dynamically allocated array. `ERASE` clears a static array but frees a dynamic one (then `UBOUND` → error 9).
- **PRINT** turns each number into `STR$(e) + " "`. Comma zones: 14 columns on a screen and in files; **10 on the
  Windows console**, where `PRINT` with a comma and redirected stdout **never terminates** (`func_pos` asks the
  console cursor). A `;` is auto-inserted next to string literals (`PRINT "a"1`). `PRINT … USING` may follow
  ordinary items in one statement.
- **WRITE** does not escape embedded quotes; a trailing comma leaves the separator. **INPUT** prompts must be string
  literals. **INPUT #** reads every integer narrower than 64 bits through a floating-point reader.
- **Runtime-error UI:** an untrapped error opens a native dialog, even under `$CONSOLE:ONLY`, unless
  `QB64PE_NOPROMPT=y` is set; then the message goes to stderr. Either way the program **exits with code 0**.
- With `-x`, a compile that only issues warnings exits 0, and `-q` hides the warning.

Measured for the first slice of the new compiler (2026-10-03, `m2-workspace-and-slice`; `verification\v13*`,
`tests\corpus\slice\`):

- **A variable is a name plus a type.** `x%` always means (x, INTEGER); plain `x` means (x, SINGLE) until
  `DIM x AS T` and (x, T) from that statement on, so `x` and `x&` are one variable after `DIM x AS LONG` while `x!`
  stays separate. `DIM x%` leaves plain `x` alone. `DIM` of a variable that already exists (even implicitly) is
  "Name already in use", and so is a second `DIM x AS …` of the same plain name with any type; a plain `DIM x`
  after `DIM x AS T` changes nothing.
- `INSTR(0, a$, b$)` does not raise an error: a start of 0 behaves like 1.
- Negating an integer is believed `_INTEGER64` (`-x%` with `x% = -32768` prints ` 32768`); negating a float keeps
  its type. `7E38` is SINGLE and prints `inf`.

Malformed literals (2026-10-04, while checking the PR review of the slice; the new compiler already behaves the
same in the first three):

- A string without its closing quote ends at the line end: `PRINT "hello` prints `hello`.
- `&H`, `&O`, `&B` with no digits are 0: `PRINT &H` prints ` 0 `.
- An exponent letter with no digits is exponent 0: `PRINT 1E+` prints ` 1 `.
- A literal wider than 64 bits is not caught: `PRINT 340282366920938463463374607431768211455&&` fails in the C++
  compiler. The new compiler reports "overflow".
- An `&&` literal above `_INTEGER64` range wraps: `PRINT 18446744073709551615&&` prints `-1`. The new compiler
  reports "overflow"; decided 2026-10-08 to do as QB64pe, as for every out-of-range suffixed literal
  (`m2-numeric-types`, `DECISIONS.md`; QB64pe converts such a literal only where its C++ casts it, §5 below).

Measured for procedures and error handling (2026-10-04, `m2-procedures-and-errors`; `verification\v14_*`,
`v15_*`, `tests\corpus\slice\s08`–`s12`; full tables in `openspec\changes\archive\2026-10-04-m2-procedures-and-errors\design.md`, Context):

- **Arguments:** a plain variable of exactly the parameter's type is passed by reference; anything else (other
  type, expression, `(x)`) goes into a by-value temporary, converted as for an assignment, with no copy-back. A
  string is passed by reference even in parentheses: `addbang (s$)` changes `s$`.
- **A function call prints with the function's own type** (`PRINT twice&(n)` as LONG), not as an integer
  operation (`_INTEGER64`).
- **Scopes follow file order.** A SUB before `DIM SHARED g` does not see `g` (its `g` is an implicit local).
  `SHARED h AS LONG` in a SUB creates main's `h&` if missing and types main's plain `h` as LONG for the code after
  it; plain `SHARED g` always binds the SINGLE `g!`. A local `DIM` shadows a `DIM SHARED` name. A main-module
  `DIM h` after such a `SHARED` makes the C++ compile fail (the new compiler rejects it).
- `DECLARE` is ignored: one that disagrees with the definition, or names nothing, is accepted. `EXIT SUB` and
  `EXIT FUNCTION` are interchangeable inside any procedure.
- **Reserved names:** a keyword, or a built-in written without a required suffix (`LEN`, `CLS`, `NAME`, `ERR`),
  cannot name a variable, parameter or procedure, with any suffix (`len&`). `left` and `chr` are free (`LEFT$`,
  `CHR$` need their `$`); `WIDTH` is free; no name starting with `_` is (1,030 forms checked, `v15_builtin_names`).
  There is no unary `+` (`PRINT +5` is a compile error).
- **Signs the old compiler drops (`v25_signs`, `v25_signs_const`, `v25_x01`–`x10`, measured 2026-10-11; 99
  one-line forms were tried with both compilers):** before it reads an expression it removes signs from its
  text (`fixoperationorder`, `qb64pe.bas` 23558–23601). First **a `+` right after a `+` or a `-` goes**: `a + +b`
  is `a + b`, `a - +b` is `a - b`, `-+b` is `-b`, `"a" + +s$` joins two strings. Then **two `-` right after any
  operator go** (a word operator, a relation and a unary minus count; `(`, `,` and the `=` of an assignment do
  not): `a * - -b` is `a * b` (so `2 * - -i%` is -65536 for -32768, no negation happens), `"a" + - -s$` joins
  two strings, `- - -b` is `-b`, and `- -b` at the start of an expression stays two negations (`s$ = - -s$` is
  "Cannot convert string to number"). Everything else with a `+` and no operand before it is "Expected
  variable/value before '+'" or "… after '*'": `+x`, `2 * +x`, `2 ^ +x`, `(+x)`, `f(+x)`, `NOT +x`, `2 < +x`,
  `+-x`, in `PRINT`, an assignment, `IF`, `FOR`, `CASE`, a bound, an index and an argument alike. A bound
  written `-+3` is one number (the `DIM` is a static array); `2 + +3` is not. **In a `CONST` the evaluator has
  its own rule (`DWD`, `const_eval.bas` 1059–1085): every run of `+` and `-` is one sign, `-` when its number of
  `-` is odd.** So `-+5`, `+-3` and `- - -3` are -5, -3 and -3, `2 - -3` is `2 + 3`, and a run that amounts to
  `+` where a value must start is "Unexpected element '+'": `+5`, `--5`, `- + -5`, `2 * - -3`.
- **Errors:** `RESUME` re-runs the whole statement that raised, including `PRINT` items already printed; `RESUME
  NEXT` skips the rest of it, including the line end. An `ERROR` inside a FUNCTION called from a `PRINT`, with a
  `RESUME NEXT` handler in main, continues inside the function. `ON ERROR GOTO` inside a SUB may name a main-module
  label; a label inside a SUB used that way is a compile error. `ERR` is 0 again after `RESUME`; `ERL` is 0 without
  line numbers.
- `ERROR 0` and `ERROR -1` raise 5; a fractional value is rounded half to even; `ERROR 256` (and 257, 500+) is
  critical and ends the program even with a handler. Under `QB64PE_NOPROMPT=y` an untrapped error ends the
  program; `QB64PE_NOPROMPT=continue` reports it and goes on with the next statement.
- The generated `main0.txt` must start with `error_track_line(0,0,NULL);`; without it the runtime reports a
  critical error with a line number instead of "Enable $ErrorLocation:ON …" (`libqb\src\error_handle.cpp`).

Measured for parser breadth (2026-10-05, `m2-parser-breadth` M1–M8; `verification\v16_*`, one program per
question, include files in `v16_inc\` and `v16_*.bi`):

- **Comment metacommands (M1):** `PRINT "x" '$INCLUDE:'f.bi'` and `PRINT "x": REM $INCLUDE: 'f.bi'` include `f.bi`
  after the statement. `' $DYNAMIC` (blank before `$`) counts; `' note $DYNAMIC` does not (then `REDIM` of the
  `DIM a(5)` array fails at run time, error 10); `'$FOO x$DYNAMIC` counts (anywhere, glued). With two `$INCLUDE`s
  in one comment only the last is included. Blanks around the colon are allowed (`'$INCLUDE :  'f.bi'`).
  **`$INCLUDE` without a comment is a syntax error** (`$INCLUDE:'f.bi'`): only the comment forms exist.
- **`DATA` (M2):** an unquoted item keeps `'`, `REM`, inner `"` and its letter case (`a'b`, `a REM b`, `a"b`);
  **a `:` outside quotes ends the statement** (`DATA a: PRINT 1` runs the `PRINT`); inside quotes it is data.
  Blanks around unquoted items are dropped, quoted blanks kept; an empty item (`,,`, a trailing `,`, `DATA ,`
  gives two) reads as `""` or 0; `&H10` reads as 16. `READ` of a text item into a number is runtime error 2.
- **Line numbers (M3):** `10 PRINT`, `20 :`, `30` alone, `70 '…`, indented `10`, glued `10PRINT`, `10.5`, `10&`,
  `4294967296`, out of order, and the same number in main and in a SUB are accepted. A number after a label on
  one line (`lab: 10 PRINT`) and a number after `:` are syntax errors; `10 lab: PRINT` is accepted. The same
  number twice: "Duplicate label (10)"; `GOTO 99` missing: "Label '99' not defined". A number before a `SUB`
  header is accepted (`10 SUB s`); inside a `SUB`, `10 SUB t` is "Expected END SUB/FUNCTION before SUB"
  (`v16_m3_sub_header`, `v16_m3_nested_sub`, added in the group 5 review).
- **Blocks (M4):** `NEXT j, i` closes both; wrong order or the outer variable: "Incorrect variable after NEXT".
  A `NEXT` inside a multi-line or single-line `IF`, closing an outer `FOR`: "NEXT without FOR". `LOOP` closing a
  `WHILE` or crossing blocks (`DO`/`FOR`/`LOOP`/`NEXT`): "PROGRAM FLOW ERROR!"; `WEND` closing a `DO`: "WEND without
  WHILE"; `END IF` alone or after a single-line `IF …: END IF`: "END IF without IF"; `EXIT FOR` outside: "EXIT FOR
  without FOR". A missing `END IF`/`END SELECT` is reported against the auto-included `vwatch_stub.bm` ("IF without
  END IF in line 3 of internal\support\vwatch\vwatch_stub.bm"). Single-line `IF`: **each `ELSE` binds to the
  innermost `IF`**; `THEN 10 ELSE 20` and `IF 0 GOTO 20` work. `ENDIF` is accepted; `ELSE IF` is an `ELSE` holding
  a new `IF` block. Only comments may stand between `SELECT CASE` and the first `CASE` (a statement: "Expected
  CASE expression"). `EXIT FOR` inside `DO` inside `FOR`, and `EXIT DO` inside `FOR` inside `DO`, work.
- **Blocks, more (M4, group 6, 2026-10-05; `verification\v16_m4_*` added then):** a whole block inside a
  single-line `IF` works (`IF c THEN : FOR …: NEXT: PRINT`; `THEN :` is a single-line `IF`); a `FOR` opened there
  and closed on the next line is "END IF without IF" (the old compiler turns a single-line `IF` into a block with
  an implied `END IF` at the line end). `IF c THEN REM x` is a single-line `IF` (`qb64pe.bas` 25203), `THEN ' x` a
  block. Empty branches (`THEN ELSE PRINT`, `PRINT … ELSE` at the line end) are fine. After a jump (`THEN 10`, `GOTO
  20`, `ELSE 30`) and a `:`, the statements that follow still belong to the branch (`v16_m4_line_if_jump_colon`).
  In a block `IF`, `ELSEIF c THEN stmt` and `ELSE stmt` may carry a statement on the same line, and `ELSE` and
  `END IF` may follow a `:`.
  Crossings: `ELSE` inside a `FOR` inside a block `IF` passes the BASIC checks and fails in C++; `CASE` inside an
  `IF` inside a `SELECT` is "CASE without SELECT CASE"; `END SUB` with a `FOR` open is "FOR without NEXT"; an `IF`
  open at a `SUB` header is "IF without END IF"; `DO WHILE` closed by `LOOP UNTIL` is "PROGRAM FLOW ERROR!".
  `TYPE`: every field form found in the inputs works (`AS LONG b, c`, `AS STRING * 2 t`, `_UNSIGNED _BYTE`,
  element arrays, a nested type); a statement inside, or a missing `END TYPE`, is "Expected element-name AS type,
  AS type element-list, or END TYPE"; a `TYPE` inside a `SUB` is accepted. `DECLARE LIBRARY`: `ALIAS "c_name"`,
  `ALIAS c_name`, `BYVAL`, `~&` work; a statement inside is "Expected SUB/FUNCTION definition or END DECLARE".
  `DEF FN` (either form) is "Command not implemented".
- **Labels (M3, group 6):** a label stands only at the start of a line (after an optional line number): `PRINT
  "a": lab: PRINT "b"` is a syntax error, and `PRINT "a": s: PRINT "b"` calls `s`
  (`v16_m3_label_after_colon`, `v16_m3_call_after_colon`; `qb64pe.bas` has `WHILE … : increaseUDTArrays: WEND`).
- **Reserved names with a suffix:** `name$` and `not$` are variables for the old compiler (checked with
  `qb64pe.exe`; found in `qbasic_testcases`, which also uses `SHARED Key$`); `verification\v15_builtin_names`
  measured only the bare name and `&` for keywords and built-ins without a required suffix, so other suffixes are
  "not supported yet" until measured.
- **Template statements (M5):** a wrong `LINE` form (unknown word, no parentheses, an argument too many) is
  "Syntax error - Reference: <the template>". With variables `B = 7` and `BF = 9`: `LINE …, B` and `LINE …, BF, BF`
  take the first as a colour and the second as the box word; words match in any case (`bf`, `step`). Graphics
  `PUT (x, y), arr()` and file `PUT #1, , v` / `PUT 1, , v` (no `#`) are both accepted; `PUT (0, 0), x` with a
  scalar compiles and fails at run time (error 5).
- **`$IF` (M6):** operators `=`, `<>`, `<`, `>`, `<=`, `>=` (and, read in the source, not measured: `=<`, `=>`,
  `><`), `AND`, `OR`, `XOR`, and a bare name (true when its value is not `0` or empty); names and values are
  compared in upper case. `NOT`, and a value of two words, are "Invalid Resolution of $IF; check statements";
  `(A = 1)` and `DEFINED(A)` are **silently false**. `name = DEFINED` / `name = UNDEFINED` test existence.
  Predefined on Windows 64-bit: `WIN`, `WINDOWS`, `64BIT`, `_QB64PE_` true; `LINUX`, `MAC`, `MACOSX`, `32BIT`,
  `_ARM_` false; `VERSION` compares as a version (`VERSION >= 4.7.0` true). `$LET A = 2` redefines `A`, but
  **`$LET WIN = 0` does not override `WIN`** (it adds a second entry; the bare-name test stops at the first true
  one). `$LET E` without `=` is an error. `$ENDIF` is accepted; garbage and unknown metacommands in an inactive
  branch are ignored, nested `$IF`s there are counted, but their preprocessor lines are still checked (measured
  2026-10-07 with `qb64pe -z`: a `$IF` without `THEN`, a duplicate operator and a second `$ELSE` are errors in an
  inactive branch; `$LET` and `$ERROR` there are skipped, `qb64pe.bas` 1836–1900). `$IF`/`$LET` **only at the start of a line**: after `:` or
  in a single-line `IF` it is "Unexpected character on line". **`$IF` and blocks must nest properly**: `$IF`
  pushes an entry on the block stack and `$END IF` pops the top one, whatever it is (`qb64pe.bas` 3430–3436), so a
  block header inside an active `$IF` closed outside it (one header, or two split by `$ELSE`), and an `IF` opened
  before a `$IF` and closed inside it, are both "END IF without IF" (reported at the `END IF`). A `$IF` inside a
  block, an inactive header, a whole `SUB` inside an active `$IF`, and `EXIT FOR` from inside a `$IF` inside a
  `FOR` are fine. Messages: "$IF without THEN", "$IF without $END IF", "$IF block already has $ELSE statement in
  it", active `$ERROR x`: "Compilation check failed: X".
- **`$INCLUDE` (M7):** a nested include is looked up in the including file's folder, then as written relative to
  **the compiler's own folder**, **never in the main file's folder** (a file only there, compiled from that
  folder, is "File … not found"; `qb64pe.bas` 3120–3171). Every QB64 program, `qb64pe.exe` included, changes to
  its exe's folder at start (`libqb.cpp` 25997), so the caller's working directory plays no part. The upstream
  tests `include_paths\include_fixed_compile_location` and `include_multiple` depend on this
  (`'$include:'tests/compile_tests/extra/include_extra.bi'`). `.\` or `./` at the front is dropped. The same file
  twice is included twice; `$INCLUDEONCE` works on its first line and on a later line. Blocks may cross files (a
  `FOR` closed by a `NEXT` in an include, a `SUB` closed by an `END SUB` in one); a file with a `SUB` included
  inside a `SUB` is "Expected END SUB/FUNCTION before SUB"; at the end of main it is fine. A `$LET` in an include
  reaches the main file, and a `$IF` of the main file may be closed by a `$END IF` in an include (accepted, also
  with a `FOR` around both; measured 2026-10-07; "not supported yet" in the new compiler, like a `$IF` an include
  leaves open). Missing file: "File x not found". There is no cycle check: a self-include stops at 100
  levels ("Too many indwelling INCLUDE files", listing every level), and one guarded by `$IF` and `$LET` is
  accepted (included twice, the second time inactive; `v16_m7_self_guarded`). Messages from inside an include
  name the file by its full path and carry a 0x01 byte before " in line n of …".
- **Member access (M8):** `a.b` without a `TYPE` is a plain variable (separate from `a`); `DIM x.y AS LONG` and
  `x.z$` too. A plain `a.b`, then `DIM a AS t` with member `b`, is accepted and `a.b` is the member from then on.
  With `a` a `TYPE` variable, `a.c` for a non-member is "Element not defined". `a(2) .b` and `a(2). b` (blanks
  around the dot) are member access.

Measured for the control-flow slice (2026-10-06, `m2-control-flow-slice` task 1.1; `verification\v17_*`, one
program per question, `v17_probe_*` the three probes of the change's design; handlers resume next unless named):

- **Pending errors (`v17_a_pending`):** a raising call yields a placeholder (`ASC("")` 0, `CHR$(-1)` `""`, `SQR(-1)`
  0, `(-8) ^ (1 / 3)` 0) and **a scalar store still happens**: after `x = 5: x = ASC("")`, `x` is 0; after `x = 10 +
  ASC("")` it is 10; `s$ = "<" + CHR$(-1) + ">"` stores `"<>"`. **A SUB called with a raising argument is entered
  and returns at once** (every procedure starts with `if (is_error_pending()) goto exit_subfunc;`, `qb64pe.bas`
  5822), so its body does nothing. Array element stores are guarded (`qb64pe.bas` 26957–27021, read, not run).
  `MID$("abc", 0, 1)` does not raise (gives `""`).
- **Errors in block headers (`v17_b_*`, `v17_probe_headers`):** an `IF`, `WHILE`, `DO WHILE` or `DO UNTIL` condition
  that raises lets control into the branch or body; a `LOOP WHILE`/`LOOP UNTIL` condition leaves the loop; a `FOR`
  header that raises in its start, limit or step stores all three from placeholder values and runs the body with
  the variable unchanged (`i = 99` before: the body sees 99, `NEXT` adds the step to it). `IF c GOTO x` with a
  raising `c` **jumps**, and so does `IF c THEN 100` (`IF c THEN 300 ELSE 400` goes to 300; `v17_b_if_then_line`);
  a name after `THEN` (`IF c THEN jumped`) is "Syntax error" (`v17_b_if_then_label`). `RESUME` re-runs the whole header (`FOR i = ASC(CHR$(k)) - 63 TO 4` with `k` fixed runs 2,
  3, 4). Untrapped errors under `QB64PE_NOPROMPT=continue` behave like `RESUME NEXT`. **`ELSEIF` tests the
  placeholder value** (`ELSEIF CHR$(-1) = "" THEN` is taken); when it is false the error stays pending past the
  `ELSEIF` and is handled at the end of the next statement that runs: the next `ELSEIF` (whose branch then runs), or
  the first statement of the `ELSE` branch, a `PRINT` there being skipped by its item check, so with `RESUME` the
  `ELSE` branch's `PRINT` is the statement re-run.
- **`FOR` (`v17_c_*`, `v17_probe_for`):** temporaries (`qb64pe.bas` 6510–6522): `_BYTE` → INTEGER, INTEGER →
  LONG, LONG and `_INTEGER64` → `_INTEGER64`, SINGLE → DOUBLE, DOUBLE and `_FLOAT` → `_FLOAT`; start, limit and step
  are converted to that type with rounding (`FOR i% = 1 TO 2.6` runs to 3, `STEP 0.6` is 1, `STEP -0.6` is -1); the
  step's sign is taken once at the header. An `_INTEGER64` loop near the maximum wraps and goes on (`STEP 5` from
  9223372036854775800 continues at -9223372036854775806). A `_BYTE` loop 120 to 127 step 5 ends at -126. `FOR d# = 0
  TO 1 STEP 0.1` runs 11 times (`NEXT` adds the step to the stored DOUBLE). The variable may be a `TYPE` member
  (`FOR r.v = 1 TO 2` works); a string, a `CONST` or an array element is "Unsupported variable used in FOR
  statement"; a string limit is "Illegal string-number conversion". `NEXT i!` closes `FOR i` (the same variable,
  `v17_c_next_suffix_single`) and `NEXT i&` closes `FOR i` after `DIM i AS LONG` (`v17_c_next_suffix_dim`); `NEXT i` after `FOR i%` is "Incorrect variable after NEXT".
  String conditions: `IF a$ THEN` is "Expected IF LEN(stringexpression) THEN", `WHILE "x"` "WHILE ERROR! Cannot
  accept a STRING type.", `LOOP UNTIL "x"` "LOOP ERROR! …" (and `DO` "DO ERROR! …", `qb64pe.bas` 6355, read).
- **Jumps (`v17_c_jumps`, `v17_d_label_in_block_sub`):** `GOTO` into an `IF` branch runs the rest of the branch,
  then continues after `END IF`. `GOTO` into a `FOR` body that never ran finds the temporaries at 0 and loops with
  the variable at 0 forever (capped in the program); in a SUB the same (the old temporaries there are uninitialised
  locals that happened to be 0). `GOTO` into a `FOR` that ran before reuses its old limit and step. `GOTO` into
  `WHILE` and `DO … LOOP UNTIL` bodies, `EXIT FOR` from a `WHILE` inside a `FOR`, `EXIT DO` from a `FOR` inside a
  `DO`, `EXIT WHILE` from an `IF`: all as expected.
- **`GOSUB`/`RETURN` (`v17_d_*`):** **one return stack for the whole program**: a `RETURN` in a SUB while a main
  `GOSUB` is pending pops main's entry, finds no matching case in the SUB, raises error 3, and main's `GOSUB` is
  lost (its later `RETURN` raises 3 too). A SUB left by `EXIT SUB` with its own `GOSUB` pending leaves the entry on
  the stack. `GOSUB` works from an error handler (`ERR` kept), recursively, in a FUNCTION called from a `PRINT`, in
  a SUB called from a main subroutine. `RETURN label` works in main; **with nothing pending it raises error 3 and
  breaks the stack: the next `GOSUB` crashes** (exit code 139; `qb64pe.bas` 9948 decrements the counter
  regardless). `RETURN label` in a SUB is "RETURN linelabel/linenumber invalid within a SUB/FUNCTION".
- **Labels per body:** the same label in main and in two SUBs is three labels, each `GOTO`/`GOSUB` staying in its
  body. A jump from a SUB to a main label, or from main to a SUB label, is "Label 'x' not defined"; a label twice in
  one SUB is "Duplicate label (a)".
- **`CONST` (`v17_e_*`):** a constant is a compile-time fact: one in a skipped `IF` block, in a single-line `IF` or
  in a `FOR` body is defined all the same. **A main constant's name used before its `CONST` line**, in main or in a
  SUB earlier in the file, plain, as `c1$`, or with a numeric suffix (`c1% = 3`, `PRINT c1&`), **is "Name already
  in use"** (reported at the use); a variable then a
  `CONST` of its name likewise. A SUB's constant may reuse the name of a main constant (it shadows it in the SUB)
  or of a main variable; it is not visible in main (`pc` there is an implicit variable). A parameter may have a
  main constant's name; a `DIM` of it in a SUB is "Name already in use". `CONST a = 1` twice is accepted, with
  different values "Name already in use". A plain constant used as `c%`, `c&`, `c!`, `c#` is the constant; `c$` is
  "Type mismatch". **Typing: an integer result, or a float result with an integer value within `_INTEGER64`
  range, is `_INTEGER64`** (`CONST i3 = 3: PRINT i3 * 1000000000` gives 3000000000 where `3 * 1000000000` wraps;
  `4 / 2`, `2.5 * 2`, `2.5E+10` are `_INTEGER64`); other floats print as DOUBLE (`1 / 3` gives `.3333333333333333`,
  `2 ^ 0.5` 16 digits, `1E+30` gives `1D+30`); `2 ^ 70` gives -9223372036854775808 while `1E+19 / 1` stays `1D+19`.
  `&HFFFF` is -1. `\`, `MOD`, precedence and suffix rounding as at run time (`CONST rh% = 2.5` is 2); `^` is
  right-associative (512). `1 / 0` gives 0; **`1 \ 0` and `5 MOD 0` crash the compiler** ("Runtime error: Division
  by zero", no executable); `(-8) ^ (1 / 3)` is "UNEXPECTED INTERNAL COMPILER ERROR!". Errors: `--5` ("Unexpected
  element '+'"), `LEN(…)` ("Unexpected element 'LEN'"), a variable, `"a" + 1`, `"a" < "b"`, `CONST n% = "x"` and
  `CONST s$ = 5` ("Type mismatch"), assignment to a constant ("Expected variable =, look for conflict with a CONST
  name"), `DIM` of a constant name; `lbl1: CONST k = 4` is "NULL string; nothing to evaluate".
- **`OPTION _EXPLICIT` (`v17_f_*`): it applies to the whole program wherever it stands**: after other statements,
  inside an `IF` block, as the last line, inside a SUB (then main's implicit variables are errors too), twice. A
  variable used implicitly anywhere, before or after the `OPTION` line, is "Variable 'x' (SINGLE) not defined". It
  is satisfied by `DIM`, `CONST`, `DIM SHARED`, `SHARED x` of a declared main variable, `STATIC`, a parameter and
  the function's own name; not by a `FOR` variable or a `SHARED w AS LONG` naming nothing in main. After `DIM x AS
  LONG`, `x&` is fine and `x%` is "Variable 'x' (INTEGER) not defined". `OPTION EXPLICIT` (no underscore) is
  "Expected OPTION BASE or OPTION _EXPLICIT or OPTION _EXPLICITARRAY". `OPTION _EXPLICITARRAY` allows implicit
  scalars and rejects an implicit array ("Array 'a' (SINGLE) not defined"). `SHARED w AS LONG` in a SUB that comes before the main module's `DIM w AS LONG`, and `SHARED x` (SINGLE) beside a main `DIM x AS LONG`, are both "not defined": `SHARED` needs the main variable of that name and type declared earlier in the file (`v17_f_explicit_shared_*`).
- **Operators (`v17_g_*`, `v17_probe_ops`):** the precedence table of `02` §1.3 holds (`NOT 1 = 2` is -1, `5 MOD 3
  \ 2` is 0, `0 _ORELSE 0 OR 2` is -1, `1 _ANDALSO 2 AND 4` is 0, `_NEGATE 0 AND 2` is 2, comparisons chain left to
  right). `NOT`, `AND`, `OR`, `IMP` on INTEGER and LONG compute in 32 bits, with an `_INTEGER64` in 64. **Float
  operands are rounded half to even before `_ANDALSO`, `_ORELSE` and `_NEGATE` too** (`0.4 _ANDALSO 1` is 0,
  `_NEGATE 0.4` is -1). A float beyond 2^63 converts modulo 2^64 (`1E+19 \ 3` is -2815581357903183872). Strings
  compare by unsigned bytes (`CHR$(128) < "a"` is 0). SINGLE and DOUBLE compare at SINGLE (`s! = d#` with both 0.1
  is -1). **`5 IMP 3 IMP 0` gives 7** (the left `IMP`'s text loses its parentheses: `~~5|3|0`); every other
  `IMP`/`EQV` combination tried matches the definition. `^` typing as `02` §1.4 (`2% ^ 0.5` SINGLE, `2& ^ 0.5`
  DOUBLE); `(-8) ^ (1 / 3)` raises 5. `MOD` by zero is fatal like `\` (not trapped). **The smallest LONG or
  `_INTEGER64` divided by -1 with `\` or `MOD` crashes the program** (exit code 127, no output, even with a
  handler). String operands of `AND`, `MOD`, `^`, `-`, `NOT`, `_NEGATE`, `_ANDALSO` and a string compared with a
  number are compile errors.

Measured for the arrays-and-`TYPE` slice (2026-10-07, `m2-arrays-and-types` task 2.1; `verification\v18_*`, one
program per question; handlers print `ERR` and resume next):

- **Store rule per place (`v18_a_*`, `v18_b_member_store`, `v18_h_member_index_order`, `v18_h_string_member`):** an
  **element store evaluates the index first; with an error pending after it the value is not evaluated and nothing
  is stored** (`x(11) = ASC("")` reports 9 only, no element changes); a value that raises is stored as its
  placeholder (`x(9) = ASC("")` leaves 0). A **member store is not guarded**: `u.m = ASC("")` leaves 0, and **a
  member of an element with a bad index writes element 0** (`a(9).m = 5` and `a(-1).m = 6` set `a(0).m`, also for a
  `STRING` member): `array_check` returns 0 after raising (`qbx.cpp` 474). In a member-of-element store the
  **value is evaluated before the index** (C++17 sequences the right side of `=` first), so `a(9).m = ASC("")`
  reports 5 and writes the placeholder 0 into `a(0).m`. **The first error of a statement wins** (`error()` in
  `error_handle.cpp` keeps a pending error). **A read with a bad index gives element 0's value**, not 0 (`y = x(11)`
  is `x(0)`, also for strings and members of elements); `x(x(11) + 3) = 9` computes its index from `x(0)` and is
  skipped.
- **By reference (`v18_c_*`):** an element or a member (also of an element) passed to a parameter of its exact type
  is passed by reference (`bump x(3)`, `CALL bump(x(3))`, `bump u.m`, `bump a(2).m` change it; `f(x(3))` in a FUNCTION
  too); in parentheses, or of another type (`DOUBLE`, `INTEGER` element to a LONG parameter), a copy. **An element
  with a bad index raises 9 and the SUB is not entered** (procedure entry check), so nothing is written, also for
  `a(9).m`. Arguments are evaluated left to right and the first error wins (`two x(9), ASC("")` reports 9, `two
  ASC(""), x(9)` 5).
- **Indexes (`v18_d_*`):** a float index is **rounded half to even** (`x(1.5)` and `x(2.5)` are `x(2)`, `x(0.5)` and
  `x(-0.5)` are `x(0)`, `x(-0.6)` raises 9, `x(10.5)` is `x(10)`), as an assignment to `_INTEGER64`. Indexes
  beyond 32 or 64 bits and `1E+30` raise 9. **Each dimension is checked on its own** (`m(1, 4)` for `DIM m(2, 3)`
  raises 9 though its flat position is inside). Several indexes are evaluated left to right, first error wins. A
  string index is "Illegal string-number conversion"; a wrong number of indexes is "Cannot change the number of
  elements an array has!"; an array name without indexes in an expression (`PRINT x`) is the scalar `x`.
- **Bounds and `DIM` (`v18_e_*`):** `DIM b(-2 TO 3, 5)`, `DIM z(0)` (0 to 0), `DIM one(7 TO 7)`, bounds from
  `CONST`s and any constant expression (`-(2 ^ 2) TO 10 \ 3` is -4 to 3), **float bounds rounded half to even**
  (`DIM a(2.5)` is 0 to 2). `DIM a(-1)` and `DIM a(5 TO 1)` are "Invalid array bounds"; **a second `DIM` of the
  same static array is "Cannot redefine a static array!"**, also after an implicit use (`a(1) = 5` then `DIM a(5)`,
  `v18_g_before_dim`). **The `DIM` of a static array does nothing when it runs** (in a loop, values survive).
  A non-constant bound in the main module compiles (a dynamic array); `DIM t(5)` in a SUB is new on each call. A
  static array of 10,000,001 LONGs works. Every element starts at 0 or `""`.
- **`LBOUND`/`UBOUND` (`v18_e_bounds*`, `v18_f_*`):** both are **`_INTEGER64`** (`UBOUND(b) / 7` prints 16
  digits, `UBOUND(b) * 1000000000` does not wrap; `qb64pe.bas` 22048 `INTEGER64TYPE`, the built-in table's LONG is
  wrong). Without a dimension, the first; the dimension is rounded half to even (`LBOUND(b, 1.5)` is dimension 2); a
  dimension below 1 or above the number of dimensions raises 9 (`func_lbound`, `libqb.cpp` 16291). **`LBOUND(x)` of a
  scalar or an unknown name gives 0**: it makes an implicit array `x()`. `LBOUND(x())` is "Expected ." for numeric
  and `TYPE` arrays.
- **Names (`v18_g_*`):** an array and a scalar of the same name are different variables (`a = 5: a(1) = 2`). An
  array is a name plus a type like a scalar: after `DIM c(3) AS LONG`, `c&(2)` is the same array, but **`c!(1)` is
  another, implicit array**; `DIM e(2) AS STRING` is `e$()`. An array and a FUNCTION of the same name is "Name
  already in use". An element as `FOR` variable is "Unsupported variable used in FOR statement". **An array used
  without `DIM` is implicit**: in the main module 0 to 10 (`z(11)` raises 9); in a SUB, where a main array is not
  seen without `SHARED`, the name is an implicit array of the SUB that raises 9 on every access. `DIM SHARED`
  arrays work in SUBs and FUNCTIONs; `SHARED x() AS LONG` in a SUB works. Element types behave as scalars of their
  type (stores round half to even, `i(1) + 1` wraps on store, `c(1) / 3` prints as DOUBLE for LONG).
- **`TYPE` (`v18_h_*`):** members of every slice type and of another `TYPE`; **the layout is the members in order
  without padding** (`LEN` of INTEGER, LONG, `_INTEGER64`, SINGLE, DOUBLE, `_FLOAT` and a 6-byte inner type is 64:
  **`_FLOAT` takes 32 bytes**); `LEN` of a type with a `STRING` member is "UDT must have fixed size". A member reads
  and stores like a scalar of its type (`p.i = 2.5` stores 2, `p.l / 3` prints as DOUBLE). `p.x&` and `p.s$` with
  the member's own suffix are fine; another suffix (`p.x%`) is "Incorrect symbol after element name"; a member
  declared with a suffix (`x&` or `x& AS LONG`) is an error. **A `TYPE` may be used before its block** (types are
  collected first) and **a `TYPE` block inside a SUB is accepted**. Names like built-ins (`v18_h_type_named_*`,
  `v18_h_member_names*`, corpus `30_type_udt`): `TYPE Point` and `TYPE cls` are accepted, `TYPE len`, `TYPE print`
  and `TYPE long` are "Name already in use"; members `left`, `len`, `color`, `name` are fine, `print` is "Name
  already in use" (as variable names all of these are taken: a rule of its own, not found). `DIM pt AS pt` is fine; `DIM p& AS pt` is
  "DIM: Expected ,". A local `TYPE` variable is zero on each call, a `STATIC` one keeps its value, `SHARED m AS pt`
  and `DIM SHARED` work. A whole value in an expression, `PRINT p` and `y& = p` are "User defined types in
  expressions are invalid"; `p = 5` is "Expected = similar user defined type"; **`q = p` and a whole `TYPE` passed
  to a `TYPE` parameter are accepted** (a copy; by reference). `a.b` for an **array** `a` of a `TYPE` is "Invalid
  expression" (not a dotted name); a member of an element of a numeric array likewise; an unknown member of an
  element is "Element not defined". `STRING` members start empty (also locals on each call), follow the same store
  rules, and pass by reference to a `STRING` parameter.

Measured during parser breadth groups 7 and 8 (2026-10-07, `m2-parser-breadth` session 23; `verification\v19_*`):

- **Procedure names (`v19_proc_names`, 1,948 programs: every keyword and built-in as a SUB called bare and with
  `CALL`, and as a FUNCTION bare and with `&`):** a SUB name is taken only by a built-in **statement** of that
  name without a required suffix (`CLS`, `BEEP`, `WIDTH`), a FUNCTION name only by a built-in **function** (`ABS`,
  `LOC`, `EOF`); `_` names and keywords are taken, a built-in that needs `$` leaves the bare name free (`SUB left`).
  So `SUB loc`, `SUB abs`, `FUNCTION beep`, `FUNCTION close` are accepted, and in an expression the name of such a
  SUB still means the built-in function. Inside `FUNCTION close`, `close = 3` is the result assignment; `end = 5`
  and `system = 5` stay the statements ("Expected variable/value before '='"). Two programs compile with `-z` and
  fail the C++ build (`CALL peek`, `FUNCTION poke`). A name starting with two underscores is valid (`validname`
  refuses a single leading `_` only; `qb64pe.bas` has a parameter `__name$`).
- **Blanks around a dot (`v19_dot_blanks_*`):** the old compiler drops them: `a . s` is `a.s`, for a `TYPE` member,
  a plain dotted name and in `ERASE a . S`.
- **Preprocessor:** besides the predefined names (M6), the old compiler sets `_EXPLICIT_`, `_EXPLICITARRAY_`,
  `_ASSERTS_`, `_CONSOLE_`, `_DEBUG_`, `_SOCKETS_` from the whole program (`qb64pe.bas` 1713–1722, recompiling
  until stable); upstream `precomp-flags/*` test them.
- **Included files:** an `OPTION _EXPLICIT` that stands only in an included file applies to the whole program,
  also to lines before the include (`v19_explicit_in_include`, `v19_explicit_before_include`). An untrapped runtime
  error in an included file reports the line in that file and its name, "Line: 3 (in raise.bi)"
  (`v19_include_runtime_error`; the generated code passes `evnt(line, line in file, "file")`).
- **Template statements (7.5):** each of `LINE (0, 0) (9, 9)`, `LINE … , 1, XX`, `PSET 1, 2`, `CIRCLE (1, 1)`,
  `OPEN "f" FOR INPUT #1`, `LOCATE` with six arguments, `SCREEN 12 13` is rejected ("Syntax error - Reference: …",
  `SCREEN 12 13` "Expected operator in equation"); `tests\frontend\template_errors.bas`.

Measured for the core built-ins, `SELECT CASE` and `ON … GOTO/GOSUB` (2026-10-07, `m2-core-builtins` task 2.1;
`verification\v20_*`, and the C++ the old compiler writes, read with `qb64pe -z`):

- **Argument slots (`v20_a_slots`):** a **LONG slot** (`LEFT$`, `RIGHT$`, `MID$`, `SPACE$`, `STRING$`, `CHR$`,
  `ASC`'s position, `INSTR`'s start, `_TOSTR$`'s digits) converts as a store into a LONG: a float rounded half to
  even to `_INTEGER64`, then the low 32 bits (`LEFT$(s, 2.5)` 2 characters, `3.5` 4, `-0.5` 0; `4294967298#` and
  the `_INTEGER64` 4294967298 are 2; `3E9` is negative). A **`_FLOAT` slot** (`SQR`, `SIN` … `EXP`, `_ATAN2`,
  `_HYPOT`) takes the argument as it is: the C++ passes the value in its own type (`std::hypot(*__INTEGER_I,
  *__LONG_L)`, `func_sqr(*__INTEGER_I)`). A **DOUBLE slot** (`_PI`) converts like a float widening. An
  **any-numeric slot** (`STR$`, `_TOSTR$`, `HEX$`, `SGN`, `ABS`, …) keeps the argument's type.
- **Result types (`v20_b_result_types`, C++):** `ABS`, `INT`, `FIX` have the argument's type (`ABS` of the smallest
  INTEGER is -32768; `INT`/`FIX` of an integer emit the value itself; of a float `std::floor`, `func_fix_double`,
  `func_fix_float`). `SQR`, `SIN`, `COS`, `TAN`, `ATN`, `LOG` are **SINGLE for `_BYTE`, INTEGER, SINGLE; DOUBLE
  for LONG, DOUBLE; `_FLOAT` for `_INTEGER64` and `_FLOAT`**. **`EXP` is SINGLE for 8- and 16-bit integers and
  SINGLE (`func_exp_single`), `_FLOAT` for everything else (`func_exp_float`)** (`qb64pe.bas` 21066). `SGN` is LONG.
  `CINT` INTEGER, **`CLNG` LONG** (the table's INTEGER is wrong), `CDBL` DOUBLE, `_ROUND` `_INTEGER64`, `CSNG`
  SINGLE: **`CSNG` of an integer is not narrowed** (`((double)(e))` typed SINGLE: `CSNG(16777217&) - 16777216` is
  1, `PRINT CSNG(l)` prints `1.677722E+07`). `_PI` is DOUBLE; `_ATAN2`, `_HYPOT` `_FLOAT` (as the table). `ASC`,
  `LEN`, `INSTR` LONG. **`VAL(s$)` is `_FLOAT`; `VAL(s$, SINGLE)` and `VAL(s$, DOUBLE)` are narrowed to their type;
  `VAL(s$, t)` for any integer type `t` is `_INTEGER64`, not narrowed** (`qbs_val<int64_t>`: `VAL("40000",
  INTEGER)` prints 40000); `VAL(s$, _FLOAT)` is `_FLOAT`; `VAL(s$, STRING)` is "VAL TYPE unsupported".
- **String edges (`v20_c_string_edges`):** **a zero or negative length or count gives an empty string and no
  error** in `LEFT$`, `RIGHT$`, `MID$`, `SPACE$`, `STRING$`; a length past the end gives what there is; `MID$`
  with a start of 0 or below starts at 1 (`MID$("abc", 0, 2)` is `a`: the length counts from position 0), past the
  end gives `""`. **`ASC("")`, `ASC(s, 0)`, `ASC(s, past end)`, `ASC(s, -1)` raise 5 and give 0.** `CHR$(256)`,
  `CHR$(-1)` raise 5 and give `""`. `STRING$(n, code)` uses the code's low 8 bits (256 is 0, -1 is 255) without
  error; `STRING$(n, s$)` uses the first byte, **read even from an empty string** (`s->chr[0]`). `STR$` puts a
  blank before a value that is not negative (`-0` too), `D` for a DOUBLE exponent. `_TOSTR$` drops the blank; with
  digits it rounds; digits -1 raise 5. `LTRIM$`, `RTRIM$`, `_TRIM$` remove spaces only (not tabs or NULs).
  `UCASE$`/`LCASE$` change ASCII letters only. `INSTR(0, …)` searches from 1; a start past the end gives 0.
- **`VAL` (`v20_d_val`):** blanks anywhere are skipped (`" 1 2"` is 12), a tab before the number too; `&H`, `&O`,
  `&B` alone are 0, `&HFFFFFFFF` is 4294967295; `1e3` and `1d3` are 1000; junk ends the number (`12abc` 12,
  `1,5` 1); `1e400` is `inf` without error; `VAL("1e30", _INTEGER64)` is the smallest `_INTEGER64`.
- **`HEX$`, `OCT$`, `_BIN$` (`v20_e_radix`, `qb64pe.bas` 20978–21062):** the width a negative value is printed
  with comes from the argument's believed type: 8 bits 2 hex digits, 16 bits 4, 32 bits 8; **64 bits: 16 for a
  variable, element or member, 0 for any other expression** (an integer `+`, `*`, `AND` or negation is 64 bits
  in the old compiler's belief: `HEX$(i% * 1)` with `i% = -2` is `FFFE`). With width 0, libqb's `func_hex`
  prints the shortest width, and **`HEX$` of a 64-bit expression equal to -1 is the empty string**; `OCT$` and
  `_BIN$` print at least 16 bits. A float uses `func_hex_float` (rounded half to even, then 8 digits for a
  negative value); beyond the `_INTEGER64` range it raises 6 and gives `""`.
- **Math edges (`v20_f_math_edges`):** `SQR(-1)`, `LOG(0)`, `LOG(-1)` raise 5 and give 0. `EXP` overflow raises
  6 and gives 0, at the result type's limit (`EXP(709)` overflows: 709 is INTEGER, so SINGLE), `EXP(-1000)` is 0.
  `CINT` outside -32768.5 … 32767.5 (half to even at the ends) raises 6 and gives 0, also for a LONG or
  `_INTEGER64` argument; `CLNG` likewise at its limits. `CSNG(1D+300)` and `CDBL` of a `_FLOAT` beyond DOUBLE raise
  6. `_ROUND(1E30)` gives the smallest `_INTEGER64` without error. Halves round to even in `CINT`, `CLNG`,
  `_ROUND`; `INT` floors, `FIX` truncates. `SGN(-0)` is 0. `_PI(0)` is 0.
- **`LEN` (`v20_g_len`, `qb64pe.bas` 20961):** a string expression gives its length; a variable, element or member
  gives its size (INTEGER 2, LONG 4, `_INTEGER64` 8, SINGLE 4, DOUBLE 8, `_FLOAT` 32, `_BYTE` 1, a `TYPE` its
  layout size, a `STRING` element its length; an implicit variable its default type's). **`LEN(5)` and `LEN(i + 1)`
  are compile errors** ("String expression or variable name required in LEN statement"). The result is LONG.
- **Rejections (`v20_x01`–`x30`):** every one is a compile error: the wrong number of arguments ("Incorrect number
  of arguments - Reference: …"), a string where a number is needed ("Number required for function", for `HEX$`
  "Expected numeric value") and the reverse ("1st function argument requires a string", `VAL(5)` "Expected STRING
  argument"), `LEN$(…)` and `LEFT%(…)` ("Illegal string-number conversion"), a function as a statement ("Syntax
  error"), `SIN()`, `LEN()`, `_PI()` ("Expected (...)"), a mismatched `CASE` item or range ("Expected numeric
  expression" / "Expected string expression"), `ON s$ GOTO` ("Expected numeric expression"), a label of another
  body ("Label 'l1' not defined"). `ON 1.5 GOTO l1, l2` compiles and goes to `l2` (`x28`).
- **`SELECT CASE` (`v20_h_select`, C++):** **a plain variable selector (a parameter too) is read at each test**; any
  other selector (an element, a member, an expression, a FUNCTION call) is evaluated once into a hidden variable of
  its believed type (`i% + 1` an `int64`, an INTEGER element an `int32`, a string a `qbs`), **`static` also in a
  procedure** (recursion overwrites it: `rec(1) = 2`, Q-002 pinned). **Each item is converted to the selector's
  type**: to an integer selector by `qbr_double_to_long` (`CASE 2.4` and `CASE 1.5` match 2, `CASE 2.5 TO 3`
  matches 2, `CASE IS > 1.9` does not), to a float one exactly. A test is `if ((items)||is_error_pending())`: items
  joined by `||`, a range `sel>=a&&sel<=b` (`9 TO 1` never matches), then the body and a `goto` to the end. **An
  error in the selector or in an item runs that `CASE`'s body** (the selector's placeholder 0 matches `CASE 0`; an
  item that raises makes its `CASE` match), as `IF` does. `EVERYCASE` tests every `CASE` (a plain variable changed
  by a body is seen by later tests) and runs `CASE ELSE` only when none matched, through a flag (`sc_N_var`, per
  call in a procedure). A `GOTO` into a `CASE` body runs the rest of that body, then leaves the `SELECT`. A
  `SELECT` with no `CASE` compiles. A `CASE` with no match and no `ELSE` does nothing.
- **`ON n GOTO/GOSUB` (`v20_i_on_goto`, `qb64pe.bas` 27620):** `n` is stored into a `static int32`: an integer as
  it is (the low 32 bits: `_INTEGER64` 4294967298 goes to the second label), **a float through `qbr_float_to_long`,
  which narrows to SINGLE first** (1.5, 2.4, 2.5 go to the second label; DOUBLE 4294967298 raises 5). Then each
  label is tested (`n==k`), **with the value even when `n` raised** (an error in `n` leaves 0: no jump; `ON ASC("")
  + 1` goes to the first label), then `n < 0` raises 5; 0 and values past the count (255, 256, 258, 65537) fall
  through. `ON … GOSUB` pushes a return point and continues after the statement on `RETURN`; it works in a SUB.
- **Parse only, as the language server does it** (`m2-language-server` task 1.3, 2026-10-08, release build, best
  of 5, `parse` with no `sema`): `qb64pe.bas` with its includes (40 trees, 1.3 MB) 111 ms; the largest corpus
  program (`slice\s28_select_case`, 6 KB) 0.2 ms; 1,000 lines with a syntax error on every tenth 1.8 ms (stopped at
  the 100-error cap). The 0.6 s of `study\27` §2 was a whole `--dump tree` process (start, parse, printing the
  trees); a 100 ms debounce costs more than any parse but the old compiler's own source.

Measured for the new numeric types and fixed-length strings (2026-10-08, `m2-numeric-types` group 1;
`verification\v21_*`: 15 programs with output, `v21_x01`–`x36` one rejection each, and the C++ read with
`qb64pe -z`):

- **Declarations (`v21_a_decls`, `v21_a_suffixes`):** `AS` takes `_BYTE`, `INTEGER`, `LONG`, `_INTEGER64`,
  `_OFFSET`, each with `_UNSIGNED`, and `_BIT`, `_UNSIGNED _BIT`, `_BIT * n`, `_UNSIGNED _BIT * n` (`_BIT*7` without
  blanks too). **n is a number literal from 1 to 64** (0 and 65 are errors; a `CONST` name or an expression is
  "Number expected after *"); a `_BIT` **array** stops at 63. `BYTE` without the underscore is "Unknown type";
  `$NOPREFIX` is refused ("deprecated feature"). `_UNSIGNED SINGLE` is "Type cannot be _UNSIGNED", `AS _UNSIGNED`
  alone "Unknown type", but **`_UNSIGNED STRING` and `_UNSIGNED STRING * n` are accepted and behave as `STRING`**.
  Suffixes `%%`, `~%%`, `~%`, `~&`, `~&&`, `%&`, `~%&`, `` ` ``, `` `n ``, `` ~` ``, `` ~`n `` work on implicit and
  DIMmed names (`` `65 `` "Invalid symbol", `` `0 `` an error); `DIM a%% AS _BYTE` is "Expected ,". **One name with
  each suffix is a different variable** (`nm%%`, `nm~%%`, … `nm~`2`: 14 variables). Stores wrap to the width (`b%% =
  200` is -56, `x~% = -1` 65535). `LEN` of `_BYTE` 1, of the unsigned types their width, of `_OFFSET` 8; **`LEN` of
  any `_BIT` variable is an error** ("Variable/element cannot be _BIT aligned").
- **Literals (`v21_a_literals`, `v21_d_const`, C++):** **a suffixed decimal literal is emitted as its own digits**
  (`ll`/`ull` added for the 64-bit types) and only *typed* by its suffix: the value is converted only where the
  generated code casts (`PRINT 300~%%` is `qbs_str((uint8)(300))`, 44; a store), **not inside an expression**
  (`300~%% + 0` is 300, `-1~& < 0` is true, `200%% + 0` is 200, `-1~&& + 0` is -1). In range, the two agree. The same
  holds for `%`, `&`, `&&` (`PRINT 32768%` is -32768). **A `CONST` with a suffix behaves the same way** (`CONST
  b~%% = -1`: `PRINT b~%%` is 255, `b~%% + 0` is -1 and `b~%% < 0` is false, because the evaluator's unsigned value
  is written as `18446744073709551615`). `&H`/`&O`/`&B` literals are converted when read (`&HFF%%` is -1 everywhere;
  `&H1FF~%%` is 511 raw). **Bit literals and bit constants are never converted** and are believed `_INTEGER64`
  (`` PRINT 1` `` is 1, `` 3`2 `` is 3, `` CONST j~`3 = -1 `` prints -1). `18446744073709551616~&&` fails in the C++
  compiler; `5%&` is "Cannot use _OFFSET symbols after numbers"; `2.5~%`, `1.5%%` and `3E2%%` "Unexpected
  character". `HEX$` of a suffixed literal uses the suffix's width (`HEX$(-1%%)` `FF`, `HEX$(-1~&&)` `""` as a
  64-bit non-place, `` HEX$(-1`) `` `F`: a `_BIT * n` gets ceil(n / 4) digits).
- **FUNCTION names (`v21_a_functions`):** every new suffix works, the result stored as into a variable
  (`` fb3`3(5) `` is -3, `fub~%%(300)` 44); **a FUNCTION named with `` ` `` or `` ~` `` (no width) can be defined but
  not called** ("Name already in use", `v21_x16`, `x17`).
- **`_BIT` stores (`v21_b_bit_stores`, C++):** unsigned `v = e & mask`; signed: assign, then sign-extend from bit
  n-1 (`_BIT * 3`: 5 → -3, 13 → -3, -5 → 3; `_BIT`: 1 → -1, 2 → 0). **A float stored into any `_BIT * n` is rounded
  by `qbr` from `_FLOAT`** (half to even, then masked): not the SINGLE or DOUBLE path of `study\02` §1.5
  (`2.5000001#` into `_BIT * 16` is 3, where INTEGER gets 2); beyond `_INTEGER64` the unsigned path of `qbr`
  (`_UNSIGNED _BIT * 64 = 1.8E+19` holds 18000000000000000000). **A `_BIT` value is read through `(int64)`**:
  `_UNSIGNED _BIT * 64` holding 2^64-1 prints -1. In arithmetic a `_BIT * n` up to 32 is its `int32`/`uint32`
  storage, so C++'s rules apply: `u3 * 1000000000` with `u3 = 7` is 2705032704 (32-bit unsigned),
  `b3 - u3` is 4294967286, `u3 > b3` is false for -3, `ub32 > -1` false.
- **Other integer stores (`v21_b_float_stores`):** from a float, **every target of 16 bits or fewer, signed or
  unsigned, is rounded from SINGLE** (`2.5000001#` into `_BYTE`, `~%%`, `%`, `~%` is 2), every wider one from
  `_FLOAT` (3); `1.8D+19` into `~&&` and `~%&` is exact, into `&&` and `%&` the same bits read signed; -1.5 into
  `~%%` is 254.
- **`_BIT` scope, overlap, places (`v21_b_bit_scopes`, `v21_b_bit_overlap`, `v21_x20`–`x30`):** `_BIT` scalars work
  `STATIC`, local (zero on each call), `DIM SHARED`. **A `_BIT * n` above 32 overwrites the `_BIT` scalar of any width
  allocated just before it** (they share conventional memory; other variables are not hit: D-009). `_BIT` arrays
  are bit-packed and work, but a `_BIT * 63` element stored -1 reads 144115188075855871. **Any SUB or FUNCTION with a
  `_BIT` parameter fails the C++ build** (the parameter is declared twice), even when never called; a `_BIT`
  member is "Cannot use _BIT inside user defined types"; a `_BIT` `FOR` variable "Unsupported variable used in FOR
  statement"; `_OFFSET ^ x` "Operator '^' cannot be used with an _OFFSET".
- **Passing (`v21_b_passing`):** a variable of the parameter's width and the other signedness is passed by
  reference (`_BYTE` ↔ `_UNSIGNED _BYTE`, …, and any two of `_INTEGER64`, `_UNSIGNED _INTEGER64`, `_OFFSET`,
  `_UNSIGNED _OFFSET`): the
  procedure reads the bytes with its own signedness and its stores reach the caller. **A `_BIT` variable is always
  passed as a copy**, even to a LONG parameter of its storage width.
- **Fixed-length strings (`v21_c_*`):** n is a literal or a `CONST` name (an expression, a float or a negative is
  "Number/Constant expected after *", 0 "Cannot create a fixed string of length 0"); **n is read as a 32-bit
  integer**: 2147483647 works (2 GB); a value that becomes negative (2147483648, 4294967295, 6442450945) fails in
  C++; 4294967296 is "length 0"; 4294967297 is length 1, 8589934595 length 3. Bytes start NUL (a local on
  each call, a member, an element); a store copies the first n bytes and pads with spaces (`""` gives n blanks);
  reading gives all n bytes; comparison and `SELECT CASE` see the padding; `MID$` as a statement writes inside the
  n bytes only. **Suffix form `name$n`** for `DIM`, implicit variables and parameters (`p$3` is another variable
  than `p$`). **A fixed string passed to a `STRING` parameter is passed by reference, also in parentheses, member or
  element**: the procedure's stores reach it, cut and padded (`t = "longer text"` leaves `longe`). **A parameter
  declared `STRING * n` (or `t$n`) is an ordinary `STRING` parameter except that `LEN(t)` is the constant n**: any
  string is accepted and changed by reference uncut (a `STRING` variable becomes `changed!`), a `STRING * 3`
  argument reports `LEN` 5. `FUNCTION f$5` returns a 5-byte padded string.
- **`FOR` (`v21_d_for`, C++):** the hidden copies are `int16` for 8-bit variables, `int32` for 16-bit, `int64` for 32-
  and 64-bit ones and `_OFFSET`, signed or not, as the control-flow spec says; so `FOR u~% = 2 TO 0 STEP -1` ends
  after 0 (the hidden -1 is past the limit, `u~%` is 65535), and **an unsigned 64-bit limit above 2^63 is negative in
  the hidden copy** (`FOR uq~&& = 1 TO 18446744073709551615~&&` runs no pass).
- **`SELECT CASE` (`v21_d_select`, C++):** **an integer item is compared in C++ as written, not converted to the
  selector's type** (`CASE -1` misses a `~%` 65535 but matches a `~&` 4294967295 and a `~&&` 2^64-1; `CASE 0 TO -1`
  matches a `~&&`); a float item is rounded to the selector's type (`qbr_double_to_long`, for a 64-bit unsigned
  selector `qbr_longdouble_to_uint64`). The hidden copy of a non-variable selector is `int32` for signed up to 32
  bits (also `_BIT` elements), `uint32` for unsigned up to 32, `int64`/`uint64` for 64 bits and `_OFFSET`; an
  expression's copy has its believed type (`ub + 0` `int64`, `uq + 1~&&` `uint64`).
- **Built-ins (`v21_d_builtins`, C++):** `HEX$`/`OCT$`/`_BIN$` of a variable use its width (`_BIT * 3` -4 is `C`, `4`,
  `100`); `ABS` keeps the type (`ABS(uq)` `func_abs((uint64)…)`); `SGN` of an unsigned is 1; `INT`/`FIX` of any
  integer is the value; `CINT`/`CLNG` raise 6 past their range (`CLNG` of an unsigned 64-bit uses
  `func_clng_uint64`); `_ROUND` of an integer is the value (typed `_INTEGER64`, an `_OFFSET` argument keeps its
  type); `SQR`/`SIN`/… are SINGLE for 8- and 16-bit and `_BIT * 3`, DOUBLE for 32-bit, `_FLOAT` for 64-bit, `_OFFSET`
  and `_BIT * 40` (other `_BIT` widths not run); `EXP` SINGLE for 8- and 16-bit, `_FLOAT` for the rest (`_BIT * 3`
  too).
  **`VAL(s$, <unsigned type>)` is libqb's `qbs_val<uint64_t>`: not narrowed, and the minus sign is dropped**
  (`VAL("-1", _UNSIGNED LONG)` is 1, `VAL("300", _UNSIGNED _BYTE)` 300); `VAL(s$, _BIT)` is "VAL TYPE unsupported".
  A LONG slot takes the low 32 bits (`LEFT$(s, uq)` with `uq` 4294967298 is 2 characters).
- **Every scenario of the change's spec deltas that the old compiler can run gives the delta's expected output**
  (`v21_e_scenarios`, and `v21_f_scenarios` for those added by task 1.6); the corrections to the deltas are listed
  in the change's `tasks.md` tasks 1.5 and 1.6.
- **Where a suffixed literal is converted (`v21_f_literal_uses`, `v21_f_radix`, task 1.6):** the literal is held as
  written (the C++ type of its digits) and converted to its suffix's type only by `PRINT`, `STR$` and a built-in
  that keeps its argument's type (`ABS(-1~&)` 4294967295, `ABS(300~%%)` 44); parentheses keep the suffix's type
  (`PRINT (300~%%)` 44). A store converts the held value to the target (`l& = 300~%%` 300, `u~%% = 300~%%` 44,
  `k% = 40000%` -25536), as do a procedure argument by value, a built-in's LONG slot (`CHR$(321~%%)` raises 5), a
  `CASE` item (`CASE 300~%%` misses 44 and matches 300), a `SELECT` on such a literal, a `FOR` limit, an `IF`
  condition and an array bound (`DIM a(258~%%)`: `UBOUND` 258). Unary minus on a parenthesised literal or a constant
  and `NOT` are not believed the suffix's type (`-(300~%%)` -300, `NOT 300~%%` -301). `HEX$` takes the held value
  (`HEX$(300~%%)` `12C`, `HEX$(40000%)` `9C40`). A suffixed `CONST` behaves the same (`CONST c~%% = 300`: `PRINT`
  44, a store into a LONG 300). Bit literals and constants the same, never narrowed (`` 9`3 `` 9 everywhere). An
  unsigned radix literal wider than its type holds its whole value (`&H1FF~%%` prints 255, `+ 0` 511; `&O777~%%`,
  `&B111111111~%%` likewise); a signed one is "Overflow" at compile time (`v21_x37`–`x41`).
- **A parameter declared `STRING * n` (`v21_f_fixed_param`):** only `LEN` of the parameter itself is n; the value is
  the caller's string uncut (`RIGHT$`, `MID$`, `INSTR`, comparison, `SELECT CASE`, `LEN(t + "!")` see the real
  string), a store is not cut and reaches a `STRING` caller (a fixed caller cuts it with its own length), and the
  parameter passed on to a `STRING` parameter is the real string. `t$4` the same. A SUB named `s` and a variable
  `s$` cannot both exist ("Name already in use"). **On a parameter, `_UNSIGNED STRING`, with or without a length,
  is "Illegal SUB/FUNCTION parameter"** though `DIM` takes it; `t$0` is "Invalid index after STRING * type" and
  `STRING * 4294967296` (0 in 32 bits) "Invalid number after STRING * type" (`v21_x42`–`x45`, 2026-10-09, task 4.3).
- **The new types everywhere else (`v21_g_*`, `v21_x49`, 2026-10-09, task group 8):** **an array element or a
  `TYPE` member of the other signedness is passed by reference too**, like a variable (`a(1)` of an `_UNSIGNED
  INTEGER` array holding 65535 is -1 in an INTEGER parameter and 65533 after `x = -3`; an `_UNSIGNED _INTEGER64`
  member reaches an `_OFFSET` parameter); in parentheses a copy. **A FUNCTION `f$5` is also called and assigned as
  `f$` and `f`**; `f$4` is "Name already in use"; never assigned it returns 5 NUL bytes; the C++ allocates the
  result with `qbs_new_fixed(mem_static_malloc(5),5,0)` on each call and returns it with `qbs_maketmp`, over pool
  bytes released on return, **so a second call in the same expression overwrites the first result**
  (`fs$5("ab") + fs$5("cd")` is `cd   cd   `, `v21_g_fixed_function_twice`; fixed by decision, `DIVERGENCES.md`
  D-014: the new compiler returns a copy). **A constant
  used with another integer suffix is a literal of that suffix with the constant's digits** (`CONST u~& =
  4294967295`: `u%` prints -1, `u% + 0` 4294967295; `CONST neg = -1`: `neg~&` 4294967295, `neg~& + 0` -1, `neg~& <
  0` false, so a negative value is written as its unsigned 64-bit digits as for a suffixed `CONST`; a float is
  rounded half to even, `fl~%` of 2.5 is 2); a float suffix converts. **A `CONST` with an `_OFFSET` suffix is an
  `_INTEGER64` literal** (`5ll`, `18446744073709551615ull` in the C++): `k~%& + 0` of `CONST k~%& = -1` is believed
  `_INTEGER64` and prints -1. Read in `qb64pe.bas` (`evaluatefunc`, 20978–21215, 22071) for the built-ins: a `_BIT *
  n` counts n bits (`HEX$` (n + 3) \ 4 digits, place or not; `SQR` SINGLE up to 16 bits; `CINT`/`CLNG` range checks by
  n), `EXP` of any `_BIT` is `_FLOAT`, `_ROUND` of an `_OFFSET` keeps its type, `VAL` with any unsigned integer type
  is `qbs_val<uint64_t>` typed `_UNSIGNED _INTEGER64`.
- **Rejections measured while bringing the types in (`v21_x46`–`x57`, 2026-10-09, task groups 4 and 8 and the review
  after them):** `_UNSIGNED` before a user type is "Type cannot be _UNSIGNED" (`x46`); a FUNCTION named `` fb` ``
  compiles while it is not called (`x48`) and a call is "Name already in use" (`x47`). **A name `x$n` beside a
  procedure of another type is "Name already in use"**: beside a SUB `s`, `s$5` as a store, a value or in `DIM`
  (`x50`–`x52`), and beside a FUNCTION `fl&`, `fl$5` (`x53`). A FUNCTION `fs$5`'s name is a procedure's only for
  variable names and calls: `SUB fs$5` is "Type symbols after a SUB name are invalid" (`x54`), `CONST fs$5` "Invalid
  constant type" (`x55`), `CALL fs$5(…)` "Expected CALL sub-name" (`x56`), a `TYPE` member `fs$5` "Invalid name"
  (`x57`). `qb64rust` reads `name$n` as a procedure's name in one place (`Checker::proc_fixed_name`) and keeps the
  forms of `x54`–`x57` "not supported yet".
- **From the differential recordings (`tests\differential`, task 2.3):** all 59 programs build and run to their
  last line with `qb64pe.exe`, and two recordings agree. **A `_FLOAT` literal is written as a C++ double**:
  `1.18973149535723176F+4932` is infinite, so the largest `_FLOAT` a program can write is about
  `1.797693134862315F+308`. **The infinite text of D-010 is not only padding**: after `INF` come NUL bytes and the
  remains of a number formatted earlier (`-INF<NUL>2979E+11`, `INF<NUL>0000E+001<NUL>…D…`), the same on every run
  of a program. **A negative literal is held in the C++ type of its text** in arithmetic (design D7, shown for the
  six old types): `(-2147483648&) * (-32768%)` is 70368744177664 and `(-2147483648&) + (-2147483648&)`
  -4294967296, because `2147483648` is a 64-bit C++ literal; `qb64rust` wrapped both to 32 bits until task group 4
  of `m2-numeric-types` (2026-10-09), which holds such literals as design D7 says: all 48 `old6` programs pass since.
- **The operator rules, corrected against the differential programs (task 5.3, 2026-10-09):** all 59 programs pass
  (and the 40 `fold` ones with folding off). What `study\02` §1.4 and the design read differently: **the markup
  believes `_UNSIGNED _INTEGER64` when both operands are unsigned and the wider one is 64 bits** (`ub~%% * uq~&&`
  is believed unsigned; `study\02` said "both unsigned 64-bit"), a `_BIT * n` counted as n bits; **a `_BIT` value is
  believed its own type** (width and signedness) in that markup and only printed through `int64` (design D3 had
  "believed `_INTEGER64`"): `UBIT7 + UINT64` prints unsigned. **A unary operator's markup sees its operand on both
  sides** (`qb64pe.bas` 19888: `-uq~&&` and `NOT uq~&&` are believed unsigned, `NOT ub~%%` `_INTEGER64`). **`EQV` and
  `IMP` are `~a^b` and `~a|b`**: `~` complements the left operand in its own promoted width before C converts it
  (`u~& EQV q&&` with all ones and 0 is 0); `qb64rust` wrote `~(a^b)`. **A literal whose suffix's type is wider than
  32 bits gets `ll` or `ull`** (`qb64pe.bas` 19730), so `` 0~`40 `` is a `uint64` (`` 0~`40 > -1 `` is false) and a
  `_BIT` literal is believed its `_BIT` type (printed through `int64`, never narrowed). `_ANDALSO`/`_ORELSE` are
  believed like the other integer operators (two unsigned 64-bit operands: true prints 18446744073709551615). With an
  `_OFFSET` operand (`qb64pe.bas` 19950–19988): `*` with a float and `/` compute in `long double` and round by `qbr`;
  every other operator, comparisons included, rounds a float operand first (`o < 7.4` with `o = 7` is false). The
  accidents among these are in `SOMEDAY.md` "QB64pe behaviours to review"; `s31_unsigned_ops` shows each.

Measured for built-in statements (2026-10-10, `m2-builtin-statements`; `verification\v22_*`, the C++ read with
`qb64pe -z`):

- **Statement calls (`v22_a_calls`, `v22_a_resume`, `v22_a_args`, `v22_x01`–`x20`):** a plain built-in statement is
  one call of its libqb entry with the arguments in order and no test between them (`sub_kill(func_chr(-1))`,
  `sub_name(a,b)`); **the call is made also after a raising argument, and the first error is the one serviced**:
  `sub_kill`, `sub_mkdir`, `sub_rmdir`, `sub_chdir`, `sub_name` and `sub_seek` return at once while an error is
  pending, `sub_environ` does not test and raises its own error 5 for the placeholder, which changes nothing (`KILL
  CHR$(-1)`, `NAME CHR$(-1) AS …`, `ENVIRON CHR$(-1)`: the handler runs once, with 5). `RESUME` runs the whole
  statement again, arguments included (a FUNCTION in the argument runs twice). Errors: `KILL` of a missing file or
  of `""` 53; `MKDIR` of an existing folder 75, of `""` 76; `CHDIR` and `RMDIR` of a missing folder 76; `NAME` of a
  missing file 53; `ENVIRON` of a text with neither `=` nor a blank (also `""`) 5, and **a blank separates name and
  value as `=` does** (`ENVIRON "A b"` sets `A` to `b`); `SEEK` to position 0 or below 63, on a number that is not
  open (also 0) 52. The same in a SUB, where the error is serviced at the statement in the SUB. **Arguments:** a
  string argument is passed as it is (a variable's `qbs`, also in parentheses `KILL (s)`, a fixed-length string, an
  element); `SEEK`'s numbers are written as they are when integers (`sub_seek(*__LONG_L,*__INTEGER_I)`, C converts)
  and through `qbr(…)` when floats, for both the LONG file number and the `_INTEGER64` position (`SEEK 1.4, 2.5` is
  file 1, position 2; `SEEK 1, 3.5` position 4). **`CALL KILL("f")` is accepted** (`x16`). Rejections: a number for
  a string "String required for sub" (one argument) or "1st/2nd sub argument requires a string", a string for a
  number "… requires a number"; a missing, extra or wrongly separated argument "Syntax error - Reference: KILL
  fileSpec$" (the statement's own syntax line); `KILL ("a", "b")` "Invalid expression"; `KILL$ "a"` "Syntax error".
- **How a template becomes a call (`qb64pe.bas` `seperateargs`, read 2026-10-10; ported as
  `crates\builtins\src\passing.rs`):** every `?` is a C argument (its value, or `NULL` when left out); a choice with
  several alternatives is a C argument too (the alternative's number from 1, or `NULL`); a choice with one
  alternative never is. Each optional block `[…]` that holds a `?`, or only one-alternative choices, gets the next
  bit of the trailing `passed` mask (blocks of the outermost level first, then the next level, left to right); a
  block with a several-alternative choice needs none. The table's `arg_types` are the types of the C arguments in
  order. Checked against the C++: `sub_open(name,mode,access,lock,number,length,passed)` with modes `RANDOM` 1,
  `BINARY` 2, `INPUT` 3, `OUTPUT` 4, `APPEND` 5, access `READ WRITE` 1, `READ` 2, `WRITE` 3, lock `SHARED` 1, `LOCK
  READ WRITE` 2, `LOCK READ` 3, `LOCK WRITE` 4, and bit 1 for `LEN =`; `sub_open_gwbasic(mode$,number,name$,length,
  passed)`; `LINE STEP(1,2)-STEP(3,4), 5, BF, 6` is `sub_line(1,2,3,4,5,2,6,31)`.
- **Files: `OPEN`, `CLOSE` (`v22_b_open`, `v22_x26`–`x29`, `x41`, `x42`, `x51`):** every mode, access and lock word
  compiles and opens; no `FOR` is `RANDOM` (mode `NULL`). The file number is a LONG slot: `AS #1.5` and `AS d` with
  2.5 open file 2 (`qbr`), 4294967297 opens file 1 (C++ keeps the low 32 bits), 0 and -1 raise 52, 256 and 70000
  are fine. Opening a number that is open: 55; a missing file for `INPUT`: 53 (`BINARY`, `APPEND` create it); `""`:
  76; `LEN = 0`: 5, `LEN = -1` and `2.5` accepted; `FOR INPUT ACCESS WRITE` 75; `FOR OUTPUT ACCESS READ` 53 (odd,
  libqb's). The old form takes the first letter of the mode string in either case (`"INPUT"` works), a bad or empty
  one raises 54. **`CLOSE` is one `sub_close(n,1)` per number, in order, or `sub_close(NULL,0)` alone**; `CLOSE
  1.5` closes 2; a number that is not open (also 0, -1, 300) is no error; a trailing comma is accepted (`x50`).
  Rejections: a string for a number "5th sub argument requires a number" (the C argument's place), a number for
  the name "1st sub argument requires a string", an unknown mode word and a second `FOR` "Name already in use
  (FOR)", no `AS` "Syntax error - Reference: OPEN …", `CLOSE "a"` "Illegal string-number conversion".
- **Files: `PRINT #` (`v22_b_print`, C++):** `tab_spc_cr_size=2; tab_fileno=tmp_fileno=<number>;`, a test for a
  pending error, then **one `sub_file_print(tmp_fileno, text, extraspace, tab, newline)` per item, each followed by
  the test** (`goto skipN`), and `tab_spc_cr_size=1;` after the skip label. A string is passed as it is with
  `extraspace` 0, a number as `qbs_str((type)(x))` with `extraspace` 1 (libqb adds the blank); `tab` is 1 when a
  comma follows the item; `newline` is 1 on the last item unless the statement ends with a separator. A comma with
  no item before it is `sub_file_print(tmp_fileno,nothingstring,0,1,0)`, `PRINT #1,` is `(…,nothingstring,0,0,1)`,
  `PRINT #1, ;` writes nothing. **Zones are 14 columns in a file** (`"a", "b"` is `a` and 13 blanks; a 14-character
  item is followed by 14 blanks). A raising item leaves what was written before it without a line end (`ra`, then
  the next statement's text on the same line). **No automatic `;`**: `PRINT #1, "a" "b"` and `"a" 1` are "Expected
  operator in equation" (`x54`, `x55`), unlike console `PRINT`; `PRINT #1 "x"` is "Expected # ... ," (`x25`), a
  string for the number "Illegal string-number conversion" (`x46`). `TAB` and `SPC` work in it (not compiled yet).
  A number that is not open (also 0, -1) raises 52, a file open for input 54; `#1.5` is file 2.
- **`WRITE` (`v22_b_print`, `x39`, `x56`–`x60`, C++):** each item is a string built in C++: a number is
  `qbs_ltrim(qbs_str((type)(x)))`, a string is `"` + s + `"` (a quote inside is not doubled); every item but the
  last gets `,` appended (`qbs_add(…,qbs_new_txt_len(",",1))`), and the last ends the line. To a file each is
  `sub_file_print(tmp_fileno,text,0,0,newline)` inside the frame of `PRINT #`, to the console `qbs_print(text,
  newline)`, each followed by the pending-error test. **A trailing comma is accepted and gives `1,` without a line
  end**; no items give an empty line; `;` between items is "Invalid expression" and adjacent items "Expected
  operator in equation". A raising item leaves `"ra",` without a line end.
- **`INPUT #`, `LINE INPUT #` (`v22_b_input`, `x21`–`x24`, `x40`, `x45`, `x47`–`x49`, C++):** `tmp_fileno=<number>;`,
  the pending-error test, then per target, each followed by the test: a string target `sub_file_input_string(
  tmp_fileno, place)`, `LINE INPUT` `sub_file_line_input_string(tmp_fileno, place)`; a numeric target is a store
  of `func_file_input_float(tmp_fileno, typecode)` (every integer narrower than 64 bits and every float),
  `func_file_input_int64(tmp_fileno)` (`_INTEGER64`, `_OFFSET`) or `func_file_input_uint64` (their unsigned
  forms), by the store rule of its place (an element: index into `tmp_long` first, the store skipped while an error
  is pending; a `_BIT` with its mask or sign extension around `((int64)func_file_input_float(…))`). **The type
  code** is the old compiler's type value: the width in bits, plus `0x20000000` for a float, `0x10000000` for
  unsigned, `0x08400000` for a variable (`0x08C00000` an element, `0x08600000` a member), and `0x03000000` more
  for a `_BIT`; libqb reads the width, the float bit and the unsigned bit only. Measured: a value outside the
  target's range (300 into `_BYTE`, -1 into an unsigned type, `&HFFFF` into INTEGER, `1e40` into SINGLE) **raises 6,
  stores 0 and skips the later targets**, whose fields stay unread; a float is rounded half to even into an
  integer narrower than 64 bits (2.5 is 2) **but 2.5 into `_INTEGER64` is 3** (another reader); text that is no
  number is 0 (`12x` is 12); `&H10` is 16; an empty or blank field is 0; a quoted number is 0 for a numeric target.
  String fields lose the blanks around them, keep everything inside quotes and drop what follows a closing quote
  up to the comma. Fields are also ended by LF, CR and CR LF; with fewer fields on a line than targets the next
  lines are read. Past the end: 62, and the target is set to empty or 0. A target with a raising index reads
  nothing (error 9) and ends the statement. `LINE INPUT #` takes one string place (a variable, an element, a
  fixed-length string, a member); it reads a last line without a line end and works on a `BINARY` file, where
  `INPUT #` raises 54. Not open: 52; a file open for output: 54 (`INPUT #`), 75 (`LINE INPUT #`). Rejections: a
  numeric `LINE INPUT` target "Expected string-variable", a second target "Too many variables"; a literal, an
  expression or a `CONST` as a target "Expected variable-name"; no target "Expected , ..."; a whole `TYPE` variable
  "Unexpected internal code reference to UDT". **`INPUT #1, a()` (a whole array) is accepted** (`x49`).
- **File functions (`v22_b_funcs`, `x31`–`x38`, `x43`, `x44`, `x52`, `x53`):** `EOF`, `LOF`, `LOC`, `SEEK`,
  `FREEFILE`, `_FILEEXISTS`, `_DIREXISTS` are believed LONG (`qbs_str((int32)(func_lof(1)))`) and `_CWD$` a string;
  **in libqb `func_lof`, `func_loc` and `func_seek` return `int64`** (so `LOF(1) * 1000000000` is computed in 64
  bits and printed through `int32`), `func_eof`, `func_freefile` and the two existence functions `int32`. The file
  number is a LONG slot (`func_loc(qbr(*__DOUBLE_D))`). `FREEFILE` and `_CWD$` are called without parentheses only
  (`func_freefile()`, `func__cwd()`): `FREEFILE(1)` and `_CWD$(1)` are "Incorrect number of arguments", `FREEFILE()`
  "Expected (...)", `DIM FREEFILE` "Name already in use", `FREEFILE = 1` and `_CWD$ = "x"` "Expected variable =",
  `_CWD` "Invalid variable name". `FREEFILE` gives the lowest free number, the same until a file is opened. `EOF`
  is -1 on an empty input file and after the last line, 0 on an output file; on a number that is not open (also 0
  and -1) each of the four raises 52. `SEEK` gives the next byte's position from 1 (`RANDOM`: the record), `LOC` 0
  on a sequential file here. `_FILEEXISTS` is 0 for a folder and `_DIREXISTS` 0 for a file; `""` gives 0. Also
  measured: `NAME` onto an existing file 5; `NAME` and `KILL` of an open file 75; `RMDIR` of a folder that is not
  empty 75; `KILL` of a folder 53; `KILL` with `*` removes every match and raises 53 when nothing matches.
- **`DATA` (`v22_c_order`, `x69`, `x73`, `x85`, C++):** the program's data is every `DATA` item **in file order**:
  the main module, a block that never runs (`IF 0 THEN`, also on one line), an included file at its include, the
  lines after `SYSTEM`, each SUB and FUNCTION where it stands, a `DATA` between two procedures, a `DATA` after a
  colon. **Bare `DATA` is one empty item.** In `global.txt`: `ptrszint data_size=<bytes>;`, `uint8 inline_data[]={`
  the bytes as decimal numbers `,0};`, `uint8 *data=&inline_data[0];` (without data: `uint8
  *data=(uint8*)calloc(1,1);`). The bytes are each item's text followed by a comma, an unquoted item without the
  blanks around it, a quoted one in its quotes, with the closing quote added when the line ended first. Text after
  a closing quote is "Expected , after quoted string in DATA statement".
- **`READ` (`v22_c_types`, `v22_c_errors`, `x66`, `x67`, `x70`, `x71`, `x74`–`x79`, `x84`, `x87`, `x90`, C++):** one
  line per target and **no test for a pending error between them**: `*__LONG_L=func_read_float(data,&data_offset,
  data_size,<type code>);` for every numeric type but the 64-bit integers (the type code as for `INPUT #`; a `_BIT`
  through `((int64)func_read_float(…))` and its store mask), `func_read_int64` / `func_read_uint64` without a code
  for `_INTEGER64` and `_OFFSET`, `sub_read_string(data,&data_offset,data_size,<place>)` for a string; an element is
  stored by its own rule (`tmp_long=array_check(…); if (!is_error_pending()) …=func_read_float(…)`). In libqb each
  reader returns at once while an error is pending (0, a string target left as it is) and **puts the position back
  when it raises**, so: an item that is no number raises 2 and one outside the target's range 6, the target gets
  0, the later targets of that `READ` get 0, and the next `READ` meets the same item again; with the data used up,
  error 4, 0 or `""`. A target with a raising index takes no item (error 9). No number: any unquoted text that is
  no number, **every quoted item, also `"12"`**, `1 2`, `--1`; an empty item is 0, `1e` is 1. Number forms: `&H10`,
  `&O17`, `&B101`, `1e3`, `1d3`, `1f2`, `+5`, `.5`; `&HFFFFFFFF` is -1 in a LONG and 4294967295 in an `_INTEGER64`.
  **2.5 read into an `_INTEGER64` or `_OFFSET` is 3 and `.5` is 1** (half away from zero), into the narrower types
  2 and 0 (half to even), as for `INPUT #`. A string target takes any item's text; a fixed-length one is padded or
  cut. Rejections: a literal, an expression, a `CONST`, a function call or **the name of the FUNCTION the statement
  stands in** as a target "Expected variable" (`x87`; `INPUT #` and `LINE INPUT #` the same with "Expected
  variable-name", `x88`, `x89`); no target "Expected variable"; a whole `TYPE` variable "Unexpected internal code
  reference to UDT". **Accepted: `READ (a)`, which reads into `a` (`x84`), and `READ a()` (`x70`: only `a(0)` is
  set).** A parameter is a target like any variable (`x90`).
- **`RESTORE` (`v22_c_restore`, `x65`, `x68`, `x72`, `x80`–`x83`, `x86`, C++):** `data_offset=0;` or
  `data_offset=data_at_LABEL_<NAME>;`, with `ptrszint data_at_LABEL_<NAME>=<byte offset>;` in `global.txt`: the
  offset of the first item after the label in file order (a label on the `DATA` line itself counts as before it),
  `data_size` when no `DATA` follows (the next `READ` raises 4). **The label has no scope**: one of the main module
  is named from a procedure and one of a procedure from the main module or another procedure; a name that two
  bodies have is "Ambiguous DATA label" when a `RESTORE` names it (`x72`, `x83`; without a `RESTORE` it is accepted,
  `x82`). A line number is a target too (`RESTORE 100`). An unknown label "Label 'nowhere' not defined"; two
  targets or an expression "Syntax error - too many parameters"; a string "Invalid label". `MID` cannot be a label
  ("Invalid label").
- **`SWAP` (`v22_d_swap`, `x91`–`x107`, `x135`, `x136`, C++; `qb64pe.bas` 11377):** one call chosen by the first
  operand's type: `swap_8`, `swap_16`, `swap_32`, `swap_64` by the width (`swap_32(&*__SINGLE_S1,&*__SINGLE_S2)`,
  `swap_64` for DOUBLE and `_OFFSET`), `swap_longdouble` for `_FLOAT`, `swap_string(a,b)` for strings, and for a
  whole `TYPE` `swap_block(<second>,<first>,<size>)` (`swap_8`…`swap_64` when the size is 1, 2, 4 or 8). The two
  types must be equal **but for their signedness** (`SWAP l&, u~&` is accepted and exchanges the bits: -1 becomes
  4294967295); any two strings go together, also `STRING` with `STRING * n` and two fixed lengths (each side is
  cut or padded to its own length). An element is addressed inside the argument (`&((int32*)(…))[array_check(…)]`)
  with no test after it, so **an operand with a bad index raises 9 and the exchange is made with the array's
  first element**. Rejections: types that differ "Type mismatch" (LONG and DOUBLE, INTEGER and LONG, SINGLE and
  DOUBLE, `_OFFSET` and `_INTEGER64`, a string and a number); two `TYPE`s "Expected SWAP with similar user defined
  type"; **a `_BIT` of any width "Cannot SWAP bit-length variables"**; a literal, an expression, a `CONST` or the
  name of the FUNCTION it stands in "Expected variable"; one operand or three "Expected SWAP ... , ...". Accepted:
  `SWAP (a), b` (`x103`) and two whole arrays, `SWAP x(), y()` (`x101`).
- **The `MID$` statement (`v22_d_mid`, `x108`–`x117`, C++):** `sub_mid(<target>,<start>,<length>,<value>,<passed>)`
  with `0,…,0` for a length left out and `1` with one; start and length are LONG slots (2.5 is 2). It never changes
  the target's length: a start below 1 or past the end, a length below 1 or an empty value change nothing and
  raise nothing; the bytes written are the smaller of the length, the value and what is left of the target. The
  target is a string variable, a fixed-length one, an element or a member; the value may use the target (`MID$(s,
  2, 3) = s`). A raising start or value raises once and changes nothing, and so does an element with a bad index
  (error 9; `sub_mid` returns while an error is pending, unlike the `swap_` entries). Rejections: a target that is no string variable (a literal, a number, an
  expression, a `CONST`, the FUNCTION's own name) "MID$ expects a string variable/array-element as its first
  argument"; a string start or a number as the value "Illegal string-number conversion"; one argument "Expected
  MID$(...)=..."; four "Invalid expression"; `MID(s, 1) = "x"` without `$` is a store into an array `MID`.
- **`RANDOMIZE`, `RND`, `TIMER` (`v22_d_random`, `v22_d_randomize_noseed`, `x118`–`x134`, `x137`, C++):**
  `sub_randomize(<seed>,1)` and, with `USING`, `sub_randomize(<seed>,3)`; the seed is a `double` parameter and an
  integer is passed as it is. `RANDOMIZE n` mixes the seed with the generator's state, so the values after it
  depend on what was drawn before; `RANDOMIZE USING n` sets the state (the same sequence each time). **Without a
  seed the program prints `Random-number seed (-32768 to 32767)? ` and waits for input**, on the console too.
  `RANDOMIZE (5)` and `CALL RANDOMIZE(5)` are accepted. `RND` is `float func_rnd(float n, int32 passed)`:
  `func_rnd(NULL,0)` bare, `func_rnd(<x>,0|1)` with an argument passed in its own type; believed SINGLE (`RND *
  1000000000` is computed in `float`). `RND(0)` repeats the last value, a negative argument reseeds from it.
  `TIMER` is `double func_timer(double accuracy, int32 passed)`, **believed SINGLE** (`qbs_str((float)(
  func_timer(NULL,0)))`) and held `double` (`d# = TIMER(.001)` stores the `double`); `TIMER(0)` raises 5. Both are
  called without parentheses; `RND()` and `TIMER()` are "Expected (...)", two arguments "Incorrect number of
  arguments", a string "Number required for function", `RND = 1` "Expected variable =", `DIM RND` and `RND!` "Name
  already in use". `RANDOMIZE "a"` is "Number required for sub", `RANDOMIZE 1, 2` "Syntax error".
- **Console `INPUT` and `LINE INPUT` under `$CONSOLE:ONLY`, standard input from a file (`v22_e_input`, `v22_e_redo`,
  `v22_e_eof`, `v22_e_bit`, `v22_e_randomize`, `v22_x138`–`x158`, C++; the same output on two runs):** the C++ is
  `qbs_print(qbs_new_txt_len("prompt",n),0);` when a prompt is written, `qbs_print(qbs_new_txt("? "),0);` for
  `INPUT` without a prompt or with `;` after it (never for `LINE INPUT`), then per target
  `qbs_input_variabletypes[k]=<type>;` and `qbs_input_variableoffsets[k]=<address>;`, then `qbs_input(<count>,1);`
  (`0` with a `;` before the prompt: no line end after the answer), `if (stop_program) end();`. The type is the
  old compiler's type value without `ISPOINTER`, `ISREFERENCE`, `ISUDT` and `ISINCONVENTIONALMEMORY`: `32` for a
  `LONG` variable or member, `8388640` for a `LONG` element (`ISARRAY` stays), `1048640` for `_OFFSET`
  (`ISOFFSET`), `16777217` for `_BIT`, `ISSTRING` for a string of any kind, `ISSTRING+512` for the target of `LINE
  INPUT`. The address is `&(<place>)`, or the `qbs` of a string (`qbs_new_fixed(…,1)` for a fixed-length one).
  **The runtime does everything else, and it never asks again ("Redo from start" does not exist):** it reads the
  line one character at a time as key presses, and a character that does not fit the field is dropped and the
  text so far printed again (nothing is echoed otherwise, so the recorded output holds only those reprints).
  Dropped: a letter in a number, `.` in an integer, `+`, a blank inside or before a number, a `,` after the last
  target, a digit that would take the number past a fixed count of digits (`300` into `_BYTE` gives 30, `70000`
  into `INTEGER` 7000, `99999999999` into `LONG` 999999999, twenty nines into `_INTEGER64` eighteen, `1e50` into
  `SINGLE` 1e5), a character past the length of a fixed-length string, text after a closing quote. `-` is taken
  only where the type is signed (`-1` into `_UNSIGNED _BYTE` gives 1). `&H10` is 16 in an integer target. Fewer
  fields than targets: the rest are 0 or empty (every target is cleared first); an empty line gives 0 and `""`.
  A string field keeps its leading and trailing blanks (` two words ` stays); a quoted field may hold commas and an
  unclosed quote runs to the line end. `LINE INPUT` takes the line as it is. **A `_BIT` variable is accepted and
  never stored** (the runtime skips targets with `ISOFFSETINBITS`: a variable holding 2 still holds 2); a `_BIT`
  element is "INPUT cannot handle _BIT array elements". An element with a bad index raises 9 while the address
  is taken, after the prompt is printed, and nothing is read (the answer goes to the next `INPUT`). **When the
  input ends the program waits for ever** (the run times out, under `run.sh` and under the corpus runner): a
  `.stdin` file must hold every answer. **A program whose input is a file must end with `SYSTEM`**: `END` waits
  at "Press any key to continue" until the timeout. Compile errors: a target that is no variable (`INPUT 5`,
  `INPUT l + 1`, a `CONST`, the FUNCTION's own name) is "Expected variable"; a whole `TYPE` variable "User defined
  types in expressions are invalid"; no target, a prompt alone, two literals, no separator after the prompt
  "Syntax error - Reference: INPUT …"; a prompt that is a variable or an expression (`INPUT p; l`) and a `;`
  between targets "Invalid expression"; `INPUT l, , s` and `INPUT l,,` "Expected variable"; `LINE INPUT l`
  "Expected string variable"; `LINE INPUT s, p` "Too many variables". **One `,` after the last target is
  accepted** (`INPUT l,` and `LINE INPUT s,`; not in the `#` forms, not in `READ`).
  `RANDOMIZE` without a seed is `sub_randomize(NULL,0);`: the runtime prints `Random-number seed (-32768 to
  32767)? ` and reads the answer the same way (five digits at most: `70000` gives 7000).
- **`SHELL`, `COMMAND$`, `ENVIRON$` (`v22_d_shell`, `v22_d_environ`, `v22_x159`–`x172`, `x178`, `x179`, C++ of
  `tests\callsite\shell_*`, `func_shell_*`, `func_command_*`, `func_environ_*`):** the statement is one of three
  entries by its first word: `sub_shell(<command>,1)`, `sub_shell2(<command>,<mask>)` after `_HIDE` and
  `sub_shell3(<command>,<mask>)` after `_DONTWAIT`; in the last two the mask is 1 for the second word (`_DONTWAIT`
  after `_HIDE`, `_HIDE` after `_DONTWAIT`) plus 2 for the command, and a command left out is `NULL` (`SHELL` alone
  is `sub_shell(NULL,0)`: an interactive shell). The command's output comes between the program's own lines, in
  order, also with the output sent to a file; with `_HIDE` it is not shown. An empty command with `_HIDE` raises
  5; a raising argument raises once and the program goes on. `SHELL 5` is "String required for sub", `SHELL "a",
  "b"` "Syntax error", `SHELL _HIDE _HIDE "a"` "Invalid variable name"; `CALL SHELL("x")` is accepted. **The
  function is `int64_t func_shell(qbs*)`, an `_INTEGER64`** (`SHELL("cmd /c exit 2") * 1000000000000` is
  2000000000000): the command's exit code (300 and -1 come back as they are), with the command's output shown; a
  raising argument raises 5. Named alone or with two arguments it is "Incorrect number of arguments", with a
  number "String required for function". `COMMAND$` is `func_command(NULL,0)` bare and `func_command(<n>,0|1)`
  with an index converted as a LONG slot (1.5 is 2); without arguments on the command line it is empty, as is any
  index but 0 (the program's name), also a negative one (no error). `COMMAND$("a")` is "Number required for
  function", `COMMAND$()` "Expected (...)". **`ENVIRON$` is two libqb entries chosen by C++ overloading**:
  `func_environ(<string>)` (the old compiler passes a string argument as it is, `qb64pe.bas` 20952) and
  `func_environ(<n>)` with a number converted as a LONG slot (1.4 is 1, 1.6 is 2). A name is found in any letter
  case; an unknown or empty name is empty; index 0 and a negative index raise 5, an index past the end is empty.
  `ENVIRON$` alone and with two arguments is "Incorrect number of arguments". The `ENVIRON` statement: `NAME=` sets
  the name to nothing, a blank works as the `=` (`"V x y"` sets `V` to `x y`), neither raises 5; a child started
  with `SHELL` sees the value.
- **Functions added by demand (`v22_f_functions`, `v22_x173`–`x177`, C++ of `tests\callsite\func_acos_*` and the
  others):** all are believed `_FLOAT` and take their argument in its own type. `_ACOS`, `_ASIN`, `_SINH`,
  `_COSH`, `_TANH` and `_CEIL` are `std::acos` and the others, so C++ overloading decides the computation: a
  SINGLE is computed as a `float` (`_ACOS(.5!)` prints 1.047197580337524, `_CEIL(s!) / 3` .3333333432674408), an
  integer as a `double`. `_COT`, `_CSC`, `_SEC`, `_D2R` and `_R2D` are libqb's `func_cot(double)` and the others,
  always a `double`. `_ACOS(2)` and `_ASIN(-2)` are NaN without an error; `_SINH(1000)` and `_COSH(1000)` are
  `inf`; **`_COT(0)` and `_CSC(0)` raise 5** and give 0 (libqb tests the tangent and the sine for 0), `_SEC(0)` is
  1. `_STRCMP` and `_STRICMP` are `int32_t func__str_compare(qbs*,qbs*)` and `func__str_nc_compare`, LONG: -1, 0
  or 1. A string where a number is wanted is "Number required for function", a number for a string "1st function
  argument requires a string". **`_STARTDIR$`** (`v22_f_startdir`, `x186`–`x188`) is `func__startdir()`, no
  argument, called bare only (`_STARTDIR$()` is "Expected (...)", `_STARTDIR$(1)` "Incorrect number of
  arguments"): the folder the program was started in, which `CHDIR` does not change.
- **A function that needs an argument, named alone (`v22_x162`, `x170`, `x175`, `x180`–`x185`):** "Incorrect
  number of arguments", for every one tried (`INSTR`, `ABS`, `CHR$`, `EOF`, `LEN`, `SIN`, `SHELL`, `ENVIRON$`,
  `_CEIL`), in an assignment and in `PRINT`. The name is never read as a variable.
- **`CSRLIN` and `POS` under `$CONSOLE:ONLY` (libqb `func_csrlin`, `func_pos`):** on Windows both ask the real
  console for its cursor (`GetConsoleScreenBufferInfo` on `CONOUT$`), which does not move while the output goes to
  a file (corpus `237`: row 1, column 1) and is not set at all without a console. Not compiled in step 9 for that
  reason: their value can only be checked with the screen (step 9a).

Programs with a window (2026-10-10, `m2-screen-state-check`; the tables are in its `design.md`, the programs in
`tests\corpus\screen\`; `verification\v23_post_keys.py` posts keys to a window):

- **Every exit ends in `exit()` on the thread that owns the window**, with the image memory still there:
  `SYSTEM`, `END`, running off the end, the window closed, an unhandled error under `QB64PE_NOPROMPT=y` (exit code
  0), also with `$SCREENHIDE`. So a function registered with `atexit` by another source file sees the final
  screen. A program with a window writes nothing to stdout or stderr, not even the text of a runtime error.
- **`END` and running off the end draw on the screen first:** "Press any key to continue" in the bottom text row
  of the displayed page, which becomes the destination; then they empty the key buffer and wait for a new key.
  `SYSTEM` draws nothing.
- **Screen pages:** `SCREEN 0` has eight pages from the start; a graphics mode has one until another is used.
  Pages of a mode with a palette share page 0's. Modes 1, 2, 7 to 13 hold one byte per pixel (the palette index);
  `PALETTE 1, 63` stores `00FF0000`, the other entries `FFrrggbb`. A `$CONSOLE:ONLY` program has the eight text
  pages too, and a console image as destination.
- **The key buffer is the BIOS one:** 16 slots at `cmem[0x41e]`, head and tail at `0x41a` and `0x41c`, so 15 keys;
  a 16th key is dropped. `INKEY$`, `INPUT` and `LINE INPUT` in a window read it.
- **Keys posted to the window as messages** reach it without the window in the foreground, with rules of their
  own: both keyboard callbacks return at once while the window believes it has no focus; the character after a
  key that makes no character (arrows, function keys, also Enter and Esc) is ignored once; GLFW drops `WM_CHAR`
  below 32, so Enter, Esc, Tab and Backspace must be key messages; a letter's `WM_CHAR` from `TranslateMessage`
  arrives behind messages already posted. `keyboard_keydown` and `keyboard_keyup` can be called from another
  source file and are below all of that.
- **Extra link input:** `qb64pe -f:ExtraLinkerFlags=<value>` is a setting for one run (not written to
  `config.ini`); the `Makefile` puts it at the end of the link command, where a C++ source file is compiled with
  the flags of the link, libqb's include path among them.

Measured for step 10 (2026-10-11, `m2-defxxx-common-arrays` group 1; `verification\v24_*`, one program per
question, `v24_x*` one accepted or rejected form each; handlers print `ERR` and resume next):

- **Default types (`v24_a_*`, `v24_x01`–`x37`):** `DEFINT`, `DEFLNG`, `DEFSNG`, `DEFDBL`, `DEFSTR` and `_DEFINE …
  AS type` write one table of 27 entries (A to Z and `_`), all SINGLE at the start. **The table follows the file,
  not the program and not the procedure:** a statement applies to every line below it, whether it runs or not
  (skipped by `GOTO`, inside an `IF` block that does not run), and **one inside a SUB or FUNCTION goes on applying
  after `END SUB`**, to the procedures and to the main-module lines below it (`v24_a_procs_subs`,
  `v24_x30_def_between_subs`). A procedure above the statement is not reached: its parameters, result and
  variables keep the earlier default; one below gets the new one for all three (`v24_a_procs_function_above`).
  **A procedure's own variable of the name and the default type comes before a main-module `DIM SHARED … AS` of
  that name** (`v24_a_param_shared`): with `DIM SHARED x AS LONG` and `SUB s (x)`, `x` in the body is the
  parameter, in a FUNCTION too, with a parameter typed INTEGER by a `DEFINT` above, with a parameter written
  `x!` under the SINGLE default, and with a local variable first written with the default's suffix (`x% = 3`
  under `DEFINT`). A parameter or local of another type does not count (`m%` under SINGLE: `m` is the shared
  variable), nor does a main-module variable bound by `SHARED x!` (`x` stays the `DIM SHARED` one). Below a
  `DEFxxx` in the body that changes the default, the plain name is no longer the parameter's and means the shared
  variable (a store there changes it).
  **A name without a suffix is the name plus the default type at that line**, so after `a = 1.5: DEFINT A-Z`, `a`
  is `a%`, a new variable holding 0, and `a!` still holds 1.5; the same for a variable declared by `DIM c` without
  a type, for a parameter (inside the SUB, after a `DEFINT`, `p` is a new local), and for an array (`DIM t(3)`,
  then `DEFINT A-Z`, then `t(1)` is an implicit array `t%()` of 0 to 10). **A variable declared with `AS` is found
  by its bare name whatever the default** (`DIM d AS LONG`). The default types every name that may stand without a
  suffix: an implicit variable, `DIM`, `DIM SHARED`, `REDIM`, `STATIC`, `SHARED` (which then needs the main
  variable of that type), `COMMON`, an array, a `FOR` variable, a parameter, a FUNCTION result. **Not a `CONST`:**
  it takes the type of its value (`CONST s1 = 5` under `DEFSTR S` is the number 5, `CONST k = "abc"` under `DEFINT`
  a string). **A FUNCTION is found by its bare name whatever the default at the call**, and by the suffix of its
  type; any other suffix is "Name already in use (f)" (`v24_x26`). Inside it the bare name stays the result, also
  after a `DEFxxx` in its body (`v24_x29`). Letters: any case; a range may be written backwards (`z-w`); a letter
  may be named twice; a later statement replaces an earlier one per letter; **`_` is a letter of its own** for
  names that start with an underscore (`__a` stays SINGLE under `DEFINT A-Z`, is INTEGER after `DEFINT _, a`; the
  range `y-_` is Y, Z and `_`). An underscore that ends the line is a line continuation first (`DEFINT _` joins
  the next line). Types of `_DEFINE`: every numeric type with and without `_UNSIGNED`, `_OFFSET`, `_FLOAT`,
  `STRING`, **also `STRING * n`, `_BIT`, `_BIT * n` and `_UNSIGNED _BIT`**; a parameter typed `STRING * 5` this
  way behaves as one written so (`LEN` 5, the value not cut) and a FUNCTION result is padded to 5; a parameter
  typed `_BIT` this way fails the C++ build as one written so does. Rejected with "Cannot convert type (X) to
  symbol": a user type, `_MEM`, an unknown type, `_UNSIGNED STRING`, `_UNSIGNED SINGLE`, `STRING * 0`, `_UNSIGNED`
  alone. Syntax errors, all worded `_DEFINE: …` for the five `DEFxxx` too: no `AS` or no letters ("Expected ... AS
  ..."), a digit, a word of two letters or a comma at the end ("Expected an alphabet letter or the underscore
  character (_)"), `a-` ("Syntax incomplete"), `a b`, `DEFINT a AS LONG` and `_DEFINE a AS INTEGER, b AS LONG`
  ("Expected - or ,"); `DEFINT` alone is "Syntax error". Follow-on errors are the ordinary ones of the type: a
  `FOR` variable typed `STRING` is "Unsupported variable used in FOR statement", `s = 5` "Illegal string-number
  conversion", and `OPTION _EXPLICIT` names the default type ("Variable 'v' (LONG) not defined").
- **Static or dynamic (`v24_b_*`, `v24_x38`, `x55`, `x88`, `x98`–`x101`):** an array is dynamic when it is first
  declared by `REDIM`, by `DIM` with a bound that is not constant, or by `DIM` (also `DIM SHARED`, `STATIC a(n)`, a
  `DIM` in a `SUB … STATIC`) below `'$DYNAMIC`. **"Constant" is a matter of spelling, not of value
  (`v24_b_constant_bounds`, `_2`, found 2026-10-11 in task 3.2): a bound is constant only when it is one number
  or one `CONST` name**, in any number of parentheses: `5`, `(5)`, `((5))`, `5&`, `5%`, `&H5`, `5.0`, `2.5`, `1E1`,
  a negative number (`-1`, `- 1`, `(-1)`), a `CONST` with or without its suffix, also a `CONST` defined by an
  expression. **Anything with an operator makes a dynamic array though its value is known:** `2 + 3`, `k + 1`,
  `k * 2`, `1 - 2`, `10 \ 3`, `2 ^ 2`, `NOT -6`, and a minus that is not part of a number (`-k`, `-(1)`, `--1`).
  One such bound among several is enough (`DIM a(5, 2 + 3)`). So `DIM f(-(2 ^ 2) TO 10 \ 3)` is allocated when the
  `DIM` runs: skipped by `GOTO`, its elements raise 9; `REDIM` of it works. This also explains the two answers
  to a bound that is 0 under `OPTION BASE 1` below: `DIM a(0 + 0)` is dynamic and raises 5 at the `DIM`. **`$DYNAMIC` and `$STATIC` follow the file like `DEFxxx`:** one
  inside a SUB goes on for the SUBs below it; a `DIM` above the comment in the file is not changed by it, wherever
  it runs. `REM $STATIC` works; `$DYNAMIC` as a statement is "Syntax error". **An array used without `DIM` below
  `$DYNAMIC` is dynamic too, and it is allocated by the first statement in the file that names it**: skipped by
  `GOTO`, every use raises 9 (`x88`); that statement run again raises 10 and is skipped (`x101`); `REDIM` of it
  works. **What an array is, is settled by its first declaration; a later `DIM` is only a statement that runs:**
  `DIM a(n)` twice (one statement in a loop, or two statements) raises 10 the second time and changes nothing;
  `DIM a(5)` then `DIM a(n)` compiles and raises 10 (static stays 0 to 5); `DIM a(n)` then `DIM a(5)` compiles and
  raises 10 (still dynamic, `REDIM` works); `REDIM d(5): ERASE d: DIM d(3)` allocates the dynamic array 0 to 3.
  Only a constant `DIM` of an array that is static already is the compile error "Cannot redefine a static array!".
  Constant bounds that are reversed or a single negative one are "Invalid array bounds" in `REDIM` and under
  `$DYNAMIC` as in a static `DIM`. A dynamic `DIM SHARED` is seen in a SUB. Float bounds round half to even at run
  time (`DIM q(x)` with 2.5 is 0 to 2; `1.5 TO 3.5` is 2 to 4).
- **Allocation (`v24_b_dynamic`, `v24_c_redim`, `v24_x89`–`x96`, the C++ of `DIM d(n)`):** in this order: a static
  array raises 10; `DIM` of an allocated array raises 10 (not when an error is pending already); the bounds are
  evaluated and **a dimension with no element (count 0) raises 5 and the statement goes on**: the array is then
  allocated with the new bounds and no element (`LBOUND` 0, `UBOUND` -1, every index raises 9, a second `DIM`
  raises 10); **a negative count, a count or a size in bytes beyond the address space, and a failed allocation are
  error 257, "Out of memory", which libqb treats as critical: the message is printed and the program ends, no
  handler runs** (`error_handle.cpp` 444). So `n = -3: DIM f(n)` ends the program, also with `REDIM` and `REDIM
  _PRESERVE`. **A bound that raises does not stop the statement either**: `DIM s(ASC(""))` reports 5 and allocates
  0 to 0; `REDIM y(ASC("") TO 5)` reports 5 and gives the array 0 to 5 with new elements. Elements start at 0, `""`
  or NUL bytes. An index outside raises 9 per dimension as for a static array.
- **`DIM a()` and `REDIM a()` declare a scalar (`v24_b_dim_empty*`, `v24_x43`, `x44`, `x86`, `x87`):** with nothing
  between the parentheses, and `REDIM x AS LONG` without parentheses, the old compiler makes the scalar variable
  `a` (`a = 100000.5: PRINT a; LEN(a)` prints 100000 and 4 after `DIM a() AS LONG`) and no array at all. `a(1)`
  after it is an ordinary array used without `DIM` (`a!()`, 0 to 10; "Array 'a' (SINGLE) not defined" under `OPTION
  _EXPLICITARRAY`), and `DIM a(3) AS LONG` after it is an ordinary static array.
- **Not allocated (`v24_b_unallocated`):** after `ERASE` (and before the first allocation) `LBOUND`, `UBOUND` with
  and without a dimension, a read and a store each raise 9; the functions and the read give 0.
- **`REDIM` (`v24_c_redim*`, `v24_x39`–`x56`, `x97`):** declares the array when it is the first to name it (also
  `REDIM _PRESERVE`), gives new bounds and new elements each time, of an array made by `DIM d(n)` too; of a static
  array raises 10 and changes nothing (also `REDIM _PRESERVE`, also for a `DIM SHARED` array in a SUB). Without a
  type it keeps the type of an array declared with `AS` (by `REDIM` or `DIM`), in a SUB for a `REDIM SHARED`
  array too. `REDIM x(5) AS LONG` then `REDIM x&(7)` is the same array; `REDIM x!(7)` after it another one;
  `REDIM y&(5)` then `REDIM y(7) AS LONG` is the same array, and a bare `y(…)` is then `y!()`, used without `DIM`.
  Compile errors: another numeric type with `AS` ("Name already in use (x)"), another number of dimensions
  ("Cannot change the number of elements an array has!", also with `_PRESERVE`), `STRING` against `STRING * n`
  and another length ("Type mismatch"), `REDIM SHARED _PRESERVE` ("Invalid variable name"; `REDIM _PRESERVE
  SHARED` is the order). A count of 0 raises 5 and leaves the array allocated with no element (the old elements
  are gone). Several arrays in one statement are done in order. A scalar of the same name is not touched.
- **`REDIM _PRESERVE` (`v24_c_preserve`):** keeps the smaller of the old and the new number of elements **by flat
  position**, the rest of the new block zero or empty: growing and shrinking, strings (each kept element assigned,
  the old ones released), fixed-length strings, a user type (raw bytes), two dimensions ((1, 1) holding 1 2 3 4 to
  (2, 1) reads 1 2 3 / 4 0 0), another lower bound (0 to 3 holding 1 2 3 4 to 10 to 15 reads 1 2 3 4 0 0). Of an
  array that is not allocated it is `REDIM`. The new block is made first, then the copy, then the old block is
  released, then the descriptor is committed (the C++ of `REDIM _PRESERVE p(n + 1) AS STRING`).
- **`ERASE` (`v24_d_*`, `v24_x57`–`x63`):** a static array is cleared (0, `""`, NUL bytes, every member of a user
  type) and keeps its bounds, in every dimension; an array used without `DIM` likewise. A dynamic array is
  released: `UBOUND` and every element raise 9; a second `ERASE` does nothing; `REDIM`, and the `DIM` that made
  it, allocate it again. Several names are done in order. A scalar of the same name is not touched. **Anything
  that is no declared array is "Undefined array passed to ERASE"**: a scalar, a string variable, a name never
  used (no array is made for it), `a()`, an element `a(1)`, a suffix of another type (`a%` for a LONG array; `a&`
  names it). `ERASE` alone is "Syntax error".
- **`OPTION BASE` (`v24_e_option_base`, `v24_x64`–`x79`, `x83`–`x85`, `x102`, `x103`):** follows the file like
  `DEFxxx` (a statement skipped by `GOTO` applies; one inside a SUB applies to the main-module lines below the
  SUB); it may stand anywhere and any number of times. It sets the lower bound of every dimension written without
  `TO` below it: `DIM`, `REDIM`, a variable bound, an array used without `DIM` (1 to 10, also `LBOUND(nn)` of a
  name that is no array), in a SUB below it. **Only a bound written as the number 0 in plain digits (`0`, `00`)
  stays 0 to 0**, per dimension (`DIM z2(0, 3)` is 0 to 0 and 1 to 3), in `DIM`, `REDIM`, `STATIC` and a local
  `DIM`. Any other constant bound that is 0 (`0&`, `0%`, `0.0`, `0.2`, `&H0`, `(0)`, `-0`, a `CONST`) makes a
  static array 1 to 0: **the program compiles and raises error 5 when it starts**, before its first statement
  ("Illegal function call" at line 1, no handler set yet). `DIM a(0 + 0)` is a dynamic array (a bound with an
  operator, above), so its error 5 is raised at the `DIM` line and a handler set before it runs (`v24_x72`,
  `x103`: the array then exists with no element). A variable that is 0 raises 5 at the `DIM` likewise. **Only the
  word after `BASE` is read:** it must be `0` or `1` in plain digits, zeros before it allowed (`01` and `001` are
  base 1, `00` base 0, `v24_x239`, `x240`, `x243`; "Expected OPTION BASE 0 or 1" for `2`, `1.0`, `1&`, `&H1`, a
  `CONST`, a variable, nothing, `x241`, `x242`), and what follows is ignored (`OPTION BASE 0 + 1` is base 0, `OPTION BASE 1 x` base 1).
- **`_BIT` arrays (`v24_b_bit_arrays`, `v24_x80`–`x82`, `x104`, the C++ of `DIM b(9) AS _BIT * 3`):** plain packed
  storage, `count * bits / 8 + 1` bytes, read with `getbits` (`getubits` unsigned) and stored with `setbits` at
  the checked flat position; values wrap as a `_BIT` variable's do (15 into `_BIT * 4` is -1, 8 is -8; a float
  rounds half to even first). Static, dynamic, two dimensions, used without `DIM` (`` im`(3) ``), `ERASE`, `REDIM`
  and `REDIM _PRESERVE` behave as for other element types. An element passed to a LONG parameter is a copy.
  `SWAP` of two elements is "Cannot SWAP bit-length variables"; `LEN` of an element "Variable/element cannot be
  _BIT aligned". **As a target (`v24_x251`–`x253`, measured 2026-10-11 with task 4.2):** `READ` and `INPUT #` store
  into a `_BIT` element, static or dynamic, and a value outside its range raises error 6 and stores 0 (9 into a
  `_BIT * 4`); console `INPUT` into one is the compile error "INPUT cannot handle _BIT array elements".
- **Measured while dynamic arrays were built (`v24_x244`–`x250`, 2026-10-11, task 4.1):** "Invalid array bounds"
  is decided **per dimension**: a single negative constant bound, and constant `lower TO upper` in the wrong
  order, are compile errors also beside a dimension whose bounds are not constant (`DIM half(n TO 2, -1)`, `DIM
  part(n, 3 TO 1)`); a lower bound that is not constant is not compared (`DIM none(n TO 2)` with n = 3 raises 5
  when it runs and is 3 to 2). A second `DIM` with constant bounds of a static array **below `$DYNAMIC`** is no
  compile error: it runs and raises 10. `DIM` with another number of dimensions than a dynamic array has is
  "Cannot change the number of elements an array has!", as for `REDIM`. **A minus before a hexadecimal number
  (`-&H5`) is one number**, so the array is static. **`SHARED` on a later `REDIM` of an array is accepted and
  ignored**: after `REDIM ns(3)`, `REDIM SHARED ns(4)` gives the new bounds, and a SUB below still does not see
  `ns` (its `ns(2)` is an array of its own, error 9). **`PRESERVE` and `RETAIN` without their underscore are
  names (`v24_x254`, `x255`):** `REDIM preserve(3)` and `REDIM retain(4)` declare arrays, and `REDIM PRESERVE
  x(5)` is "DIM: Expected ," (`$NOPREFIX` itself is an error in this compiler: "QB64(PE) specific keywords MUST
  have the underscore").
- **Arrays used without `DIM` (`v24_f_*`, `v24_x105`–`x120`, `x146`, `x176`):** in the main module the bounds are
  `OPTION BASE` to 10 in each of as many dimensions as the first use has indexes (one for `LBOUND(a)`/`UBOUND(a)`
  of a name that is no array); the type is the suffix or the default type at that line. The array is static: it
  exists from the start (a first use skipped by `GOTO` changes nothing), `REDIM` of it raises 10, a later `DIM` is
  "Cannot redefine a static array!" (also after `LBOUND(a)`), another number of indexes "Cannot change the number
  of elements an array has!". A whole array never declared may be passed (`fill nx()` makes it). **In a SUB or
  FUNCTION the old compiler makes a local dynamic array and writes nothing that allocates it** (the C++ of the
  procedure holds the descriptor in its not-allocated state and no allocation), so **every use raises 9**, on
  every call, also for `LBOUND`; this is the explanation of the `v18_g_implicit_in_sub` note. A later `DIM` or
  `REDIM` of that name in the procedure allocates it (`x176`, `v24_g_shared`). The same holds after `STATIC y()`
  and, below `$DYNAMIC`, in a `SUB … STATIC` (`x120`). **Only in a `SUB … STATIC` without `$DYNAMIC` does it work:**
  the array is static, 0 to 10, and keeps its values between calls. Which names may be one: a name that needs a
  suffix as a built-in is free without it or with another (`chr(1) = 5`, `left(3)`, `abs$(1)`), two leading
  underscores are fine (`__foo`); `_foo(1)` is "Invalid array name"; a keyword, a built-in statement and `len%`
  are "Name already in use" (`to`, `cls`); a built-in function's own spelling and a FUNCTION defined anywhere in
  the program are calls, so a store to one is "Expected variable =, look for conflict with a CONST name" (`rnd(1)
  = 2`, `f(3) = 5`); the name of a SUB is "Name already in use". Under `OPTION _EXPLICITARRAY` both the use and
  `LBOUND(q)` are "Array 'q' (SINGLE) not defined". **Measured before group 5 was built (`v24_x256`–`x263`):**
  `OPTION _EXPLICIT` gives the same error, and both options apply to the whole program (a use above the `OPTION
  _EXPLICITARRAY` line is rejected). A scalar of the name declared with `AS` does not type the array (`DIM x AS
  LONG`, then `x(3) = 2.5` is a SINGLE array: 2.5, `LEN` 4); a `CONST` of the name does not stand in the way
  (`CONST k = 1: k(1) = 2`). `ERASE` of one clears it (bounds kept); `ERASE` above its first use in the file is
  "Undefined array passed to ERASE". In a SUB, the name of a main array that is not shared, of a `DIM SHARED`
  scalar typed with `AS` and of a parameter each make the local array that raises 9, and `ERASE` of it there
  changes nothing. **Below `$DYNAMIC` the allocation is part of the statement that names the array first**
  (`x101`: run a second time it raises 10, the statement's element store is skipped and `RESUME NEXT` goes on
  after the statement). Where it runs when the first use stands in the line of a block (`x262`): before an
  `IF`, a `FOR` (bound or `STEP`), a `WHILE`, a `DO WHILE` (once, not per round) and a `SELECT CASE`; inside
  the block for an `ELSEIF` (not reached: every use raises 9), a `CASE` value and a `LOOP UNTIL` (every round:
  error 10 from the second); for a one-line `IF` inside its `THEN` part. The old compiler's C++ for the local
  array (read from `x263`): a descriptor per call in the "not allocated" state in `dataN.txt`, and in
  `freeN.txt` the release of its block if it is allocated, then `free_mem_lock` of the descriptor's lock.
  **Measured by the review of group 5 (`v24_x264`–`x268`):** the allocation runs where the statement evaluates
  the name, so in a statement that works item by item what stands before it has happened when it raises 10 on a
  second run: `PRINT " x"; dp(1); " y"` prints ` x`, `READ a, dq(1), b` reads `a` (and the next `READ` gets the
  item after it), `WRITE i, dr(1)` writes `2,`, `INPUT` shows its prompt (also when the first target names the
  array) and reads nothing. Nothing differs from "first in the statement" for a store (also with the name in
  its index), `SWAP`, a call's arguments, `LINE INPUT` without a prompt, the first item of a console `PRINT`
  and the first target of `READ`. **In a bound of `REDIM`** the array is dynamic also without `$DYNAMIC` and is
  allocated as the `REDIM` allocates: each run makes it anew (its elements are zero again) and raises nothing,
  `_PRESERVE` keeps it, and a later `REDIM` of it works; in a bound of `DIM` it is as anywhere else (static
  without `$DYNAMIC`; below it, error 10 on the second run). In a SUB, `REDIM sd(lc(1) + 4)` of a shared array
  with such a name in its bound raises 9 and leaves `sd` as it was (`x268`). **A `DIM` of several dynamic
  arrays is one statement (`x267`):** run twice it raises 10 once, an array of the list that is not allocated
  is still allocated after the error, and `RESUME NEXT` goes on after the whole `DIM`. (The new compiler made
  it one statement per array until 2026-10-11.)
- **Arrays in procedures (`v24_g_local*`, `v24_x121`–`x129`, `x177`):** `DIM` and `REDIM` in a SUB or FUNCTION make
  an array that is new, zero or empty, on every call, with constant and with variable bounds, for strings too, and
  in every level of a recursive call; one `DIM` run twice in a call raises 10. `EXIT SUB` and an error inside the
  procedure leave nothing behind (the next call starts new). `STATIC c(3)` is made once and keeps its values.
  **`STATIC l()` declares a dynamic array that lives for the program and is not allocated**: `DIM l(3)` after it
  allocates on the first call and raises 10 on the second (values kept); `REDIM` gives new elements on each call;
  `REDIM _PRESERVE` keeps them; the number of dimensions comes from the statement that gives bounds. It is keyed
  by name and type: `STATIC a() AS DOUBLE` then `DIM a(3)` is another, local array. In a `SUB … STATIC` a `DIM`
  with constant bounds is a static array (values kept, no error on the second call) and one with a variable bound
  a dynamic one that lives for the program (10 on the second call). Compile errors: `STATIC a(3)` twice, `STATIC
  a()` twice, `DIM a(3)` then `STATIC a(3)` ("Name already in use (a)"); `STATIC a()` then `STATIC a(3)` ("Array
  already listed as STATIC"); `STATIC` in the main module ("STATIC must be used within a SUB/FUNCTION"); `DIM
  SHARED` and `REDIM SHARED` in a procedure ("DIM/REDIM SHARED invalid within a SUB/FUNCTION"). **`STATIC v(n)`
  with a bound that is not constant is accepted and fails the C++ build.**
  **Measured while group 6 was built (`v24_x269`–`x282`, slice `s56_proc_arrays`, and the old compiler's C++ for
  `STATIC c(2)`), with four corrections of the lines above:**
  - **`STATIC c(3)` is no static array.** It is a dynamic array that lives for the program, and its allocation
    (as a `DIM` would allocate it) is written into `maindata.txt`, the fragment at the start of the main module:
    it runs when the program starts, and again whenever an error handler is entered, since a handler enters the
    main module anew. An allocated array is left alone there (no error 10: an error is pending). So `ERASE c`
    releases it (the next use raises 9), and the next error that reaches a handler allocates it again, empty
    (`x278`, `s56` line 24); `REDIM c(5)` gives it other bounds without error 10 (`v24_b_metacommand_sub`); below
    `$DYNAMIC` it is the same array (`x278`). The bounds are written into the main module's code as they stand:
    `STATIC v(2 + 3)` works (`x280`), `STATIC v(n)` with a parameter fails the C++ build (`x121`).
  - **A procedure with `STATIC` after its header is static only down to its first `STATIC` statement.** The
    old compiler's flag (`dimstatic`, 2 from the header) is set to 1 by a `STATIC` statement and to 0 at its end
    (`qb64pe.bas` 5827, 8805, 9558). Above that statement every variable keeps its value, a scalar used without
    a declaration and a string too, a `DIM` with constant bounds is a static array and an array used without
    `DIM` a working 0 to 10 one; below it the procedure is one like any other: new variables are made on each
    call, a `DIM` is a local dynamic array (`x279`, `s56` line 21). A variable made above the statement stays
    kept below it. A `FUNCTION … STATIC` keeps its variables the same way.
  - **`DIM a()` and `REDIM a()` in a procedure declare scalars**, as in the main module (`x273`).
  - **A listed array gets its number of dimensions from what names it first**, an index or `LBOUND` included:
    after `STATIC y()`, `LBOUND(y)` then `y(1, 2)` is "Cannot change the number of elements an array has!"
    (`x282`); used with no `DIM`, `STATIC a() AS DOUBLE` then `a(1)` is the listed array (`LEN` 8) and every use
    raises 9 (`x276`).
  Also: a use without `DIM`, then `STATIC a(3)`, is "Name already in use" (`x269`). **A `DIM` in a procedure of
  the name and type of a shared main array is that array**: with constant bounds too it compiles and raises 10
  when it runs, the shared array untouched (`x270` static, `x271` dynamic); in a procedure with `STATIC` after
  its header it is "Cannot redefine a static array!" (`x281`). `STATIC a(5)` beside a shared main array of the
  name compiles, and the program ends when it starts ("Duplicate definition", `x272`).
  **Measured by the review of group 6 (`v24_x283`–`x285`):** in a procedure with `STATIC` after its header,
  `DIM k` of a `CONST` name is "Name already in use" as in any other procedure (`x283`), and `DIM w` of the
  name of a `DIM SHARED` variable (typed with `AS` there or not) makes the procedure's own variable, which
  keeps its value between calls and leaves the shared one untouched (`x284`). **A `DIM c(3)` of an array that
  `STATIC c(3)` declared above it compiles and raises 10 each time it runs**; the array stays as it is and keeps
  its elements (`x285`).
- **Main-module arrays seen from a procedure (`v24_g_shared*`, `v24_x130`–`x134`):** `SHARED a() [AS type]` names
  the main array of that name and type, static or dynamic (the procedure may `REDIM` a dynamic one); the main
  array must be declared above the SUB in the file and with that type, otherwise "Array 'a' not defined" (no
  array is made for it). `SHARED a(3)` is "Expected ()". `SHARED a AS LONG` without parentheses is the scalar.
  **Visibility follows the file**: a `DIM SHARED`, `REDIM SHARED` or `COMMON SHARED` array is seen by the
  procedures below its declaration; in one above it the name is an array used without `DIM` (every use raises
  9). A main array that is not shared is not seen; a local `DIM` of its name is another array.
  **Measured while group 6 was built (`v24_x274`, `x275`, `x277`):** `SHARED a()` twice is accepted; it names a
  main array that was used without `DIM` as well as a declared one, and a list (`SHARED m(), g()`) names each;
  **below an array of that name and type that the procedure has already** (a local `DIM`, or a use without
  `DIM`) it is accepted and changes nothing: the name goes on meaning the procedure's own array.
- **Array parameters (`v24_g_params`, `v24_x135`–`x160`, `x178`–`x180`):** the argument is a whole array written
  `name()`, static or dynamic, a `DIM SHARED` or local array or the procedure's own parameter passed on, also in
  `CALL f(x())` and as an argument of a FUNCTION (`pick(x(), 2)`); the procedure works on the caller's array:
  stores, `LBOUND`/`UBOUND`, `REDIM` and `REDIM _PRESERVE` (10 for a static argument), `ERASE` (clears a static
  one, releases a dynamic one), a `DIM` of the parameter's name (an allocation: 10 if it is static or allocated).
  A dynamic array that is not allocated may be passed and allocated inside. One array may be two arguments.
  **The element type must have the same kind and width; `_UNSIGNED` and `_OFFSET` are not compared**
  (`qb64pe.bas` 12029: size in bits or user type, bit-width flag, string, fixed length, float): an `_UNSIGNED
  LONG` array goes to a LONG parameter and back, `_UNSIGNED INTEGER` to INTEGER, `_INTEGER64` to `_OFFSET`, a
  `_BIT * 4` array to a `_BIT * 4` parameter (which is allowed, unlike a `_BIT` scalar parameter); INTEGER to
  LONG, SINGLE to LONG, `STRING * 3` to `STRING` or to `STRING * 4`, and one user type to another are "Incorrect
  array type passed to sub". An element or a scalar is "Expected arrayname()"; a whole array to a scalar
  parameter "Whole array cannot be passed to a scalar SUB parameter"; **an array in parentheses (`fill (x())`)
  is accepted and fails the C++ build**. **The number of dimensions belongs to the parameter and comes from its
  uses and its arguments together:** what is between the parentheses of the parameter is ignored (`a()`, `a(,)`,
  `a(1)`, and any other words: `a(n)`, `a(n + 1, "x")`, `a(-1)`, `a(1 TO 5)`, `v24_x303`; only a second `(`,
  "Syntax error - too many opening brackets", and the word `AS`, "Syntax error - check your brackets", are
  errors there, `x304`, `x305`, `qb64pe.bas` 2882–2904); an index count, a `REDIM` or an argument that disagrees with another is "Cannot change the number of
  elements an array has!" at the use, or "Passing arrays with a differing number of elements to a SUB/FUNCTION
  is not supported" between two calls of a procedure that never indexes it; the call may stand above or below
  the procedure, and the count travels through arrays passed on (upstream `array_arg_dimensions_transitive`).
  `LBOUND(a())` is "Syntax error".
  **Measured before group 7 (`v24_x286`–`x296`):** an array made by passing a name that names none has **one
  dimension**, 0 to 10, whatever the procedure does with its parameter: `fill nx()` for a SUB that stores
  `a(1, 2)`, then `nx(1, 2)` in the main module, is "Cannot change the number of elements an array has!" at that
  later line (`x286`, `x287`), and a SUB that only reads `LBOUND`/`UBOUND` prints 0 and 10 (`x293`). Below
  `$DYNAMIC` it is a dynamic array allocated as part of the calling statement: the call in a loop raises 10 on
  its second round and the SUB does not run (`x295`). **In a SUB such a name is "Expected (...)"** (`x292`): no
  array is made by passing it there. **An array listed by `COMMON c()` or `STATIC k()` may be passed before
  anything gave it a number of dimensions and gets the parameter's**: a SUB that does `REDIM a(3, 4)` leaves `c`
  with two dimensions, used as `c(1, 2)` and `UBOUND(c, 2)` after the call (`x288`, `x289`). An array parameter
  and a scalar parameter of one name are two variables (`x290`); two array parameters of one name and type are
  "Name already in use (a)" (`x291`). A parameter passed on to its own procedure, one array as two arguments
  (a `REDIM` through one is seen through the other, 10 for a static one), and string, `STRING * n`, user-type
  and two-dimensional `DOUBLE` array parameters work as expected (`x294`, `x296`).
- **`COMMON` (`v24_g_common`, `v24_a_common`, `v24_x161`–`x175`, `x181`):** a scalar is declared as by `DIM`, with
  `SHARED` as by `DIM SHARED` (a SUB sees it), with any type, a fixed-length string and a user type, several in
  one statement, also as `COMMON [SHARED] AS type a, b()`. It may stand below executable statements. **`COMMON
  a()` declares a dynamic array that is not allocated**: a `DIM a(3)` of that name and type allocates it when it
  runs (so a second `DIM` raises 10 at run time and `REDIM` works), a use before that raises 9, and what is
  written between the parentheses is ignored (`COMMON a(3)` allocates nothing). `COMMON SHARED a()` makes that
  array shared; `DIM a(3) AS LONG` after `COMMON SHARED a()` without a type is another array, not shared.
  `COMMON x` then `DIM x(3)` is a scalar and an array. Compile errors: `COMMON x` twice, `DIM x` or `DIM SHARED
  x` of a `COMMON` scalar ("Name already in use (x)"); inside a procedure ("COMMON cannot be used within a
  SUB/FUNCTION"). **A block name is not accepted: `COMMON /blk/ x` is "Invalid variable name"**, with and
  without `SHARED` and without a variable.
  **A listed array's number of dimensions (`v24_x297`–`x302`, measured 2026-10-11):** an index gives it, and a
  `DIM` or `REDIM` with another number below is "Cannot change the number of elements an array has!", for
  `COMMON c()` and `STATIC y()` alike (`x299`, `x300`). **`LBOUND` and `UBOUND` differ:** of an array listed by
  `STATIC y()` they make it one dimension (`LBOUND(y)` then `DIM y(2, 5)` or `y(1, 2)` is that error, `x282`,
  `x297`); of one listed by `COMMON c()` they say nothing (`UBOUND(c)` or `UBOUND(c, 2)` then `c(1, 2)`, `c(1)`
  or `DIM c(2, 5)` all compile, `x298`, `x301`, `x302`).
- **`TYPE` member arrays without a marker (`v24_h_member_*`, `v24_x182`–`x205`, `x217`–`x226`):** **the elements
  are part of the variable's bytes** (`LEN` of `id AS LONG`, `v(0 TO 3) AS LONG`, `tail AS INTEGER` is 22; the
  members around the array keep their values when its ends are stored), in both declaration forms (`v(0 TO 3) AS
  LONG`, `AS INTEGER q(2), r`), of every member type: numeric, `STRING * n`, `STRING`, another user type, with
  one or more dimensions. Bounds are constant expressions (`CONST`s too; a variable is "TYPE member array bounds
  must be numeric constant", empty parentheses "Array bounds missing", reversed "Invalid array bounds"), read
  where the `TYPE` block stands: **`OPTION BASE 1` above the block makes `v(3)` 1 to 3, one below the block does
  not, and the literal 0 has no special rule there** (`z(0)` under base 1 is "Invalid array bounds"). Every
  variable and every array element has its own elements, zero or empty at the start (a local on each call).
  An element is named through any chain (`x(0).n(1).q(2)`); `LBOUND`/`UBOUND` take a member array, with a
  dimension and also written `t.v()`; a float index rounds half to even; an index outside raises 9, a wrong
  number of indexes is "Cannot change the number of elements an array has!", none "Expected array index after
  v", a string "Illegal string-number conversion". **Stores are not guarded, like member stores:** the value is
  evaluated first, then the indexes left to right (`a(one(1)).v(one(2)) = one(3)` prints 3 1 2), the first error
  wins, and **an index outside writes element 0 of that dimension** (`t.v(9) = 5` sets `t.v(0)`; `a(9).v(1) = 5`
  sets `a(0).v(1)`; `a(1).v(9) = 6` sets `a(1).v(0)`); a read with an index outside gives that element's value.
  An element passed to a parameter of its exact type is by reference (a copy in parentheses or for another
  type); with an index outside it raises 9 and the SUB is not entered. `SWAP` of two elements works. A whole
  member array may be passed to an array parameter (`fill t.v()`). **`REDIM`, `REDIM _PRESERVE` and `ERASE`
  never change the bounds:** `REDIM t.v(0 TO 3)` (the declared bounds, written with `TO`, or as `REDIM t.v(3) AS
  LONG` with the member's type) clears the elements of that one variable or element, also strings; `REDIM
  _PRESERVE` with the declared bounds does nothing; `ERASE t.v`, `ERASE x(0).n(1).q` and `ERASE x(0).n` clear.
  Compile errors: other bounds, also the same count shifted ("Cannot change the number of elements an array
  has!"); a bound that is not constant ("TYPE member array bounds must be numeric constant"); the upper bound
  alone without `AS` ("REDIM of a TYPE member array must include AS <type> or explicit lower TO upper bounds");
  another type ("REDIM type does not match the declared TYPE member array"). `ERASE t` of a variable is
  "Undefined array passed to ERASE". A `_BIT` member array is "Cannot use _BIT inside user defined types"; the
  `_STATIC` and `_DYNAMIC` markers are "_Static/_Dynamic require $Unstable:TypeFields".
  **Measured 2026-10-11 before and while group 9 was built (`v24_x310`–`x326`):**
  **What finds the place a member array is part of.** `LBOUND`, `UBOUND` and `LEN` do not: `UBOUND(a(one(1)).v)`
  calls no FUNCTION, `LBOUND(a(9).v)` and `UBOUND(x(1).n(9).q)` raise nothing, only the dimension argument is
  evaluated (outside 1 to the number of dimensions: error 9; `LBOUND(t.v, 1.5)` is dimension 2) (`x311`).
  `REDIM _PRESERVE` with the declared bounds does not either, and raises nothing for `a(9).v` (`x312`). `ERASE`
  and `REDIM` find it once and are not guarded: **`ERASE a(9).v` raises 9 and clears `a(0).v`**, `REDIM
  a(9).s(0 TO 1)` likewise, `ERASE x(1).n(9).v` clears `x(1).n(0).v`, `REDIM a(ASC("")).v(0 TO 3)` raises 5 and
  clears `a(0).v`; several targets in one `ERASE` or `REDIM` work (`x312`).
  **The order of a store through a member (`x313`, `x316`).** A number is evaluated before the indexes, the
  indexes left to right, outermost first, also with two dimensions and through a chain
  (`x(one(1)).n(one(2)).q(one(3)) = one(4)` prints 4 1 2 3). **A string is evaluated after them**:
  `a(setz(1)).s = z$`, where `setz` assigns `z$`, stores the new value, also for `z$ + "!"`, a `STRING * n`
  member and `a(setz(1)).v(setz(1)) = z$`; `a(setz(1)).k = n` stores the old `n`. (The old compiler writes a
  string store as one call, `qbs_set(place, value)`, and its C++ compiler evaluates the place first; a numeric
  store is an assignment, whose right side comes first.) After an index outside, a FUNCTION in a later index
  is entered with the error pending and returns at once, so that index is 0. With an index outside the store
  goes to position 0 of that dimension (`t.g(2, 9) = 8` sets `t.g(2, 0)`; strings too: `t.s(9) = "new"` sets
  `t.s(0)`); a read gives that position's value.
  **A whole value with member arrays (`x310`).** For a type without a `STRING` both sides are found once. Each
  element of a `STRING` member array is a part (a LONG, `s(0 TO 2) AS STRING` and a LONG array: five), each
  element of a member array of a type that holds strings gives its own parts (two elements of that type and a
  LONG: eleven), and any other member array is one part (a `STRING`, a LONG array, a `STRING * 2` array and a
  two-dimension INTEGER array: four; an array of a type without strings and a `STRING`: two).
  **Arguments (`x314`, `x315`).** An element of a member array by reference with an index outside, of the
  member array or of an array on the way, raises 9 and the SUB is not entered; a `STRING` and a `STRING * n`
  element go to a `STRING` parameter by reference (the second cut and padded on the way back); an element of
  a user type goes to a parameter of the type, also as a whole value (`x.n(0) = x.n(1)`, `q = x.n(0)`).
  `SWAP` takes elements of member arrays and members of array elements; with an index outside it raises 9 and
  exchanges with position 0; `SWAP a(one(1)).v(one(2)), a(one(3)).v(one(4))` prints 1 2 3 4. **A whole member
  array as an array argument** (`fill t.v()`, `fill a(1).v()`): in the procedure `LBOUND`/`UBOUND` give the
  member's bounds, also of two dimensions, a `STRING` member array works, **`ERASE` clears it and `REDIM`
  raises error 10** (also with its own bounds), `fill a(9).v()` raises 9 and the SUB is not entered, the place
  is found once.
  **Forms settled by one program each (`x317`–`x326`).** Compile errors: `t.m(1) = 2` for a member the type
  does not have ("Element not defined"); an index after a member that is no array and `LBOUND` of one
  ("Element is not an array"); `fill t.v` without parentheses ("Expected: Array Name()"); `PRINT t.v()`
  ("Expected array index"); `REDIM AS LONG t.v(0 TO 3)` ("Unknown type"). Accepted: `ERASE t.v()` (clears) and
  `REDIM SHARED t.v(0 TO 3)` (clears). **In `REDIM t.v(3) AS LONG` the upper bound alone goes by `OPTION BASE`
  as it stands at the `REDIM`**: below `OPTION BASE 1` it is accepted for a member declared `1 TO 3` and
  "Cannot change the number of elements an array has!" for one declared `0 TO 3`.
- **`STRING` members, whole values, `TYPE` parameters (`v24_h_string_members`, `v24_h_whole`,
  `v24_h_type_params`, `v24_x206`–`x216`):** a `STRING` member starts empty in a variable, a local (on each
  call), a `STATIC` one (kept), each element of a static and of a dynamic array and in a nested type; it holds
  any length; `ERASE` of a static array empties it, `REDIM _PRESERVE` keeps it, `REDIM` and `ERASE` of a dynamic
  array release it; it passes by reference to a `STRING` parameter; an array of such a type may be an array
  argument. `LEN` of the variable is "UDT must have fixed size". **`q = p` copies every member, a `STRING`
  member as its own string and a member array element by element**, between variables, an element and a variable
  both ways, two elements, members of a user type (`bx.b = bx.a`), and to itself. **The copy is not guarded
  either: `a(9) = p` raises 9 and overwrites `a(0)`; `q = a(9)` raises 9 and copies `a(0)`** (`a(8) = b(9)` copies
  `b(0)` to `a(0)`). Another user type, also one with the same members, and a number are "Expected = similar
  user defined type". `SWAP` of two variables or two elements of one type exchanges them, `STRING` members
  included. A user-type parameter takes a variable, an element or a member of that type by reference, in a SUB
  and a FUNCTION, and may be passed on; an element with an index outside raises 9 and the procedure is not
  entered. Another type is "User defined types in expressions are invalid", a number "TYPE 't1' required for
  sub"; **a variable in parentheses (`move (q)`) is "UNEXPECTED INTERNAL COMPILER ERROR!"**. A FUNCTION cannot
  have a user type as its result (`FUNCTION make AS t1` is "Expected (").
  **The order of a whole-value copy (`v24_x306`, `x307`, measured 2026-10-11 before group 8):** the target's
  indexes are evaluated before the source's (`a(one(1)) = b(one(2))` prints 1 2). **For a type with a `STRING`
  member somewhere in it, both are evaluated again for every part of the value**: a FUNCTION in an index is
  called once per part (1 2 1 2 1 2 for a LONG, a `STRING` and a `STRING * 2`; `q = b(one(3))` prints 3 3 3;
  five times each for a type of a nested type of those three, a LONG and a `STRING`; three times for
  `c(one(7)).i = d(one(8)).i`). A part is a `STRING` member, a member of any other type, or a whole member of
  a user type that holds no `STRING` (a type of such a member and two members of three parts: seven,
  `tests\corpus\slice\s60_type_whole` line 13). The reason is in the old compiler's C++: `copy_full_udt`
  (`utilities\type.bas`) writes both address expressions into each `memcpy` and `qbs_set`; for a type without
  a `STRING` it writes one `memcpy`. A `STRING` member is 8 bytes of the value, the pointer to its string
  (`qbs_new(0,0)` per member when the value is made, `qbs_free` when it goes, `->len=0` for `ERASE` of a
  static array).
  **More forms of a parameter of a user type and of `STRING` members (`v24_x308`, `x309`):** the argument may
  be a nested member of an element (`move bs(1).a`); a whole value is copied into and out of a parameter (`p =
  s`, `p = l` with a local); a parameter's member is passed on, as a `STRING` and as a user type; a `STRING`
  member is a target of `READ`, of the `MID$` statement and of `SWAP`, is named with its suffix (`q.n$`) and
  compared like any string. An array of a type with `STRING` members works as an array parameter with `REDIM
  _PRESERVE` and `ERASE` in the SUB (static and dynamic argument), as `STATIC k(2)`, as a `DIM SHARED`
  variable and as a local of a recursive SUB.
- **`$CONSOLE`, `$SCREENHIDE` and `_DEST _CONSOLE` (`v26_*`, measured 2026-10-12 before step 11; 51 programs,
  standard output redirected; `x40` and later while the step was built):**
  - **What reaches standard output (`v26_b_dest_switch`, `v26_c_dest_in_sub`).** In a program with `$CONSOLE`
    and no `:ONLY`, a `PRINT` reaches standard output only while the console is the destination: output before
    `_DEST _CONSOLE`, after `_DEST 0` and after `_DEST` with the saved first handle goes to the window and is not
    seen. `_DEST _CONSOLE` in a SUB or FUNCTION stays in force after it returns. `_CONSOLE` is -10; `_DEST` and
    `_SOURCE` are -2 at the start; `_DEST _CONSOLE` leaves `_SOURCE` at -2 and `_SOURCE _CONSOLE` makes it -10.
    Under `$CONSOLE:ONLY` all three are -10 after the metacommand, and `_DEST 0` there sends `PRINT` to a window
    that is never shown (`v26_x24`).
  - **`PRINT` forms (`v26_a_dest_console`).** `;`, `TAB`, `SPC`, `PRINT USING`, `WRITE` and a `PRINT` without a
    line end give the text they give under `$CONSOLE:ONLY`. A comma does not: `PRINT "z1", "z2"` printed `z1` and
    then blanks without end (1.8 MB until the 60 s timeout), where `$CONSOLE:ONLY` gives zones of 10 columns. No
    test program may use it ("Test hygiene" in §10 said so already); the recorded program has no comma.
  - **`INPUT` (`v26_d1`, `v26_d2`).** `INPUT` and `LINE INPUT` read standard input after `_DEST _CONSOLE` alone;
    `_SOURCE _CONSOLE` changes nothing for them. The prompt and the echo are as under `$CONSOLE:ONLY`.
  - **The end of the program (`v26_e_end`, `v26_e2_fall_off`).** `END`, and running off the last line, print an
    empty line and `Press any key to continue` and wait, hidden window or not; `SYSTEM` leaves at once.
  - **Errors (`v26_f1`, `v26_f2`, `v26_x21`).** A trapped error works as before (`ERR`, `_ERRORLINE`, `RESUME
    NEXT`). An untrapped one, with `QB64PE_NOPROMPT=y`, prints an empty line, `Runtime error: Line: 7 (in main
    module)` and `Illegal function call`, and the exit code is 0. `_DEST -99` raises error 258 and leaves the
    destination; `_DEST 1.6` raises nothing and the next `PRINT` is not seen.
  - **`_ECHO` (`v26_g_echo`, `v26_x17`).** It writes its string and a line end to the console whatever the
    destination is, and leaves the destination as it was. Without `$CONSOLE` or `$CONSOLE:ONLY` in the program it
    is the compile error "_ECHO requires $CONSOLE or $CONSOLE:ONLY to be set first".
  - **More forms of the four built-ins (`v26_x62`–`x73`, measured while step 11's group 3 was built).** `_ECHO`
    above the line of `$CONSOLE` compiles and writes (`x62`: the check reads the flag, not the lines above), and
    it works under `$CONSOLE:ONLY`, also with the window as destination (`x72`). After `_DEST 0` the function
    `_DEST` gives -2, not 0 (`tests\corpus\slice\s64_dest_console_b`: `echo 2:-2`). `CALL _ECHO("x")`, `CALL _DEST(_CONSOLE)` and `CALL _SOURCE(_CONSOLE)` are
    accepted and do what the statements do (`x69`, `x70`); so are parentheses around the handle, and the three
    functions in arithmetic (`x73`). Rejected: `_ECHO 5` "String required for sub" (`x63`); `_ECHO` with no
    argument or two, `_SOURCE` with none, `_DEST 0, 1`: "Syntax error - Reference: …" (`x64`–`x66`, `x71`);
    `_SOURCE "a"` "Number required for sub" (`x67`); `d& = _DEST(1)` "Incorrect number of arguments" (`x68`).
  - **The metacommands count for the whole program (`v26_x01`, `x04`, `x08`, `x16`, `v26_h_flag_console`).**
    `$CONSOLE` and `$SCREENHIDE` may stand twice, after `SYSTEM` as the last lines, and inside a SUB; the console
    is there from the first statement either way. The precompiler flag `_CONSOLE_` is 1 above the line of
    `$CONSOLE` as well as below it. A `$CONSOLE` in a `$IF` branch that is not taken does not count.
  - **Without `$CONSOLE` (`v26_x05`, `x06`, `x16`).** `_DEST _CONSOLE` compiles and raises nothing, and the
    `PRINT` after it is not seen. `$SCREENHIDE` alone compiles and the program ends with exit code 0 and no
    output. `$SCREENSHOW` after `$SCREENHIDE` is accepted (`x07`; the window shows).
  - **Both `$CONSOLE:ONLY` and `$CONSOLE` (`v26_x02`, `x03`, `v26_h2_flag_both`).** In either order the program
    is a console-only one: a `PRINT` with no `_DEST` is seen from the line of `$CONSOLE:ONLY` on (a `PRINT` above
    it is not, as before). With `$CONSOLE:ONLY` first and `$CONSOLE` after it, `_CONSOLE_` is 1, not 2. This is
    the shape of the 19 upstream `glut` tests on Windows: `$CONSOLE:ONLY`, a `PRINT` and `SYSTEM` inside `$IF
    WINDOWS OR MACOSX`, then `$CONSOLE`, `_DEST _CONSOLE` and a window built-in that is compiled and never run.
  - **Forms the old compiler rejects.** "Syntax error" for `$CONSOLE:OFF` (`x09`), a word after `$CONSOLE` or
    `$SCREENHIDE` (`x10`, `x11`), a comment after either (`x12`, `x35`), a colon and a statement after either
    (`x14`, `x37`) and a tab before `$CONSOLE` (`x23`). "Unexpected character on line" for `x = 1: $CONSOLE`
    (`x13`) and `y = 2: $SCREENHIDE` (`x36`). Lower and mixed case and blanks before and after are accepted
    (`x15`). `_DEST` with no argument: "Syntax error -
    Reference: _DEST imageHandle&" (`x18`); with a string: "Number required for sub" (`x19`); `_CONSOLE = 1`:
    "Expected variable =, look for conflict with a CONST name" (`x20`). The handle may be a variable of any
    numeric type (`x22`). `$CONSOLE:ONLY` is as strict: a comment or a word after it and blanks around its colon
    (`$CONSOLE : ONLY`) are "Syntax error" (`x29`–`x31`), as is a word after `$SCREENSHOW` (`x32`).
  - **The two line rules hold for every metacommand (`x40`–`x50`, 2026-10-12).** A tab before the `$` is
    rejected for `$IF`, `$LET`, `$INCLUDEONCE` (in an included file), `$CONSOLE:ONLY` and `$SCREENHIDE` as for
    `$CONSOLE`: the line is not taken as a metacommand and is read as a statement that starts with `$`. The
    message depends on what follows: "Syntax error" (`x42`–`x44`), "Invalid expression" for `$LET` (`x41`),
    "THEN without IF" for `$IF` with a tab before its `$END IF` too (`x49`), and "$END IF without $IF" at the
    `$END IF` when that one has no tab (`x40`). A statement and a colon before `$IF`, `$LET`, `$INCLUDEONCE` or
    `$CONSOLE:ONLY` is "Unexpected character on line" (`x45`–`x48`), as for `$CONSOLE` and `$SCREENHIDE`. Inside
    a `$IF` branch that is not taken, a tabbed `$ELSE` or `$END IF` is skipped text: it does not end the branch
    and is no error (`x50` compiles and prints only what follows the untabbed `$END IF`); the same holds for
    `x = 1: $ELSE` and `x = 1: $END IF` there (`x57`). A tab after a metacommand is "Syntax error" too
    (`$CONSOLE:ONLY` and a tab, `x58`; `$INCLUDEONCE` and a tab, `x59`; `RTRIM$` takes blanks only), where blanks
    after it are fine (`x15`). A word after `$INCLUDEONCE` and a letter right after `$CONSOLE:ONLY`
    (`$CONSOLE:ONLYX`) are "Syntax error" as well (`x61`, `x60`): the whole line is compared. In the
    source each rule is one place that names no metacommand: a line is a metacommand only when its first byte
    after `LTRIM$` is `$` (`qb64pe.bas` 1834, 3408, 3427; `LTRIM$` takes blanks, not tabs), and the line reader
    rejects a `$` that is not the first thing on its line (25415). Metacommands in a comment (`'$INCLUDE`,
    `'$DYNAMIC`) are found another way (25461) and are not part of this.
  - **`$CONSOLE:ONLY` a second time (`x25`).** It makes the console the destination again: `$CONSOLE:ONLY`,
    `PRINT`, `_DEST 0`, `PRINT`, `$CONSOLE:ONLY`, `PRINT` prints the first and the third line.
  - **The precompiler flags (`x26`, `x28`, `x33`).** With `OPTION _EXPLICIT` on the last line, `_EXPLICIT_` and
    `_EXPLICITARRAY_` are both 1 on the lines above it; with `OPTION _EXPLICITARRAY` there, only
    `_EXPLICITARRAY_` is. `_ASSERTS_`, `_DEBUG_` and `_SOCKETS_` are 0 when nothing sets them, and `_CONSOLE_` is
    2 under `$CONSOLE:ONLY` alone. A `$CONSOLE` that stands only inside `$IF _CONSOLE_ = 0 THEN` counts: the
    program has a console and the flag is 1 below it (the old compiler compiles again with the flag set and keeps
    what the first round asked for; `utilities\statevars.bas`: a state is locked once it is not 0).
  - **A flag's own branch, and a round before the flags are settled (`x38`, `x39`).** A `$CONSOLE:ONLY` that
    stands only inside `$IF _CONSOLE_ = 0 THEN` leaves a program with a console and a window: `_CONSOLE_` is 2,
    `_DEST` is -2, a `PRINT` with no `_DEST` is not seen and `_ECHO` works (the flag is kept; the line that makes
    the program console-only is no longer compiled). The output alone does not show the window (a `PRINT` above
    `$CONSOLE:ONLY` is not seen either, `x03`); the build does: `internal\temp\compilelog.txt` of that compile
    links `libqb_make_00000000`, the libqb with the window, with `-mconsole`, and `global.txt` has `console=1`.
    With `$CONSOLE` on line 1 and below it `OPTION _EXPLICIT`
    inside `$IF _CONSOLE_ = 0 THEN`, `_EXPLICIT_` is 0: the first pass sets `_CONSOLE_` at the metacommand's
    line, so the branch is not taken even in the first round (`qb64pe.bas` 1980–2006: `$CONSOLE`,
    `$CONSOLE:ONLY`, `$ASSERTS` and `$DEBUG` do this; `OPTION _EXPLICIT` does not, 2099).
  - **More on the flags (`x51`–`x56`, 2026-10-12).** `$IF _EXPLICIT_` between two `OPTION _EXPLICIT` lines is
    1 (`x51`). `$ASSERTS:CONSOLE` gives the program a console: `_CONSOLE_` is 1 above and below its line and
    `_ASSERTS_` is 1 (`x52`, written with `_ECHO`). Three rounds work: with the only `$CONSOLE` inside `$IF
    _EXPLICIT_` and the only `OPTION _EXPLICIT` on the last line, the program has a console and both flags are
    1 (`x53`). **`OPTION _EXPLICIT` is in force by its flag:** when it stands only inside `$IF _EXPLICIT_ = 0
    THEN`, the flag is 1 (`x54`) and a variable that is not declared is "Variable 'x' (SINGLE) not defined"
    (`x55`), though the last round does not compile the line. `_ASSERTS_` has a running value in the first
    pass as `_CONSOLE_` has: `OPTION _EXPLICIT` inside `$IF _ASSERTS_ = 0` below the line of `$ASSERTS` is
    never met (`x56`). In the source the `$IF` values are set from the settled state at the first line of each
    pass and at the end of the source (`setPrecompFlags`, `qb64pe.bas` 1816, 3373).
  - **Where the old compiler's two passes disagree (`x27`, `x34`).** Between `$CONSOLE:ONLY` and a later
    `$CONSOLE`, a `$IF` on `_CONSOLE_` is 2 for the first pass (each metacommand sets the flag at its line there)
    and 1 for the second. A `PRINT` in each of its branches prints nothing at all; a `CONST k` in each branch is
    "Name already in use (k)". Nothing sensible to copy: the new compiler marks such a `$IF` "not supported yet".
  - **In the old compiler's source.** `$CONSOLE` sets `console=1` and the make flag `DEP_CONSOLE=y` (links with
    `-mconsole`); `$CONSOLE:ONLY` sets `DEP_CONSOLE_ONLY=y` too and writes `sub__dest(func__console());
    sub__source(func__console());` where it stands; `$SCREENHIDE` and `$SCREENSHOW` set `screen_hide_startup` to
    1 and 0, the last one in the file counting (`qb64pe.bas` 3590–3624, 13076–13086, 13708–13717). `_DEST`,
    `_SOURCE`, `_CONSOLE`, `_ECHO`, `_SCREENHIDE` and `_SCREENSHOW` are plain rows of the built-in table.
  - **The build of each mode (read 2026-10-12 from `internal\temp\compilelog.txt` and `global.txt` after
    `qb64pe -x`; the make line itself is kept nowhere on Windows).** `$CONSOLE:ONLY`, with or without
    `$CONSOLE`: `DEP_CONSOLE_ONLY=y DEP_CONSOLE=y`, libqb variant `libqb_make_00100000`, `-mconsole`, the
    objects `window-console.o` and `main-thread-console.o`, `console=1`. `$CONSOLE` alone: `DEP_CONSOLE=y`,
    variant `…00000000`, `-mconsole`, `window-gui.o` and `main-thread-gui.o`, `console=1`, and
    `screen_hide_startup=1` with `$SCREENHIDE`. A program that names `_ICON`: `DEP_ICON=y` as well, a third
    variant `…00000100`, `-DDEPENDENCY_ICON` in place of `-DDEPENDENCY_NO_ICON`. The clone's `Makefile` reads
    `DEP_CONSOLE` only to choose `-mconsole`, which `DEP_CONSOLE_ONLY` chooses too.
  - **The calls of the window built-ins (read 2026-10-12 from the C++ of `qb64pe -z`; the table of every form
    is design D6 of `m2-console-screenhide`).** Each is the plain call its table row and template give, every
    numeric slot a LONG (`int32_t` in libqb; a float argument goes through `qbr`). `_FREEIMAGE` with no argument
    is accepted (`sub__freeimage(NULL,0)`). `_MOUSEHIDE` passes `NULL`, 1 (`_HIDE`) or 2 (`_DISABLE`) and no
    mask. `_SCREENEXISTS` has no return type in the table: it is treated as a 32-bit integer (stores,
    operators, `IF`, `SELECT CASE`, arguments), `PRINT` and `STR$` write `qbs_str(func_windowexists())` with no
    cast (the same overload, the C++ type being `int32_t`), and only `HEX$` differs from a LONG:
    `func_hex(…,0)` against `func_hex(…,8)`. `_WINDOWHANDLE` is an `_UNSIGNED _OFFSET` value like a variable of
    that type (`qbs_str((uint64)(…))`; `/` rounds the quotient with `qbr`).
  - **Forms of the window built-ins, accepted and rejected (`x74`–`x113`, measured while step 11's group 4 was
    built).** Accepted (`x74`, as code that is not reached): every one of the statements after `CALL`, with the
    forms of its template between the parentheses (`CALL _ICON`, `CALL _ICON(1, 2)`, `CALL _FREEIMAGE`, `CALL
    _MOUSEHIDE(_HIDE)`, `CALL _MOUSECURSOR(1, (2, 3))`, `CALL _WINDOWSIZELIMIT((1, 2)-(3, 4))`); the function
    `_SCREENHIDE`; `_SCREENEXISTS` as the argument of `HEX$`, `OCT$` and `_BIN$`. Rejected, one program each: a
    function that takes no argument called with one is "Incorrect number of arguments" (`_SCREENEXISTS(1)`,
    `_SCREENX(1)`, `_TITLE$(1)`, `_DESKTOPWIDTH(1)`, and `ERR(1)`, `ERL(1)`: `x75`–`x77`, `x99`, `x102`, `x113`),
    with empty parentheses "Expected (...)" (`x98`, `x100`, `x101`, `x108`); `_NEWIMAGE` with one or four
    arguments or bare, `_WINDOWHANDLE(1, 2)` likewise "Incorrect number of arguments" (`x104`, `x105`, `x107`,
    `x109`); a string where a number is wanted and a number where a string is ("1st sub argument requires a
    number", "String required for sub", "Number required for sub", `x79`, `x80`, `x90`, `x93`, `x106`, `x110`); a
    statement in a form that is none of its template's is "Syntax error - Reference: …" (`_ICON 1, 2, 3`,
    `_MOUSESHOW "a", "b"`, `_MOUSEHIDE 1`, `_MOUSEHIDE _SHOW`, `_MOUSECURSOR`, `_MOUSECURSOR 1, 2, 3`,
    `_MOUSECURSOR 1, (2)`, `_WINDOWSIZELIMIT 320, 240`, `_WINDOWSIZELIMIT (320)`, `_WINDOWSIZELIMIT (1, 2)-(3)`,
    `_FREEIMAGE 1, 2`, `_TITLE`, `_TITLE "a", "b"`: `x78`, `x81`–`x89`, `x91`, `x92`, `x94`); `_SCREENHIDE 1`,
    `_SCREENSHOW 1` and `_SCREENICON 1` are "SUB does not require any arguments" (`x95`–`x97`); `n = _TITLE`
    "Invalid variable name" (`x103`); `_SCREENX = 1` "Expected variable =" (`x111`); `_NEWIMAGE 1, 2` as a
    statement "Syntax error" (`x112`).
  - **The constants of the auto-included file (`x114`–`x126`, measured while step 11's group 5 was built; their
    values and types by slice program `s67_auto_constants`).** A constant of `beforefirstline.bi` is used with
    its own suffix, with another or with none (`_TRUE%`, `_TRUE&&`, `_TRUE#`, `_ONE_KB&&`, `_LONG_MAX%`,
    `` _BIT_MIN` ``, `_E#`), in a `CONST` and as the length of a `STRING * n` (`x114`). Its name is a reserved
    name: `CONST _TRUE = 1`, and with the same value `CONST _ONE_MB = 1048576`, are "Invalid name" like `CONST
    _MYOWN = 1` (`x115`, `x116`, `x125`); `DIM _TRUE AS LONG` is "Invalid variable name" (`x117`); `_TRUE = 5`
    "Expected variable =" (`x118`); `FOR _TRUE`, `INPUT _TRUE`, `SWAP n, _TRUE` are errors (`x122`–`x124`);
    `_TRUE$` is "Type mismatch" (`x119`), `_TRUE(1)` "Invalid array name" (`x120`), a string constant stored in
    a number "Illegal string-number conversion" (`x121`), `LEN(_BYTE_MAX)` "String expression or variable name
    required in LEN statement" (`x126`). The values of the constants the new compiler does not compute yet
    (`x114`): `_E` prints 2.718281828459045; `_DOUBLE_MAX` and `_DOUBLE_MIN` ±1.797693134862315D+308;
    `_FLOAT_MAX` and `_FLOAT_MIN` print `INF 7693134862315D+308` and `-INF 7693134862315D+308` (their literals
    are beyond what the old evaluator reads) and `_FLOAT_MIN_FRAC` 0; `_INTEGER64_MIN` and `_OFFSET_MIN` are
    -9223372036854775807 (one more than the type's least value), `_UINTEGER64_MAX` and `_UOFFSET_MAX`
    9223372036854775807 (half of the type's greatest). Arithmetic on a constant with a suffix is computed
    beyond its type (`s67`: `_BYTE_MAX + 1` is 128, `_LONG_MAX + 1` is -2147483648, `_INTEGER64_MAX + 1` wraps).
  - **Precompiler flags across files (`x127`).** A setting and a `$IF` on its flag in an included file work as
    in one file: with `$CONSOLE:ONLY` in the included file, `$IF _CONSOLE_ = 2` in the main file above the
    include is taken, and an `OPTION _EXPLICITARRAY` the included file's `$IF _CONSOLE_ = 2` guards sets
    `_EXPLICITARRAY_` for the main file's lines below.
  - **`SWAP` after `CALL` (`x128`).** `CALL SWAP(a, b)` is accepted and exchanges the two variables, as `SWAP a,
    b` does.
  - **What the 37 upstream programs with plain `$CONSOLE` need (`qb64rust --report unsupported`, 2026-10-12).**
    Two need nothing but the three features (`data\test`, `keyboard\devices_windows`). The 19 `glut` tests also
    need one or more window built-ins as compiled, never-run code (`_SCREENHIDE`, `_SCREENSHOW`,
    `_SCREENEXISTS`, `_TITLE`, `_TITLE$`, `_ICON`, `_SCREENICON`, `_MOUSESHOW`, `_MOUSEHIDE`, `_MOUSECURSOR`,
    `_DESKTOPWIDTH`, `_DESKTOPHEIGHT`, `_SCREENX`, `_SCREENY`, `_WINDOWHANDLE`, `_WINDOWHASFOCUS`,
    `_WINDOWSIZELIMIT`, `_NEWIMAGE`, `_FREEIMAGE`), and 12 of them `_TRUE` or `_FALSE`. The seven
    `precomp-flags` tests need the precompiler flags in `$IF` (three of them nothing else; the others `$ASSERTS`,
    `$DEBUG` or `_OPENCLIENT`). The other nine need sound, images, `$EMBED` or devices.
  - **The GitHub Windows runner.** The upstream tests say in a comment that GLFW cannot create a window with an
    OpenGL context there, which is why the `glut` tests are console-only on Windows. A program with `$CONSOLE` and
    `$SCREENHIDE` does start there: the `tier2` job is green with `s63_console_window` and the other slice
    programs that have a hidden window (seen 2026-10-12, task 2.6 of `m2-console-screenhide`). A window that is
    shown is not measured on the runner.
- **A program with a window as its output, and the screen statements (`v27_*`, measured 2026-10-12 before the
  first step of M3, OpenSpec change `m3-window-output`; 65 programs):**
  - **The build of a program with no console (read from `internal\temp\compilelog.txt` and `global.txt` after
    `qb64pe -x`).** With neither `$CONSOLE` nor `$CONSOLE:ONLY`: no make flag, the libqb variant
    `libqb_make_00000000` (the one a `$CONSOLE` program with a window links), `window-gui.o` and
    `main-thread-gui.o`, `-mwindows` in place of `-mconsole`, `console=0`; with `$SCREENHIDE` alone the same and
    `screen_hide_startup=1`. The C++ of `PRINT`, `INPUT`, `LINE INPUT`, `ERROR`, `END` and `SYSTEM` is the same
    text as under `$CONSOLE:ONLY`: the mode is in the two globals and in libqb, not in a statement's code.
  - **The calls (read from the C++ of `qb64pe -z`, every form; the table is design D2 of `m3-window-output`).**
    `SCREEN`, `CLS`, `COLOR`, `LOCATE`, `WIDTH`, `VIEW PRINT`, `VIEW`, `WINDOW`, `PSET`, `PRESET`, `LINE`,
    `CIRCLE`, `DRAW`, `PALETTE`, `PCOPY`, `_LIMIT`, `_DELAY`, `SLEEP` and `_DISPLAY` are the plain call of their
    table row and template, `LINE` too. A slot the table types LONG or `_UNSIGNED LONG` takes an integer as it is
    and a float through `qbr`; a `_FLOAT`, SINGLE or DOUBLE slot takes any number as it is (libqb's parameters
    are `float` or `double`, C++ converts). `PAINT` is the one exception: its fill is a number or a string and
    the old compiler writes a cast before it, `(uint32)` or `(qbs*)`, which picks one of libqb's two `sub_paint`
    (`(uint32)NULL` when it is left out). `WIDTH #1, 80` is `qbsub_width(1,1,80,…)` and `WIDTH LPRINT 80`
    `qbsub_width(2,80,…)`: the same entry with the choice as its first argument.
  - **The functions.** `INKEY$` is `qbs_inkey()`, `CSRLIN` `func_csrlin()` (a LONG), `POS(n)` `func_pos(n)` (a
    LONG), `POINT(x, y)` and `POINT(n)` one entry, `func_point`, with a `passed` flag for the second argument (a
    `double`, believed DOUBLE), `SCREEN(r, c[, f])` `func_screen` (a `uint32`, believed `_UNSIGNED LONG`).
  - **Accepted forms (`v27_x01`, as code that is not reached).** Each statement with nothing after its name
    where its whole template is optional (`SCREEN`, `COLOR`, `LOCATE`, `WIDTH`, `VIEW`, `WINDOW`, `PALETTE`,
    `SLEEP`, `VIEW PRINT`); a comma with nothing after it where the template has an optional part behind the
    comma (`SCREEN 12,`, `CLS ,`, `COLOR 1,`, `LOCATE 1,`, `LINE (1, 2)-(3, 4),` and `…, 5,`, `CIRCLE (1, 2), 3,`,
    `PAINT (1, 2),`, `VIEW (1, 2)-(3, 4),`); `COLOR` with four arguments; `SCREEN 12, , , , _MANUALDISPLAY 1`;
    `WIDTH #1`, `WIDTH #1, 80, 50`, `WIDTH LPRINT`; a blank between `STEP` and its parenthesis and around the `-`
    of `LINE`; `B` and `BF` in lower case; `LINE -(5, 5)` and `LINE STEP(1, 1)-STEP(2, 2)`. `l = INKEY` and `k =
    CSRLIN$` are variables, not the functions.
  - **After `CALL` (`v27_x02`, `x14`, `x15`).** Every one of the statements is accepted after `CALL`, with the
    form of its template between the parentheses (`CALL SCREEN(12)`, `CALL CLS`, `CALL PSET((1, 2), 3)`, `CALL
    LINE((1, 2)-(3, 4), 5, BF)`, `CALL PALETTE`, `CALL SLEEP`); `CALL PSET(1, 2)` is a syntax error (the point's
    own parentheses are missing) and `CALL VIEW PRINT` is "Expected CALL sub-name (...)".
  - **Rejected forms, one program each (`v27_x03`–`x50`).** "Syntax error - Reference: …" for a form that is
    none of the template's: `LINE (1, 2), 3` (`x03`), `LOCATE` with six arguments and with four commas (`x04`,
    `x18`), `VIEW PRINT 2` (`x05`), `PALETTE 1` and `PALETTE 1, 2, 3` (`x06`, `x43`), a fifth argument of
    `SCREEN` without `_MANUALDISPLAY`, alone and after four others (`x16`, `x17`), a comma after the box word of `LINE` and
    a box word that is neither `B` nor `BF` (`x19`, `x41`), `PSET (1, 2),` and `PSET 1, 2` (`x20`, `x36`),
    `CIRCLE (1, 2),` and eight arguments (`x21`, `x45`), `VIEW (1, 2)-(3, 4), 5,`, `VIEW SCREEN`, `VIEW
    STEP(…)` (`x22`, `x24`, `x35`), `WINDOW SCREEN` and `WINDOW` with a colour (`x23`, `x47`), `WIDTH` with five
    values (`x25`), `_LIMIT` with none (`x27`), `SLEEP 1, 2` (`x28`), `PCOPY 1` (`x34`), `CLS` with four (`x37`),
    `PAINT` with five (`x46`), `DRAW t, k` (`x48`). "SUB does not require any arguments" for `_DISPLAY 1`
    (`x26`). A string where a number is wanted: "1st sub argument requires a number" (`COLOR t`, `x10`; the
    argument's place in the C call is counted: "2nd" for `WIDTH t`, `x38`, "5th" for the colour of `LINE`,
    `x42`, "4th" for the border of `PAINT`, `x12`), "Number required for sub" (`_DELAY t`, `x49`); a number
    where a string is: "String required for sub" (`DRAW 1`, `x11`), "5th sub argument requires a string" (the
    tile of `PAINT`, `x13`). Functions: `INKEY$()` and `CSRLIN()` "Expected (...)" (`x07`, `x08`); `INKEY$(1)`,
    `CSRLIN(1)`, `POS` bare, `POINT` bare, `POINT(1, 2, 3)`, `POINT(, 2)`, `SCREEN(1)` "Incorrect number of
    arguments" (`x31`, `x32`, `x29`, `x33`, `x09`, `x50`, `x30`); a string argument "Number required for
    function" (`POS(t)`, `x40`) or "Nth function argument requires a number" (`POINT(1, t)`, `SCREEN(t, 1)`,
    `x39`, `x44`).
  - **`INKEY$`, `CSRLIN` and `POS` in a program with a console and no window (`v27_a_console_functions`,
    standard input and output redirected).** They compile and run: `INKEY$` gives an empty string; `POS(0)` is
    1 wherever the output stands (after `PRINT "abc";` too); `CSRLIN` does not change as lines are printed (its
    difference from an earlier `CSRLIN` is 0). So `POS(0)` and a difference of two `CSRLIN` are stable in a
    console program's recorded output; the value of `CSRLIN` itself is not recorded.
  - **A statement's name, parentheses and `=` (`v27_x51`–`x57`; measured for 44 statement names while group 3
    was built).** A statement that starts `name (…) =`, the `=` right after the matching parenthesis, is an
    assignment to the old compiler whatever follows, and for a built-in statement's name an error: "Name
    already in use (CLS)" for `CLS (1) = 5`, `LOCATE (1) = 2, 3`, `PCOPY (1) = 1, 2`, `KILL (t) = t`, `OPEN`,
    `NAME`, `SWAP`, `LINE`, `PSET`, `PAINT` and the others (`x52`–`x54`); another message where the name is
    also a function or has a leading underscore ("Incorrect number of arguments" for `SCREEN (12) = 12` and
    `_DEST (1) = 1`, "Expected variable =" for `SEEK (1) = 2, 3`, "Invalid array name" for `_LIMIT (60) = 60`
    and `_TITLE (1) = 1`: `x55`–`x57`). With something between the parenthesis and the `=` the line is the
    statement with a comparison as its argument: `CLS (1) + 2 = 5`, `CLS ((1) = 5)`, `CLS (1) <> 5`, `CLS -(1) =
    5`, `LOCATE (1) * 2 = 2` compile (`x51`). **`WIDTH` is the exception:** `WIDTH (80) = 80` stores into an
    array named `WIDTH` (made 0 to 10 like any array used without `DIM`), `WIDTH = 80` into a variable, and
    `PRINT WIDTH(1)` reads the array (`x51`).
  - **A statement's name and a colon at the start of a line (`v27_x58`–`x63`; measured while group 5 was
    built).** It is the statement with no argument, never a label: `CLS: l = 1` and `CLS : l = 2` are
    `sub_clsDest(NULL,NULL,NULL,0)` and then the assignment, and so for `SCREEN`, `COLOR`, `LOCATE`, `WIDTH`,
    `VIEW`, `WINDOW`, `PALETTE`, `SLEEP`, `_DISPLAY`, `CLOSE`, `RANDOMIZE`, `SHELL`, `_SCREENHIDE`, `_ICON`,
    `_MOUSEHIDE` and `_FREEIMAGE` (`x58`), also with nothing after the colon. A statement that needs an argument
    is "Syntax error - Reference: …" (`PSET:`, `DRAW:`, `KILL:`, `_LIMIT:`, `x59`–`x62`; `LINE:`, `PCOPY:` and
    `CIRCLE:` likewise), and `GOTO CLS` after a line `CLS: l = 1` is "Invalid label!" (`x63`). Civil War
    Strategy starts ten lines with `CLS :`.
  - **`PALETTE USING` (`v27_x64`).** `PALETTE USING a(0)` and `PALETTE USING a` are accepted: the old compiler
    reads the form by hand (`qb64pe.bas` 7238), outside the template of `PALETTE` (`[?,?]`). It is the only
    built-in of `m3-window-output` with a form of its own; `SLEEP`'s two mentions in the source are for the
    debugger, and `WIDTH`'s make its name free for a variable or an array.
  - **The cast before the fill of `PAINT` and an expression (read from the C++ of `qb64pe -z`, 14
    expressions).** The cast stands before the first operand only: `PAINT (1, 2), l + 1` is `(uint32)*__LONG_L+
    1`, `l AND 3` `(uint32)*__LONG_L& 3`, `l * q` `(uint32)*__LONG_L**__INTEGER64_Q`. C++ then computes in
    unsigned (or 64-bit) arithmetic, which gives the same low 32 bits as casting the whole value, and every
    other operation is one call or parenthesised operand behind the cast (`(uint32)qb_safe_idiv(…)` for `\`,
    `qb_safe_mod` for `MOD`, `(uint32)qbr(…)` for `/`, `^` and anything with a float, `(uint32)-(…)` for a
    comparison and a negation, `(uint32)~(…)` for `NOT`). So the fill libqb gets is the value's low 32 bits
    in every case.
  - **The first screen programs of the new compiler (2026-10-12).** Of the 45 programs of `tests\corpus\screen`
    42 compile and all 42 leave the recorded state on their first run, with and without folding: the calls
    are the old compiler's, and what they draw is libqb's. The three others wait on `PLAY` (`g03`), graphics
    `GET` and `PUT` (`g06`) and `_RGB32`/`_RGBA32` (`m32`).
- **What Civil War Strategy needs to build, apart from sound (`v28_*`, 100 programs: 83 measured 2026-10-12
  before the design of `m3-get-put-bload`, `v28_d`, `v28_e`, `v28_f` and `x82`–`x95` on 2026-10-13 as its tasks 1.2 and 4.4; the
  C++ read with `qb64pe -z` from programs outside the repository).**
  - **`TAB` and `SPC` are functions (`v28_a_tab_spc`, `x01`, `x03`–`x09`, `x53`).** `PRINT TAB(5); "a"` is
    `qbs_set(tqbs,func_tab( 5 ))`, `makefit`, `qbs_print(tqbs,0)`: the lines of any string item. To a file it is
    `sub_file_print(tmp_fileno,func_tab( 5 ), 0, 0, newline)`; `WRITE TAB(5)` writes the string in quotes.
    `t$ = TAB(5)`, `SPC(i) + "x"` and `LEN(TAB(l))` compile. The slot is a LONG: an INTEGER, a LONG and an
    `_INTEGER64` are passed as they are, a SINGLE and a DOUBLE through `qbr`. A string item may follow without a
    separator (`PRINT TAB(5) "a"`, `PRINT "x" TAB(8) "y"`), a number or another `TAB` may not ("Expected
    operator in equation", `x03`, `x04`). Rejected: a string argument ("Number required for function"), no
    argument, two, no parentheses, `TAB(5) = 1`. In a file the column is the file's (`"abcdefgh"; TAB(5)`
    starts a new line there); on the console `POS(0)` is always 1, so `TAB(n)` is n - 1 blanks wherever the
    line stands (`"a"; TAB(5); "b"` is `a`, four blanks, `b`).
  - **`DEF SEG`, `BLOAD`, `BSAVE`, `POKE`, `PEEK` are plain calls (`v28_b_bload`, `x01`, `x26`–`x35`,
    `x42`–`x44`, `x61`, `x62`).** `DEF SEG` is `sub_defseg(NULL,0)`, `DEF SEG = x` `sub_defseg(x,1)` (LONG slot);
    `BLOAD f$` `sub_bload(f,NULL,0)`, `BLOAD f$, x` `sub_bload(f,x,1)`; `BSAVE f$, x, n` `sub_bsave(f,x,n)`;
    `POKE x, v` `sub_poke(x,v)`; `PEEK(x)` `func_peek(x)`, a LONG. Rejected: `DEF SEG 5`, `DEF SEG =`, a string
    for the segment, `DEF` alone ("Command not implemented"), `BLOAD` with none or three arguments or a number
    for the name, `BSAVE` with two. After `CALL` `BLOAD`, `BSAVE`, `POKE`, `PUT` and `_FULLSCREEN` are accepted
    (`x65`, `x66`, `x68`–`x70`), `CALL DEF SEG` is not (`x67`).
  - **`VARPTR` and `VARSEG` are written by the compiler, and put their variable into conventional memory.**
    `VARPTR(a(i))` is `((array_check((i)-A[4],A[5]))*2)`: the element's index from the lower bound times the
    element's size, with the bounds check (two dimensions: the combined index); `VARSEG(a(i))` is
    `( ( ((ptrszint)(A[0])) - ((ptrszint)(&cmem[0])) ) /16)`, the same for every element. A scalar is
    `((unsigned short)(((uint8*)V)-&cmem[1280]))` and segment 80; a string's `VARPTR` is its
    `cmem_descriptor_offset`. `PRINT` and `+` take both as `_INTEGER64` (`qbs_str((int64)(…))`; `qb64pe.bas` sets
    `typ& = 64&`, against the table's LONG). The index inside `VARSEG(a(i))` is not written at all. A variable named in `VARPTR`, `VARSEG`, `SADD` or `VARPTR$`
    anywhere in the program is allocated differently (`cmemlist` in `qb64pe.bas`, with a second pass of the
    compiler): a static array's data comes from `cmem_static_pointer`, rounded up to 16 bytes, with error 257
    past 65,536 bytes and flags `1+2+4` in place of `1+2`; a dynamic one from `cmem_dynamic_malloc`; a scalar
    from `dblock+cmem_sp`; a string from `qbs_new_cmem`. A parameter named so makes the callers' arguments
    conventional too (read in `qb64pe.bas`, 12081 and 12810; not measured). Arrays lie in the order they are declared (`v28_b_bload`: three arrays of 20 bytes are 2
    segments apart). `BSAVE` writes 7 bytes of header and the block; `BLOAD` reads it back at the given offset
    or, with none, at the saved one; a missing file is error 53. Rejected: a literal, an expression, a function
    call or a string literal as the argument ("Expected reference to a variable/array"), `SADD` of a
    fixed-length string or a literal, `VARPTR$` of a numeric array, `VARPTR(i)` as a statement.
  - **Graphics `GET` and `PUT` are plain calls with an argument made of an array's bytes (`x01`,
    `x10`–`x25`, `x58`–`x60`, `x63`, `x64`).** `GET (x1, y1)-(x2, y2), a` is `sub_graphics_get(x1,y1,x2,y2,
    byte_element((uint64)(&(((int16*)(A[0]))[0])),((2*(A[2]&1)*A[5])-(2*(0))),byte_element_N),NULL,step)`: the
    address of the element, the bytes from it to the array's end (0 for an array not allocated), and a
    temporary of 12 bytes declared with the body's variables. `a`, `a()` and `a(lower bound)` differ only in
    the index (`a(5)`: `array_check(( 5 )-A[4],A[5])` in both places); two dimensions multiply the sizes.
    `step` is 1 for `STEP` before the first point and 2 before the second. `PUT (x, y), a[, [_CLIP] [action][,
    colour]]` is `sub_graphics_put(x,y,bytes,action,colour,flags)`: the action `NULL` or 1 to 5 for `PSET`,
    `PRESET`, `AND`, `OR`, `XOR`; flags 1 for `STEP`, 2 for `_CLIP`, 4 for a colour. The coordinates are passed
    as they are, whatever their type. The array may be static or dynamic, of the main module or a procedure, a
    parameter, of a user type, of fixed-length strings (accepted, `x12`, `x13`, `x20`); a name with no array is
    that plain variable's bytes (`GET …, i`; an undeclared name too, `x59`). Rejected: no array, one point for
    `GET` or two for `PUT`, an expression or a `CONST`, an unknown action word, two actions, `_CLIP` after the
    action. `GET (1, 2)-(3, 4), 5` is "UNEXPECTED INTERNAL COMPILER ERROR!" (`x15`).
  - **`_RGB32`, `_RGBA32`, `_FULLSCREEN` (`x01`, `x45`–`x52`).** `_RGB32` with one to four LONG slots is
    `func__rgb32(…)` with the arguments it is given (C++ overloads), `_RGBA32` `func__rgba32(r,g,b,a)`; both
    `_UNSIGNED LONG` (`PRINT` `(uint32)`, `HEX$` width 8, `+ 1` an `_INTEGER64`). `_FULLSCREEN [mode][,
    _SMOOTH]` is `sub__fullscreen(mode,smooth)`: `NULL` or 1 to 4 for `_OFF`, `_STRETCH`, `_SQUAREPIXELS`,
    `OFF`, and 0 or 1; the function is `func__fullscreen()`, a LONG. Rejected: none or five arguments for
    `_RGB32`, three for `_RGBA32`, a string, an argument left out, `_FULLSCREEN 5`, `_SMOOTH` first, two modes.
  - **A line `name:` with the name of a SUB (`v28_c_labels`, `x71`–`x81`; `validlabel` and the label check of
    `qb64pe.bas`, 27121 and 13027).** With no jump to the name anywhere in the program the line is a call of the
    SUB: in the main module, in the SUB itself (`x77`), twice (`x73`), before any `DECLARE`
    (`x78`), and with a statement after the colon (`x79`). When any `GOTO`, `GOSUB`, `RESTORE` or `ON ERROR
    GOTO` names the word, the compiler starts again with the word as a label in every body (`x80`, `x81`: a jump
    in one SUB makes the main module's line a label too), `CALL name` still calls the SUB, and the label rules
    hold: twice in a body is "Duplicate label" (`x74`), a jump from a body without it "Label 'one' not defined"
    (`x72`, also with no such line at all, `x71`). A SUB that needs an argument is always a label (`x76`), as is
    a FUNCTION (`x75`). Civil War Strategy has all three cases: `topbar:` in its main module is a call (no jump
    names it), `ships:` and `cannon:` in `SUB navy` are labels (`GOTO ships`, `GOTO cannon`) beside `CALL ships`
    and `CALL cannon`. **Which statements make the word a label (`x82`–`x92`, each with the statement as code
    that is not reached):** `RESUME one`, `RETURN one`, `ON n GOTO two, one`, `ON n GOSUB one`, `ON KEY(1) GOSUB
    one`, `ON TIMER(1) GOSUB one`, `ON STRIG(0) GOSUB one` and `IF n = 5 GOTO one` do; `IF n = 5 THEN one` and
    `CALL one` do not (the line stays a call, and `THEN one` is one too); `RUN one` is "Name already in use
    (one)". These are all the places of `qb64pe.bas` that make a reference to a label (3963 and 4025 make the
    label itself; 7627, 7807 and 7968 the three `ON <event> GOSUB`; 9717 `GOTO`; 9808 `RUN`; 9968 `RETURN`; 10027
    and 10092 `RESUME` and `ON ERROR GOTO`; 10137 `RESTORE`; 27593 and 27675 `GOSUB` and `ON n GOTO/GOSUB`).
  - **Conventional arrays used as any array is, and the index of `VARPTR` and `VARSEG` (`v28_d_varptr_index`,
    `v28_e_cmem_arrays`, `x95`).** A FUNCTION in the index of `VARPTR` is called once; in the index of
    `VARSEG` it is not called, and an index outside the bounds raises nothing there (`VARSEG(a(99))` is the
    array's segment). `VARPTR(a(11))` of `a(1 TO 10)` raises error 9 and gives 0. Stored into an INTEGER, a
    SINGLE or an `_INTEGER64` the value goes as it is; `STR$` and operators take it as an `_INTEGER64`;
    `HEX$(VARPTR(…))` is `func_hex(…,0)`, a width of 0 (accepted, `x95`). **An array listed in `COMMON` is
    dynamic to the old compiler** (`COMMON SHARED cm()` with `DIM SHARED cm(1 TO 20)`): in conventional memory
    its descriptor starts with flag 4 in place of 0 and nothing else of its declaration changes, because every
    allocation of a dynamic array of numbers tests that flag when it runs (`if (A[2]&4){ A[0]=(ptrszint)
    cmem_dynamic_malloc(…); … memset … }else{ … calloc … }`, also for the new block of a `REDIM`, and
    `cmem_dynamic_free` or `free` for the old one and in `ERASE`). So `DIM dy(1 TO n)`, `REDIM`, `REDIM
    _PRESERVE`, `ERASE` and passing to a SUB that erases or redims work on a conventional array as on any
    other, and `PEEK` reads the new block after each (`v28_e`). Static arrays lie from the start of the block
    in the order of their `DIM`; dynamic ones come from the other end (a later one has a lower segment). A
    static one larger than a segment (`DIM big(1 TO 40000) AS INTEGER`, 80,000 bytes) ends the program at its
    start with "Out of memory" (error 257), before its first statement (`v28_f_cmem_too_large`).
  - **`GET` and `PUT` with a parenthesised first argument are the graphics form (`x93`, `x94`).** `GET (1), ,
    x` and `PUT (i), 2, x` are "Syntax error - Reference: GET [STEP] (column1, row1)-…": a statement that
    starts with `(` is never the file form. In the array's place an element's index is written twice
    (`byte_element((uint64)(&(…[array_check((FUNC_F(…))-…)])),(…-(2*(array_check((FUNC_F(…))-…)))),…)`), so a
    FUNCTION in it runs twice. Other kinds in that place, each accepted: an array of a `TYPE`
    `(uint64)(((char*)R[0])+((index)*6+0))`, a fixed-length string array `&((uint8*)(FS[0]))[(index)*4]`, a
    string variable `T->chr,T->len`, a `TYPE` variable `((char*)ONE)+((0+0)),6-((0+0))`, a name with no
    array the plain variable (`(uint64)__SINGLE_TS,4`).
- **Sound and `$EXEICON` (`v29_*`, 130 programs: 110 measured 2026-10-13 before the design of `m3-sound`, as its
  task 1.1; `v29_d`, `v29_f`, `v29_g`, `v29_h` and `e29`–`e31` the same day as its task 1.2, `x87`–`x98` as its
  task 2.1, `e32` by the review of the change; the C++ read with
  `qb64pe -z` from programs outside the repository; numbers `x33`–`x39` are not used).**
  - **Every sound form is a plain call (`x01`–`x86`: 43 accepted, 36 rejected).** `SOUND f, d` is
    `sub_sound( 440 , 1 ,NULL,NULL,NULL,NULL,NULL,0,1)`: seven slots (frequency, duration, volume, pan position:
    SINGLE; waveform: LONG; waveform parameter: SINGLE; voice: `_UNSIGNED LONG`), then 1 for `WAIT`, 2 for
    `RESUME` or 0, then the mask of what was given. A SINGLE in the waveform slot goes through `qbr`. Accepted:
    `SOUND` with no argument at all (`x03`; error 5 when it runs), `SOUND WAIT`, `SOUND RESUME`, `WAIT` or
    `RESUME` after the arguments without a comma (`x13`, `x15`, `x29`), a trailing comma (`SOUND 440, 1,`,
    `x21`), `CALL SOUND(440, 1)`, lower case. Rejected: one argument, no frequency, no duration, eight
    arguments, a comma before `WAIT` ("Name already in use (WAIT)"), `WAIT RESUME`, a string, `SOUND(440, 1)`,
    all commas (`x31`), `SOUND = 1`, `l = SOUND`, a variable named `wait`. `PLAY a$[, b$][, c$][, d$]` is
    `sub_play(a,b,c,d,mask)`, a fixed-length string taken as a string, a trailing comma accepted (`x50`);
    rejected: none or five strings, no first string, a number, `PLAY(t, t)`, `PLAY(0)` and `CALL PLAY(0)` ("1st
    sub argument requires a string"), `PLAY = 1`. The function is `func_play( 0 ,0|1)`, a DOUBLE, also bare
    (`d = PLAY`); rejected: `PLAY()`, a string, two arguments. `BEEP` is `sub_beep()`, also after `CALL`;
    `BEEP 1` and `BEEP()` are "SUB does not require any arguments".
  - **A built-in statement with a template after `CALL` (`x27`, `x54`, `x85`, `x87`–`x98`; `v26_x74`,
    `v27_x02`, `x14`, `x15`, `v28_x65`–`x70`; `qb64pe.bas` 10617–10627).** After `CALL name` comes nothing, or
    an opening parenthesis as the third element and a closing one as the last, with something between them;
    the compiler drops the two and reads the rest as the statement without `CALL`, by its template. So `CALL
    SOUND(440, 1 WAIT)`, `CALL SOUND(WAIT)`, `CALL SOUND`, `CALL LINE(-(3, 4))`, `CALL PLAY(t, t)` and `CALL
    CLS` are accepted; `CALL CLS()`, `CALL SOUND(440, 1) WAIT`, `CALL LINE((1, 2)-(3, 4)), 5`, `CALL LOCATE 1,
    2`, `CALL VIEW PRINT` and `CALL DEF SEG` are "Expected CALL sub-name (...)"; `CALL SOUND(440)`, `CALL PLAY`
    and `CALL PSET(1, 2)` are the statement's own "Syntax error - Reference: …"; `CALL PLAY(0)` is "1st sub
    argument requires a string".
  - **`PLAY` is no event word to the old compiler (`x61`–`x64`).** `ON PLAY(5) GOSUB 10` is `ON <number> GOSUB`
    with the function (`ongo = qbr_float_to_long(func_play( 5 ,0|1))`); `PLAY ON`, `PLAY OFF` and `PLAY STOP`
    are "Name already in use (ON)" and so on.
  - **The name and a colon (`x24`, `x65`, `x72`, `x78`–`x83`).** `BEEP:` and `SOUND:` are the statement with no
    argument, alone on a line or before another statement; `PLAY:` is the statement without its string, a
    syntax error. `GOTO BEEP` and `GOTO SOUND` after such a line are "Invalid label!".
  - **What a program can observe of sound (`v29_a_sound_queue`, `v29_b_sound_wait`, `v29_f_queue_slice`).**
    `PLAY(n)` is the seconds left to play in voice n (voice 0 for `PLAY`, `PLAY(4)` and `PLAY(-1)`). In the
    foreground, the default, a statement holds the program for 0.95 times what is queued minus a quarter of a
    second (`AwaitPlaybackCompletion` in `audio.cpp`), so a sound under about 0.26 s returns at once. After
    `PLAY "MB"` every statement returns at once and adds its length to `PLAY(0)`: a `SOUND` its duration
    divided by 18.2, a `PLAY` string its notes and pauses by tempo, length and dots (`T120 L4 C` 0.5 s, `T60 L1
    C` 4 s, `T60 L4 C.` 1.5 s; a frequency of 0 and a duration of 0 are a pause and nothing). Tempo, length
    and octave hold from one `PLAY` to the next; `MS`, `ML` and `MN` do not change a note's length in the
    queue. `PLAY "MF"` makes the next statement wait for all that is queued. Read directly before and after a
    statement and rounded to tenths, with lengths of whole tenths, the numbers were equal in three recordings
    and in 24 runs made eight at a time (`v29_f`); `v29_a` prints between its readings and has three lengths
    at a rounding edge, so its numbers are equal only run alone.
  - **`SOUND WAIT` and `SOUND RESUME`.** After `SOUND WAIT` no statement holds the program and what is queued
    is not in `PLAY(0)` (0 until `SOUND RESUME`, then the whole of it). A second `SOUND WAIT` or `SOUND RESUME`
    is no error.
  - **Voices (`v29_g_voice`).** Background and foreground are set per voice: with voice 0 in the background,
    `SOUND 440, 18.2, , , , , 1` holds the program and adds nothing to `PLAY(0)`. `MB` in a voice's own string
    (`PLAY "", "MB"`) puts that voice in the background. **A `PLAY` with more than one string never holds the
    program**, whatever its voices' modes, **and it ends a `SOUND WAIT`** (`v29_h`, line 25): `PlayMML` pauses
    all voices, reads the strings and resumes all.
  - **Volume 0 (`v29_d_volume_zero`).** `SOUND f, d, 0` and `PLAY "V0 …"` queue and wait exactly as at the
    default volume. The volume is the voice's, kept for the next statement and shared by `SOUND` and `PLAY`
    (read in `audio.cpp`; not observable).
  - **Run-time errors, all error 5 (`v29_c_sound_errors`, `v29_h_sound_ranges`).** `SOUND` with no argument; a
    frequency below 20 other than 0, or above 32767; a duration below 0 or above 65535; a volume outside 0 to
    1; a pan position outside -1 to 1; a waveform outside 1 to 10; a waveform parameter outside 0 to 1, only
    in a statement that itself gives waveform 9 (`SOUND 440, 1, , , 4, 2` and `SOUND 440, 1, , , , 2` after a
    waveform 9 raise nothing); a voice outside 0 to 3. A `PLAY` string: an unknown command (`H`), `L0`, `L65`,
    `N85`, `V101`, in any voice; no error for `O7`, `T31`, `T256` (QuickBASIC's limits), `V0`, `V100`, an
    empty string. **Without a sound device** (read in `PlayWaveform` and `PlayMML`, not measured): the first
    three checks of `SOUND` and its voice are made before the device is asked for and raise the error on any
    machine; volume, pan, waveform and its parameter come after and raise nothing; no `PLAY` string is read,
    and `PLAY(n)` is 0.
  - **`BEEP` is not libqb's audio.** `sub_beep` is in the window part (`gui.cpp`, the system's bell); it has no
    dependency in the table, while `SOUND` and `PLAY` have `DEPENDENCY_MINIAUDIO`: the make flag
    `DEP_AUDIO_MINIAUDIO=y`, written last (`qb64pe.bas` 13721), `-DDEPENDENCY_AUDIO_MINIAUDIO` and `audio.a` in
    the compile log.
  - **`$EXEICON:'file'` (`e01`–`e31`; `qb64pe.bas` 3829–3880 and 13551–13604).** The line's text in upper case
    starts with `$EXEICON` (`$EXEICONS:'…'`, `$EXEICON '…'` and lower case are the same, `e20`, `e08`, `e10`);
    the name is what stands between the first two `'`, blanks at its ends dropped; the rest of the line is
    dropped, a statement after a colon included (`e09`, `e22`, `e24`: `PRINT "same line"` does not run). Where
    it stands the compiler writes `sub__icon(NULL,NULL,0)`, the call of `_ICON` with no argument (`e28`: between
    two `PRINT`s; also in a SUB, after `SYSTEM`, in an included file, beside an `_ICON` statement), and the
    program is compiled with `-DDEPENDENCY_ICON`. The build gets `internal\temp\icon.rc` with the line `0 ICON
    "icon.ico"`, a copy of the file beside it, `DEP_ICON_RC=y` (after `DEP_CONSOLE=y`), `windres` and
    `icon.o`. **Where the file is looked for:** a name that starts with `./` or `.\` in the folder of the main
    source file (`e01`, `e21`; from an included file too, `e12`), the rest of the name added to the folder as
    text, so `.//v29_inc/icon.ico` is found there too (`e32`, measured 2026-10-13 by the review of the change;
    `qb64pe.bas` 3853); a name with a drive letter as written; one
    leading slash on the compiler's drive (both tried outside the repository); **any other relative name in
    the old compiler's own folder**, so a file beside the program is "File 'icon.ico' not found" (`e02`,
    `e08`, `e18`; the message holds only the last part of the name, and the resolved folder for a `./` name,
    `e04`). Errors: none or an empty name, double quotes, no closing quote: "Expected $EXEICON:'filename'"; a
    second one: "$EXEICON already defined in line n", before its own name is read (`e05`, `e27`), also when
    the second is in an included file (`e29`), with n a line of the main file (the `$INCLUDE` line for a first
    one in an included file, `e31`); one in a `$IF` branch that is not taken does not count and is not read
    (`e11`, `e30`). After a colon it is "Unexpected character on line" (`e13`), after a tab "Syntax error"
    (`e23`): the line rules of every metacommand. In a comment or after `REM` it is a comment (`e14`). A file
    that is not an icon passes the front end and fails in the build (`e16`).
- **`PRINT USING`, `GET` and `PUT` on a file, `LOCK`, `EXIT SELECT`, `_IIF`, `_MIN`, `_MAX`, `_CLAMP`, the
  shift functions, `CHAIN` and `LPRINT`, `FIELD`, `LSET` and `RSET` (`v30_*`, 177 programs and one table: 100
  measured 2026-10-13 before the design of step 4 of M3, what the twelve corpus programs that still wait name,
  67 for `FIELD`, `LSET` and `RSET` as the step's first task group, `x66` and `x67` while the step was
  built, and `x171`–`x178` by the review of the change; the C++ read with `qb64pe -z` from programs outside the repository).** Everything here that happens when the program runs is libqb's, which
  both compilers link: the new compiler has to write the same calls, and the recorded outputs are what a slice
  program may expect.
  - **`PRINT USING` at compile time (`x01`–`x19`; `xprint`, `qb64pe.bas` 27713–27851, and 27285 for a file).**
    The form is `PRINT [#n,] [items] USING format; value {;|,} value … [;|,]`. The format is any string
    expression (a variable, `f + f`, `LEFT$(…)`, a fixed-length string, one in parentheses) and ends at the first
    `;` outside parentheses; at least one value follows. `?` works as `PRINT` (`x14`). Rejected with "Expected
    PRINT USING formatstring ; ..." : a comma after the format, no value, nothing after the `;`, a number as
    the format, no separator after the format (`x01`–`x04`, `x06`), and `PRINT USING #1, …` (`x17`); with a
    file number the message is "Expected PRINT USING #filenumber, formatstring ; ..." (`x07`, `x16`). No format
    at all and an empty value (`; ;`) are "Syntax error" (`x05`, `x09`, `x10`); two values with no separator
    "Expected operator in equation" (`x08`); a second `USING` "Name already in use (USING)" (`x11`, `x12`); a
    `TYPE` value "User defined types in expressions are invalid" (`x15`). A whole array (`a()`) is accepted as a
    value (`x18`, which prints 0). Measured with task 4.1 of `m3-console-files` (24 forms, one program each,
    read with `qb64pe -z` outside the repository): **two values always need a separator**, also next to a
    string literal (`PRINT USING "&&"; "a" "b"` and `…; 5 "a"` are "Expected operator in equation"), where a
    plain `PRINT` takes the two as joined by `;`; an item before `USING` needs none (`PRINT "a" USING "##"; 5`
    and `PRINT 1 USING …` are accepted); `;;` at the end and `n, , 5` are "Syntax error"; one trailing `,` is
    accepted and leaves the line open as `;` does; a `TAB(n)` alone counts as the value the statement needs;
    `TAB(3) + "x"` is a string value, not a `TAB`; `LPRINT #1, "a"` is "Invalid expression".
  - **The calls of `PRINT USING`.** `tqbs=qbs_new(0,0); tmp_long=0;`, the format copied once into a string of
    the statement's own (`print_using_formatN`) before any value is evaluated, then one call per value, each
    followed by `if (is_error_pending()) goto skip_puN;`: a string `tmp_long=print_using(fmt,tmp_long,tqbs,s)`;
    a SINGLE `print_using_single(fmt,v,tmp_long,tqbs)`, a DOUBLE `print_using_double`, a `_FLOAT`
    `print_using_float`, an `_UNSIGNED _INTEGER64` `print_using_uinteger64`, every other integer type
    `print_using_integer64` with no cast. `TAB(n)` or `SPC(n)` among the values prints what is built, prints the
    blanks and empties the string (`qbs_print(tqbs,0); qbs_set(tqbs,func_tab(…)); makefit(tqbs);
    qbs_print(tqbs,0); tqbs->len=0;`), and the place in the format is kept. At the end `qbs_print(tqbs,1)`, or
    `qbs_print(tqbs,0)` when the statement's last element is `;` or `,`; after an error what was built is
    printed with no line end and the error stays (`g_tmp_long=new_error; new_error=0; qbs_print(tqbs,0);
    new_error=g_tmp_long;`). To a file `sub_file_print(tmp_fileno,tqbs,0,0,nl)` in each of those places (and no
    `makefit` around a `TAB`); there the check after the format jumps to the statement's first label, `skipN`,
    past the two `qbs_free` calls, where the console form jumps to `skip_puN`. The string of the format is
    `static qbs *print_using_formatN;` among the body's data, N the number of the statement's `skipN`. After
    items before `USING` the console form empties its string first (`qbs_set(tqbs,qbs_new_txt_len("",0));`).
    Items before `USING` are printed first as a plain `PRINT` prints them (`PRINT "a"; USING "##"; 5`, `PRINT
    1; 2; USING …`, `PRINT "a"; "b" USING …`, `PRINT TAB(5); USING …`, `PRINT ; USING …`, and after a comma,
    which a redirected console cannot show). A comma between two values does what a semicolon does: no zone.
  - **What libqb prints (`v30_a_using_numbers`, `v30_b_using_strings`, `v30_c_using_file`).** A number field is
    its `#` signs, right-aligned, a minus sign taking one place; a number that does not fit is printed whole
    after `%`. A point sets the decimals, rounded half away from zero (`0.25` in `#.#` is `0.3`, `2.5` in `##`
    is ` 3`), and a second point starts a new field. `+` before the field puts the sign at the field's first
    place (`+  5`), after it at the end; `-` after it is a blank or a minus; `-` before it is text (`- -5`).
    `**` fills with stars, `$$` puts a dollar before the number, `**$` both, each counting as places; one `$`
    or `*` is text. A comma left of the point groups the thousands and counts as a place; right of the point
    it is text. `^^^^` is an exponent of two digits, `^^^^^` of three, and its letter is the value's type: `E`
    for a SINGLE or an integer, `D` for a DOUBLE, `F` for a `_FLOAT`; three carets are text. `_` makes the next
    character text. `!` is the first character (a blank for an empty string), `&` the whole string, `\`, n
    blanks and `\` a field of n + 2 characters, padded with blanks; anything else between the backslashes
    makes them text. Text after a field is printed up to the next field or the format's end; when values are
    left the format starts again, and text after the last value used is printed only up to the next field
    (`"## ## ##"` with two values is ` 1  2 `). A format with no field prints its text and raises error 5; an
    empty format too, after one stray character (`[a` in `v30_b`: libqb reads past the empty string). A
    string for a number field or a number for a string field is error 13, a value that is not finite error 5,
    each after the text before it is printed. To a file the same text is written; a file open for input gives
    error 54, one not open 52, and an error in a value comes before the file's.
  - **`GET` and `PUT` on a file, the forms (`x20`–`x48`, `v30_d_binary`; `qb64pe.bas` 11880–11955).** The
    template is `[#]?[,[?][,?]]`. With a variable: `sub_get(n,position|NULL,byte_element((uint64)address,
    bytes,byte_element_N),mask)` and `sub_put(…)`, mask 1 when the position is given; the place and size are
    those of graphics `GET` (above): a numeric variable `(uint64)__INTEGER_I,2`, a fixed-length string
    `X->chr,6`, a `TYPE` variable or member, an element `&(…[array_check(…)])`, a whole array `a()` from
    element 0 with the bytes of all its elements (also of a `TYPE`, of fixed-length strings, dynamic, of two
    dimensions: `x43`–`x46`). A variable-length string or an element of a string array goes to other entry
    points: `sub_get2(n,position|NULL,s,mask)` with the string itself, `sub_put2(n,position|NULL,
    byte_element((uint64)s->chr,s->len,…),mask)`. With no variable (`GET #1`, `GET #1, 2`, `GET #1,`, `PUT #1,
    3`) the call is `field_get(n,position|NULL,mask)` or `field_put(…)`. The file number is any numeric
    expression after `#` (`GET #k + 1, …`), but a first argument that starts with `(` is the graphics form and
    is rejected as one (`x36`). `CALL GET(1, 1, c)` and `CALL PUT(#1, 1, c)` are accepted. The third argument
    must be a variable: a literal, an expression, a function's result and a `CONST` are "Expected variable
    name/array element" (`x20`–`x22`, `x33`, `x34`), a variable in parentheses is accepted (`x23`), and `GET
    #1, , "lit"` is "UNEXPECTED INTERNAL COMPILER ERROR!" (`x24`). Also rejected: a `_BIT` variable or array
    ("Variable/element cannot be _BIT aligned"), a whole array of variable-length strings ("Cannot pass array
    of variable-length strings"), a `TYPE` with a `STRING` member ("UDT must have fixed size"), a whole member
    array `r.v()` ("Expected array index", `x48`), `GET #1, ,`, no argument and four arguments ("Syntax error -
    Reference: …"), `GET #, , c` ("Invalid expression"), a string as the file number or the position ("1st" or
    "2nd sub argument requires a number"). Measured with task 5.1 of `m3-console-files` (`x67`, and 43 forms
    read with `qb64pe -z`): **a name alone is a scalar variable**, also when only an array has the name (`PUT
    #1, 1, a` with `DIM a(1 TO 3) AS INTEGER` writes the 4 bytes of a new SINGLE `a`; with a LONG scalar `b`
    beside an array `b` its 4 bytes); only `a()` is the whole array. Measured by the review of the change (`x171`–`x178`): a
    FUNCTION's name with empty parentheses is "Expected (...)" as the variable of `GET` and `PUT` and as the
    string of `FIELD`, `LSET` and `RSET`; an array that is not declared is accepted by `GET` and `PUT`
    (`und()`, and `und$()` too, although a declared array of variable-length strings is rejected). The position is passed as it is, an
    `_INTEGER64` too (`sub_put( 1 ,*__INTEGER64_Q,…)`: libqb's parameter is an `int64`), a float through `qbr`.
    A fixed-length string element is `qbs_new_fixed(…)->chr`, a `TYPE` variable `(((char*)__UDT_R)+((0+0)))`
    with the type's size, a whole array of fixed-length strings `&((uint8*)(A[0]))[(0)*n]` and of a `TYPE`
    `(((char*)A[0])+(((0)*size+0)))`, each with the bytes of all elements. `sub_put2` names a string element
    twice, for `->chr` and for `->len`, so an index is evaluated twice there.
  - **A file open `FOR BINARY` (`v30_d_binary`).** `PUT` writes the variable's bytes (INTEGER 2, LONG 4,
    SINGLE 4, DOUBLE 8, `_INTEGER64` 8, `_BYTE` 1, `_FLOAT` 32, `_OFFSET` 8; a string its characters with no
    length), `GET` reads as many: a variable-length string as many as it is long, so an empty one reads
    nothing. The position is a byte from 1, rounded as a number is (`2.5` is 2, `3.5` is 4); left out, it is
    the byte after the last one read or written (`SEEK(n)`; `LOC(n)` is one less). A `GET` at or past the end
    sets the bytes it cannot read to 0, sets `EOF`, moves on only by what it read and does not grow the file; a
    `PUT` past the end fills the gap with 0. Position 0 or below is error 63 and moves nothing. `GET` or `PUT`
    with no variable is error 54 here. A file number that is not open is error 52; a file open for `OUTPUT`,
    `INPUT` or `APPEND` error 54 for both; `ACCESS READ` gives error 75 for `PUT`, `ACCESS WRITE` for `GET`.
    `PRINT #` to a binary file is error 54 (`177_open_binary` of the corpus expects that); `LINE INPUT #`
    reads from one.
  - **A file open `FOR RANDOM` (`v30_e_random`).** The record length is `LEN =`, 128 when left out (also for
    `OPEN f AS #1`); 0 is error 5 at the `OPEN`, a fraction is rounded, 40000 is accepted, and a binary file
    ignores it. The position is a record number from 1; left out, the record after the last one (`SEEK(n)`;
    `LOC(n)` the last one). **`PUT` writes only the variable's bytes at the record's start**, so the file ends
    after them and not at the record's end, and a later record leaves a gap of 0. A variable-length string is
    written as two bytes of length and its characters, and read back by that length. A value longer than the
    record (a string with its two bytes included) is error 59 for `PUT` and for `GET`, and nothing is written.
    A record past the end reads as 0 and sets `EOF`; record 0 or below is error 63, as is `SEEK #n, 0`. With no
    variable `GET` reads a record and `PUT` writes a whole one (the buffer `FIELD` would name: `PUT #1, 9` on
    records of 14 bytes makes the file 126 long), and each moves on by one record.
  - **`LOCK` and `UNLOCK` (`v30_i_lock`, `x50`–`x56`).** Template `[#]?[,[?][{To}?]]`:
    `sub_lock(n,first|NULL,last|NULL,mask)` with mask 0 (`LOCK #1`, also `LOCK #1,`), 1 (`LOCK #1, 2`) or 3
    (`LOCK #1, 2 TO 4`), `sub_unlock` the same, both also after `CALL`. `TO` with no first record, with no last
    one or with no comma before the records is "Name already in use (TO)"; none or three arguments a syntax
    error; a string "1st sub argument requires a number". When it runs: a record of 0 or below is error 5, a
    range the wrong way round error 75, an `UNLOCK` that matches no `LOCK` error 70, a second `LOCK` of the
    same bytes error 70; the program itself reads and writes what it locked, a second `OPEN` of the file
    cannot read it (error 70), and `CLOSE` ends the lock. Every mode takes it; a file not open is error 52.
  - **`EXIT SELECT` (`v30_f_exit_select`, `x60`–`x65`; `qb64pe.bas` 7491–7503).** It is `goto sc_N_end;` for
    the innermost `SELECT` that has a `CASE` or `CASE ELSE` open around it, from inside an `IF` and from inside
    loops within the `CASE`; a loop around the `SELECT` goes on. In `SELECT EVERYCASE` it leaves the whole
    block, where `EXIT CASE` goes on with the next `CASE`. `EXIT CASE` in a plain `SELECT` is accepted and
    leaves the block (`x62`). With no `CASE` open, also in a SUB called from one, it is "EXIT SELECT without
    SELECT"; between `SELECT` and the first `CASE` "Expected CASE expression" (`x61`). `EXIT CASE` in a `CASE`
    is `goto sc_ec_<n + 1>_end;`, n the number of `CASE`s closed so far in the file, and that label stands at
    the end of each `CASE` body, before what ends the `CASE` (`goto sc_N_end;`, or the mark of a match of
    `SELECT EVERYCASE`, which is therefore still set); in a `CASE ELSE` it is `goto sc_N_end;`. **So an `EXIT
    CASE` followed in its `CASE` by a `SELECT` with a `CASE` jumps into that inner `SELECT`**, to the end of
    its first `CASE`: the program goes on after the inner `END SELECT` and runs the rest of the outer `CASE`
    (`x66`, measured 2026-10-13 with task 3.1 of `m3-console-files`: `a after the inner SELECT end`; with
    `EVERYCASE` the inner selector's copy was never stored, so its later `CASE`s compare with 0).
  - **`_IIF` (`v30_g_iif_min_max`, `x70`–`x76`, `x88`, `x89`, `x99`; `qb64pe.bas` 20737).** It is C++'s
    `((condition)?(a):(b))`, so only the part chosen is evaluated. The condition is a number (a string is "1st
    function argument requires a number") taken as an integer: a float goes through `qbr`, so `0.5`, `0.4`,
    `-0.5` and `1E-30` are false and `4294967296` is true. The two parts are both strings or both numbers
    ("falsePart and truePart must be of the same type"); two strings may be of different kinds and the value
    is the one chosen, with its length. Two, four or no parenthesised arguments are "Incorrect number of
    arguments - Reference: …", `TYPE` values are rejected, `_IIF` as a statement is a syntax error. Whole
    arrays (`a()`, `b()`) are accepted as the two parts (`x99`, which prints 0).
  - **The type of the result of `_IIF`, `_MIN`, `_MAX` and `_CLAMP` (`v30_result_types.py` and its table, 794
    forms).** One rule for all four (`_CLAMP(a, b, b)` and `_IIF(1, a, b)` have `_MIN(a, b)`'s type for all
    256 pairs), and **not** the usual one: a `_FLOAT` among them gives `_FLOAT`, else a DOUBLE gives DOUBLE,
    else a SINGLE gives SINGLE, whatever the integer beside it (`_MAX(s, q)` is `std::max<float>`); of two
    integers the wider wins, and of two of one width the unsigned one, so `_MIN(b, ub)` with -1 and 255 is 255
    and `_MIN(i, ub)` is an INTEGER. A `_BIT` type counts as narrower than a `_BYTE`; two `_BIT` types give a
    32-bit C++ type printed as an `_INTEGER64`. An `_OFFSET` beats every integer but an `_UNSIGNED _INTEGER64`
    and is printed as an `_INTEGER64`. Two pairs depend on the order: `_INTEGER64` with `_OFFSET`, and
    `_UNSIGNED _INTEGER64` with `_UNSIGNED _OFFSET`, each the first one's C++ type. (`Type_PromoteArithmeticType`
    in `utilities\type.bas` reads as if a SINGLE beside a wider integer gave DOUBLE; the C++ says SINGLE.) The
    result is then converted as any value of that type. **The rule as built** (task 2.1 of
    `m3-console-files`, `widest` in `sema\src\check\ops.rs`, on what the two arguments are believed; 110 more
    forms read with `qb64pe -z`: literals, operator results, elements, members, parameters): a float wins
    over any integer and of two floats the wider; of two unsigned integers the one of more bits, **the second**
    when they have as many; of two integers of one width the unsigned one, **the first** when both are signed;
    else the one of more bits; a `_BIT * n` counts n bits (`_MIN(b40, l)` is an `int64_t`). The result is
    believed that type: `_MIN(o, q)` is an `_OFFSET` to a following operator (`_MIN(o, o) ^ 2` is the `_OFFSET`
    error), `HEX$(_MIN(bt, bt))` has the width of one bit. The arguments are written as they are and C++
    converts them (`std::min<float>(*__INTEGER64_Q,*__SINGLE_S)`). The two special cases of
    `Type_PromoteArithmeticType` (an `_OFFSET`; a SINGLE beside a wider integer made DOUBLE) are never taken:
    each compares with a type value that carries a flag (`ISPOINTER`) no argument's type has.
  - **`_MIN`, `_MAX`, `_CLAMP` (`v30_g_iif_min_max`, `x77`–`x81`, `x94`).** `std::min<T>(a,b)`,
    `std::max<T>(a,b)`, `func_clamp<T>(value,low,high)`, every argument evaluated once, in order. A value that
    is not a number comes out when it is first and not when it is second (`_MIN(nan, 1)` against `_MIN(1,
    nan)`). `_CLAMP` with its bounds the wrong way round still gives a value between them. A string or a `TYPE`
    value is "Expected numeric argument"; a wrong count "Incorrect number of arguments - Reference: …".
  - **`_SHL`, `_SHR`, `_ROL`, `_ROR` (`v30_h_shifts`, `x82`–`x87`).** `_SHL` and `_SHR` are plain functions,
    `func__shl(value,count)` and `func__shr`, of an `_UNSIGNED _INTEGER64` and a LONG giving an `_UNSIGNED
    _INTEGER64`, whatever the argument: `_SHR(-1, 1)` is 9223372036854775807, a float is rounded by `qbr`, and a
    count of 64 or more or below 0 is taken modulo 64 (`_SHL(1, 65)` is 2). `_ROL` and `_ROR` are
    `func__rol<uintN_t>(value,count)` and `func__ror<…>`, N the width of the first argument's type (`_BYTE`
    8, INTEGER 16, LONG 32, `_INTEGER64` and `_OFFSET` 64; a literal by its size: `1` is 16, `70000` 32,
    `5000000000` 64), the result unsigned of that width, the count modulo the width, a negative count turning
    the other way. A float as either argument and a string are "Expected non-floating-point numeric argument"
    (`x84`–`x87`), a `_BIT` type "Unsupported integral type size".
  - **Not in a `CONST`.** `_IIF`, `_MIN`, `_MAX`, `_CLAMP`, `_SHL`, `_SHR`, `_ROL` and `_ROR` in a `CONST`
    expression are "Unexpected element '…'" (`x89`–`x93`).
  - **`CHAIN` and `LPRINT` (`x13`, `x95`–`x98`).** `CHAIN name$` is `sub_chain(name)` for any string
    expression; none, a number or two arguments are rejected. `LPRINT` is `PRINT`'s code with `qbs_lprint` and
    `lprint_makefit` in place of `qbs_print` and `makefit`, between `tab_LPRINT=1;` and `tab_LPRINT=0;`, with
    `USING` too. Neither is run here: `CHAIN` starts `qb64pe.exe -c` beside the program through `cmd`
    (`tests\corpus\README.md`), and `LPRINT` prints a page; the programs hold them behind `IF 0 THEN`.
  - **`FIELD`, `LSET` and `RSET` (`v30_j_field`, `v30_k_field_errors`, `v30_l_lset_rset`, `x100`–`x132`,
    `x140`–`x170`: 67 programs, measured 2026-10-13 as the first task group of `m3-console-files`;
    `qb64pe.bas` 7340–7432 and 11327–11374, libqb's `file-fields.cpp` and `string_functions.cpp`).** The old
    compiler writes calls and nothing else; the record buffer, the list of a file's field strings and the link
    from a string to its field are libqb's (a `field` pointer in every `qbs`), so a field variable is an
    ordinary variable-length string for the compiler.
    - **The calls.** `FIELD #n, w1 AS a, w2 AS b` is `field_new(n);` then `field_add(a,w1);` and
      `field_add(b,w2);`, one call per part, in order, with nothing between them. The file number and each width
      are converted as a LONG is (`qbr(…)` for a float: a width of 2.5 is 2, of 3.5 is 4; a DOUBLE file number
      is accepted). The target is the string itself: `__STRING_A`, an element with its bounds check
      (`((qbs**)(__ARRAY_STRING_E[0]))[array_check(…)]`), a `STRING` member (`*((qbs**)((char*)__UDT_R+(6)))`),
      also of an array element. `LSET a = v` is `sub_lset(a,v);` and `RSET` `sub_rset(a,v);`: the target as a
      string value (a fixed-length string, member or element as `qbs_new_fixed(address,length,1)`), then the
      value as any string expression. The target is evaluated before the value, and in `FIELD` the file number
      before a target's index before its width (`fn ix wd`).
    - **Accepted forms of `FIELD`.** The `#` may be left out, the file number is any numeric expression (`#k +
      1`, `(1)`), a width any numeric expression, also in parentheses. A target is a variable-length string
      variable, an array element, a `STRING` member of a `TYPE` variable or element; a name not seen before with
      `$` (`fresh$`, and `und$(2)`, which makes the array of 0 to 10); a variable in parentheses (`(a)`, `x106`);
      and a whole array `e()`, which is taken as its element 0 with no bounds check (`x109`). One variable may
      stand in two parts (`x127`: it is the last one). `FIELD` works in a SUB on shared and local strings.
    - **Rejected forms of `FIELD`.** A fixed-length string, also as a member or element: "Fixed length strings
      cannot be used in a FIELD statement". "Expected FIELD #filenumber, characters AS variable$, ..." for: a
      number, a literal, an expression, a function's result, a string `CONST`, a `TYPE` value, a numeric member,
      a new name with no `$`; `FIELD` alone, with the file only, with a comma after it and nothing more, no
      `AS`, no width, no target, a comma after the last part, `#` with no number, no comma after the file. Two
      parts with no comma are "Name already in use (AS)", a semicolon between parts "Invalid expression", a
      string as the file number or as a width "Illegal string-number conversion", `CALL FIELD(…)` "Expected CALL
      sub-name [(...)]".
    - **`FIELD` when it runs.** Each `FIELD` starts again at the buffer's first byte, and the strings of an
      earlier `FIELD` on the file stay fields: after `FIELD #1, 3 AS a, 5 AS b` and `FIELD #1, 8 AS w`, an `LSET`
      to `w` changes `a` and `b` and one to `b` changes `w`. `FIELD` gives the string the width as its length
      and the bytes the buffer holds (0 in a new file). A string named again leaves its earlier field. The
      widths of one statement may add up to the record length; the part that goes beyond is error 50, and that
      part and every part after it are left as they were (the parts before it are fields). A negative width is
      error 5, a width of 0 is accepted (the string is empty). A file not open, and file number 0, is error 52;
      a negative file number, a file open `FOR BINARY`, `OUTPUT` or `INPUT` error 54; an element out of range
      error 9. The default record length is 128 (`100 AS a, 28 AS b` is accepted, `29` is error 50).
    - **`GET` and `PUT` with no variable.** `GET #1[, record]` fills the buffer and every field string of the
      file; a record past the end reads as bytes of 0 and sets `EOF`. `PUT #1[, record]` writes the whole
      buffer, so the file ends at the record's end. Record 0 or below is error 63; a file not open 52; a file
      that is not random 54; `ACCESS READ` gives error 75 for `PUT`, `ACCESS WRITE` for `GET`. `GET` and `PUT`
      **with** a variable do not go through the buffer: `GET #1, 1, l` leaves the field strings as they are, and
      after `PUT #1, 1, x` they hold the old bytes until a `GET` with no variable.
    - **`=` to a field variable** changes the string and not the buffer, and the string stays a field: `a =
      "QQ"` gives `a` the length 2, and the next `LSET` to any field of the file or `GET` with no variable gives
      it its width and the buffer's bytes again. `LSET` to a field string whose length was changed by `=` works
      on that length first and then on the field (`b = "": LSET b = "k"` leaves the buffer's part of `b` as five
      blanks). After `CLOSE` a field string keeps its value and its length; `LSET` to it works as on any
      string; when the file number is opened again the string is no field of the new file.
    - **`LSET` and `RSET` on any string.** The target keeps its length: a shorter value is padded with blanks
      at the right (`LSET`) or the left (`RSET`); a longer one is cut at its end by **both** (`RSET a =
      "abcdef"` into three characters is `abc`); an empty value gives blanks; an empty target stays empty. The
      target may be a fixed-length string, an element, a member of either kind, also of an array element, and
      a name not seen before with `$` (an empty string, so nothing happens). The value may name the target
      (`LSET a = MID$(a, 3)`). An error in the value leaves the target as it was (`LSET a = CHR$(-1)`); an
      element out of range is error 9. A whole array `e()` as the target is accepted and is element 0 (`x153`).
    - **Rejected forms of `LSET` and `RSET`.** "LSET/RSET expects a string variable/array-element as its first
      argument": a numeric variable, element or member, **a `TYPE` value** (`LSET r = r2`, which QuickBASIC
      uses to copy a record, and `LSET r = "abcd"`), a literal, an expression, a `CONST`, a function's result,
      `MID$(a, 2, 2)`, a new name with no `$`. "Expected LSET/RSET stringvariable=string": no `=`, no value,
      `CALL LSET(a, "x")`. "Expected LSET ..." (or `RSET`) for the word alone. A number or a `TYPE` value as
      the value, and `LSET a = b = "x"` (the first `=` outside parentheses ends the target, the rest is the
      value), are "Illegal string-number conversion". `LSET = "a"` and `LSET (a) = "x"` are read as an
      assignment to a variable or array named `LSET`: "Name already in use (LSET)".
- **Civil War Strategy played from a keys file (measured 2026-10-14 for `m3-scripted-game`, task 1.1: a copy
  of `CWSTRAT.BAS` with the edits of the change's design D1, built by `qb64pe.exe` with `screen_state.cpp`,
  outside the repository).**
  - **What makes the game as it stands unscriptable.** `RANDOMIZE TIMER` (line 218); `_FULLSCREEN` (line 168);
    `SUB TICK` (68 calls) waits up to a time or until a key and eats that key; 14 places wait for a key (the
    menu's `crsr:` loop, twelve `DO WHILE INKEY$ = "": LOOP`, one `INPUT`); the title music and `SUB shen` only
    look for a key (`IF INKEY$ <> ""`) and run only with `noise` 2. `CWS.CFG` holds `side, graf, noise,
    difficult, player, turbo!, randbal, …, history, bold`; `history` 1 makes a new game `SHELL "copy cws.his
    oldhist.ory"` and write `DATE$` into `cws.his`.
  - **Byte 0x4F0 of conventional memory is free.** libqb's `DEF SEG` alone points at 0x500 (`&cmem[1280]`) and
    no line of libqb names `cmem[0x4F0]`; `DEF SEG = 0: POKE 1264, (PEEK(1264) + 1) AND 255: DEF SEG` before each
    wait counted every one of about 25 waits of a short game exactly, with no write from elsewhere.
  - **A menu announces once per key**: `SUB menu` returns to `crsr` after every arrow key, so each arrow is a
    wait of its own; the keys file has a `prompt` before every key.
  - **A short game** (new game as the Union, END TURN, the computer's turn with one battle, the May menu, Files,
    Quit, Yes, END's prompt): 1.1 to 1.4 s with `TICK` empty and no sound, four runs byte for byte equal. The
    computer's turn waits three times (the month's events, the battle, "hit any key to continue"). A new
    executable's first start once took more than 1.5 s to its first wait.
  - **Size of a state**: one `SCREEN 12` state is 95 kB (the title) to 121 kB (the map with the main menu), as
    written by `screen_state.cpp`.
  - **`$EXEICON`** of the copy needs `cws.ico` beside it when it is built; the corpus runner copies
    `<name>.files\` into the program's folder before it compiles.
- **The image, colour, font and mouse built-ins (`v31_*`, 113 programs, measured 2026-10-14 as task 1.1 of
  `m3-images-fonts-mouse`: 225 accepted forms read with `qb64pe -z` from programs outside the repository,
  kept as code that is not reached in `v31_x01_accepted`; 108 rejected forms, one program each, `x02`–`x109`;
  the values in `v31_a`–`v31_d`, each printed to the console of a program with a hidden window).**
  - **Every accepted form is the plain call of its table row and template.** The slots are the table's, a float
    into an integer slot is `qbr(…)`, an integer or a float into a SINGLE or DOUBLE slot is passed as it is, an
    `_UNSIGNED LONG` into a LONG slot as it is, a literal as written (`&HFF102030` into an `_UNSIGNED LONG` slot
    is `-15720400`). A left-out slot is `NULL`; a function's optional slots are counted in a last argument
    `0|1|2…` (`func__rgb(1,2,3,NULL,0)`, `func__loadimage(f,NULL,s,0|2)`), a statement's in a plain number
    (`sub__putimage(…,616)`). The words of a template are numbers in their slot: `_PRINTMODE` `_FILLBACKGROUND`
    1, `_KEEPBACKGROUND` 2, `_ONLYBACKGROUND` 3; `_DEPTHBUFFER` `ON` 1, `OFF` 2, `LOCK` 3, `_CLEAR` 4;
    `_MAPTRIANGLE` `_CLOCKWISE` 1, `_ANTICLOCKWISE` 2 in its first slot, `_SEAMLESS` a bit of the mask,
    `_SMOOTH` 1, `_SMOOTHSHRUNK` 2, `_SMOOTHSTRETCHED` 3 in its last; `_PUTIMAGE`'s `_SMOOTH` and `STEP` and
    `_CLEARCOLOR`'s `_NONE` are bits of the mask only. Every statement takes `CALL` with its template in
    parentheses (`CALL _PUTIMAGE((10, 10), img)`, `CALL _MAPUNICODE(9786 TO 1)`) and writes the same call.
  - **Result types, as the table says**: a LONG for the handles, sizes, `_RED` to `_ALPHA`, `_RED32` to
    `_ALPHA32`, `_CLEARCOLOR`, `_BLEND`, `_PALETTECOLOR`, `_AUTODISPLAY`, `_FONT`, `_FONTWIDTH`, `_FONTHEIGHT`,
    `_PRINTWIDTH`, `_PRINTMODE`, the `_U…` functions, `_MAPUNICODE`, `_MOUSEINPUT`, `_MOUSEBUTTON`,
    `_MOUSEHIDDEN`, `_MOUSEDISABLED`, `_SCREENIMAGE`, `_CLIPBOARDIMAGE`; an `_UNSIGNED LONG` for `_RGB`,
    `_RGBA`, `_HSB32`, `_HSBA32` (and `_RGB32`, `_RGBA32`); a DOUBLE for `_HUE32`, `_SATURATION32`,
    `_BRIGHTNESS32`, `_MOUSEX`, `_MOUSEY`, `_MOUSEWHEEL`, `_MOUSEMOVEMENTX`, `_MOUSEMOVEMENTY`; a SINGLE for
    `PMAP` (`HEX$` of it is `func_hex_float`).
  - **Forms the template allows that look odd**: `_CLEARCOLOR` alone (`sub__clearcolor(NULL,NULL,0)`) and
    `_CLEARCOLOR _NONE 4` (the word and a colour, mask 3) are accepted; `_PUTIMAGE` alone, `_PUTIMAGE , , , ,
    _SMOOTH` and every other choice of its four parts; `_MAPTRIANGLE` with no source handle (`(…) TO (…)`) and
    with a z on some of the destination's points only (`(10, 10, -1)-(50, 10)-(10, 50)`, mask 6); `_MOUSEWHEEL`
    bare and with an axis.
  - **`_UCHARPOS`'s second argument** (`qb64pe.bas` 20901–20918) must be a whole array or an element of an
    array whose type is a 32-bit integer that is not an offset: LONG and `_UNSIGNED LONG` are accepted, also
    with two dimensions or used without `DIM` (`nd&()`); any other type, a scalar, a literal and an array name
    with no parentheses are "Expected LONG array-name". It is passed as the bytes from the element (or from
    the first element of a whole array, its `LBOUND`, with no bounds check) to the array's end, as graphics `GET` passes its array:
    `byte_element((uint64)(&(((int32*)(__ARRAY_LONG_P[0]))[0])),((4*(__ARRAY_LONG_P[2]&1)*__ARRAY_LONG_P[5])-(4*(0))),byte_element_1)`.
  - **Rejected** (`x02`–`x109`): a function with too few or too many arguments, also when bare, is "Incorrect
    number of arguments - Reference: …" (`_COPYIMAGE`, `_LOADIMAGE`, `_RGB(1, 2)`, `_MOUSEBUTTON`,
    `_MOUSEX(1)`, `PMAP(1)`; also `_SCREENIMAGE(1, 2)`, `x81`: its template `[?,?,?,?]` is one group, so it
    takes no argument or four, the only function template of the table whose counts are not a plain range,
    found by task 5.1); empty parentheses "Expected (...)"; a string for a number "1st function argument
    requires a number" or "Number required for function" (a function with one slot), a number for a string
    "… requires a string"; a statement in a form its template does not have "Syntax error - Reference: …"
    (`_PUTIMAGE (1, 2)-`, `_PRINTMODE` alone or with a number, `_DEPTHBUFFER 1`, `_MAPUNICODE 1, 2`, `_FONT`
    alone, `_PRINTSTRING` with a `-` or no text, `_UPRINTSTRING STEP`, `_MAPTRIANGLE` with two points, with
    `STEP`, with a z in the source, with `_SEAMLESS` before `_CLOCKWISE`, `_CLIPBOARDIMAGE l` without `=`);
    `_SETALPHA 128 TO 9` "Name already in use (TO)"; `_AUTODISPLAY 1` "SUB does not require any arguments";
    a function as a statement (`_MOUSEINPUT`, `_WIDTH 5`) "Syntax error".
  - **Values (`v31_a`–`v31_d`).** In `SCREEN 13` `_RGB(255, 0, 0)` is 40 and `_RED(4)`, `_GREEN(4)`,
    `_BLUE(4)`, `_ALPHA(4)` are 170, 0, 0, 255. `_RGB32(5)` is `FF050505`, `_RGB32(5, 6)` `6050505`;
    `_HSB32(120, 50, 50)` is `FF408040`; `_RGB32(10, 200, 30)` has hue 126.3157894736842, saturation 95,
    brightness 78.43137254901961; a colour part out of range is clamped. `_WIDTH(_NEWIMAGE(64, 48, 32))` is
    64; a text image `_NEWIMAGE(10, 10, 0)` has `_PIXELSIZE` 0. `_LOADIMAGE` of a file that is not there is
    -1. `_COPYIMAGE(a&, 32)` of a 256-colour image is error 5 (only 33, a hardware copy, changes the mode).
    `_WIDTH` of a freed handle is error 258 ("Invalid handle"), as is freeing it again; **`_WIDTH(12345)`,
    a number that was never a handle, is no error and gives the screen's width** (640); `_FREEIMAGE 0` is
    error 5. `_CLEARCOLOR` of `SCREEN 12` is -1 (none); of a 32-bit image after `_CLEARCOLOR _RGB32(1, 2, 3)`
    it is 0. `_PALETTECOLOR 3, _RGB32(10, 20, 30)` then `_PALETTECOLOR(3)` is `FF0A141E` in `SCREEN 12` and in
    a 256-colour image. `_BLEND` is -1 for a 32-bit image, 0 for a palette one. After `WINDOW (0, 0)-(100,
    100)` in `SCREEN 12`, `PMAP(50, 0)` is 320, `PMAP(50, 1)` 240, `PMAP(320, 2)` 50.07825. `_AUTODISPLAY` is
    -1 before and after `_DISPLAY` (in a hidden window).
  - **Saved and loaded again (`v31_c`).** `_SAVEIMAGE` writes BMP, PNG, JPG, TGA, QOI and HDR by the name's
    extension; each loads again at its size with the pixels as drawn (JPG changes `FFFF0000` to `FFFE0000`). A
    name with no extension gets `.png`; `_SAVEIMAGE "x.dat", h, "bmp"` writes `x.dat.bmp`. A palette image
    saved and loaded with mode 256 keeps its indices. **`_SAVEIMAGE` with a handle that was never made writes
    a file and raises no error.**
  - **Found by the screen programs of group 2** (`tests\corpus\screen\i01`–`i08`, 2026-10-14). In 32 bits
    `_CLEARCOLOR c, h` makes the pixels of colour `c` transparent (alpha 0) and keeps no colour:
    `_CLEARCOLOR(h)` is 0 after it and after `_CLEARCOLOR _NONE, h`, and `_ALPHA32` of such a pixel of a copy is
    0 (`i05_alpha_blend`); in 256 colours it is the index (2). `_DONTBLEND` makes `_BLEND` 0, `_BLEND` -1 again.
    `_LOADIMAGE(f$, , "")` loads in the screen's mode (`i02`). **`PRINT` into a 32-bit image of 5 by 5 pixels,
    whose 16-pixel font does not fit, and then `_FREEIMAGE` of it ends the program with exit code 0xC0000374,
    a heap corruption** (found while `i01_new_copy_put` was written; libqb's, so the program uses an image of 40
    by 20).
  - **Fonts (`v31_d`).** In `SCREEN 12` the font is 16, 8 by 16 pixels; `_FONT 8` is 8 by 8, `_FONT 14` 8 by
    14; `_PRINTWIDTH("abc")` is 24. `_LOADFONT("cour.ttf", 16)` and `_LOADFONT("arial.ttf", 20)` load
    Windows' fonts by their bare names (handles above 31); a font that is not there gives 0, no error. A
    loaded font that is not loaded `MONOSPACE` has `_FONTWIDTH` 0; Courier New 16 is 16 high, `_PRINTWIDTH`
    of "abc" 30, `_UFONTHEIGHT` and `_ULINESPACING` 18; Arial 20 is 20 high, `_UFONTHEIGHT` 22. `_UCHARPOS("abc",
    p())` is 3 and gives the left edge of each character and the end (0, 10, 20, 30), also into an
    `_UNSIGNED LONG` array; with no array it gives the count only. `_FONT` and `_FREEFONT` of a freed handle
    are error 258, also `_FREEFONT 16` (a built-in font). `_PRINTMODE` is 3 at first; after
    `_KEEPBACKGROUND` 1, `_ONLYBACKGROUND` 2, `_FILLBACKGROUND` 3 (other numbers than the statement's).
    `_MAPUNICODE(1)` is 9786 before and after `_MAPUNICODE 9786 TO 1`, `_MAPUNICODE(65)` 65. In the screen
    programs of task 3.2 (`f01_builtin_fonts`, `f02_loaded_fonts`): `_PRINTSTRING STEP(x, y)` without an image
    counts from another image's last point, not the destination's (after `SCREEN 13: PSET (100, 0)` the text
    starts at (4, 4); `SOMEDAY.md`, "QB64pe behaviours to review"); `_FONTWIDTH` of a freed font is error 258 too.
  - **The build (task 1.2, six programs built with `qb64pe -x` outside the repository, read from the clone's
    `compilelog.txt`).** `_LOADIMAGE` and `_SAVEIMAGE` set `DEP_IMAGE_CODEC=y` (define `DEPENDENCY_IMAGE_CODEC`,
    variant `libqb_make_01000000`); `_SCREENIMAGE` and `_CLIPBOARDIMAGE` set `DEP_SCREENIMAGE=y` alone, and the
    clone's `Makefile` builds the image codec in too (`01000010`, both defines); `_LOADFONT` and the `_U…`
    functions set `DEP_FONT=y`, which has no define and links FreeType's objects in place of the font stub
    `font.o`; the variant stays `00000000`. All three with `$CONSOLE:ONLY` is `01100010`, `-mconsole`. The order
    of the flags is `qb64pe.bas` 13706–13721.
  - **The mouse of a window, fed through libqb's queue** (`Mouse_QueuePositionEvent` and the others, from a
    thread of `tools\screen\screen_state.cpp`, read by a program of the old compiler that prints every event):
    in `SCREEN 12` the window's pixel is the screen's (40, 30 gives `_MOUSEX` 40, `_MOUSEY` 30); in `SCREEN 0`
    a column and a row (40, 30 is 6, 2; 100, 50 is 13, 4). A move is two events in the queue (the move, then a
    copy with the movement 0), a button one, a wheel turn two (the steps, then 0). `_MOUSEBUTTON(1)` is -1 while
    the left button is down, `_MOUSEBUTTON(2)` for the right. A scroll of yOffset -1 (down, GLFW's sign) gives
    `_MOUSEWHEEL` 1; up is negative. **The user's real cursor over a shown window adds events** (30 before the
    first scripted one in that run), and the movement of the first scripted move is counted from the real
    cursor's last position; between two scripted moves it is their own difference. The queue is a
    single-producer ring buffer (`ring-buffer.h`), so the scripted thread is a second writer beside the window's
    callbacks: a real event at the moment of a step can lose or overwrite one. The position and scroll callbacks
    drop events while the window has no focus or the cursor is outside it (`mouse.cpp` 646–661).
    **`_MOUSEX` and `_MOUSEY` map an event when they are read, by the frame libqb last drew** (`mouse.cpp` 268–
    340: through `environment_2d__screen_x1`, the scale and the screen's size, set in `libqb.cpp` 25390 for each
    frame). A program that reads its events right after `SCREEN 12`, before the first frame, gets -1 for both
    (the screen's size is still 0, so the clamp gives width - 1); the movement and the buttons are right (found
    while task 4.3 recorded `u01_mouse_queue`; the program of task 1.2 printed to its console first and so read
    later). So a mouse step of the keys file waits until the window shows the displayed page
    (`tools\screen\README.md`). In `SCREEN 0` a pixel at or beyond the right or bottom edge (639, 399; 700, 450)
    is column 80, row 25, and 0, 0 is 1, 1 (`u02_mouse_text`). After the queue is empty the values of the last
    event stay (`u01`: 60, 40).
- **`_MEM`, its functions and statements, `_OFFSET`, `MKI$` … `_CV` (`v32_*`, 158 programs, measured 2026-10-14 as
  tasks 1.1 and 1.2 of `m3-mem`: about 230 accepted forms read with `qb64pe -z` from programs outside the
  repository, kept as code that is not reached in `v32_x01_accepted`; 115 other forms, one program each,
  `x02`–`x116`, all rejected but `x80`; the values in `v32_a`–`v32_c`; the runtime errors and the life of a block
  in `v32_d01`–`d34`; by the review of the change, `x117`–`x120`).** The C++ of every kind of place is in design
  D3 of the change.
  - **The type.** `_MEM` is user type 1, defined before the program's first line (`qb64pe.bas` 1588): 56 bytes,
    `OFFSET`, `SIZE`, two lock members a program cannot name, `TYPE`, `ELEMENTSIZE` (8 bytes each), `IMAGE` and
    `SOUND` (LONG). It is accepted wherever a user type is: `DIM`, `DIM SHARED`, `REDIM` arrays, `STATIC`, a
    member of a `TYPE` (`LEN` of `a AS INTEGER, mm AS _MEM, v(1 TO 2) AS LONG` is 66), a parameter `p AS _MEM`;
    `LEN(m)` is 56; `n = m` is a `memcpy` of 56 bytes and `SWAP m, n` `swap_block(…,56)`. A member is read as any
    member (`*(ptrszint*)(((char*)__UDT_M)+((0+8)))`, printed as an `_INTEGER64`; `IMAGE` and `SOUND` as `int32`).
    A store to a member is "Cannot set read-only element of _MEM TYPE", by `=` and also by `READ m.SIZE` and
    `INPUT #1, m.SIZE` (`x117`, `x118`; the old compiler's `setrefer`, `qb64pe.bas` 26850, which `FOR` calls
    too); `m.LOCK_ID` "Element not defined"; `m =
    5` and `m = k` "Expected = similar user defined type"; `PRINT m` "User defined types in expressions are
    invalid".
  - **A function that gives a block** (`_MEM`, `_MEMELEMENT`, `_MEMNEW`, `_MEMIMAGE`, `_MEMSOUND`) is accepted
    only as the value stored into a place of type `_MEM` (a variable, an element, a member): `mem_block funcN;` in
    `global.txt` and `memcpy(dest,(((char*)&(funcN=call))+(0)),56)`. As an argument (of `_MEMEXISTS`,
    `_MEMFREE`, a SUB or FUNCTION) it is "TYPE '_MEM' required for function" (or "… for sub"); with a member
    read (`_MEM(k).SIZE`, `_MEMNEW(4).SIZE`) "Invalid expression". **Stored into a number (`k = _MEMNEW(4)`,
    `x80`) it is accepted and the C++ build fails** ("assigning to 'int32_t' from incompatible type
    'mem_block'").
  - **A block as an argument** (of `_MEMEXISTS`, `_MEMFREE`, both blocks of `_MEMCOPY`, the first argument of
    the `_MEMGET` function) is a place of type `_MEM`, passed as its address:
    `(void*)( ((char*)(__UDT_M)) + ((0+0)) )`, of an element `(void*)( ((char*)(__ARRAY_UDT_RM[0])) +
    (((array_check(( 1 )-__ARRAY_UDT_RM[4],__ARRAY_UDT_RM[5]))*56+0)) )`, of a member `(void*)( ((char*)(__UDT_U))
    + ((0+2)) )`; the statements `_MEMGET`, `_MEMPUT` and `_MEMFILL` write it as `(((char*)__UDT_M)+((0+0)))`.
    Anything else is "TYPE '_MEM' required for function", "1st function argument requires TYPE '_MEM'", "4th sub
    argument requires TYPE '_MEM'" or "Expected _MEM type". `CALL _MEMFREE(m)` and `CALL _MEMCOPY(m, 0, 8 TO n, 0)`
    are the statements; `CALL _MEMPUT(…)`, `CALL _MEMGET(…)` and `CALL _MEMFILL(…)` are "Command not implemented".
  - **`_MEM(reference)`** is `func__mem(address, size, type value, element size, lock)` (the "-7 helper",
    `qb64pe.bas` 22730–23038). The type value (`Type2MemTypeValue`, 14731): the bytes of a number (1, 2, 4, 8,
    32 for `_FLOAT`) + 128 an integer, + 256 a float, + 1024 unsigned, + 8192 an offset; 512 a fixed-length string
    (its length not added); 4096 a `_MEM`, 32768 any other user type and any member (`u.a` of an INTEGER is
    32768); + 65536 when the place's variable is an array. The lock: **the `sf_mem_lock` of the SUB, FUNCTION or
    main module where `_MEM` stands**, not of the one the variable lives in, for anything but an array variable
    or its element, which take the lock in slot `4 * dimensions + 4` of the descriptor (`d34`: a block of a
    main-module or `STATIC` variable taken in a SUB stops being valid when the SUB returns). The size and element
    size by kind (`v32_a`): a variable its bytes; a whole array the bytes of all elements (0 while not
    allocated), an element the bytes from it to the end; a whole `TYPE` value, a member, a member of an element
    and the element of a member array of a variable: the bytes from the place to the end of the variable (or of
    the whole array) with element size 1 for a member that is no user type and the type's size for a whole value
    or a member of a user type (a member of type `_MEM` is typed 4096, measured while group 3 was built); a whole member
    array (`u.v()`, `ut(1).v()`) its own bytes, typed as an array of its elements, with the `sf_mem_lock`; an
    element of a member array of an array element (`ut(1).v(2)`): the whole member array's bytes from that
    element's address, typed 98304, element size the elements'. Errors: a variable-length string "_MEM cannot
    reference variable-length strings" (also an array of them), a value "Expected variable name/array element",
    a `_BIT` "Variable/element cannot be _BIT aligned", `_MEM()` "Expected (...)", three arguments "Incorrect
    number of arguments".
  - **An array parameter whose number of dimensions nothing above has given** (`_MEM(p())` before any index of
    `p` and with no call passing an array above the procedure; `v32_a`, SUB `y`) takes slot 0 of the descriptor
    as its lock (`4 * -1 + 4`): the pointer to the elements, so **the first 8 bytes of the elements are read as
    the lock's id** and the block stops being valid when element 0 changes ("after q(0) = 6: 0"). Once an index
    or such a call stands above, the slot is 8.
  - **`_MEMELEMENT(reference)`** (the "-8 helper", 22406) is the one element or value: of an element its own
    bytes; of a whole array the same as `_MEM` (of a whole array of a user type with the offset of an element,
    `((0)*18+0)`, where `_MEM` writes 0); of a member the member's own type (`u.a`: 2, 130, 2; `u.d`: 8, 264, 8);
    of any whole value of a user type, a `_MEM` too, the type's size typed 32768; but of an element of a member
    array the whole member array's bytes and type value of the element (`u.v(1)`: 16, 132, 16).
    `_MEMELEMENT(u.v())` is "Expected array index", `_MEMELEMENT(s$)` "_MEMELEMENT cannot reference
    variable-length strings". Of a member of an element of a user-type array, the member alone in its own type
    (`ut(1).d`: 8, 264, 8; `ut(1).s`: 5, 512, 5); of an element of a member array of such an element the same
    block as `_MEM` (`ut(1).v(2)`: 16, 98304, 4; `x119`). `_MEMGET(m, o, _UNSIGNED STRING * 4)` is "Invalid TYPE
    name" (`x120`).
  - **`_MEM(offset, size)`** is `func__mem_at_offset(offset, size)`: the offset must be an `_OFFSET` or
    `_UNSIGNED _OFFSET` ("Expected _MEM(_OFFSET-value,...)" for any other type), the size any number (a float
    through `qbr`; a string "2nd function argument requires a number"). Type 16384, element size 1, `IMAGE` -1;
    a negative size raises error 301 and gives a block of 0 bytes.
  - **`_MEMNEW(n)`** (`func__memnew( 64 )`; a float through `qbr`): type 16384, element size 1, `IMAGE` -1.
    `_MEMNEW(0)` has `OFFSET` 1 and `SIZE` 0; `_MEMNEW(-1)` raises error 5, which is trapped, and gives a block
    that exists, of `SIZE` 0 and `OFFSET` 0. **`_MEMFREE` of a block of `_MEMNEW(0)` taken after another block
    was freed ends the program with exit code 127** (`d33`): libqb sets no lock type for a block of 0 bytes, so
    a lock taken again from the freed list keeps type 1 and its old pointer, which is freed a second time.
  - **`_MEMIMAGE[(handle)]`** is `func__memimage(NULL,0)` (the destination) or `func__memimage(h,0|1)`: of a
    32-bit image of 4 by 2, `SIZE` 32, `TYPE` 3204 (4 + 128 + 1024 + 2048), `ELEMENTSIZE` 4, `IMAGE` the handle;
    of a 256-colour image of 3 by 3, 9, 3201, 1. After `_MEMFREE` of it, or `_FREEIMAGE` of the image, it is not
    valid (`_MEMEXISTS` 0, a read error 308). **`_MEMSOUND(h[, channel])`** is `func__memsound(h,NULL,0)` or
    `(h, c,0|1)`, with libqb's audio part.
  - **The `_MEMGET` function** is `*(int32*)(func__memget((mem_block*)(((char*)__UDT_M)+((0+0))),offset,4))` by
    the type word: any numeric type (`_OFFSET` through `ptrszint`, `_FLOAT` `long double` of 32 bytes), `STRING *
    n` (`qbs_new_txt_len((char*)func__memget(…,n),n)`), a user type (`((void*)+func__memget(…))`, copied by a
    `memcpy` of the type's size: `u = _MEMGET(m, o, t)`). The offset is converted to `_OFFSET`. Errors:
    `STRING` "Expected STRING * ...", `_BIT` and `_BIT * 4` "_BIT TYPE unsupported", a number or an unknown word
    "Invalid TYPE name", a user type printed "Number required for function", `STRING * 4` into a number "Illegal
    string-number conversion".
  - **`_MEMGET`, `_MEMPUT` and `_MEMFILL` as statements are written inline** (10168–10500): `tmp_long=offset;`,
    then `if ( ((mem_block*)(blk))->lock_offset ){ if ( tmp_long < …->offset || (tmp_long+(size)) > ( …->offset
    + …->size) || ((mem_lock*)…->lock_offset)->id != …->lock_id ){ if (…id != …lock_id) error(308); else
    error(300); }else{ copy } }else error(309);`. The copy of a variable is by its size as the "-5" helper
    spells it: 1, 2, 4 or 8 as an `int8` … `int64` (a DOUBLE as `int64`, a `STRING * 4` as `int32`), anything
    else `memmove`. A variable is a number, a fixed-length or variable-length string (its characters and its
    present length, `__STRING_S->len`), a value of a user type, an element, a member, a whole array `a()` or a
    whole member array; `_MEMPUT` also takes a string expression, held in `g_tmp_str`. `_MEMPUT … AS type` stores
    the value converted to the type (`*(uint8*)tmp_long=*__LONG_K;`: a LONG of 1000 `AS _BYTE` is -24, 2.7 `AS
    LONG` is 3). `_MEMFILL` is a call after all: `sub__memfill_2(blk,offset,bytes,value)` (`_1`, `_2`, `_4`, `_8`,
    `_SINGLE`, `_DOUBLE`, `_FLOAT`, `_OFFSET`) or `sub__memfill(blk,offset,bytes,(ptrszint)address,size)`, a
    string expression through `tmp_long`. Errors: an expression that is not a string "Expected variable
    name/array element"; `AS` a string, `STRING * n`, a user type or `_BIT` "_MEMPUT requires numeric type" (and
    `_MEMFILL`'s); an unknown word "Invalid type"; a string `AS LONG` "Illegal string-number conversion"; a
    wrong count "Expected _MEMPUT mem-reference, offset, variable|value[AS type]"; `_MEMGET … AS` "Name already in
    use (AS)"; a `_BIT` variable "Variable/element cannot be _BIT aligned"; an array of variable-length strings
    "Cannot pass array of variable-length strings".
  - **`_MEMCOPY a, aoffset, bytes TO b, boffset`** is `sub__memcopy(a,aoffset,bytes,b,boffset)`; without `TO`
    or with four parts "Syntax error - Reference: …".
  - **`_OFFSET(reference)`** is `((uptrszint)(address))`, an `_UNSIGNED _OFFSET`; the address as `_MEM` takes it
    (a whole array its first element, a fixed-length string or variable-length string its characters). A value
    is "_OFFSET expects the name of a variable/array", a `_BIT` array "_OFFSET cannot reference _BIT type arrays",
    a `_BIT` variable "Variable/element cannot be _BIT aligned".
  - **The runtime errors** (`v32_d01`–`d32`): each of 300 to 313 is reached, each from `ON ERROR GOTO`, and none
    runs the handler: the program stops with `Runtime error: Line: N (in main module)` and libqb's message, exit
    code 0 (`fix_error`, `libqb\src\error_handle.cpp` 368). 300 a region out of range (also before the block's
    offset), 301 a negative size of `_MEM(o, n)`, `_MEMFILL` or `_MEMCOPY`, 302–304 `_MEMCOPY` beyond the source,
    the destination or both, 305, 306 and 313 a freed source, destination or both, 307 `_MEMFREE` twice, 308 a
    freed block (also after its SUB returned, its array was redimensioned, its image freed), 309 a block never
    set, 310–312 `_MEMCOPY` with one or both never set.
  - **When a block stops being valid** (`v32_b`, `d27`–`d29`, `d34`): freed; the procedure where `_MEM` stood
    returns; its array is redimensioned (`REDIM`, `REDIM _PRESERVE`); its image freed. **Not after `ERASE`**: of a
    static array (the elements are cleared, the block still holds them) and **of a dynamic array, whose block
    stays valid and whose released memory `_MEMPUT` then writes with no error** (`d29`; libqb renews the lock
    only in `REDIM`).
  - **`MKI$` … `_CV`** (`qb64pe.bas` 21217–21296; `v32_c`): `MKI$(x)` is `i2string(x)` with `x` converted as a
    store into an INTEGER (`qbr_float_to_long( 1.7E+0 )`: 1.5 and 2.5 give 2), `MKL$` into a LONG (`qbr`;
    3000000000 gives `005ED0B2`), `MKS$` and `MKD$` as they are (`s2string(*__LONG_K)`); `CVI(s)` is
    `string2i(s)`, and so `CVL`, `CVS`, `CVD`. `_MK$(type, x)` and `_CV(type, s)` take every numeric type word
    (`f2string` of 32 bytes for `_FLOAT`, `o2string`, `uo2string`), `_BIT` and `_UNSIGNED _BIT`
    (`bit2string(1,x)`, `string2bit(s,1)`, an `_INTEGER64`); **a `_BIT` width cannot be written**: `_BIT * 4` is
    "Cannot convert type (_BIT * 4%) to symbol" (the parser hands the type over with a suffix on its number). A
    string shorter than the type raises error 5 (also `_CV(_BIT, "")`); a longer one gives its first bytes
    (`CVI("ABC")` 16961). Errors: `CVL(5)` "CVL requires a STRING argument", `_CV(LONG, 5)` "_CV requires a STRING
    argument", `MKL$("a")` "Number required for function", `_MK$(LONG, "a")` "2nd function argument requires a
    number", `_MK$(STRING, k)` "_MK only accepts numeric types", `_CV(STRING, s)` "_CV cannot return STRING
    type!", `_MK$(STRING * 4, k)`, `_MK$(FOO, 1)` and `_MK$(k, 1)` "Cannot convert type (…) to symbol", a wrong
    count "Incorrect number of arguments - Reference: …".
- **`$ASSERTS`, `_ASSERT`, `$DEBUG`, the checksum, compression and base64 functions, `END n` and `SYSTEM n`
  (`v33_*`, 99 programs, measured 2026-10-14 as task 1.1 and 1.2 of `m3-asserts-debug-zlib`; every accepted form
  kept as code that is not reached in `v33_x01_accepted`, 54 other forms one program each, `x02`–`x55`).**
  - **The metacommands' lines** (`x02`–`x16`, `a11`–`a13`, `d08`, `d09`; `qb64pe.bas` 1978–2008, 3553): the
    rules of `$CONSOLE` (above): the whole line in upper case without the blanks at its ends is compared, so
    `$asserts`, `$asserts:console`, `$debug` and blanks after them are fine, and a tab or a word after them,
    `$ASSERTS:CONSOLEX`, `$ASSERTS:`, `$ASSERTS:ONLY` and `$ASSERTS : CONSOLE` are "Syntax error";
    `x = 1: $ASSERTS` is "Unexpected character on line". Each counts for the whole program wherever it stands:
    on the last line (`a05`: an `_ASSERT` on line 4 fails), in an included file (`a11`), in a SUB never called
    (`a13`, `d08`), twice or mixed with `$ASSERTS:CONSOLE` and `$CONSOLE:ONLY` (`x14`, `x15`); in a `$IF`
    branch not taken it does not count (`a12`, `d09`).
  - **`_ASSERT condition[, message$]`** is the row `sub__assert` (`?[,?]`; the condition an INTEGER slot, a
    float through `qbr_float_to_long`): `sub__assert(*__LONG_K,NULL,0)`,
    `sub__assert(-(*__LONG_K== 1 ),qbs_new_txt_len("m",1),1)`, and `CALL _ASSERT(k)` the same (`a14`). Without
    `$ASSERTS` the global `asserts` is 0 and the call returns at once, **after the condition and the message are
    evaluated** (`a03`: both functions print); with it `int32 asserts=1;` (global.txt). The condition is passed
    as a 16-bit value: `1E+10` passes and `4294967296` fails (`a08`). Errors: no argument, `_ASSERT 1,`,
    `_ASSERT , "a"`, three arguments, `CALL _ASSERT` "Syntax error - Reference: _ASSERT condition[,
    errorMessage$]"; a string condition "1st sub argument requires a number"; a number as the message "2nd sub
    argument requires a string"; a `TYPE` value "User defined types in expressions are invalid"; `k =
    _ASSERT(1)` "Invalid array name"; `_ASSERT$ 1` "Syntax error" (`x17`–`x24`, `x47`, `x50`).
  - **A failing assertion** raises error 314 ("_ASSERT failed"), or, in a program with a console (`console=1`:
    `$CONSOLE`, `$CONSOLE:ONLY` or `$ASSERTS:CONSOLE`) and with a message, writes the message to the console as
    `_ECHO` does and raises 315 ("_ASSERT failed (check console for description)") (`a02`, `a06`, `a14`); with a
    console and no message 314 (`a07`); in a program with a window only 314 (`a09`, nothing written). **Neither
    is trapped by `ON ERROR`**, like 300–313 (`a01`): the handler does not run, the program stops with
    `Runtime error: Line: N (in main module)` and the message, exit code 0. In a SUB the line named is the
    `_ASSERT`'s (`a10`).
  - **`$DEBUG` outside the IDE** (`d01`–`d11`; `vwatch.bm` 684): a program prints what it prints without
    `$DEBUG` (`ERR`, `ERL`, `_ERRORLINE`, an error in a SUB, an untrapped error's message, start-up time,
    `OPTION BASE 1` and `OPTION _EXPLICIT` after it, a variable named `vwatch_linenumber`, a window program). **Two
    things differ**: `_SOCKETS_` is 1 (`d01`: the included `vwatch.bm` uses `_OPENCLIENT`, which sets the sockets
    dependency, and the program is compiled again with the flag; its build links `DEP_SOCKETS=y DEP_HTTP=y`;
    also above a `$DEBUG` on the last line, `d11`),
    and the names of `vwatch.bi` and `vwatch.bm` are taken: a SUB named `vwatch_stoptimers` is "Name already in
    use (vwatch_stoptimers) in line 22 of internal\support\vwatch\vwatch.bm auto-included" (`x49`; the globals
    `vwatch_linenumber` and the like are `DIM SHARED` there, so a variable of that name is that global). With
    `-w` the old compiler prints `<file>:<line>: warning: $DEBUG features only work from the IDE` once per round
    of compiling, so twice for a program whose first round changed a flag (read 2026-10-14 by hand).
  - **The six functions** (`subs_functions.bas` 4019–4100; `b01`–`b03`, `x25`–`x36`, `x45`, `x46`, `x53`): each
    a plain row with the dependency `DEPENDENCY_ZLIB`, which the build turns into `DEP_ZLIB=y` (also for a call in
    a SUB never called, `b03`). `_ADLER32` and `_CRC32` are `_UNSIGNED LONG` (`_CRC32("abc")` 891568578,
    `_ADLER32("abc")` 38600999, of `""` 0 and 1); `_DEFLATE$(s[, level])` (a LONG slot, a float through `qbr`:
    `func__deflate(__STRING_S,qbr(*__DOUBLE_D),0|1)`) writes a zlib stream (`78 9C …`, 8 bytes for `""`);
    every level is accepted, -2 to 12 alike but 0, which stores (1011 bytes for 1000 `a`s, 24 otherwise);
    `_INFLATE$(s[, size])` (an `_INTEGER64` slot): with a size the result has that length (fewer bytes cut; with
    more, the bytes past the data are not set: libqb's `qbs_new`, `parts\data\compression.cpp` 59), 0 or less
    gives `""`; **bytes that are not a zlib stream give no error but a string of zero bytes, 10485760 of them
    without a size** (`b02`; with a size, that many bytes not set). `_BASE64ENCODE$` pads with `=`;
    `_BASE64DECODE$` of anything that is not whole groups of four valid characters (`YQ`, `YWJ`, `!!!!`, `Y Q = =`)
    is `""`. Errors: without `$` (and `_CRC32$`) "Invalid array name"; a number for a string "String required for
    function" (`_ADLER32`) or "1st function argument requires a string" (`_INFLATE$`); a string for the level or
    the size "2nd function argument requires a number"; a wrong count "Incorrect number of arguments - Reference:
    …"; `_DEFLATE$()` "Expected (...)".
  - **`END n` and `SYSTEM n`** (`qb64pe.bas` 9850–9895; `c01`–`c25`, `x37`–`x44`, `x48`, `x51`, `x52`): `if(qbevent){evnt(N);}`,
    then `exit_code=<n>;` (libqb's `int64 exit_code`), then the usual end (`sub_end();`, or for `SYSTEM` `if
    (sub_gl_called) error(271);`, `close_program=1;`, `end();`). `n` is converted by `evaluatetotyp(…,
    ISINTEGER64)`, whose width is read as 0: an integer as it is, **a float through `qbr_float_to_long`** (2.7 gives
    3, 2.5 and a `_FLOAT` 1.5 give 2, `2 ^ 40` 0); a string "Illegal string-number conversion", a `TYPE` value "User
    defined types in expressions are invalid", `SYSTEM 1, 2` "Invalid expression", `SYSTEM ()` "Unexpected (";
    an `_OFFSET` is accepted; both in a SUB, a FUNCTION and a one-line `IF` too. The process's exit code is the
    value's low 32 bits: `END 7` 7, `END -1` 0xFFFFFFFF, `END 300` 300 (measured by hand through
    `press_any_key.py`, since `END` waits for a key); as Git Bash reports them (`exit=`), `SYSTEM 3` 3, `-1` 127,
    256 and `2 ^ 40` 0, 258 2, an `_UNSIGNED _INTEGER64` of 18446744073709551615 127. With a window `SYSTEM 5`
    gives 5 (`c21`). `SYSTEM f` where `f` raises an error under a handler (`c23`): the handler runs and the exit
    code is `f`'s value; not trapped (`c25`), the runtime error at the line of `ERROR`, exit code 0.
  - **The test runners and exit codes**: the corpus runner records no exit code and compares none; the upstream
    suite fails a program whose exit code is not 0; the verification suite writes `exit=` as Git Bash reports it.

## 6. Bug-compatibility choices (all decided)

From the end of `09` and `10` §2.8, §3.2 and the measurements in §5. **All decided by the user on 2026-10-08**
(step 8 of `STATUS.md` "Next", `DECISIONS.md`), except where an earlier date is given. The rule applied: a
deterministic behaviour that changes results is **kept** (QB64pe programs rely on it, and the differential tester can
then expect equal output); a crash, memory corruption or failed compile is **fixed**. A difference from QB64pe is a
row in `DIVERGENCES.md`; a kept behaviour that differs from QuickBASIC 4.5 is a row in `DIVERGENCES-QB45.md`.

**Kept, as QB64pe:**
- Numbers: LONG and `_INTEGER64` overflow **wraps** (2026-10-03, D-001, D-002; also in the old compiler's default
  build, not its `-O2` build: `16` §8, `verification\v11_wrap_o2`); INTEGER arithmetic computed in 32 bits inside
  expressions (`i% + 1` gives 32768; Q-004); rounding half to even, with a DOUBLE narrowed to SINGLE before it is
  rounded into an INTEGER target (`d# = 2.5000001: x% = d#` gives 2, `l& = d#` gives 3); integer division by zero
  (`\ 0`, `MOD 0`) **fatal**, error 11 not trappable (Q-003).
- `CONST`: `^` right-associative (`CONST c = 2 ^ 3 ^ 2` is 512, 64 at run time; Q-008) and an integer-valued float
  typed `_INTEGER64`. **With a new compiler warning** for a `CONST` expression that chains `^` without parentheses
  (no effect on output).
- Control flow (`m2-control-flow-slice`): `RESUME NEXT` after an error in a `WHILE` condition loops forever; an
  error in an `ELSEIF` condition tests the placeholder value and is handled at the next statement; one `GOSUB` stack
  for the whole program (a SUB's `RETURN` consumes main's entry); a SUB called with a raising argument does
  nothing; a raising `CASE` item runs its body (as `IF`). `ON n GOTO` with n > 255 continues with the next statement
  and the static `SELECT CASE` copy (2026-10-07, Q-001, Q-002).
- Arrays (`m2-arrays-and-types`): a read with a bad index gives element 0's value; the value of a member-of-element
  store is evaluated before its index (so `ERR` reports the value's error); `LBOUND`/`UBOUND` typed `_INTEGER64`.
  Except: a member store into an element with a bad index **is skipped** (2026-10-07, D-004).
- Built-ins (`m2-core-builtins`; "for now" on 2026-10-08, final the same day): `HEX$` of a 64-bit expression that
  is not a place prints `""` for -1; `STRING$(n, "")` reads the first byte of the empty string (Q-007); `CSNG` of an
  integer and `VAL(s$, <integer type>)` not narrowed; a DOUBLE `ON n` narrowed to SINGLE before rounding (beyond
  LONG it raises 5); `_ROUND` and `VAL(…, _INTEGER64)` beyond the `_INTEGER64` range give its smallest value with no
  error.
- Not implemented yet, decided now: fixed-length strings start as NUL bytes (Q-005); `REDIM _PRESERVE` of several
  dimensions keeps each element's flat (column-major) position; `INPUT` prompts are string literals only (an
  expression prompt as an extension is in `SOMEDAY.md`).
- **Console comma zones exactly as QB64pe** (Q-006): a comma pads to a multiple of 10 columns on the Windows console
  and new-lines past `width - 10` (`libqb.cpp` `tab()`, measured); on the Linux and macOS console it prints **one
  space** (read, not run: console `PRINT` writes to `std::cout` without moving the page's cursor, so the text-screen
  code sees column 1). Window, graphics screens and files keep their 14-column (or 112-pixel) zones. The runtime
  tracks the console column itself instead of asking Windows, so a redirected program no longer hangs (D-011).
  Making it 14 everywhere is to be evaluated (`SOMEDAY.md`).

**Fixed (no compatibility value):** `RETURN label` with no `GOSUB` pending breaking the `GOSUB` stack (2026-10-06,
D-003); `a IMP b IMP c` computing `a OR b OR c` (D-005); the smallest LONG/`_INTEGER64` `\ -1` and `MOD -1`
crashing the program (D-006); `CONST 1 / 0` giving 0 and `CONST 2 ^ 70` wrapping while `1E+19 / 1` does not
(D-007); `label: CONST …` on one line failing to compile (D-008); `_BIT * n` with n > 32 overwriting 4 bytes
of the `_BIT` scalar allocated before it (D-009; the victim measured by `verification\v21_b_bit_overlap`); `INF` printed with padding and a stray `D` (D-010); the console `tab()` hang and the
`CONOUT$` handle leak (D-011); runtime errors exiting 0 (D-012); `_LogMinLevel` and `_ScreenExists` registered
without a return type (D-013); two calls of a FUNCTION `f$n` in one expression reading memory the FUNCTION had
released (2026-10-09, D-014). Rejected by both compilers, so no register row, only a proper message: `ELSE` while an
inner `FOR` is open (the old one fails in C++), `CONST … \ 0` and `MOD 0` (the old compiler crashes) and
`CONST (-8) ^ (1 / 3)` (an internal compiler error).

Found by the `m2-numeric-types` measurements and decided the same day (`DECISIONS.md`): out-of-range suffixed
literals and `CONST`s, bit-suffixed literals and constants, `STRING * n` parameters and `f$n` FUNCTIONs behave as in
QB64pe (first "not supported yet", then brought in by the user's "do what QB64pe does" rule; listed for review in
`SOMEDAY.md`); a `_BIT` parameter, a literal beyond 64 bits and a `STRING * n` whose n becomes 0 or negative in 32
bits are compile errors, as QB64pe fails to build them, and a signed radix literal wider than its type is one as in
QB64pe ("Overflow"); `STRING * n` wraps n to 32 bits,
a `_BIT` value reads as `_INTEGER64` and a FUNCTION named with a bare `` ` `` cannot be called, as in QB64pe.
Improvements in `SOMEDAY.md`.

Which test pins each choice is the "Pinned by" column of its register row. After `m2-numeric-types` (2026-10-09) every
row is pinned by a slice program, a snapshot or a CLI test except D-010 to D-012 and Q-006 (they wait for the runtime
copy, M3), D-013 (it waits for its two built-ins) and Q-007 (measured on the old compiler only).

The full catalogue of about 45 accidental behaviours: `01` §11.3, `02` §9.2, `04` G.4; only the ones above have been
run. A newly measured oddity follows the same rule, without asking the user: a crash, hang, memory corruption or
failed build is fixed (a `DIVERGENCES.md` row); anything else is implemented as QB64pe does it and, if it looks
questionable, listed in `SOMEDAY.md` "QB64pe behaviours to review" (user, 2026-10-08, `DECISIONS.md`).

## 7. What can be dropped (implementation, not behaviour)

- Whole-program restarts and `RCStateVar` → a real symbol table and a semantic pass before code generation.
- String-of-elements representation, the scratch `id` + `findanotherid` protocol, the 64 MB hash table.
- Hidden parameters passed through globals (`dimshared`, `dimstatic`, `reginternalsubfunc`, …).
- Duplicated logic: two include managers; TYPE/CONST/SUB-header parsing in both passes; argument passing written
  twice; two interpreters for the optional-argument format.
- Text-fragment stitching through `qbx.cpp`, buffer handles swapped under the emitters, `retN.txt` pasted at every
  RETURN (but M3 must still produce what `qbx.cpp` includes).
- "Did the exe appear?" as the only build result.
- Fixed limits (100 parameters, 1000 control levels, 4095 UDTs, …).
- All IDE coupling: the `ide(0)` coroutine, shared globals, the vWATCH wire protocol (known defects; `06`).

## 8. Runtime (libqb)

From `03` and `07` Session 7. Leave alone: vendored libraries, `libqb\src` modules, the bit-exact units
(`rounding.h`, `qbs_str.cpp`, `qbs_val.cpp`, PRINT USING, RND, number parsers). Fix early, low risk: `#define int32`
macros leaking everywhere; the 892 `static` locals that make functions non-reentrant (`SUB _GL` and event handlers
re-enter). Redesign only in M6: the error model (`error()` returns, every function early-outs, a trapped error calls
`QBMAIN` recursively and never unwinds), the moving string heap, flag-based thread synchronisation. The IR models
"statement may raise; resume point here" explicitly so the error model can change without touching the front end.

## 9. Built-in table

`tools\builtins\extract_builtins.py` → `tools\builtins\builtins.json` (`10` §3; rerun when the reference clone moves):

- 455 registrations, 404 distinct names, 289 functions and 166 subs; 172 use the `specialformat` mini-language,
  28 are stubs. Argument and return types decoded for every entry.
- 86 names are also handled by hand in `qb64pe.bas` (type-generic math, variadic `_MIN`/`_MAX`/`_CLAMP`/`_IIF`,
  `_MEM*`, `VARPTR`, `LBOUND`/`UBOUND`, statements with their own parsers such as PRINT, INPUT, OPEN, GET/PUT,
  LINE): these need code, not just a table row.
- **Gap:** names defined in the auto-included BASIC files (`_TRUE`, `_FALSE`, color constants) are not in the
  table yet (`10` §3.4).
- **Used by `sema`** since `m2-core-builtins` (2026-10-08): the supported built-ins are rows `(name, Rule)` in
  `crates\sema\src\builtins.rs`; the table gives slots, optional masks, required suffixes and plain return types,
  and a rule overrides it where the old compiler special-cases a function. **Facts of the table found wrong by
  measurement** (§5, `verification\v20_*`; the table is not edited, the rule says so): `CLNG` returns LONG (table:
  INTEGER); `ASC` takes two arguments (table: one slot); `LBOUND`/`UBOUND` are `_INTEGER64` (table: LONG); `ERR` is
  LONG (table: `_UNSIGNED LONG`); `ABS`, `INT`, `FIX`, `EXP` and the float functions are typed by their argument
  (table: `_FLOAT` or any-numeric); `VAL` returns a number (table: STRING) and takes a type name as its second
  argument; `_FLOAT` slots pass the argument in its own type, not converted.

## 10. Testing position

- **Runner:** `tools\legacy_tests\run_legacy_tests.py` re-implements the bash suites on Windows (compile,
  qbasic, format) for the old or the new compiler; `known_failures.txt` marks expected failures.
- **Baseline** (`baselines\`, old compiler at `16f629784e`): compile_tests 330 of 331 expected-output, 56 of 56
  expected-error, 17 of 17 compile-only; qbasic_testcases 143 of 143 (compile only); format_tests 24 of 24. The one
  failure (`http/read_example`) depends on a web page that changed.
- **Reusable as black-box conformance:** the 331 expected-output tests (357 of 404 programs are `$CONSOLE:ONLY`)
  and the 143 qbasic programs. Tied to the old implementation: the `.err` texts, `qb64pe/*` internals tests,
  formatter tests, licence tests.
- **Planned layers (R11):** (1) existing suite; (2) golden corpus: run the 143 qbasic programs and QB64Fresh's 261
  `runtime_comparison` programs through the old compiler and freeze their output (**the 261 done 2026-10-03**:
  `tests\corpus\`, 263 programs with `verification\` v11 and v12; 237 `.output`, 20 `.err`, 1 compile-only, 5
  known failures; with the 12 `slice\` programs written for the new compiler, 275; baseline `baselines\qb64pe-16f629784e-win64-corpus.json`; how often it runs: `study\19`. The 143
  qbasic programs are still compile-only); (3) differential testing of
  random expressions old vs. new; (4) formatter goldens from `-y` over all available `.bas`; plus golden images for
  LINE/CIRCLE/PAINT/DRAW/GET/PUT (only 12 image tests exist).
- **Not covered by anything yet:** the IDE, real graphics output, audio, interactive input, run-time behaviour of
  the 143 qbasic programs, and classic QBasic areas (PRINT USING, file modes, string functions) beyond light use.
- **Test hygiene:** run every program with `QB64PE_NOPROMPT=y`; detect fatal errors from output, not exit code;
  never use screen `PRINT` with a comma under a redirected `$CONSOLE`.
- `verification\` holds 77 small programs with recorded outputs behind `09`, `10`, `16`, the slice's design
  (`v13`, `v13b`: type suffixes on DIMmed names) and `m2-procedures-and-errors` (`v14_*`: scopes, reserved names,
  compile errors, `ERROR` values; `v15_*`: names of built-ins, unary `+`) (`run.sh` reruns them).
- **New compiler (2026-10-03, `crates\README.md`):** tier 1 `cargo test` runs unit and snapshot tests (`insta`),
  `tests\frontend\` by mode line (`' TEST: parse-ok|check-ok|check-fail|typed|ir|cpp`), and the front end over every
  corpus program (no panic, exact byte round trip; no diagnostics for `tests\corpus\slice.list`; at least one error
  for every `.err` program). Tier 2 runs the programs of `slice.list` end to end with the corpus runner's
  `--list` (70 on 2026-10-06): all pass, also with constant folding off. The full corpus with `qb64rust`: those
  pass, and so do the `.err` programs that get a real error; the 5 known failures are not run; everything else is
  rejected with a diagnostic (`tests\corpus\README.md`).
  Intentional differences from the old compiler are in `DIVERGENCES.md` (D-001, D-002: integer overflow wraps);
  the numeric rules are the spec `openspec\specs\language\numeric-semantics`, procedures and error handling the
  specs `language\procedures` and `language\error-handling`.
- **The call-site check (kept 2026-10-10, `DECISIONS.md`; `tools\callsite\README.md`):** for one statement per
  program of `tests\callsite` (256 on 2026-10-10), the libqb call the new compiler writes is compared with the
  old compiler's (`-z` on both, six normalisation rules, none naming a built-in). Part of tier 2, about a minute.
  It is how a plain built-in comes in: a row in the list, a program here, a line in a slice program. It does not
  see behaviour, element addressing, stores or the cleanup of string temporaries; the corpus does.

## 11. Other repositories and sources: conclusions (reviews archived)

What may be taken from other projects is `CLAUDE.md` rule 5. The reviews behind these verdicts are closed and kept
in `study\archive\` (`11`–`14`, plus the measurement scripts); nothing in the active plan needs them.

| Source | Conclusion |
|---|---|
| `qb64pe-vscode` (community extension) | Reference only, nothing copied. Its regex-based checks can flag code the compiler accepts, which our design avoids. Its feature list is a parity checklist. |
| `vscode-qb64fresh` (the user's extension) | Not the base (written fresh instead); small self-contained pieces may be taken after reading them in full. Lessons: one structured output parser, not a list of known messages; one file per concern. |
| QB64Fresh (the user's earlier Rust rewrite) | Not a base: matched QB64pe output on 19 of 331 tests; front end not lossless, IR name-based. Taken: its 261 `runtime_comparison` BASIC programs (recorded against the old compiler at the start of M2), small pieces per rule 5, and process lessons: measure against `qb64pe.exe` from day one; one layer at a time; no special cases for one program; no status claims without measurements. |
| `rewrite-decision.md` (earlier external review) | Advised against an empty-repo rewrite; considered and overruled 2026-10-02 (`DECISIONS.md`). |
| FreeBASIC (`..\FreeBASIC`, GPL/LGPL) | No code. More modular than QB64pe but single-pass with a symbol-aware lexer: not a front-end model. Useful for runtime-call tables, defined C output (`-fwrapv`), lowering notes, test conventions (`16`, panel-reviewed). |
| QB64pe wiki | Primary documentation source. Fetched 2026-10-02 with `tools\wiki\fetch_wiki.py` (1,124 pages, git-ignored cache). No licence stated: local reference only until the maintainers are asked. |
| Microsoft QuickBASIC manuals (text, on `<share>`) | Best source for QB4.5 questions; copyrighted, never commit. |
| `docs-new-2` branch (internals docs on the user's QB64pe fork) | A map for M3/M5, never a spec: 5 of 13 checked explanations were wrong. |

In every case: the wiki, manuals and docs are hypotheses; `qb64pe.exe` is the answer.

## 12. Open questions and next steps

Kept in one place, `STATUS.md`, so they don't go stale here. As of 2026-10-03 M1 is done and no open question blocks M2.

## 13. Remaining study gaps

Closed: DIM/REDIM/STATIC/COMMON/ERASE, PRINT/INPUT/WRITE emission, the built-in table (`10`); "nothing was
executed" (`09`, `10`). Deferred with the array features: the member-array layer. Still open, to be read when the
matching milestone starts:

| Gap | Milestone | Where |
|---|---|---|
| ON TIMER/KEY/STRIG, FIELD, `_MEM*` statements (skimmed; FOR/DO/WHILE closed by `verification\v17_*` and the `FOR` temporaries at 6510–6522, 2026-10-06) | M2/M3 | `qb64pe.bas` 6212–8765 |
| Runtime drawing/printing/input bodies (PAINT, CIRCLE, DRAW, GET/PUT, `qbs_input`, `print_using`) | M3/M6 | `libqb.cpp` |
| CHAIN array COMMON handling | M4 | `10` §1.6 |
| Most of the ~45 accidental behaviours (only those in §6 were run) | M2–M4 | `01` §11.3, `02` §9.2, `04` G.4 |

## How reliable this is

`01`–`05` come from reading source; each states what was read fully and what was sampled (`04` was only partly
read for this synthesis: summary, Parts A, G, H). Claims marked ✓ above were confirmed by running the old compiler
(`09`, `10`); three study claims were corrected that way (`09`, "Corrected"). Facts about other projects
(`archive\11`–`14`) come from reading and running them on 2026-10-02. Anything not marked as run should be treated
as a reading of the code, not a measurement.

## Document map

| Doc | Content |
|---|---|
| `01` | Compiler front end: lexer, passes, name resolution, restarts, metacommands |
| `02` | Expressions and code generation: typing, storage, arrays, built-in calls, generated C++ |
| `03` | Runtime library: error model, strings, screen modes, graphics, files, input |
| `04` | IDE and debugger (vWATCH protocol), with the parity list used by `06` |
| `05` | Build, bootstrap, CI, vendored libraries, test suites |
| `06` | IDE features mapped to LSP, DAP, tasks and settings |
| `07` | Expert panel: strategy, architecture, roadmap, recommendations R1–R13 |
| `08` | What a strict QuickBASIC 4.5 mode would need (not planned) |
| `09` | Study claims checked by running the old compiler; bug-compatibility list |
| `10` | Study gaps closed: DIM family, PRINT/INPUT/WRITE, built-in table |
| `15` | Pre-coding review: bytes not strings, early vertical slice, diagnostics policy, minimal M1 backend, libqb copy |
| `16` | FreeBASIC: lessons to take (module split, runtime-call tables, `-fwrapv`, lowering notes, test conventions), things not to take, proposed follow-ups and the panel review |
| `17` | How VS Code extensions are tested (runners, Node in the extension host, what popular extensions do); how M1 compares |
| `18` | Other VS Code extension practices (bundling, manifest, workspace capabilities, status bar, notifications, CI) compared with five large extensions; proposals A–G |
| `19` | Test cadence: four tiers, what a full run costs, how the new compiler's runs are kept fast |
| `20` | Review of code and plan after the first slice (2026-10-04); panel outcome; the order of work (replaced by `22` §5) |
| `21` | Rust review of the workspace setup: lints adopted and not, CI, the repo check, how to apply them |
| `22` | Second review (2026-10-04): test inputs from QB64pe and QB64Fresh and what is taken, the upstream yardstick (x of 279); its order of work is replaced by `23` §4 |
| `23` | Third review (2026-10-05): path check of the codebase, the panel's outcome; its order of work is replaced by `24` §4 |
| `24` | Fourth review (2026-10-07): code against plan after the control-flow slice's groups 1–5, why arrays and `TYPE` move before the type table and built-ins, `STATUS.md` as entry point; the current order of work |
| `25` | IR review (2026-10-07): what the lowering does and the emitter still does, keep or merge, the place question for the arrays-and-`TYPE` slice with the old compiler's store rules to measure |
| `26` | Fifth review (2026-10-07): a wrong-code bug in `$IF`, clean programs that tier 2 never runs, member arrays no longer behind `$UNSTABLE`, `$CONSOLE` tests as M2 work, blockers by kind and the new order (all accepted 2026-10-07; plain member arrays un-deferred) |
| `27` | Sixth review (2026-10-08): architecture holds, no bug found; the corpus is blocked by built-in statements, file I/O and `DATA` that no step names (step 9 to become "by demand"), upstream by types and arrays; "`Ty` as a type table" dropped; the language server's design points (one binary, parser only, the encoding boundary, includes from open documents, two diagnostic sources); all accepted 2026-10-08 |
| `28` | Seventh review (2026-10-09): not going astray; the expected outputs come from the reference clone while CI links the 4.7.0 release's libqb (16 commits apart); graphics are on the critical path to M4 (built-ins used by `qb64pe.bas`, the corpus and upstream, counted), a screen-state oracle and a call-site check proposed; Civil War Strategy assessed as the real-program graphics target; `STATUS.md` cut to the current state; the formatter off M2's exit criterion; the user's answers and what was done are its §9 (the local clone is the reference, the game copied, the formatter out of M2); the screen-state check and the call-site check wait for a yes |
| `29` | Eighth review (2026-10-10, before step 9a): nothing to change in the code, no bug found; the screen-state hook needs neither the libqb copy nor a libqb change (one extra C++ file linked through `-f:ExtraLinkerFlags`/`CXXLIBS_EXTRA`, `atexit`, programs ending with `SYSTEM`, keys by `PostMessage`), so the copy can stay at M3; keep 9a small, then step 11 before 10 (the game's needs, measured); a demand report over the input sets; tests for the runner; the user's answers and what was done are its §8 (all accepted; the demand report's first numbers; the CI recommendation withdrawn: the reference commit's compiler differs from the release's in typed `VAL`) |
| `progress-record` | Closed record of what `STATUS.md` held until 2026-10-09: the per-change history of M0 to M2 step 8, unchanged |
| `archive\11`–`14` | Closed reviews of other repositories (VS Code extensions, QB64Fresh, documentation sources, `docs-new-2`); conclusions in §11 |

Decisions taken on the basis of these documents are logged in `DECISIONS.md`.
