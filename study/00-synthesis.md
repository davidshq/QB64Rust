# 00 — Synthesis: start here

The one-page entry point to this folder. It summarises every study, the decisions, the plan, what was
measured, and what is still open. Details live in the numbered documents; each section points to them. The reviews
of other repositories (`11`–`14`) are closed and live in `archive\`; their conclusions are in §11 here.

First written 2026-10-02 from `01`–`05`; rewritten 2026-10-02 (session 5) to cover `06`–`14`, `baselines\` and
`verification\`; trimmed 2026-10-03 when `11`–`14` were archived. Tree studied: `..\QB64pe`, HEAD `16f629784e`,
`Version$ = "4.7.0-GLFW"` (upstream QB64pe `main`).
Current phase and next steps: `STATUS.md`. Rules and the decision log: `CLAUDE.md`.

## 1. Decisions

All taken 2026-10-02; the authoritative list is in `CLAUDE.md`.

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
| M2 | Front end | Lossless parser with recovery over the whole language (no false syntax error on any program the old compiler accepts: corpus, upstream tests, `qbasic_testcases`, its own sources), resolution, type checker, a thin language server; formatter matching `-y` | **in progress**: golden corpus; Rust workspace and end-to-end slice; procedures and error handling (`crates\`; 54 corpus programs pass end to end). Order of work: `STATUS.md`, `study\22` §5 |
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
  reports "overflow" (not yet in `DIVERGENCES.md`).

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
  number twice: "Duplicate label (10)"; `GOTO 99` missing: "Label '99' not defined".
- **Blocks (M4):** `NEXT j, i` closes both; wrong order or the outer variable: "Incorrect variable after NEXT".
  A `NEXT` inside a multi-line or single-line `IF`, closing an outer `FOR`: "NEXT without FOR". `LOOP` closing a
  `WHILE` or crossing blocks (`DO`/`FOR`/`LOOP`/`NEXT`): "PROGRAM FLOW ERROR!"; `WEND` closing a `DO`: "WEND without
  WHILE"; `END IF` alone or after a single-line `IF …: END IF`: "END IF without IF"; `EXIT FOR` outside: "EXIT FOR
  without FOR". A missing `END IF`/`END SELECT` is reported against the auto-included `vwatch_stub.bm` ("IF without
  END IF in line 3 of internal\support\vwatch\vwatch_stub.bm"). Single-line `IF`: **each `ELSE` binds to the
  innermost `IF`**; `THEN 10 ELSE 20` and `IF 0 GOTO 20` work. `ENDIF` is accepted; `ELSE IF` is an `ELSE` holding
  a new `IF` block. Only comments may stand between `SELECT CASE` and the first `CASE` (a statement: "Expected
  CASE expression"). `EXIT FOR` inside `DO` inside `FOR`, and `EXIT DO` inside `FOR` inside `DO`, work.
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
  branch are ignored, nested `$IF`s there are counted. `$IF`/`$LET` **only at the start of a line**: after `:` or
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
  reaches the main file. Missing file: "File x not found". There is no cycle check: a self-include stops at 100
  levels ("Too many indwelling INCLUDE files", listing every level), and one guarded by `$IF` and `$LET` is
  accepted (included twice, the second time inactive; `v16_m7_self_guarded`). Messages from inside an include
  name the file by its full path and carry a 0x01 byte before " in line n of …".
- **Member access (M8):** `a.b` without a `TYPE` is a plain variable (separate from `a`); `DIM x.y AS LONG` and
  `x.z$` too. A plain `a.b`, then `DIM a AS t` with member `b`, is accepted and `a.b` is the member from then on.
  With `a` a `TYPE` variable, `a.c` for a non-member is "Element not defined". `a(2) .b` and `a(2). b` (blanks
  around the dot) are member access.

## 6. Bug-compatibility choices still to make

From the end of `09` and `10` §2.8, §3.2. Each choice goes into the divergence register (R2).

**Decide (result-changing; current default is "keep"):** CONST `^` right-associativity; INTEGER
arithmetic computed in 32 bits inside expressions (`i% + 1` gives 32768; LONG wrap is decided below); round-half-to-even with single-precision narrowing for INTEGER targets;
NUL-filled fixed-length strings (QB4.5: spaces); linear `REDIM _PRESERVE`; fatal integer division by zero; console
comma zones 10 wide (or 14 like everywhere else); INPUT prompts as literals only (or allow expressions).

**Decided:** LONG overflow **wraps** (two's complement), defined in the generated code and in constant folding;
matches the old compiler's default build, differs from its `-O2` build (`16` §8, `verification\v11_wrap_o2`).

**Fix (no compatibility value):** `_BIT * n` with n > 32 overlapping the next variable by 4 bytes; static `SELECT CASE`
temporaries overwritten by recursion; `label: CONST …` on one line failing to compile; `ELSE` while an inner `FOR`
is open passing the front end and failing in C++; `INF` printed with padding and a stray `D`; the console `tab()`
hang and the `CONOUT$` handle leak; `_LogMinLevel` and `_ScreenExists` registered without a return type;
`ON n GOTO` with n > 255 falling through silently (QB4.5: error 5). Runtime errors should exit non-zero.

The full catalogue of about 45 accidental behaviours: `01` §11.3, `02` §9.2, `04` G.4; only the ones above have been
run.

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
  for every `.err` program). Tier 2 runs the 54 programs of `slice.list` end to end with the corpus runner's
  `--list`: all pass, also with constant folding off (2026-10-04). The full corpus with `qb64rust`: those 54 pass,
  the 5 known failures are not run, everything else is rejected with a diagnostic (`tests\corpus\README.md`).
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
| `rewrite-decision.md` (earlier external review) | Advised against an empty-repo rewrite; considered and overruled 2026-10-02 (`CLAUDE.md`). |
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
| FOR/DO/WHILE bodies, ON TIMER/KEY/STRIG, FIELD, `_MEM*` statements (skimmed) | M2/M3 | `qb64pe.bas` 6212–8765 |
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
| `16` | FreeBASIC: lessons to take, things not to take, proposed follow-ups |
| `17` | How VS Code extensions are tested; how M1 compares |
| `18` | Other VS Code extension practices compared with five large extensions; proposals A–G |
| `19` | Test cadence: four tiers, what a full run costs |
| `20` | Review of code and plan after the first slice; panel outcome; the order of work (replaced by `22` §5) |
| `21` | Rust review of the workspace setup: lints, CI, the repo check |
| `22` | Second review: test inputs from QB64pe and QB64Fresh, the upstream yardstick (x of 279), the current order of work |
| `archive\11`–`14` | Closed reviews of other repositories (VS Code extensions, QB64Fresh, documentation sources, `docs-new-2`); conclusions in §11 |
