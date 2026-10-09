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
| M2 | Front end | Lossless parser with recovery over the whole language (no false syntax error on any program the old compiler accepts: corpus, upstream tests, `qbasic_testcases`, its own sources), resolution, type checker, a thin language server; formatter matching `-y` | **in progress**: golden corpus; Rust workspace and end-to-end slice; procedures and error handling; upstream tests; control flow and constants; arrays and `TYPE`; parser breadth (every accepted program parses, no false error) (`crates\`; 147 corpus programs and 34 of 279 upstream ones pass end to end). Order of work: `STATUS.md`, `study\24` §4 |
| M3 | Code generation to the existing ABI | Emits `qbx.cpp` fragments, links with libqb, passes expected-output and differential tests; programs without `$CONSOLE:ONLY` with a screen-state oracle | started early: the slice already links with libqb |
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
without a return type (D-013). Rejected by both compilers, so no register row, only a proper message: `ELSE` while an
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
| `archive\11`–`14` | Closed reviews of other repositories (VS Code extensions, QB64Fresh, documentation sources, `docs-new-2`); conclusions in §11 |

Decisions taken on the basis of these documents are logged in `DECISIONS.md`.
