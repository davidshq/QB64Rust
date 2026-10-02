# 00 — Synthesis: what QB64pe is, and what a rewrite has to decide

Written 2026-10-02 from the five study reports in this folder (01–05), all of which I read in full except
`04-ide-debugger.md`, where I read the summary, Part A (compiler interface), Part G (assessment) and Part H (coverage).
Tree studied: `..\QB64pe`, HEAD `16f629784e`, `Version$ = "4.7.0-GLFW"`, remote `davidshq-contribute/QB64pe`.

## Confidence

The reports come from reading source; nothing was built or run. Each report states what was read fully and what was
only sampled. What I checked myself against the source:

| Claim | Check | Result |
|---|---|---|
| ELSE handler tests `controlstate(controllevel)`, not the matched IF's level | read `qb64pe.bas:6627-6638` | as described |
| Procedure prologue, `do{…if(!qbevent)break;evnt(…);}while(r);` wrapper, `S_n` labels, `-(a==b)` booleans, `\|\|is_error_pending()` on conditions, `#line` emission | read `internal\source\main1.txt:1-40` (the compiler's own generated C++) | as described |
| Integer `+` emitted with no casts on dereferenced pointers | same file, line 32 | as described |
| `mainfree.txt` is generated but never included | grep of `internal\c` `.cpp`/`.h` | no include found |
| Test counts (404 / 331 / 143 / 56) | file counts on disk | match |
| Fork keywords (`_RETAIN`, `TYPEFIELDS`, `$USELIBRARY`, `$ERRORLOCATION`, `_ARRAYCOPY`) present | grep of `qb64pe.bas` | 24 hits |

Everything else below is as reported by the studies. Two reports said no generated C++ was available to compare
against; that is wrong in a useful way: `internal\source` *is* generated output (the compiler compiled by itself,
2,079 files), so codegen claims can be checked there without building anything.

## 1. The system in one page

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
| Vendored libs `internal\c\parts` | ~1,100 files | GLFW, miniaudio, FreeType, curl, stb, etc. (table in 05 §3) |
| Bootstrap `internal\source` | 2,079 files | Generated C++ of the compiler, committed by CI |
| Tests | 404 + 143 `.bas` | 331 with expected stdout, 56 with expected error text |

## 2. Five properties that define the current design

1. **No AST, no IR.** A line is a delimiter-separated string; expressions are rewritten in place into bracketed
   strings and then into C++ text. Types are one bit-packed LONG. Lvalues are strings like `id␚udt␚element␚offset`.
2. **Parse, check, emit and format in one step.** Each statement handler emits C++ and also builds the pretty-printed
   line. There is no formatter separate from the compiler, and no analysis without code generation.
3. **Fixpoint by restart.** Facts discovered late (a variable needs a 16-bit address, an array parameter's dimension
   count, a metacommand, a label that is really a SUB call) set a flag and re-run both passes from scratch.
4. **C++ is the semantic back end.** Arithmetic typing is delegated to C++ promotion rules; the runtime ABI is
   "whatever libqb's C++ overloads accept", including `passed` bitmasks for optional arguments.
5. **The IDE lives inside the compiler.** It is a coroutine entered by `ide(0)` and left by `GOTO`; it reads compiler
   globals directly (variables, warnings, formatted line, UDT tables).

## 3. What a rewrite must reproduce (compatibility contract)

These are the behaviours user programs depend on. Full lists: 01 §11.1, 02 §9.1, 03 §9.1.

**Language front end**
- Lexical rules: suffix forms, `&H/&O/&B` typed by digit count with signed wrap, float literals typed by significant
  digits, periods in names vs. UDT member access, 40-char names, `?` = PRINT, DATA captured raw.
- Name resolution: `x%`, `x&`, `x$` are different variables; `musthave`/`mayhave` suffix rules; four-step lookup
  (local, local+DEFtype suffix, global, global+suffix); scalar and array of the same name coexist.
- Implicit declaration of scalars and of arrays (upper bound 10); `OPTION _EXPLICIT` turns it off.
- Operator precedence (16 levels): `MOD` below `\` below `* /`; unary minus below `^`; `^` left-associative;
  `NOT` below comparisons.
- Single-line IF forms, position-dependent DEFxxx, `'$DYNAMIC` taking effect on the next line, `$IF` with its
  left-to-right, no-parentheses evaluation.
- Three different compile-time evaluators (CONST, `$IF`, and ordinary constant folding) with different rules.

**Numeric semantics** (highest risk of silent differences)
- Float → integer is round-half-to-even everywhere, via x87 `fistp`, with no range check on store.
- Integer `+ - *` on ≤32-bit operands is computed in 32-bit C `int`; no overflow error.
- `int / int` is `long double`; `^` always goes through `long double` pow; `_FLOAT` is 80-bit `long double`.
- SINGLE literals are emitted as C++ doubles.
- Comparisons yield -1/0; logical operators are bitwise; float-vs-float comparisons narrow to the smaller type.

**Runtime model**
- By-reference arguments, with a silent by-value temporary (no copy-back) on type mismatch, expression, or `(x)`.
- Arrays: column-major, descriptor layout, static-vs-dynamic rule, `REDIM _PRESERVE` by linear position.
- Errors and events are serviced only at statement boundaries; `RESUME`/`RESUME NEXT` are statement-granular.
- One DATA blob for the whole program in source order.
- Emulated DOS memory is observable: SCREEN 13 at `&HA000`, SCREEN 0 at `&HB800`, BIOS keyboard buffer, DGROUP at
  segment `&H50`, `VARPTR`/`VARSEG` values, DAC ports, INT 33h mouse.
- Bit-exact library behaviour: `STR$`/PRINT number formatting, PRINT USING, `VAL`, the RND generator, `TIMER`
  quantisation, MBF conversions, screen-mode table, GET/PUT image format, INKEY$/`_KEYHIT` codes, file semantics.

## 4. What can be dropped (implementation, not behaviour)

- Whole-program restarts and `RCStateVar` → a real symbol table and a semantic pass before code generation.
- String-of-elements representation, the scratch `id` + `findanotherid` protocol, the 64 MB hash table.
- Hidden parameters passed through globals (`dimshared`, `dimstatic`, `reginternalsubfunc`, …).
- Duplicated logic: two include managers; TYPE/CONST/SUB-header parsing in both passes; argument passing written
  twice; two interpreters for the optional-argument format.
- Text-fragment stitching through `qbx.cpp`, buffer handles swapped under the emitters, `retN.txt` pasted at every RETURN.
- "Did the exe appear?" as the only build result.
- Fixed limits (100 parameters, 1000 control levels, 4095 UDTs, …).
- The IDE-as-coroutine protocol and its shared-memory results.

## 5. Decisions the design has to make first

Each of these changes the shape of everything after it. They are listed in dependency order.

| # | Decision | Options | What the study says |
|---|---|---|---|
| 1 | **Target dialect** | Stock QB64-PE, or this tree including its extras (TYPE member arrays, `REDIM _RETAIN`, `_ARRAYCOPY`, whole-array assignment, `$USELIBRARY`, `$ERRORLOCATION`) | The extras are woven through `evaluate`, `refer`, `setrefer`, `dim2`, `allocarray`. They were flagged but not studied in depth, and nobody compared against upstream. 211 of the 404 compile tests are in `arrays`, much of it this layer. |
| 2 | **Scope of "rewrite"** | (a) compiler only, keep libqb + Makefile; (b) compiler + runtime; (c) all three including IDE | (a) keeps ~600 runtime entry points and every bit-exact behaviour for free, but binds the new compiler to the current ABI (pointer-per-variable, `qbs`, descriptor layout, `passed` bitmasks, `qbx.cpp` fragments). The runtime report lists which modules are reusable as-is (03 §9.3). |
| 3 | **Back end** | Emit C++ as today; emit C; own IR + LLVM/other; interpreter/VM | Today's numeric semantics *are* C++ promotion rules plus x87. Any non-C++ back end must encode those rules explicitly. Emitting C++ against the existing libqb is the lowest-risk first target. |
| 4 | **Implementation language** | BASIC (self-hosting, as now) or another language | Self-hosting forces the new compiler to compile itself early and keeps the `internal\source` bootstrap loop. Another language removes the bootstrap problem but the compiler stops being a QB64 program. |
| 5 | **IDE** | **Decided 2026-10-02: do not port the text-mode IDE. Reach feature parity with a VS Code extension.** | Requires the compiler to expose a language-server boundary (04 Part G.2) and a debug adapter. Formatting must become separable from code generation. Mapping: `06-vscode-parity.md`. |
| 6 | **Error model at run time** | Keep flag-polling + recursive `QBMAIN`; or structured unwinding | Changes every runtime function and the statement wrapper. Only relevant if the runtime is in scope. |
| 7 | **Bug compatibility** | Reproduce the ~45 catalogued accidental behaviours, or fix them | Lists: 01 §11.3, 02 §9.2, 04 G.4. Some are harmless (unused `mainfree.txt`), some change program results (`_BIT*n` >32 gets 4 bytes; CONST `^` is right-associative; integer divide by zero is fatal, unlike QB4.5). |

## 6. Testing position

- Reusable as a black-box conformance suite: 331 expected-output tests (357 of 404 test programs are
  `$CONSOLE:ONLY`, so stdout only) and 143 compile-only real-world programs.
- Tied to the current implementation: 56 expected-error-text tests, `qb64pe/*` internals tests, formatter tests,
  licence tests.
- Not covered at all: the IDE, real graphics output, audio, interactive input, and most classic QBasic semantics
  (PRINT USING, file modes, string/math functions are only lightly tested).
- The runners are bash. A Windows harness (PowerShell or Python) is a prerequisite; rules are in 05 §5.7.
- The existing compiler is the oracle. Differential testing (same program through old and new, compare stdout and,
  for the C++-emitting option, compare generated code shape) covers the gap the test suite leaves.

## 7. Gaps in the study

| Gap | Why it matters | Where |
|---|---|---|
| DIM / REDIM / STATIC / COMMON block and `dim2` numeric branches | Core declaration semantics; only entry conditions and samples read | `qb64pe.bas` 8767–9685, 17624–18939 |
| PRINT / INPUT / WRITE / PRINT USING emission | Most-used statements; located, not read | `qb64pe.bas` 11017–11316, `xprint` 27713 |
| Built-in table | 455 registrations studied by grep and sampling; no per-built-in argument table | `subs_functions.bas` |
| FOR/DO/WHILE bodies, ON TIMER/KEY/STRIG, FIELD, ERASE, `_MEM*` statements | Skimmed in 01; partly covered in 02 §6 from a helper's read | `qb64pe.bas` 6212–8765 |
| Fork-specific member-array layer | Flagged, not studied; upstream comparison not done | throughout core functions, `arrcpy.bm`, `array-copy.cpp` |
| Runtime drawing/printing/input bodies | Outlined only (PAINT, CIRCLE, DRAW, GET/PUT, `qbs_input`, `print_using`, `display()` middle) | `libqb.cpp` |
| Nothing was executed | All "probable defect" entries are unconfirmed | — |

The first three are the ones worth closing before design starts; they are bounded reads (roughly 3,500 lines of
`qb64pe.bas` plus the 4,000-line registration table).

Update (session 3): the first three are closed in `10-gaps.md`, which also covers ERASE. "Nothing was executed"
is addressed by `09-verification.md` and the checks in `10-gaps.md`. The member-array layer is deferred
(`SOMEDAY.md`); the other rows remain open and can be read when the matching part of the rewrite starts.
