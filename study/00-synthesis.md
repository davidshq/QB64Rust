# 00 — Synthesis: start here

The one-page entry point to this folder. It summarises every study, the decisions, the plan, what was
measured, and what is still open. Details live in the numbered documents; each section points to them. 
Sections or parts moved out of this file (§5 and §13 whole) are in `archive\00-synthesis.md`, under the same headings.
Closed documents live in `archive\` and are read only when needed (`CLAUDE.md` working rule 7): 
- the reviews of other repositories (`11`–`14`, conclusions in §11 here) and, 
- the reviews and plans whose outcomes are decided and built (`07`, `08`, `15`, `17`–`40`, `42`–`44`), and
- the studies of the old compiler that no milestone needs now (`01`, `02`, `04`, `05`, `06`, `09`, `10`, `16`).

Tree studied: `..\QB64pe`, HEAD `16f629784e`,
`Version$ = "4.7.0-GLFW"` (upstream QB64pe `main`).
Current phase and next steps: `STATUS.md`. Rules: `CLAUDE.md`. Decision log: `DECISIONS.md`.

## 1. Decisions

The decisions that set the project's direction (2026-10-02: a ground-up rewrite, no port of the text-mode IDE
but a VS Code extension, the compiler in Rust, new error messages, no strict QuickBASIC 4.5 mode, upstream QB64pe
4.7.0 as the dialect) and every later one are in `DECISIONS.md`.

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

Milestones, what each needs to be done and where it stands: `ROADMAP.md` and `ROADMAP-COMPLETED.md`.

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

Full lists: `01` §11.1, `02` §9.1, `03` §9.1. Items marked ✓ were confirmed by running the old compiler (`09`, `10`); the rest are readings of
its source.

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
  coordinates, unless only the last dimension changes (✓); `REDIM _RETAIN` keeps coordinates (✓, `v34`).
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

Moved whole to `study\archive\00-synthesis.md` on 2026-11-09, under this heading. It holds the measured
facts of the families up to `v45`, closed to new facts since 2026-10-18; a later family's facts are its
recorded output and its line in `verification\README.md`. Code comments that cite `study\00` §5 mean that
archived section (`CLAUDE.md` working rule 7).

## 6. Bug-compatibility choices (all decided)

From the end of `09` and `10` §2.8, §3.2 and the measurements in §5. **All decided by the user on 2026-10-08**
(step 8 of `STATUS.md` "Next", `DECISIONS.md`), except where an earlier date is given. The rule applied: a
deterministic behaviour that changes results is **kept** (QB64pe programs rely on it, and the differential tester can
then expect equal output); a crash, memory corruption or failed compile is **fixed**. A difference from QB64pe is a
row in `DIVERGENCES-QB64PE.md`; a kept behaviour that differs from QuickBASIC 4.5 is a row in `DIVERGENCES-QB45.md`.

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

**Fixed (no compatibility value):** each is a row of `DIVERGENCES-QB64PE.md` (D-003, D-005 to D-014 from these
decisions, later ones from the changes that found them; the list that stood here until 2026-10-18 is in
`study\archive\00-synthesis.md`). Rejected by both compilers, so no register row, only a proper message: `ELSE` while an
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
failed build is fixed (a `DIVERGENCES-QB64PE.md` row); anything else is implemented as QB64pe does it and, if it looks
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

From `03` and `07` Session 7. 
- Leave alone: vendored libraries, `libqb\src` modules, the bit-exact units
(`rounding.h`, `qbs_str.cpp`, `qbs_val.cpp`, PRINT USING, RND, number parsers). 
- Fix early, low risk: `#define int32` macros leaking everywhere; the 892 `static` locals that make functions non-reentrant (`SUB _GL` and event handlers
re-enter). 
- Redesign later (planned for M6, a `SOMEDAY.md` row since `DECISIONS.md` 2026-10-28): the error model (`error()` returns, every function early-outs, a trapped error calls `QBMAIN` recursively and never unwinds), the moving string heap, flag-based thread synchronisation. The IR models "statement may raise; resume point here" explicitly so the error model can change without touching the front end.

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

How the new compiler is tested, tier by tier: `crates\README.md`; each test set and its counts: its `tests\*\README.md`; 
- when each run happens: `CLAUDE.md` working rule 6; 
- the old compiler's baseline: `baselines\README.md`; 
- the runner and how it runs the old compiler's programs (`QB64PE_NOPROMPT=y`, fatal errors read from the output): `tools\legacy_tests\README.md`. The position as written on 2026-10-03, with the plan of R11's test layers, is in `study\archive\00-synthesis.md`.

## 11. Other repositories and sources: conclusions (reviews archived)

What may be taken from other projects is `CLAUDE.md` working rule 3. The reviews behind these verdicts are closed and kept
in `study\archive\` (`11`–`14`, plus the measurement scripts); nothing in the active plan needs them.

| Source | Conclusion |
|---|---|
| `qb64pe-vscode` (community extension) | Reference only, nothing copied. Its regex-based checks can flag code the compiler accepts, which our design avoids. Its feature list is a parity checklist. |
| `vscode-qb64fresh` (the user's extension) | Not the base (written fresh instead); small self-contained pieces may be taken after reading them in full. Lessons: one structured output parser, not a list of known messages; one file per concern. |
| QB64Fresh (the user's earlier Rust rewrite) | Not a base: matched QB64pe output on 19 of 331 tests; front end not lossless, IR name-based. Taken: its 261 `runtime_comparison` BASIC programs (recorded against the old compiler at the start of M2), small pieces per working rule 3, and process lessons: measure against `qb64pe.exe` from day one; one layer at a time; no special cases for one program; no status claims without measurements. |
| `rewrite-decision.md` (earlier external review) | Advised against an empty-repo rewrite; considered and overruled 2026-10-02 (`DECISIONS.md`). |
| FreeBASIC (`..\FreeBASIC`, GPL/LGPL) | No code. More modular than QB64pe but single-pass with a symbol-aware lexer: not a front-end model. Useful for runtime-call tables, defined C output (`-fwrapv`), lowering notes, test conventions (`16`, panel-reviewed). |
| QB64pe wiki | Primary documentation source. Fetched 2026-10-02 with `tools\wiki\fetch_wiki.py` (1,124 pages, git-ignored cache). No licence stated: local reference only until the maintainers are asked. |
| Microsoft QuickBASIC manuals (text, on `<share>`) | Best source for QB4.5 questions; copyrighted, never commit. |
| `docs-new-2` branch (internals docs on the user's QB64pe fork) | A map for M3/M5, never a spec: 5 of 13 checked explanations were wrong. |

In every case: the wiki, manuals and docs are hypotheses; `qb64pe.exe` is the answer.

## 12. Open questions and next steps

Kept in one place, `STATUS.md`.

## Document map

| Doc | Content |
|---|---|
| `03` | Runtime library: error model, strings, screen modes, graphics, files, input |
| `47` | The next big wins, and the gaps against QB64pe, FreeBASIC, other modern BASICs and modern languages (2026-11-09): reach first, then real programs and what the ports found, then editor rows, fuzzing and build time; the panel's view and three questions. Open until the user answers them |
| `48` | Language features of Python, C/C++, Rust, Go and JavaScript beside QB64Rust's (2026-11-11): a language table, a tooling table and what stands out; then each missing feature with the case for and against adding it, and the panel's view. Nothing decided; the other languages' cells are from general knowledge |
| `archive\` | Closed documents, read only when needed (`CLAUDE.md` working rule 7); what each holds: `archive\README.md`. Since 2026-10-28 also the old compiler's front end (`01`), code generation (`02`), the rest of the IDE and debugger (`04`), its build (`05`), and the reviews and studies `35`–`40`; since 2026-10-29 the closed study gaps (`10`: DIM family, PRINT/INPUT/WRITE, built-in table), FreeBASIC (`16`) and VS Code parity (`06`: the IDE's features mapped to LSP, DAP, tasks and settings; what is left is in `SOMEDAY.md`); since 2026-11-09 the first expert panel (`07`, its R1–R13 summed up in §2), the claims checked in M0 (`09`), the extension's testing and practices (`17`, `18`), the prior art of `value-trace` (`42`), the modern languages' tools (`43`, its open items now in `47` §6) and the QB64 projects on GitHub (`44`); since 2026-11-10 the process-reset proposal (`41`, its P5–P7 postponed) |

Decisions taken on the basis of these documents are logged in `DECISIONS.md`.
