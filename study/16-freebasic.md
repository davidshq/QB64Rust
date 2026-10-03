# 16 — FreeBASIC: what to learn from it

Written 2026-10-03; revised the same day after a panel review and a second pass (§7 lists what changed).
Question from the user: FreeBASIC's code is said to be more modular than QB64's; is there anything we should learn from it? No code is reused (licence, below).

Tree studied: `..\FreeBASIC` (shallow clone of `github.com/freebasic/fbc`, HEAD `5714d10adb`, 2026-04-05).

**How this was read.** One session, inline, about 25 targeted reads. Read in full or nearly: `ir.bi` (interface),
the header of `ir-hlc.bas`, `rtl-error.bas` (error-check part), `rtlib\error.c`, `cPrintStmt`, `parser-quirk.bas`
(dispatcher), `error.bas` (reporting), `tests\readme.txt`, `rtlib\gosub.c`, and FreeBASIC's own developer notes (`doc\manual\cache\Dev*.wakka`: modules,
parser, lexer tokens, memory, SELECT CASE, arrays). Sampled: `lex.bi`, `ast.bi`, `symb.bi`/`symb.bas`,
`rtl.bi`, `rtl-print.bas`, `ast-gosub.bas`, `fb.bi`/`fb.bas` (dialect options), runtime headers, `todo.txt`, a few
tests. Not read: the x86/LLVM/gas64 back ends, the optimiser, OOP parts, gfxlib2 bodies. FreeBASIC itself was not built or run; treat statements about it as a reading of the code. One claim about
QB64pe (L4) was measured with the old compiler.

## 1. Facts

| Item | Value |
|---|---|
| Licence | Compiler: GPL v2 or later. Runtime (`rtlib`, `gfxlib2`): LGPL v2 or later with a linking exception. Manual: GNU FDL. |
| Compiler `src\compiler` | 173 files, 144,559 lines of FreeBASIC (self-hosted). Largest files are back ends (`ir-gas64.bas` 8,423, `emit_x86.bas` 8,160); the largest front-end file is `symb-proc.bas` at 3,457. Compare `qb64pe.bas`: 28,828 lines in one file. |
| Runtime `src\rtlib` | 426 C files (31,111 lines) at the top level, roughly one runtime function per file, plus one folder per platform (win32, linux, dos, darwin, js, …). |
| Graphics `src\gfxlib2` | Separate library; 9,693 lines at the top level plus platform drivers. |
| Tests `tests\` | 2,515 `.bas` files in about 45 topic folders; `tests\qb` holds the `-lang qb` dialect tests. |
| Own to-do list | `todo.txt`, 430 lines; much of it is about differences between the four back ends. |

## 2. Structure

```
fbc.bas (driver)
  lex.bas + pp*.bas          lexer with 3-token lookahead; preprocessor and macro expansion inside the lexer
  parser-*.bas  (57 files)   recursive descent, one file per statement family
       parser-quirk-*.bas    statements with their own syntax: PRINT, OPEN, LINE, PUT, ON, DATA, …
  symb-*.bas    (15 files)   symbol table: hash chains, scopes, suffix and DEFtype lookup
  ast-node-*.bas (25 files)  typed AST, one file per node kind; constant folding at node construction
  rtl-*.bas     (16 files)   runtime functions declared as data tables, plus call-building helpers
  ir.bi                      back-end interface: a table of about 65 function pointers (IR_VTBL)
       ir-tac → emit_x86     32-bit x86 assembly
       ir-hlc                C for gcc
       ir-llvm, ir-gas64     LLVM IR, 64-bit assembly
```

It is a **single-pass compiler**: the parser builds AST for one statement, adds it to the current procedure,
and each procedure is handed to the back end when it ends.

## 3. Lessons worth taking

Each item says how it relates to our plan (`07` recommendations R1–R13, `15`).

**L1. One file per statement family, with a thin dispatcher.** `parser-quirk.bas` is a `SELECT CASE` on the
token that calls `cPrintStmt`, `cDataStmt`, `cLineInputStmt` and so on; each lives in its own file of 200–1,400
lines. This is the same lesson as QB64Fresh's "thin dispatchers" (`00` §11) and it visibly works at 145k lines.
*For us:* M2 parser modules follow statement families (print/input, file, graphics, data, control flow, declarations).

**L2. "Quirk" statements = hand-written parser + table-declared runtime calls.** `cPrintStmt` parses
`PRINT [#n,] [USING s;] items` and for every item builds a call to one runtime function chosen by the item type
(`fb_PrintInt`, `fb_PrintString`, …) with a mask for comma/semicolon/newline. The runtime functions are data
(`FB_RTL_PROCDEF`: name, alias, return type, calling convention, option flags, parameters with optional
defaults). The option flags include "returns an error code", "overloaded", "QB dialect only", "has `$` suffix".
*For us:* confirms R9. Two additions to the built-in table design: a **"cannot raise a runtime error"** flag per
entry (default: may raise; in libqb nearly any call can reach `error()`, so only functions known to be pure get
the flag, by reading libqb, not by extraction), and a clean split for the 86 hand-handled names (`00` §9):
syntax in the parser, lowering into ordinary table calls.

**L3. A worked example of explicit error and resume points.** Runtime functions return an error code. When
error checking is on, the compiler wraps the call (`rtlErrorCheck`): a *resume label* before the statement, the
call, `if result <> 0 then jump to fb_ErrorThrow(line, module, resume label, resume-next label)`, and a
*resume-next label* after it. `fb_ErrorThrow` returns the address of the user's handler (or ends the program);
`RESUME` and `RESUME NEXT` are computed jumps to the two stored labels. The error state is thread-local.
There is no recursion into main and no unwinding problem.
*For us:* an illustration of R5 ("statement may raise; resume point here"), **not a model to copy**. The checks
are opt-in (`-e`, `-ex`, `-exx`), exist only after runtime calls flagged as returning an error, and the labels are
per runtime call, so `RESUME` re-runs one call, not the statement. QB64pe is statement-granular (errors serviced
at statement boundaries, rest of a PRINT skipped) and stays the reference. The C back end also needs gcc's
labels-as-values (`&&label`). What carries over to M6: error state in a thread-local context, a handler lookup
that returns where to continue, and no recursion into main.

**L4. Make the generated C defined.** The C back end's header lists its rules: compile with `-fwrapv` so
signed overflow wraps, mask shift counts, do float→integer through explicit rounding helpers and never a C
cast, and represent booleans as `int8`. **Finding (measured, `verification\v11_wrap_o2`):** QB64pe's `Makefile` sets `-fno-strict-aliasing` but **not
`-fwrapv`**, so LONG overflow in generated C++ is undefined behaviour and the result depends on the
"optimize C++" setting (`-O2`):

| `x& = 2147483647` | default build | `-f:OptimizeCppProgram=true` |
|---|---|---|
| `IF x& + 1 > x&` | false | **true** |
| `PRINT x& + 1` | −2147483648 | **2147483648** |
| `y& = x& + 1: PRINT y&` | −2147483648 | −2147483648 |

INTEGER is not affected: `i% + 1` is computed in 32 bits (prints 32768 in both builds) and narrowing on store is
defined. So "QB64pe behaviour as observed" (R2) has two answers for LONG overflow. *For us:* define it. Compile
generated code with `-fwrapv` (or emit unsigned arithmetic), which matches the default build, and record it in
the numeric-semantics spec and the divergence register (differs from QB64pe `-O2`). Differential tests must run
the old compiler in its default build.

**L5. Back end behind an interface, with `supportsOp`.** The AST asks the back end whether it can do an
operation on a type; if not, the AST emits a runtime call instead. Four back ends share the interface.
*For us:* confirms R6 (the IR must not encode the libqb ABI). We keep one back end; see A4 for the cost of four.

**L6. Diagnostics policy.** At most one error per statement (later errors in the same statement are dropped),
a cap on the total (`-maxerr`), recovery by skipping to the end of the statement or the matching parenthesis
(`hSkipStmt`/`hSkipUntil`, about 214 call sites), and a context stack that turns a type error into
"at parameter 2 of F()". *For us:* a simple starting policy for M2 recovery and for the "several errors per
run" decision; our lossless tree additionally keeps the skipped tokens.

**L7. Dialects as feature flags.** `-lang fb|fblite|qb|deprecated` is a bit set (`GOSUB`, `SUFFIX`, `DEFTYPE`,
`IMPLICIT`, `PERIODS`, `NUMLABEL`, `ONERROR`, `METACMD`, …); the parser tests the feature, never the dialect
name. *For us:* model `OPTION _EXPLICIT`, `$DYNAMIC` and any future strict mode (`08`) as one options value
passed to the front end, not as scattered globals.

**L8. Test conventions.** Every test file starts with `' TEST_MODE : COMPILE_ONLY_OK | COMPILE_ONLY_FAIL |
COMPILE_AND_RUN_OK | COMPILE_AND_RUN_FAIL | MULTI_MODULE_TEST`, so the runner needs no side files. Literal
typing is tested by compile-time type assertions (`ASSERT_TYPE(32768, long)`). `tests\qb` (about 80 files) is a
list of dialect corners: literal types and sizes, every combination of REDIM with suffix / DEFtype / AS type,
COMMON with dynamic arrays, code before the first CASE, MKx/CVx, RND, OPEN on `SCRN:`/`CONS:`/`LPT:`/`COM:`.
*For us:* use a mode line in new tests; add a test-only way to assert the type of an expression (typed-AST dump)
for the lexical rules in `00` §4; use the `tests\qb` file *names* as a checklist and write our own programs,
with `qb64pe.exe` giving the expected result. The test files themselves are GPL and are not copied.
Also: compiler messages are tested against recorded text files (`tests\warnings\r\<platform>\*.txt`); the same
snapshot approach suits our new error messages.

**L9. Runtime layering.** Console, graphics and files meet through three small tables of function pointers:
`FB_FILE_HOOKS` (a file, `SCRN:`, `CONS:`, `LPT:`, `COM:` are devices behind one OPEN), `FB_HOOKSTB` (PRINT,
INKEY, LOCATE, … are swapped when a graphics mode starts, so the graphics library is optional) and `GFXDRIVER`.
Strings are a three-word descriptor (data, length, capacity). *For us:* input for M6 only; libqb stays as it is
until then (R1, R7).

**L10. `#line` in generated C.** With `-g` the C back end writes `#line` directives, so gdb shows BASIC lines without a
custom protocol. *For us:* check this against the M5 plan (debug symbol file + DAP adapter, `06`); it may make
stepping and breakpoints cheap, while variables still need our own mapping.

**L11. A second opinion on QuickBASIC behaviour.** FreeBASIC has independent implementations of QB's RND
(the same 24-bit generator), PRINT USING, MBF conversions and literal typing. When a QB64pe behaviour looks
accidental (`00` §6), `fbc -lang qb` and the QuickBASIC manuals are two further data points for the
"keep or fix" decision. `qb64pe.exe` remains the answer for what we implement.

**L12. Lowering documented as before/after pairs.** FreeBASIC's developer notes describe SELECT CASE by showing
the source next to its lowered form (temporary, compare-and-jump per CASE, end label, string temporaries freed at
scope end), including the special cases. *For us:* write each lowering (SELECT CASE, FOR, PRINT, single-line IF)
as such a pair in the spec and keep the pairs as IR snapshot tests. Side note: their SELECT temporary becomes
STATIC inside a STATIC procedure, the same construction behind QB64pe's "static SELECT CASE temporaries
overwritten by recursion" (`00` §6).

## 4. Things not to take

**A1. Single pass with a symbol-aware lexer.** The token carries the symbol chain found for its text, and the
preprocessor runs inside the lexer. The AST has no nodes for IF, DO/LOOP, EXIT or GOTO: the parser lowers control flow to labels and branches while
parsing (their own developer notes say so). No tree keeps the source text, so there is no formatter and no analysis
without compiling. R5 (lossless tree, then resolution, then typing) is the better design for a language server.

**A2. Forward references.** In `-lang qb`, `CALL name(...)` for a procedure not yet seen goes to `hForwardCall`,
which guesses the signature from the arguments; without `CALL` a DECLARE is needed. QB64 programs call SUBs defined later without DECLARE; that needs signatures
collected before bodies are checked, which our resolution pass does.

**A3. Global state and manual memory.** Compiler state lives in globals (`env`, `parser`, `lex`, `symb`);
`FBSYMBOL` is one record with a union for every symbol kind; lists and pools are hand-made; token text sits in fixed buffers (length limits), and allocation failure is not
handled. Rust enums and
passed-in context replace all of that.

**A4. An IR shaped for assembly, and four back ends.** The IR is three-address code with virtual registers. The
C back end's own header says it therefore emits low-level, ABI-dependent C: labels and jumps instead of
`if`/`else`, struct layout computed by the compiler. `todo.txt` shows the cost of maintaining four back ends (ABI
differences between them, features missing in one). *For us:* one back end; keep structured control flow in the
IR and lower to labels only where BASIC needs it (GOTO, GOSUB, RESUME).

**A5. GOSUB by setjmp/longjmp** in the C back end (a heap-allocated `jmp_buf` per GOSUB); its author notes it is far slower than the assembly
version.

**A6. Self-hosting** and its bootstrap. Already excluded by the Rust decision.

## 5. Is it more modular than QB64pe?

In structure, yes, as far as read: separate lexer, parser, symbol table, AST and IR; files of manageable size;
runtime split per function and per platform; tables instead of string mini-languages; a large test suite. The two
projects grew under different constraints (QB64pe's compiler is self-hosted in one BASIC file and has to stay
buildable by itself). FreeBASIC is a single-pass design from 2004, built for batch compilation, so it is not a
model for a compiler that must also serve an editor. Our planned
architecture already goes further in the front end; FreeBASIC is most useful for the **middle and back**:
runtime-call tables (L2), error/resume lowering (L3), defined C output (L4), and test conventions (L8).

## 6. Proposed follow-ups (for the user to accept or drop)

| # | Proposal | When |
|---|---|---|
| 1 | **Decide LONG overflow:** wrap, via `-fwrapv` or unsigned arithmetic (matches the default build; panel recommends). Added to the "Decide" list in `00` §6 | before M3 codegen |
| 2 | Built-in table gets a "cannot raise" flag per entry | M3 |
| 3 | Test files carry a mode line; typed-AST assertions for literal typing; snapshot tests for diagnostics and for lowering pairs (L12) | M2 |
| 4 | Recovery policy: one error per statement, skip to statement end, error cap | M2 |
| 5 | Evaluate `#line` directives for the debugger | M5 |
| 6 | Read FreeBASIC's file-device and console-hook design before redesigning libqb | M6 |

## 7. Panel review and second pass (2026-10-03)

Written as a panel discussion, inline, no subagents. Roles: compiler architect (A), runtime engineer (R),
compatibility and testing (T), editor tooling (E), licensing (L).

- **T on L4:** "Undefined behaviour" was asserted from a missing flag. Measure it. → Done: `v11_wrap_o2` shows the
  old compiler gives different results with and without `-O2`. The first draft was also wrong to include
  `32767 + 1`: INTEGER arithmetic is not undefined. L4 corrected and raised from a note to a decision item.
- **R on L3:** FreeBASIC's scheme is opt-in and per runtime call; QB's RESUME is per statement. Calling it a
  "candidate model" oversells it. → Demoted to an illustration; the three transferable points are listed.
- **R on L2:** A "can raise" flag cannot be extracted mechanically; in libqb almost everything can call
  `error()`. → Inverted: default "may raise", flag the few known-pure functions by hand.
- **A on A2:** The forward-call guess is only reached through `CALL`. → Corrected.
- **A, second pass:** FreeBASIC's developer notes confirm the AST holds no control-flow nodes and the lexer does
  symbol lookup. This strengthens A1/A4: nothing in its front end fits R5. Its before/after lowering notes are
  worth imitating. → L12 added.
- **E on L6 and L10:** One error per statement fits the diagnostics policy in `15` (report only what the old
  compiler reports, at first). `#line` appears only with `-g` and helps stepping, not variable display; keep it
  as an M5 evaluation, not a plan change.
- **T on L8:** Add snapshot tests for message texts, as FreeBASIC does for warnings. → Added.
- **L:** The developer notes are FDL, the tests GPL: same rule, read but do not copy. Lowering pairs and tests
  are written here from QB64pe behaviour.

**Verdict.** The overall conclusion stands: FreeBASIC is more modular than QB64pe, is not a front-end
model, and is useful for the middle and back. Changed: one finding became a measured decision item (LONG
overflow), two claims were corrected (INTEGER wrap, forward calls), one lesson was demoted (L3), one inverted
(L2 flag), one added (L12). Priority of the follow-ups: 1, then 3; the rest are notes for their milestone.

Rule: FreeBASIC is reference only (`CLAUDE.md` rule 5). Reading how a problem was approached is fine; no code,
tests, tables or documentation text are copied.

Not consulted: the earlier report `reports\FreeBASIC and QB64pe codebases.md` in `<qb64contain>\QB64pe`
(decision of 2026-10-02: no more material from there is brought into `study\`).
