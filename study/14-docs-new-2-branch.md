# 14 — The `docs-new-2` branch: what to take, what to leave

Checked 2026-10-02. The user pushed `docs-new-2` and asked what this project should take from it.

**Where it is:** not QB64Fresh. The branch is on the user's QB64pe fork, `davidshq-contribute/QB64pe`, head
`cb40765352`. It was cloned to a scratch directory; `..\QB64pe` was not touched (`CLAUDE.md` rule 3).

**Verdict:** take nothing wholesale. Use the internals documents as a **pinned, unverified reference index** for
M3 (code generation against libqb) and M5 (debugger), checking every fact against the code or `qb64pe.exe` before
it enters `study\`. Leave the rules, the review and the consolidation plan; they are for working on QB64pe itself.

## What the branch contains

Two commits on top of QB64pe `bb8dc0a603` (2026-02-01), which is **565 commits behind** current upstream `main`:

| Commit | Date | Content |
|---|---|---|
| `6ef3ff1ed4` "Documenting QB64pe extensively" | 2026-02-02 | 60 topic documents at the repo root (about 18,500 lines), `CLAUDE.md`, small edits to `docs\`. |
| `cb40765352` "docs new" | 2026-10-02 | Moves the topic documents into `docs\`; adds `ADDING-A-BUILTIN.md`, `OPERATOR-PRECEDENCE.md`, `COMPILER-PIPELINE.md` (rewritten), `MULTI_PERSPECTIVE_CODEBASE_REVIEW.md`, `DOCUMENTATION-CONSOLIDATION.md`, `SETUP-LNX.md`, `LINUX-DISPLAY-TROUBLESHOOTING.md`; five `.cursor\rules\*.mdc` files. |

These appear to be the internals documents `rewrite-decision.md` §2.7 mentions on the earlier `more-tests` branch
("machine-written in January, not checked against the current compiler").

## Accuracy

| Check | Result |
|---|---|
| Function names cited as `name(` | 164 of 169 exist in QB64pe's source; the other 5 are Windows or third-party APIs. The docs point at real code. |
| Code snippets | Some are invented. `PRINT-FORMATTING.md` shows `#define PRINT_ZONE_WIDTH 14` and `next_zone()`; neither exists in `internal\c`. The behaviour they describe (14-column zones) is right. |
| Line numbers | Correct for February, stale now. `isoperator` is cited at `qb64pe.bas` 20142; in our pinned 4.7.0 it is at 24518 (file grew from 20.8k to 25.0k lines). |
| `OPERATOR-PRECEDENCE.md` (added today) | **Wrong:** says `^` is right-associative (`2^3^4` → `2^(3^4)`). At run time QB64pe is left-associative (`2 ^ 3 ^ 2` = 64, `study\09`); only CONST evaluation is right-associative (512). Also says "AND has higher precedence than =" while its own table, and its example, show the opposite. |
| `TYPE-SYSTEM.md` | **Wrong:** "float to integer … overflow triggers error". QB64pe wraps silently (`x% = 70000` → 4464, `study\09`). |
| `CONST-EVAL.md` | Right: CONST `^` is right-associative, `2^3^2` = 512. |
| `ERROR-CODES.md`, `ERROR-HANDLING.md` | Right: error 11 (division by zero) is critical and ends the program (`study\09`). |
| `MATH-FUNCTIONS.md` | Right: `_ROUND` uses banker's rounding. |

| `STRING-INTERNALS.md` `struct qbs` | Right: matches `internal\c\libqb\include\qbs.h:14-31` field for field. |
| `ARRAY-IMPLEMENTATION.md` descriptor size and flags | Right: `(4*dims+5)*ptrsz`, flags 1 defined / 2 static / 4 cmem (matches `study\02` §3.4; flag 8 was added later). |
| `ARRAY-IMPLEMENTATION.md` generated indexing example | **Wrong:** for `arr(i, j, k)` it pairs `i` with `array[4]` and `k` with `array[12]`. Dimensions are stored in reverse order, so the first index uses the highest slot (`study\02` §3.4: `array_check((i1)-A[4*(n-1)+4], …)`). The layout table omits the reverse order; only the quoted `func_lbound` code shows it. |

Tally of 13 checks: 8 right, 5 wrong (counting the invented snippet). The pattern is consistent: **text copied
from code is reliable** (struct layouts, function names, quoted functions, error tables); **explanations, worked
examples and general rules written around the code are not** — and that is where the rewrite's hard questions
are (numeric semantics, generated-code shapes).

## How to use them in the rewrite

1. **As a map.** They are the fastest way to find which libqb function, file and generated-code pattern handles a
   feature. Go from the doc to the code, not from the doc to a conclusion.
2. **Never as a spec.** Numeric and formatting behaviour comes from `qb64pe.exe` (`study\07` R2, R8); generated
   code shapes come from `qbx.cpp` and the C++ the old compiler emits (`internal\temp`), not from doc examples.
3. **Line numbers:** translate by function name; the file has moved by about 4,400 lines since February.
4. **Our `study\01`–`05`, `09`, `10` take precedence.** They were written from 4.7.0's code and checked by
   running programs. Where a branch doc adds something they lack, verify it and add it to the matching study.

## Take

| Item | Why | How |
|---|---|---|
| `STRING-INTERNALS`, `ARRAY-IMPLEMENTATION`, `CODEGEN-PATTERNS`, `GLOBALS`, `ERROR-HANDLING`, `ERROR-CODES`, `AUTO-INCLUDING`, `DECLARE-LIBRARY`, `LIBRARY-SYSTEM`, `PREPROCESSOR`, `METACOMMANDS`, `CONST-EVAL`, `EXPRESSION-EVALUATION`, `TYPE-SYSTEM`, `PRINT-FORMATTING`, `DATA-RESTORE` | They map the libqb ABI and the C++ that `qb64pe.bas` emits; M3 emits the same fragments (`study\07` R6). Our `study\01`–`05` cover much of this, these add detail and an index. | Reference by commit (`davidshq-contribute/QB64pe@cb40765352`, `docs\`); do not copy into this repo. When a study needs a fact, verify it against 4.7.0 and write it into `study\` with a citation. |
| `DEBUG-VWATCH`, `IDE-PROTOCOL`, `IDE-ARCHITECTURE` | The `$DEBUG` / vWATCH protocol between IDE and program; input for the M5 debug adapter (`study\04`). | Same: reference, verify at M5. |
| `ERROR-CODES.md` table | A list of runtime error numbers and texts, with which ones are critical. | Cross-check with `internal\c\libqb` and use as a test checklist for error messages. |
| `ADDING-A-BUILTIN.md` | Shows which places a built-in touches today (`subs_functions.bas`, libqb, docs). | Input to the built-in table design (`study\10` §3), including the auto-include gap (`study\13`). |
| One rule idea | "When code changes, update the doc in the same change" (`documentation.mdc`). | Already close to `CLAUDE.md` rule 2; no new rule needed. |

## Leave

| Item | Why |
|---|---|
| `CLAUDE.md`, `.cursor\rules\*.mdc` | Instructions for editing QB64pe's BASIC and C++ source and running its shell test scripts. This project is Rust, does not modify QB64pe (`CLAUDE.md` rule 3), and runs tests through `tools\legacy_tests\`. |
| `MULTI_PERSPECTIVE_CODEBASE_REVIEW.md` | Improvement ideas for QB64pe itself; our equivalent is `study\07`. |
| `DOCUMENTATION-CONSOLIDATION.md` | A plan for reorganising QB64pe's docs. |
| `SETUP-LNX.md`, `LINUX-DISPLAY-TROUBLESHOOTING.md`, `BUILD-SYSTEM.md` | Building QB64pe on Linux; we build on Windows (M0) and `study\05` covers the build. |
| User-facing topic docs: `AUDIO`, `FONTS`, `CLIPBOARD`, `GRAPHICS`, `INPUT-DEVICES`, `NETWORKING`, `WINDOW-MANAGEMENT`, `IMAGE-FORMATS`, `SOUND-LEGACY`, `QUICK-REFERENCE`, `MIGRATION`, … | The runtime is reused as is (R6), and the wiki (`study\13`) is the better user reference. Consult only if a runtime question comes up. |

## If the branch is meant for upstream QB64pe

Not this project's concern, but worth noting for the user: before proposing these docs upstream, fix the two
wrong claims above, replace invented snippets with real code or mark them as illustrative, rebase onto current
`main` (565 commits behind) and refresh line numbers.
