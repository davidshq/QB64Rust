# 07 — Expert panel: how to approach the QB64pe rewrite

**What this is.** A structured design review written by Claude in one pass (no subagents), staged as a discussion
between eleven specialist viewpoints. The panelists are perspectives, not real people. Every factual claim they make
comes from the study documents `00`–`06`; where a claim is uncertain, the panel says so and turns it into a
verification task.

**Inputs already fixed by the user:** ground-up rewrite; no port of the text-mode IDE; feature parity through a
VS Code extension.

## Panel

| Role | Short | Brings |
|---|---|---|
| Pragmatic engineer | PRAG | Shipping, scope control, "what is the smallest thing that works" |
| Compiler expert | COMP | Front end, IR, code generation |
| Programming-languages specialist | PL | Semantics, specification, type system |
| Testing engineer | TEST | Oracles, conformance, differential testing |
| Refactoring engineer | REF | Incremental change of legacy code |
| Modernization expert | MOD | Legacy-replacement strategy, risk, delivering value early |
| QBasic / QuickBASIC engineer | QB45 | QB4.5 / PDS 7.1 behaviour, DOS-era programs |
| QB64 engineer | QB64 | QB64 / QB64pe extensions, community code, `$` features, DECLARE LIBRARY |
| Runtime and systems engineer | RT | libqb, threading, graphics/audio, platform layer |
| Developer-tools engineer | DX | Language server, debug adapter, VS Code |
| Build and release engineer | BLD | Toolchains, bootstrap, CI, packaging |

---

## Session 1 — Should this be a rewrite at all?

**REF:** I'll take the contrarian seat. The usual advice is never to rewrite. QB64pe works, it has users, and its
behaviour is the specification. Why not refactor in place?

**COMP:** Because the compiler has no structure to refactor toward. There is no AST. A line becomes a CHR$(13)-separated
string, every statement handler parses, type-checks, emits C++ and builds the formatter output in one step, and late
facts trigger a restart of both passes (six different triggers, `01` §2.3). You can't extract a type checker from code
where type checking is interleaved with string surgery and `GOTO`. The first 14,000 lines of `qb64pe.bas` are a single
module.

**REF:** Agreed for the compiler. The runtime is a different story. `libqb\src` already holds about 24,500 lines of
modular code, upstream has been splitting `libqb.cpp` for years, and the vendored libraries (GLFW, miniaudio, FreeType)
are fine. Rewriting the runtime from scratch throws away bit-exact behaviour in `STR$`, PRINT USING, RND, rounding,
the rasterisers.

**MOD:** So the honest framing is: **rewrite the compiler, refactor the runtime, replace the IDE.** Three different
strategies for three parts, because they are in three different states.

**PRAG:** And sequence them so something useful ships before the compiler is done. Which brings me to the IDE.

**MOD:** The VS Code extension can ship first against the *old* compiler. `qb64pe -c` / `-x` gives errors in a fixed
text format, `-y` formats, `-z` emits C++, and F5 is just "compile and run". That delivers editor value in weeks and
decouples it from the rewrite. When the new compiler lands, the extension swaps its back end.

**DX:** With one caveat: the old compiler reports one error per run, line granularity, no columns. Diagnostics will
be crude until the new front end exists. That's acceptable for a first release.

**Consensus:** rewrite the compiler; refactor the runtime behind a stable boundary; build the VS Code extension early
against the existing compiler.

---

## Session 2 — What behaviour is "correct"?

**QB45:** I want this settled before anyone writes a parser. QB64pe already diverges from QuickBASIC 4.5 in places
people notice: integer arithmetic doesn't raise overflow (QB4.5 raises error 6), integer division by zero is fatal and
not trappable (QB4.5 lets `ON ERROR` catch error 11), SINGLE literals are emitted as C doubles, `DEF FN` isn't
implemented, ON COM / ON PEN / ON PLAY are reserved words only.

**QB64:** And the people who actually run QB64pe today depend on *QB64pe* behaviour, not QB4.5. Thirty years of forum
code, `$INCLUDE` libraries, games that rely on `_NEWIMAGE`, `_PUTIMAGE`, `_MEM`, `DECLARE LIBRARY` with C headers.
If the rewrite "fixes" integer overflow to raise error 6, programs that relied on wraparound break, and every
integer `+` gets slower.

**PL:** Then we need a written specification, because "whatever the old compiler does" is not something you can
implement against. Especially numerics. The study found that the compiler *labels* `INTEGER + INTEGER` as 64-bit but
emits no cast, so C++ computes it in 32-bit `int` (`02` §1.4). That is three layers of meaning: what QB4.5 did, what
the compiler believes, what the C++ actually does. The spec has to pick one, and for compatibility it has to be the
third.

**QB45:** I'd add one thing to verify rather than trust. The codegen study says `int / int` is `long double`, so
`PRINT 1 / 3` prints more digits than QB4.5's `.3333333`. I'm not convinced that's what users see: the typing and
the printing path may round it back. That needs running, not reading.

**TEST:** That's exactly the kind of claim the baseline phase settles. Run it.

**QB64:** The other compatibility question is the fork. This tree has TYPE member arrays, `REDIM _RETAIN`,
`_ARRAYCOPY`, whole-array assignment, `$USELIBRARY`, `$ERRORLOCATION`, and the GLFW runtime. The remote is
`davidshq-contribute`. If those are the user's own additions, they are requirements. If they are someone else's
experiments, they are scope.

**PRAG:** Either way, nobody has diffed against upstream yet. That's a one-day task and it decides a lot: 211 of the
404 compile tests are in `arrays`, much of it the member-array layer.

**PL:** Proposal for the compatibility policy:
1. **Baseline = observed QB64pe behaviour** of a pinned commit, measured by running programs, not by reading code.
2. A **divergence register**: every place where the new implementation intentionally differs, with a reason.
3. Accidental behaviour that corrupts memory or crashes gets fixed and registered (e.g. `_BIT*33` scalars get 4 bytes
   for an 8-byte value, `02` §9.2). Accidental behaviour that only changes results stays until someone decides
   otherwise.

**QB45:** I can live with that if QB4.5 divergences are listed in the register too, so a future "strict QB4.5 mode"
has a starting point.

**Consensus:** compatibility target is observed QB64pe behaviour at a pinned commit, plus a divergence register.
Fork features are in or out depending on the upstream diff and the user's answer.

---

## Session 3 — Architecture: front end, IR, back end

**COMP:** Standard pipeline: lexer → parser with error recovery → AST → name resolution and type checking → typed IR →
back end. The six restart triggers disappear because each becomes a fact computed before code generation: "does any
code take `VARPTR` of this variable", "how many dimensions does this array parameter have", "is this `name:` a label
or a SUB call". All are whole-program queries over a resolved AST.

**PL:** BASIC has some parsing hazards worth calling out early, because they shape the AST:
- `name:` is ambiguous between a label and a parameterless SUB call followed by `:`.
- Periods in names: `a.b` is a UDT member only if `a` is a UDT variable *at that point*; otherwise it is one
  identifier. Resolution needs symbol information during parsing, or a parse that keeps both readings.
- Single-line IF with nested ELSE, `THEN 100`, `ELSE 20`.
- Suffixes are part of identity: `x%`, `x&`, `x$` are three variables; `musthave` / `mayhave` rules.
- DEFxxx is position-dependent; `'$DYNAMIC` applies from the next line.
- `$IF` is evaluated textually, left to right, no parentheses.

**COMP:** Fine, a concrete syntax tree that keeps tokens and trivia (comments, spacing), with a resolution pass that
can rewrite `a.b` nodes. The same tree drives the formatter, which is what the DX side needs anyway.

**DX:** Yes. A lossless syntax tree is the single most important decision for the language server: formatting,
semantic tokens, outline, go-to-definition and incremental reparse all come from it.

**COMP:** Back end. Options: (a) emit C++ against the existing libqb ABI, (b) emit C++ against a new runtime ABI,
(c) LLVM or our own native back end, (d) interpreter.

**QB64:** (c) and (d) break `DECLARE LIBRARY`. Users write `DECLARE LIBRARY "myheader"` and get a C/C++ `#include`
compiled into their program. CUSTOMTYPE libraries, static `.a` files resolved with `nm`, `SUB _GL` calling OpenGL
through generated wrappers. All of that assumes the program is C++ compiled with the runtime. A non-C++ back end has
to reimplement that interop or drop it.

**PRAG:** And (a) gives a running program on day one of code generation. Every runtime function, every bit-exact
behaviour, every vendored library, for free. The new compiler only has to produce the same calls.

**COMP:** My objection to (a) is the ABI itself: every variable is a pointer, strings are `qbs*` on a moving heap,
optional arguments use a `passed` bitmask whose bit allocation comes from a backtracking interpreter of the
`specialformat` mini-language (`02` §4.3). Binding a clean compiler to that ABI forever would be a mistake.

**RT:** Not forever. Bind to it first, then move it. Once the new compiler is the only producer of calls into libqb,
the ABI becomes internal and can change on both sides at once. While the old compiler is still the oracle, it can't.

**COMP:** Then the IR must make the ABI a back-end concern. Optional arguments in the IR are "present / absent", not
bitmasks. Arithmetic in the IR carries explicit types and explicit conversions; the C++ emitter is what reproduces
today's promotion behaviour, by emitting the operand C types the old compiler would have produced.

**PL:** Which means the numeric rules live in one place, the spec plus the IR lowering, instead of being implicit in
C++. That's the right inversion.

**QB64:** One more piece of existing contract: the auto-included BASIC files (`beforefirstline.bi`, `aftermain.bas`,
`afterlastline.bm`, color constant files, the `vwatch` debugger) and the `_IKW_` prefix that turns a BASIC routine
into a built-in keyword. Part of the language is implemented in BASIC. The new front end has to compile those files.

**COMP:** Good: that's a cheap way to implement part of the standard library. Keep the mechanism, maybe with a
clearer name than `_IKW_`.

**Consensus:**
- Lossless syntax tree → resolved, typed AST → typed IR with explicit conversions → C++ emitter.
- First back end targets the **existing libqb ABI and `qbx.cpp`**; the IR does not encode that ABI.
- Built-in table converted from `subs_functions.bas` into data (mechanically), with the `specialformat` grammar parsed
  once into a structure.

---

## Session 4 — Implementation language and self-hosting

**QB64:** The community will ask first: is it still written in QB64? Self-hosting is part of the project's identity,
and contributors know BASIC, not Rust.

**BLD:** Self-hosting is also why `internal\source` exists: 2,079 generated files committed by a CI bot after every
merged PR, and a fixed-point bootstrap that has to be seeded somehow. A new self-hosted compiler needs its own seed
strategy, and it can't self-host until it compiles a large BASIC program correctly, which is late in the project.

**DX:** And the language server wants a compiler written in a language with good concurrency, incremental computation
and an LSP library. Writing an LSP server in QB64 means building JSON-RPC, threading and incremental parsing in
BASIC.

**PRAG:** Pick what the person doing the work is fastest in. We don't know that yet. Practical candidates:
- **Rust:** strong for compilers and language servers (tower-lsp, salsa-style incremental computation), single static
  binary, no runtime. Steeper learning curve.
- **C++:** same language as the runtime, which helps if one person works on both. Weaker LSP tooling, more ways to
  write memory bugs.
- **TypeScript:** shares a language with the VS Code extension; slower and a heavier CLI distribution.
- **C# / Go:** good middle grounds; GC; acceptable LSP libraries.

**COMP:** Rust is my recommendation if there's no strong preference. Pattern matching over ASTs and enums for IR is
exactly what compilers want.

**QB64:** Then plan self-hosting as a *later option*, not a requirement. If the new compiler eventually compiles
`qb64pe.bas` itself, that's the best possible conformance test, not a build dependency.

**Consensus:** not self-hosted initially. Implementation language is the user's call; panel default is Rust.
"New compiler compiles the old `qb64pe.bas`" becomes a milestone test, not an architecture constraint.

---

## Session 5 — Testing: the oracle is the old compiler

**TEST:** Nothing gets designed until we can run the existing suite on Windows. Today the runners are bash scripts
and nothing has been built locally. Step one: build the current qb64pe (`setup_win.cmd` downloads llvm-mingw), write
a PowerShell or Python runner following the rules in `05` §5.7, and record a baseline of which tests pass on this
machine.

**TEST:** Then four layers:
1. **Existing conformance tests.** 331 expected-output tests are black-box; 357 of the 404 programs are
   `$CONSOLE:ONLY`, so stdout comparison works. The 56 `.err` tests compare exact error text; use them loosely
   (must fail, right line) until the error-message policy is decided.
2. **Golden corpus from the old compiler.** The 143 `qbasic_testcases` only check that they compile. Many can run
   headless. Generate expected output with the old compiler and freeze it.
3. **Differential testing.** Generate random expressions and statements across all numeric types, compile with old
   and new, compare printed results. This is the only practical way to pin down the numeric semantics the PL
   specialist described, including `qbr` banker's rounding and float comparison narrowing.
4. **Formatter golden tests.** Run the old compiler's `-y` over every `.bas` in the repo (thousands of lines of real
   code, including `qb64pe.bas` itself) and freeze the output. The new formatter must match byte for byte, or the
   difference goes into the divergence register.

**PL:** Layer 3 should feed the spec. Every divergence found becomes either a spec rule or a register entry.

**QB45:** Add classic DOS programs to the corpus. Most of the 143 are QB64-era. PRINT USING, file modes, INPUT #,
string functions and DATA/READ are only lightly tested today (`05` §5.1). Those are what old QuickBASIC code uses.

**RT:** And graphics: there are only 12 image-comparison tests. `AssertImage` in `tests\compile_tests\utilities`
already saves BMPs headless. Add golden images for LINE, CIRCLE, PAINT, DRAW, GET/PUT in every screen mode. If the
runtime is kept, these are regression tests for its refactoring; if parts are rewritten, they are the spec.

**TEST:** Last point: the suite runs against the old compiler in CI forever. If a golden test changes, someone has to
explain why.

**Consensus:** Windows harness and baseline first; four test layers; old compiler kept as a permanent oracle.

---

## Session 6 — Tooling, VS Code, and encoding

**DX:** Requirements on the compiler, beyond the obvious: cancellable, incremental analysis (the old IDE restarted a
full two-pass compile on every keystroke); error recovery so there can be more than one diagnostic; column
positions; a formatter that can run on a range; a stable symbol model for outline and go-to-definition; a debug
symbol file for the debug adapter.

**DX:** On encoding. Old sources contain CP437 bytes in string literals (box-drawing characters, programs depend on
them at run time). The old IDE showed raw bytes through a selectable code page. VS Code can open a file in a chosen
encoding per language, so the extension can default `.bas`/`.bi`/`.bm` to CP437. The compiler then keeps reading
bytes, nothing converts, and old files round-trip unchanged. *To verify:* that VS Code's encoding list includes
CP437 and that it round-trips all 256 bytes.

**QB64:** UTF-8 sources exist too; people use `_UPRINTSTRING` with UTF-8 literals. So it has to be a per-file choice,
not global. A `'$ENCODING` comment or a setting.

**PRAG:** Default CP437, let users switch per file in VS Code. Don't make the compiler convert anything in the first
version.

**DX:** Debugger: DAP covers breakpoints, stepping, set next statement (`goto`), watches, watchpoints as data
breakpoints, call stack. "Skip lines" has no DAP equivalent; I'd drop it unless users ask. The old wire protocol has
known defects (frame length off by 4, reversed string watchpoint comparison), so don't reuse it.

**Consensus:** compiler-as-library with incremental analysis; CP437 default encoding per file; DAP debugger later.

---

## Session 7 — The runtime

**RT:** What I'd change first, and what I wouldn't touch:
- **Leave alone:** vendored libraries, `libqb\src` modules, the bit-exact algorithm units (`rounding.h`,
  `qbs_str.cpp`, `qbs_val.cpp`, PRINT USING, RND, number parsers).
- **Fix early, low risk:** the `#define int32` type macros leaking into every file; the 892 `static` locals that make
  functions non-reentrant (they matter because `SUB _GL` and event handlers re-enter the runtime).
- **Redesign later, only when the new compiler owns the ABI:** the error model (`error()` returns and every function
  early-outs; a trapped error calls `QBMAIN` recursively and never unwinds), the moving string heap, thread
  synchronisation built on non-atomic flags.

**MOD:** The error model is the big one. It's woven into the generated code (`do{…}while(r)` wrappers,
`||is_error_pending()` on every condition, `RESUME` re-executing a statement). Changing it changes the IR lowering.

**COMP:** Which is fine as long as the IR models "statement may raise; resume point here" explicitly. Then the C++
emitter can produce today's wrappers, and a later runtime can use a different mechanism, without touching the front
end.

**QB45:** Keep the observable behaviour: `RESUME` retries the statement, `RESUME NEXT` continues after it, errors in
an IF condition fall into the block. Programs depend on all three.

**Consensus:** keep libqb as the runtime for the first compiler milestone; refactor it in place with golden tests;
defer the error-model and string-heap redesign until the ABI is owned by the new compiler.

---

## Session 8 — Roadmap

**PRAG:** Milestones, each one useful on its own:

| # | Milestone | Done when |
|---|---|---|
| M0 | **Baseline** | Old qb64pe builds on Windows; PowerShell/Python test runner; baseline results recorded; upstream diff done and fork features classified; remaining study gaps closed (DIM/REDIM/COMMON, PRINT/INPUT emission, built-in table) |
| M1 | **VS Code extension v0 on the old compiler** | Syntax highlighting, build/run tasks, diagnostics parsed from `-c` output, formatting via `-y`, CP437 default encoding |
| M2 | **Front end** | Lossless parser with recovery; name resolution; type checker; formatter matching `-y` on the golden corpus; language server serving diagnostics, formatting, outline, go-to-definition |
| M3 | **Code generation to the existing ABI** | New compiler emits `qbx.cpp` fragments and links with libqb; passes the existing expected-output tests and the differential tests |
| M4 | **Parity** | All 143 corpus programs compile and match golden output; fork features per M0 decision; new compiler compiles `qb64pe.bas` (stretch) |
| M5 | **Debugger** | Debug symbol file + DAP adapter |
| M6 | **Runtime modernisation** | ABI owned by the new compiler; error model, string heap, threading redesigned module by module behind golden tests |

**MOD:** M1 is the risk reducer. If the rewrite stalls, the user still has a modern editor for QB64pe.

**COMP:** M2 before M3 is right: a front end with a formatter and a language server is shippable and exercises the
whole parser on real code before code generation exists.

**BLD:** Pin the Windows toolchain (the setup script downloads the *latest* llvm-mingw with no checksum, `05` §1). For
the new project, pin a version and verify a hash, or the C++ side of builds will drift.

**TEST:** And M0 has to come first. Every later milestone is measured against it.

---

## Recommendations

| # | Recommendation | Agreement |
|---|---|---|
| R1 | Rewrite the compiler; refactor the runtime in place; replace the IDE with a VS Code extension | Consensus |
| R2 | Compatibility target is **observed** QB64pe behaviour at a pinned commit, plus a divergence register. Fix memory-corrupting accidents; keep result-changing ones until decided | Consensus; QB45 asks that QB4.5 divergences be registered too |
| R3 | Before design, run M0: build the old compiler on Windows, write a Windows test runner, record a baseline, diff against upstream QB64-PE, close the three study gaps | Consensus |
| R4 | Ship a VS Code extension early (M1) on top of the **old** compiler | Consensus |
| R5 | Architecture: lossless syntax tree → typed AST → typed IR with explicit conversions and explicit error/resume points → C++ emitter | Consensus |
| R6 | First back end targets the existing libqb ABI and `qbx.cpp`; the IR must not encode that ABI. Change the ABI only when the old compiler is no longer a producer | Consensus; COMP wants an ABI redesign as soon as M4 is reached |
| R7 | Keep a C++ back end for as long as `DECLARE LIBRARY` with C/C++ headers and `SUB _GL` are supported | Consensus |
| R8 | Write a numeric-semantics specification driven by differential testing against the old compiler | Consensus |
| R9 | Convert the 455-entry built-in table from code to data mechanically; parse `specialformat` once | Consensus |
| R10 | Not self-hosted initially. Default implementation language: Rust, unless the user is markedly faster in another | Majority; QB64 dissents on community grounds and asks that self-compilation of `qb64pe.bas` stay a milestone |
| R11 | Four test layers: existing suite, golden corpus, differential fuzzing, formatter goldens; plus golden images for graphics | Consensus |
| R12 | Default `.bas` encoding CP437 in VS Code, per-file override; the compiler reads bytes and does not convert | Consensus, pending a VS Code CP437 round-trip check |
| R13 | Pin and checksum the Windows C++ toolchain | Consensus |

## Decisions for the user

1. **Are the fork features yours?** (TYPE member arrays, `REDIM _RETAIN`, `_ARRAYCOPY`, whole-array assignment,
   `$USELIBRARY`, `$ERRORLOCATION`, GLFW runtime.) If yes, they are requirements. If not, the upstream diff in M0
   decides.
2. **Implementation language.** Panel default Rust. What are you fastest and most comfortable in?
3. **Self-hosting.** Is it a goal for the new compiler, or only a nice-to-have test?
4. **Strict QB4.5 mode.** Wanted at some point (overflow errors, trappable divide by zero), or out of scope?
5. **Error messages.** Keep today's exact texts (the 56 `.err` tests depend on them) or design new ones with
   columns and multiple errors?

## Verification tasks raised by the panel

| Claim | Raised by | How to check |
|---|---|---|
| `PRINT 1 / 3` prints more digits than QB4.5 | QB45 | Run it in M0 |
| VS Code round-trips all 256 CP437 bytes | DX | Open/save a byte-table file |
| Fork features are absent upstream | QB64 | Diff against upstream QB64-PE |
| The five "probable defects" in `01` §11.3 and the ~25 accidental behaviours in `02` §9.2 | TEST | One small test program each, in M0 |
