# 15 — Pre-coding review: are the broad strokes right?

Written 2026-10-02 (session 6), before the first line of M1 code. One pass over `STATUS.md`, `00`, `06`, `07`,
`archive\12`, `archive\13`, `SOMEDAY.md` and the M1 OpenSpec change (`openspec\changes\m1-vscode-extension-v0`, archived 2026-10-03 to `openspec\changes\archive\2026-10-03-m1-vscode-extension-v0`), looking for
direction errors, not nits.

**Verdict:** no major error. The strategy (rewrite the compiler, refactor libqb in place, replace the IDE with
VS Code), the oracle rule (observed behaviour of `qb64pe.exe` at a pinned commit), the pipeline (lossless tree →
typed AST → typed IR → C++ against the existing libqb ABI) and the M1-first roadmap all hold up. Five adjustments
follow; all are recorded as decisions in `CLAUDE.md`.

## 1. Bytes, not strings, in the Rust front end

`00` §2 says "the compiler reads bytes" but M2 had no written consequence yet. The lexer runs over `&[u8]`,
never `&str`; source text is never required to be valid UTF-8. QB64Fresh refused 25 of 547 test programs with
"stream did not contain valid UTF-8" because its lexer worked on `str` (`archive\12` Q6). Old sources hold raw CP437 bytes
in string literals and programs depend on them at run time.

Consequence for the language server: Language Server Protocol positions are UTF-16 code-unit offsets into the
*decoded* document, while the compiler sees bytes. Under CP437 one byte is one character, so the mapping is the
identity; under UTF-8 it is not. The server therefore has to know each document's encoding (the extension knows
it from `files.encoding`) and map byte offsets to UTF-16 offsets per file. Design this into the position type
from the start: `(file id, byte offset)` in the compiler, conversion at the server boundary only.

## 2. Validate the IR-to-ABI assumption early

R5/R6 assume the IR can stay ABI-neutral (optional arguments as present/absent, explicit conversions, explicit
raise/resume points) while the C++ emitter reproduces exactly what `qbx.cpp` and libqb expect: `qbs*` strings,
pointer variables, `passed` bitmasks, `specialformat` argument shapes, the `do{…}while(r)` error wrappers. That is
the project's riskiest technical bet and the roadmap only tests it in M3, after the whole front end exists.

Adjustment: the first thing built in M2 is a **vertical slice**: a tiny subset (a few numeric types, string
literals, `PRINT`, one built-in with an optional argument) taken from source through the lossless tree, resolution,
IR and the C++ emitter, linked with the old runtime and run. Its output is compared with `qb64pe.exe`. The slice
is throwaway in detail but fixes the IR's shape while it is small. The rest of M2 (full parser, formatter, server)
then grows around a validated core instead of a hypothesis.

## 3. Diagnostics policy for the new language server

Diagnostics were the hardest part for both reviewed extensions: `qb64pe-vscode`'s regex checks can flag code the
compiler accepts (`archive\11` E5), and QB64Fresh's server hides all other diagnostics behind one parse error and reports none for included files (`archive\12`
Q7). Until the new type checker is mature, the server reports only errors the old compiler also reports, which is
checkable by a differential run over the test corpus (every program the old compiler accepts must produce zero
errors from the new front end; every `.err` program must produce at least one at the same line). Warnings and
extra diagnostics are added when the differential run shows they are right.

## 4. Keep M1's backend interface minimal

`design.md` D1 puts the old-compiler knowledge behind `CompilerBackend { check, build, format }` so "M2+ tools
replace a backend". That is only half true: in M2 the language server replaces *check* and *format* wholesale
(push diagnostics over LSP, formatting as an LSP request), and only *build* and *run* survive as process calls.
Do not spend effort making the interface future-proof; one thin module per concern is enough.

## 5. libqb vendoring at M3

"Refactor libqb in place" means this repo needs its own copy of `..\QB64pe\internal\c` (about 1,100 files
including the vendored GLFW, miniaudio, FreeType, curl, stb). Decided: a plain copy at the pinned commit with the
source commit recorded in a file next to it; no git subtree. Upstream changes are merged by hand when wanted.
Nothing is copied before M3.

## Housekeeping done in the same session

- The seven open questions in `STATUS.md` were all answered (list there; decisions in `CLAUDE.md`). None blocked M1.
- `tools\wiki\fetch_wiki.py` written and run: 1,124 pages (main and Template namespaces) as raw wikitext in the
  git-ignored `tools\wiki\cache\`, incremental on rerun. No licence is stated by the wiki, so the text stays a
  local reference until the maintainers have been asked.
