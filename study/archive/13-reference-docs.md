# 13 — Reference documentation: what exists, what to trust, where to get it fresh

Checked 2026-10-02. Question: QB64Fresh and related folders hold language references and wiki-derived help;
are they useful, or should we fetch sources fresh?

**Short answer:** fetch the QB64pe wiki fresh through its API; keep the Microsoft QuickBASIC manuals as a local
reference only (copyrighted); treat QB64Fresh's own specs as machine-written checklists, not sources. Nothing here
outranks running `qb64pe.exe` (`study\07` R2).

## Sources found

| Source | Where | What it is | Verdict |
|---|---|---|---|
| **QB64pe wiki** (live) | `https://qb64phoenix.com/qb64wiki/` (MediaWiki 1.45; `api.php` works without login) | 1,035 articles, 9,449 edits. Raw wikitext per page via `action=query&prop=revisions&rvprop=content`; page list via `list=allpages`. Example: `PRINT`, last edited 2024-04-28. | **Primary documentation source.** Fetch fresh, store raw wikitext with page timestamps, convert ourselves. Licence: the API's `rightsinfo` is empty — **no licence stated**. Fine for our own reference; ask the QB64pe maintainers before shipping wiki text in the extension. |
| QB64pe GitHub wiki | `QB64-Phoenix-Edition/QB64pe.wiki.git` | One placeholder page from 2022 | Not a source. |
| `qb64pe-vscode\help\*.md` | `<qb64contain>\qb64pe-vscode\help` | 1,050 pages converted from the wiki by grymmjack's `qb64pe-wiki-to-markdown` (last pushed 2025-03-04, no licence on the converter) | Superseded by a fresh fetch; useful only to see how someone else rendered wiki markup to Markdown. |
| QB64pe IDE help cache | `..\QB64pe\internal\help` | Empty here (the IDE downloads pages on demand) | Not a source. |
| **Microsoft QuickBASIC manuals** (text) | `<share>\docs\qb_language_reference.txt` (977 KB), `qb_programming_in_basic.txt` (967 KB) | Plain-text copies of *Microsoft QuickBASIC: Language Reference* and *Programming in BASIC* (© Microsoft 1987, 1988) | **Best source for QB4.5 behaviour questions** (`study\08`, the QB45 panelist). Copyrighted: **do not commit** to this repo; refer to them by path and quote only short passages. |
| `QUICKBASIC_4.5_LANGUAGE_SPECIFICATION.md` (28 KB) and `…_OTHER_SOURCE_CONTENT.md` (19 KB) | `QB64Fresh\docs\QuickBasic\` | Machine-written summaries of the two manuals "in our own words", plus a map of what they leave out | Useful index into the manuals. Verify against the manual text before relying on a detail. |
| `QB64PE_LANGUAGE_SPECIFICATION.md` (85 KB) | `QB64Fresh\docs\QB64pe\` | Machine-written, "Source: QB64 Phoenix Edition source code and test suite", 2026-01-17 | Checklist only. Spot check found an error: it says `_TRUE` / `_FALSE` are not part of the language, but QB64pe defines them (`internal\support\include\beforefirstline.bi:35`) and `qb64pe.bas` uses them. Operator precedence and logical-operator examples checked correct. |
| `QB64PE_IDE_FUNCTIONALITY_CHECKLIST.md` (29 KB) | `QB64Fresh\docs\QB64pe\` | List of IDE features | Cross-check against `study\06-vscode-parity.md` when M1 scope is fixed. |
| `QB64PE_ARCHITECTURE.md`, `LIBQB_FUNCTIONALITY.md`, `QB64PE_DEBUGGING.md`, `QB64PE_OPENGL_FUNCTIONALITY.md` | `QB64Fresh\docs\QB64pe\` | Machine-written notes on QB64pe internals | Lower value than our verified `study\01`–`05` and `09`–`10`; read only for anything those don't cover (OpenGL list for `SOMEDAY.md`). |
| `QB64Fresh_LANGUAGE_REFERENCE.md`, handbook, migration guide | `QB64Fresh\docs\` | Describe QB64Fresh's own dialect, including its intentional differences | Not useful: wrong target language. |
| QB64pe repo docs | `..\QB64pe\` (and the untracked notes in `<qb64contain>\QB64pe\docs\`, see `STATUS.md`) | Build, auto-including, testing notes | Read as needed. |

## Gap found in our own work

`tools\builtins\builtins.json` is extracted from `subs_functions.bas` only. Names that QB64pe defines in the
auto-included BASIC files — at least `CONST _TRUE = -1, _FALSE = 0` in `beforefirstline.bi`, and whatever else
`beforefirstline.bi`, `afterlastline.bm` and the color-constant includes define (`study\07` Session 3) — are missing.
The front end must load those files anyway; the extractor (or a second table) should list their names so
completion, hover and highlighting know them. Recorded in `study\10` §3.

## Recommendations

1. **Write `tools\wiki\fetch_wiki.py`**: page list via `list=allpages`, raw wikitext via `prop=revisions`, polite
   rate (one request at a time, small delay), saved with title, page id and revision timestamp. Store the output
   outside git (or in git only after the licence question is answered). Rerunning it updates changed pages.
2. **Ask the QB64pe maintainers** about the wiki's licence before the extension ships wiki-derived hover text or
   help pages. Until then the extension links to the online page.
3. **Use the wiki and the manuals as hypotheses, `qb64pe.exe` as the answer.** Where they disagree with the
   compiler, the compiler wins and the disagreement goes into the divergence register (`study\07` R2).
4. **Keep the QuickBASIC manuals local** (`<share>\docs\`); a copy on this machine outside
   the repo is fine.
5. **Extend the built-in extraction** to the auto-included files (gap above).
