# 12 — Expert panel: should we use any of QB64Fresh, and how?

**What this is.** A design review written by Claude in one pass (no subagents), staged as a discussion between the
panel of `study\07`. The panelists are perspectives, not real people. Facts come from reading QB64Fresh and from
building and running it on 2026-10-02; each is listed under "Evidence".

**Verdict:** QB64Fresh's compiler, runtimes and formatter are not a base for this project. Take its test programs,
its LSP and debugger *designs* as references, and selected small pieces of code (listed in "How to use it").
Its user-facing tooling (`vscode-qb64fresh`, see `study\11`) is the most reusable part.

## Subject

| | |
|---|---|
| Location | `<qb64contain>\QB64Fresh` (GitHub `davidshq/QB64Fresh`, MIT) |
| State reviewed | Local branch `fixing-reapply` at `56ea996` (2026-02-04), cloned clean to a scratch directory. The working copy also has 12 uncommitted files, not reviewed. GitHub `main` stopped at `befcd09` (2026-01-18); other branches `broken`, `fixing`, `language-features`. |
| History | 248 commits, 2026-01-16 to 2026-02-04. README: "**VIBE CODED: Use with caution.**" 156 session logs in `AgenticLogs\`. |
| Size | About 135k lines of Rust: codegen 29.6k, Rust runtime 26.9k, semantic analysis 21.8k, parser 11.7k, tests 13.4k, debugger 7.9k, lint 2.8k, header parser 2.7k, LSP 3.0k, lexer 2.0k, fmt 1.8k. Plus 14k lines of docs, 18 ADRs. |
| Pipeline | `.bas` → preprocessor (text) → logos lexer → recursive-descent parser → AST → semantic checker → typed IR → C emitter → gcc (run by the user). Runtime: either about 11k lines of C emitted from Rust strings ("inline") or the Rust runtime crate ("external", SDL2/rodio). |

### An earlier review exists

`<qb64contain>\QB64pe\docs\rewrite-decision.md` (untracked, written 2026-10-02 13:54–14:03, before this
repo's studies began) reports a five-reviewer assessment of QB64Fresh. Its main results: output matched QB64pe on
22 of 331 expected-output tests (6.6 %); tests are shallow; typed IR is name-based; the bootstrap claim is
unconfirmed. Its recommendation was to keep QB64Fresh's front end, freeze the rest, and **not** start a new rewrite
from an empty repository. This project then decided on a ground-up rewrite (`DECISIONS.md`, 2026-10-02) without
referring to it. This review checks that document's key claims independently and answers the question for this
project's architecture (`study\07` R5–R12), which differs from what that document assumed.

## Evidence

All runs on Windows 11, Rust 1.88, MSYS2 gcc 15.2, debug build. Scripts and raw results are in `study\archive\qb64fresh-scripts\` (`measure.sh`, `measure.tsv`, `fmtcheck.py`,
`lsp_smoke.py`). They take their locations from environment variables (`QBF_WORK`, `QBF_BIN`, `QB64PE`; see each
script's header); the run used a scratch directory with QB64Fresh cloned to `qbf\` and built into `qbf-target\`.

| # | Finding |
|---|---|
| Q1 | `cargo build --workspace` passes (2 min 42 s, warnings only). |
| Q2 | `cargo test`: the runtime crate's tests **do not compile** (128 errors: functions such as `qb_print_int`, `qb_net_close` missing). Without it: about 1,380 tests pass, 40 fail: `execution_tests` 4 pass / 24 fail, `golden_tests` 2 / 8 (generated C differs; partly line endings), `bootstrap_tests` 2 fail, `compatibility` 2 fail, debugger 3 fail, one more unit target 1 fail; `proptest_tests` aborts with a stack overflow on a long line. |
| Q3 | Our `verification\` programs (outputs recorded from `qb64pe.exe`): 1 of 14 identical (`v07`); 9 run with different output; 4 are rejected — two use deliberately odd syntax (`v04`, `v06`), but `v09` fails on `a(1) = a(1) + 1` for a SINGLE array ("operator + cannot be applied to SINGLE() and LONG") and `v10` on screen `WRITE` without `#`. |
| Q4 | Probes: `PRINT 1 / 3` prints `0` (constant folded as integer division); `x / y` prints `0.333333` (QB64pe ` .3333333 `); `PRINT 1; 2` prints `12` (QB64pe ` 1  2 `); `7.5 \ 2` gives `3` (QB64pe `4`). Integer division by zero exits the process with code 127 instead of reporting the error. |
| Q5 | QB64pe test programs (547): output matches QB64pe on **19 of 331** expected-output tests (5.7 %); see "Measurement" below. |
| Q6 | Formatter `qb64fresh-fmt` over the 547 programs: 510 keep all strings, comments and tokens; 25 are refused, mostly "stream did not contain valid UTF-8" (CP437 sources); 9 change tokens (joins `_` continuation lines); 3 change comments. It inserts a space after `'`, appends a "Summary" block to `--stdout` output, and indents wrongly (code after `END TYPE` indented one level; `IF` bodies not indented). |
| Q7 | LSP `qb64fresh-lsp`: starts; for semantic errors it reports several at once with correct ranges and suggestions; hover shows the inferred type. A single parse error hides all other diagnostics. An error inside a `$INCLUDE`d file is not reported (rechecked in session 5 with `lsp_smoke.py` collecting every `publishDiagnostics` message for any URI for 5 s: none for `inc.bi`). |
| Q8 | Lexer discards whitespace and line continuations (`logos(skip)`); comments are tokens but spacing is lost. 277 keywords are distinct token kinds and the AST has about 288 statement kinds: most built-in statements are hard-coded into the grammar (`parser\graphics.rs`, `audio.rs`, `file_io.rs`). Several metacommands are single regex tokens (`$RESIZE:ON`); `$CONSOLE` patterns are commented out with "causes logos bug". |
| Q9 | Preprocessor joins continued lines and pastes `$INCLUDE` files into one string before lexing (`src\preprocessor.rs:462-530`). `Span` is byte offset + line with no file id (`src\ast\mod.rs:47`). Positions in included files and after continuations cannot be mapped back exactly. |
| Q10 | Typed IR refers to variables, arrays and procedures by name string (`src\semantic\typed_ir.rs:55`, `:83-113`, `:545`); no symbol ids. Codegen re-derives scoping (`analysis.rs` 1.4k lines, `implicit_vars.rs`) and contains special cases for `qb64pe.bas` (`HASHFLAG_`, `DEPENDENCY_`, `qberror_test`, `regid`: 9 sites). |
| Q11 | Documented intentional differences from QB64pe (`docs\QB64pe\QB64Fresh_VS_QB64pe_DIFFERENCES.md`): cross-procedure `GOTO` is an error, different `RND` sequence, `RUN` without arguments is a no-op, `_GL*` excluded, DOS features stubbed. |
| Q12 | Assets with value independent of the compiler: `tests\runtime_comparison\` (261 one-behaviour console programs, no recorded outputs); `docs\QB64pe\` (language spec 87 kB, architecture, IDE checklist — machine-written, unverified); 18 ADRs; `vscode-qb64fresh`; the earlier review's harness (`QB64pe\docs\analysis-scripts\rewrite-panel\`). |

### Measurement

Every `.bas` under `..\QB64pe\tests\compile_tests` and `qbasic_testcases` (547) was run three ways: parse only
(`--ast`), parse + check (`--typed-ir`), and for expected-output tests, emit C (`--emit-c --headless`), compile with
gcc, run, and compare with the `.output` file (all CRs and trailing blank lines ignored; looser than QB64pe's
runner, which only drops trailing line endings).
60 s timeout per stage; one timeout, no crashes.

| Set | Programs | Parses | Passes checker | Emits C | C compiles | Output matches |
|---|---|---|---|---|---|---|
| `compile_tests`, expected to compile | 348 | 144 | 121 | | | |
| … excluding `arrays\` (UDT member arrays, added to QB64pe after QB64Fresh stopped) | 173 | 128 (74 %) | 109 (63 %) | | | |
| `qbasic_testcases` (QB4.5-era programs) | 143 | 131 (92 %) | 122 (85 %) | | | |
| Expected-output tests | 331 | | 107 | 107 | 78 | **19** |
| `.err` tests (should be rejected) | 56 | | 48 rejected | | | |

Most common parse errors: "expected AS in type member definition" (88, the new `AS LONG a(12)` member syntax),
"unexpected token Dot" (48), then a long tail. The 19 matches are mostly programs that print a fixed string
(`glut\title`, `source_ordering\*`, `auto_include\*`). This agrees with the earlier review (22 of 331, measured on
the uncommitted working copy).

## Session 1 — What does QB64Fresh actually do?

**TEST:** The headline numbers in its README are not measurements of behaviour. "99.1 % QB4.5 compatible" means C
was emitted (the earlier review read the test; our run agrees in spirit). What we measured ourselves: one of
fourteen verification programs gives the same output as QB64pe, and `PRINT 1 / 3` prints `0` (Q3, Q4). Those are
not edge cases; they are the first things any BASIC program does.

**PL:** And they're design-level, not typos. Constant folding with integer division means the type rules for `/`
aren't in one place. Number formatting in `PRINT` is wrong in both runtimes. That's exactly the "numeric rules
implicit in the emitted code" problem `study\07` Session 3 set out to avoid.

**QB64:** Across QB64pe's own suite it matches 19 of 331 outputs (Q5). Parsing is better than running: 92 % of
the QB4.5-era programs parse, 74 % of the QB64 tests outside the newer array features.

**PRAG:** It also builds, it has 1,380 passing tests, and its LSP answers hover and reports three semantic errors
with ranges (Q7). It isn't nothing.

**TEST:** The passing tests are mostly "it compiled" or "the C contains this string" (earlier review §2.3; the 758
integration tests here are that kind). The ones that run programs fail 24 of 28 (Q2).

**Consensus:** QB64Fresh reliably parses a large part of the language and gives useful editor feedback; it does not
reproduce QB64pe's behaviour, and its tests could not have told it so.

## Session 2 — Does its architecture fit ours?

**COMP:** Compare against `study\07` R5–R7, point by point.

| `study\07` decision | QB64Fresh | Fit |
|---|---|---|
| Lossless syntax tree (R5) | Whitespace and continuations discarded; includes flattened (Q8, Q9) | No |
| Typed IR with explicit conversions (R5) | Name-based typed AST; conversions decided in the C emitter (Q10, Q4) | No |
| C++ back end on libqb's ABI (R6) | C back end on two home-grown runtimes | No |
| Built-in table as data (R9) | Built-ins hard-coded into lexer, AST and parser (Q8); a signature table exists in `semantic\builtins.rs` | Partly |
| Compatibility = observed QB64pe (R2) | Documented intentional differences (Q11) | No |
| CP437, compiler reads bytes (R12) | Compiler falls back to Latin-1; formatter refuses non-UTF-8 (Q6) | Partly |

**DX:** The first row decides the front end. Everything the language server does — formatting, semantic tokens,
incremental reparse, positions in include files — comes from a tree that keeps every byte. QB64Fresh's front end
throws away exactly that information *before* the lexer runs (Q9). You can't retrofit losslessness onto a
preprocessor that pastes files together; you replace the preprocessor, the lexer's skip rules, the span type and
every node that holds a span. That's the front end.

**REF:** The earlier review said "keep the front end". For *its* plan — keep QB64Fresh going and rebuild the middle —
that was reasonable. For ours, with R5 as a hard requirement, the front end is the part that conflicts most
directly. I'd normally fight for reuse. Here what's reusable is the knowledge and the tests, not the structure.

**QB64:** The grammar shape matters too. QB64pe treats most built-ins as names looked up in a table; QB64Fresh
gives 277 of them their own tokens and grammar rules. Every new QB64pe built-in means editing the lexer, the AST and
the parser. With our `builtins.json` (455 entries) that should be a table update.

**PRAG:** What about the parser *code*, re-pointed at a lossless token stream?

**COMP:** It's 11.7k lines organised around those per-statement rules, with recovery that, as Q7 shows, stops other
diagnostics after one parse error. A parser for the lossless tree is a different shape (events or green nodes,
recovery that always produces a node). The test cases in `parser\tests.rs` (2.5k lines) are worth reading for
coverage ideas; the code isn't worth porting.

**Consensus:** QB64Fresh's architecture does not fit `study\07`. None of its pipeline stages should become ours.

## Session 3 — Its runtimes and the back end

**RT:** Two runtimes that disagree with each other and with QB64pe, one of whose tests don't compile (Q2). `study\07`
R6 already chose libqb, which is the behaviour we're measuring against. There is no case for either of these.

**BLD:** One small exception worth stealing as an idea: QB64Fresh shows a Rust compiler + gcc pipeline building on
Windows. We're going to emit C++ for libqb with the llvm-mingw toolchain QB64pe already ships (M0), so even that
doesn't transfer directly.

**Consensus:** do not use either runtime or the C backend.

## Session 4 — Tooling: LSP, formatter, linter, debugger

**DX:** The LSP is the part people would actually see, and it works in a basic way (Q7). But it serves QB64Fresh's
front end, so it inherits Q7's limits: one parse error hides everything, includes aren't analysed. Our M2 language
server sits on our own lossless tree. What carries over is the design: `tower-lsp`, the capability set, how
incremental changes are applied (`lsp\analysis\incremental.rs`), the UTF-16 position conversion
(`lsp\position.rs`, 123 lines), and 730 lines of tests that describe expected behaviour.

**TEST:** The formatter is out: it refuses CP437 files, changes comments, and indents wrongly (Q6); and `study\07`
R11 wants a formatter matching `-y` byte for byte.

**DX:** The debugger (8k lines) has DAP plumbing but its README says the runtime side was never connected. For M5
we'd read its ADR-0013 and DAP message handling; nothing more.

**MOD:** And `vscode-qb64fresh`, from `study\11`, stays the strongest reusable piece: it's a thin client and doesn't
care which server it starts.

**Consensus:** reuse designs and tests from the LSP; nothing from fmt, lint or debug except as reading.

## Session 5 — What is worth taking, concretely

| Asset | Use | How |
|---|---|---|
| `tests\runtime_comparison\` (261 programs) | **Yes — highest value** | Run each through `..\QB64pe\qb64pe.exe`, record outputs as in `verification\`, add to the conformance corpus (`study\07` R11 layer 2). MIT, the user's own. |
| `vscode-qb64fresh` | **Yes** (see `study\11`) | M1 base. |
| `lsp\position.rs`, `lsp\analysis\incremental.rs` | Maybe, as code | Small, self-contained; review and copy with tests when M2 builds the server. |
| `lsp\tests.rs`, `parser\tests.rs`, `error_recovery_tests.rs` | As test ideas | Turn the inputs into program-plus-expected-diagnostics cases for our front end; don't port the assertions. |
| `src\lexer\token.rs` number-literal and suffix regexes | As reference | Our lexer must accept the same literal forms (`&H…~%%`, `1.5D3`, `##`). |
| `semantic\builtins.rs` signature table | Cross-check only | Compare with `tools\builtins\builtins.json`; ours is extracted from `subs_functions.bas` and authoritative. |
| `docs\QB64pe\*.md`, ADRs, `KEY_LEARNINGS.md` | Read, don't trust | Machine-written; useful as checklists (IDE functionality checklist for `study\06`). Any fact taken from them gets verified against `qb64pe.exe` first. |
| Earlier review + harness (`QB64pe\docs\rewrite-decision.md`, `analysis-scripts\rewrite-panel\`) | **Yes** | Move the useful parts into this repo's `study\` and `tools\` so the record isn't only in an untracked file in another clone (`CLAUDE.md` rule 2). |
| Parser, AST, semantic checker, typed IR, codegen, both runtimes, fmt, lint, debug, header parser | No | Architecture conflicts (Session 2) or behaviour wrong (Session 1). |

**PRAG:** Lessons, which cost nothing to take:
1. **Measure against `qb64pe.exe` from the first day.** QB64Fresh's own tests passed while its output was wrong.
   Our M2/M3 gates are "same output as QB64pe", never "emits code".
2. **Scope creep is the failure mode.** Parser, semantics, codegen, runtime, LSP and VS Code extension all started
   on day one (earlier review §2.5). Our milestones keep one layer at a time.
3. **Don't special-case one program** (Q10). Compiling `qb64pe.bas` stays a stretch test, not a target to fit.
4. **No status tables without a measurement behind them.**

## Recommendations

1. **Do not build on QB64Fresh's code.** Its front end conflicts with the lossless-tree requirement (R5), its typed
   IR and backend with R5–R6, its runtimes with R6, and its documented divergences with R2.
2. **Record outputs for `tests\runtime_comparison\` with the old compiler** and add the programs to this repo's
   conformance tests (with a note of origin). Run them with `QB64PE_NOPROMPT=y` so runtime errors do not open a
   message box (`study\09`).
3. **Bring the earlier review into this repo**: a short note in `study\` summarising `rewrite-decision.md` and why
   this project departs from its "keep the front end" advice, and its harness scripts under `tools\` if they
   prove reusable.
4. **Keep `vscode-qb64fresh` as the M1 base** (`study\11`); this review doesn't change that.
5. **At M2**, read QB64Fresh's LSP (`position.rs`, `incremental.rs`, tests) before writing ours; copy small
   pieces only with their tests.
6. **Reconcile the two decisions explicitly.** `rewrite-decision.md` advised against "a new rewrite from an empty
   repository"; `DECISIONS.md` records a ground-up rewrite. This review supports the ground-up rewrite *given*
   `study\07`'s architecture, but the user should confirm that the earlier advice was considered.

## Addendum: branch `broken` (checked 2026-10-02)

`origin/broken` (`eed3d21`, 2026-02-02) forks from the reviewed line at `d7e5bdc` (2026-01-21). Of its 142 commits,
140 are patch-identical to commits on `fixing-reapply`; `6bf251e` was re-applied in edited form as `b935caf`. The
only new material is `eed3d21` (211 files, +18.7k / −9.1k; its message, "IDE disclaimer dismiss…", describes almost
none of it):

| Content | Assessment |
|---|---|
| Splits of `expr.rs`, `stmt\mod.rs`, `semantic\builtins.rs` into submodules; `src\compiler_api.rs` facade (347 lines); runtime tweaks for the QB64pe IDE | Structure only. The IR stays name-based and the `qb64pe.bas` special cases remain (`rewrite-decision.md` §2.7 agrees). Built it: the probes (`PRINT 1 / 3` → `0` etc.) and all 14 `verification\` programs give **byte-identical results** to `fixing-reapply`. No behavioural progress. |
| `ide_layers\` (layer0–2g): step-by-step C and BASIC repros for running the QB64pe IDE under QB64Fresh | Tied to QB64Fresh's runtime; the IDE is not ported here (`DECISIONS.md`). Commits **23 Linux executables of ~25 MB each (~575 MB)** to git. Leave. |
| `tests\ide_equivalence\` (GUI automation via PyMCPAutoGUI) | Leave; same reason. |
| `docs\CODEBASE_EVALUATION_AND_LESSONS_LEARNED.md`, `STRATEGIC_GUIDANCE_MULTI_PERSPECTIVE_REVIEW.md` | Self-assessment. Calls the architecture "sound" and repeats "99.1 %" — both contradicted by Sessions 1–2. It misses the main lesson (no oracle) but has sound process lessons, listed below. |
| `docs\reference\TYPED_IR_CONTRACT.md`, `BYREF_BYVAL_CONTRACT.md` | Describe QB64Fresh's own structs; no QB64pe facts. The *idea* — a written contract per phase boundary, listing every variant and invariant — is worth copying for our typed IR at M2/M3. |
| `.github\workflows\ci.yml` | QB64Fresh-specific; ours comes with M2. |
| `docs\ThingsToDo\QB64PE_IDE_EQUIVALENCE_TEST_PLAN.md`, `QB64PE_IDE_GROUND_UP_BUILD.md`, `…_LAYER1_STEPS.md`, `ide_layers\*\README.md` | How to get the QB64pe IDE running on QB64Fresh's runtime, portion by portion (window, events, graphics, fonts, files, dialogs). We neither port the IDE nor replace libqb, so it doesn't apply. |
| `tests\ide_equivalence\BASELINE.md` | Meant to record native QB64pe behaviour per portion, but almost entirely unfilled ("_fill_"); the automatic capture ran with QB64pe not built (`SKIP_QB64PE_NOT_BUILT`). The one native finding — `evnt` is not callable from user code — is trivial. No usable QB64pe facts. |
| `AgenticLogs\…session-148` to `-154` | Refactoring notes and a long debugging log for QB64Fresh's SDL runtime (blocking `flock`, texture format, key delivery to the IDE's `getinput`). Facts about QB64Fresh's runtime, not QB64pe's. |

All 40 Markdown files in `eed3d21` were read or scanned for statements about QB64pe behaviour; none were found
beyond what `study\01`–`05` already record.

Process lessons from the self-assessment that apply here (added to Session 5's list):
5. **Expose one compiler API** (parse only / parse + check / generate) that the LSP, formatter and tests use, so
   tools never reach into internals.
6. **One diagnostic type across phases** (span with file id, severity, phase), which our "new error messages"
   decision (`DECISIONS.md`) needs anyway.
7. **Keep dispatchers thin from the start**; add a module per statement family instead of growing one file.
8. **Refactor in small, verified steps**: build and test after each file.
9. **Keep IDE- or tool-specific behaviour out of the runtime.**

**Verdict:** nothing on `broken` changes this review. Take the five process lessons and the idea of written phase
contracts; leave the code, `ide_layers\` and the GUI tests.

## Not verified

- The uncommitted changes in the QB64Fresh working copy.
- The Rust runtime ("external" mode), graphics, audio, networking, the debugger end to end, the linter.
- Linux and macOS.
- `vscode-qb64fresh` together with `qb64fresh-lsp` inside VS Code.
