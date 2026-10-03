# 11 — Expert panel: are the existing VS Code extensions worth building on?

**Verdict (Session 4):** use the user's own `vscode-qb64fresh` as the M1 base; take only help, language
configuration and snippets from the community `qb64pe-vscode`.

**What this is.** A design review written by Claude in one pass (no subagents), staged as a discussion between the
panel of `study\07`. The panelists are perspectives, not real people. Facts come from reading and running the code
on 2026-10-02; each is listed under "Evidence" with a file and line.

**Subject.** `<qb64contain>\qb64pe-vscode` (outside this repo; read-only for this review).
- Origin: the community extension "QB64PE" (publisher `grymmjack`), descended from the QB64Official extension.
  MIT licence, copyright "[2022] [QB64Official]". 454 commits; main authors Tom Rooker, Durus, grymmjack.
  Last commit 2025-08-23. Version 0.10.7.
- Local state: 19 files with uncommitted changes (+1192 / −345), and two `.vsix` builds in the folder.
- Size: about 9,000 lines of TypeScript in `src\`, 1,050 help pages (`help\*.md`, 6.8 MB) scraped from the
  QB64pe wiki, one TextMate grammar, 161 lines of snippets.

## Evidence

| # | Finding | Where |
|---|---|---|
| E1 | Type-checks cleanly (`tsc --noEmit`, exit 0). | — |
| E2 | No automated tests. Mocha is a dev dependency but there is no `test` script; root `test_*.bas`/`.js` are manual. | `CLAUDE.md` of the extension, `package.json` |
| E3 | Compiler diagnostics run a **full compile** (`-c src -o exe -x -w`) on every save, then delete the exe. | `src\lintFunctions.ts:48`, `:95` |
| E4 | Errors are recognised by a hard-coded list of 37 message prefixes ("Illegal ", "DIM: ", …); any other message is dropped. The column comes from a regex search for the reported code text. | `src\lintFunctions.ts:162-198`, `:227` |
| E5 | A second, regex-based "real-time" checker reports type mismatches, undefined and unused variables, missing declarations, and "deprecated" syntax. It treats `SCREEN n` and `DEF…` (including `DEFINT`) as deprecated, and identifiers as `[A-Za-z_][A-Za-z0-9_]*` (no `$ % & ! # ~` suffixes, no `.` in names). In a language with implicit declaration this produces false positives. | `src\providers\DiagnosticsProvider.ts:37-40`, `:181`, `:247-266` |
| E6 | Formatter is a line-by-line regex rewriter, off by default. It formats only the text before the first `"` on a line, so it does not corrupt strings, but everything after a string literal is left unformatted. It does not use `qb64pe -y`. | `src\providers\DocumentFormattingEditProvider.ts:212`, `:351-359` |
| E7 | "Remove line numbers" deletes **every digit on every line**: `20 x = 10: PRINT "Score 100"` becomes `x = : PRINT "Score "` (run with node on the copied code). "Renumber lines" uses the same expression. | `src\extension.ts:292`, `:320` |
| E8 | Symbols (outline, go-to-definition, references, completion, hover for user code) come from a regex `SymbolParser` with `$INCLUDE` following and an mtime cache. | `src\providers\SymbolParser.ts` |
| E9 | The "debugger" is not a debugger: it sends a shell command to a terminal. | `package.json` `contributes.debuggers`, `src\providers\DebugAdapterDescriptorFactory.ts` |
| E10 | On activation it writes `qb64pe.helpPath` into the user's **global** settings, and can create `.gitignore` and `-bak` files. | `src\extension.ts:186-197`, `:39-43` |
| E11 | Language id `QB64PE` on `.bas .bi .bm .inc`. Settings, commands (`extension.*`) and grammar scope (`source.QB64PE`) use generic or same names an extension of ours would want. | `package.json` |
| E12 | Help corpus: 1,050 Markdown pages produced by grymmjack's `qb64pe-wiki-to-markdown`, used for hover and a help view, with an online-wiki fallback. Licence of the wiki content is not stated in the repo. | `help\`, `src\extension.ts:368-377` |

## Session 1 — What is actually there?

**DX:** Feature list first, because it's long: highlighting, snippets, outline, go-to-definition, references,
completion including inline templates, signature help, hover with wiki help, quick fixes, formatting, compiler
diagnostics, a TODO tree, an ASCII chart, line-number tools, "open in QB64PE IDE". On paper that already covers most
of the "static" row of `study\06` and much of M1.

**TEST:** On paper. None of it is tested (E2), and the two parts M1 cares most about are the weakest. Diagnostics
drop any error whose text isn't on a list of 37 prefixes (E4); the old compiler has hundreds of messages. And the
regex checker (E5) invents errors. It calls `DEFINT A-Z` deprecated.

**QB64:** That one would annoy real users. Half the community code starts with `DEFINT A-Z` or `SCREEN 12`. Flagging
those as deprecated is wrong on the language, not just a style opinion. And names like `player.x` or `a$` are
ordinary QB64 identifiers that the regexes don't model.

**PRAG:** What's the cost of the full compile on save (E3)?

**BLD:** A complete C++ build of the program on every save. With our toolchain that's seconds per save, and it
competes with the user's own build for `internal\temp`. `study\06` planned `-c` output parsing too, but the old
compiler can stop after the BASIC phase; that needs checking before we copy the full-compile approach.

**REF:** Code quality: it compiles, and it's readable. It is also the typical shape of a regex-grown extension:
`CompletionItemProvider` 1,574 lines, `CodeActionProvider` 978, `DiagnosticsProvider` 891, every provider parsing text
on its own. E7 shows the risk: a destructive command with an obvious bug that nobody caught, because nothing is
tested.

## Session 2 — Fork it, or start fresh?

**MOD:** The case for forking: users exist, the Marketplace listing exists, and we'd ship M1 in days.

**DX:** The case against is the plan in `study\07`. Every language feature in this extension is meant to move to
our Rust language server in M2+. In our architecture the extension is a thin client: grammar, language
configuration, tasks, an LSP client, and later a debug adapter. About 80 % of the TypeScript here is the part we're
going to delete: regex parsing in each provider. Forking means inheriting it, then ripping it out while users of the
existing listing watch features regress.

**PL:** Plus the regex layer encodes wrong language rules (E5). If we keep it until the LSP arrives, we ship false
errors under our name for a whole phase.

**REF:** I'd normally argue for forking working code. Not here: what we'd keep is small and separable, and what
we'd remove is the bulk. Harvest, don't fork.

**QB64:** The coexistence problem matters too (E11). Many QB64 users already have this extension installed. If ours
also claims language id `QB64PE` and the `extension.*` command names, the two will fight over `.bas` files. Ours
needs its own ids and scope names and should say so in the README.

**PRAG:** And the governance question: it's someone else's project, and the local copy has 1,200 lines of
uncommitted changes. Building on that means first deciding whose version we're forking. Starting fresh avoids it.

**Consensus:** do not fork. Build the M1 extension fresh in this repo, small and tested, and take the assets below.

## Session 3 — What is worth taking?

| Asset | Verdict | Notes |
|---|---|---|
| Help corpus (`help\*.md`, E12) | **Most valuable piece.** Use it, by reference, not copied. | Offline help is the hardest part of IDE parity (`study\06`, "Help system"). Prefer regenerating it with `qb64pe-wiki-to-markdown` at build time over vendoring 6.8 MB. **Check the wiki licence before redistributing.** |
| `qb64pe-wiki-to-markdown` (the scraper, separate repo) | Take a look | Possibly the right long-term source for hover text, together with `tools\builtins\builtins.json`. |
| TextMate grammar | Reference only | 178 lines plus a keyword file. Ours should generate the keyword lists from `builtins.json` so they stay in sync with the compiler. Reading this one is useful for edge cases (REM forms, `$` metacommands). |
| `language-configuration.json`, snippets | Take, with attribution | Small, low risk; MIT allows reuse with the copyright notice. |
| Feature list | Take as a checklist | Merge into `study\06` as "what users of the existing extension will expect" (TODO tree, ASCII chart, line-number tools, open in IDE). |
| Error-message prefix list (E4) | Do not use | Parse the old compiler's output by its structure, not a fixed list of messages (see `study\09`). |
| Regex diagnostics, formatter, symbol parser | Do not use | Replaced by `-y`, `-c` output in M1 and by the language server in M2+. |
| "Debugger" | Do not use | It's a terminal command; ours will be a task in M1 and a real DAP later. |

**TEST:** One more use: its manual test files (`test_*.bas`) and the bug in E7 are good regression cases for our
own formatter and line-number tooling later.

**DX:** And a courtesy point. Once ours exists, tell the maintainers. Their help pipeline and our language server
could meet in the middle: their extension could use our LSP one day.

## Recommendations after Sessions 1–3 (point 1 is superseded by Session 4)

1. **Do not fork `qb64pe-vscode`.** Build the M1 extension fresh in this repo, small and with tests from the
   start. Use distinct ids so it can coexist with the existing extension (E11).
2. **Reuse the help pipeline, not the code.** Investigate `qb64pe-wiki-to-markdown` and the wiki content licence;
   if they're fine, generate the help at build time.
3. **Take `language-configuration.json` and the snippets** with the MIT notice; use the grammar as a reference
   only; generate keyword lists from `tools\builtins\builtins.json`.
4. **Add the existing extension's feature list to `study\06`** as a parity checklist.
5. **Check before copying the full-compile-on-save approach (E3):** does the old compiler report BASIC-level errors
   without building C++ (for example `-z`, or the order of `-c` output)? Verify with `verification\`.
6. Optional: report bug E7 to the maintainers.

## Also found in `<qb64contain>`

`QB64Fresh` (`davidshq/QB64Fresh`) is an **earlier Rust rewrite** of QB64pe: `src\codegen\c_backend`,
`src\semantic`, `runtime\`, with recent commits about a Windows QB64pe bootstrap and uncommitted work. It is not
mentioned anywhere in this repo. It is outside the scope of this review, but it bears directly on M2 (crate layout,
what was tried, what went wrong) and deserves its own review before M2 starts.

## Session 4 — The user's own extension, `vscode-qb64fresh`

**Subject.** `github.com/davidshq/vscode-qb64fresh`, written from scratch by the user as the client for QB64Fresh.
QB64Fresh's README expects it at `..\vscode-qb64fresh\`; there is no copy on this machine; GitHub was cloned to a
scratch directory for this review. **A newer working copy** is at `<share>\vscode-qb64fresh`
(same last commit `be9345a`, plus uncommitted work: `DEVELOPMENT.md`, `docs\TESTING_STRATEGY.md`, two integration
test files of 331 and 510 lines for error parsing and path resolution, a change to `extension.test.ts`). Import
from that copy, not from GitHub, or commit and push it there first. 15 commits, 2026-01-16 to 2026-01-26. MIT. Version 0.1.0, publisher `qb64fresh`.

| # | Finding | Where |
|---|---|---|
| F1 | Thin client, as `study\06` plans: `vscode-languageclient` 9 starts `qb64fresh-lsp`; format, lint and debug call separate binaries (`qb64fresh-fmt`, `-lint`, `-debug`). | `src\extension.ts:1016`, `:225`, `:623`, `:1290` |
| F2 | About 1,600 lines of TypeScript: `extension.ts` 1,150, `config.ts` 335, `utils.ts` 128. | `src\` |
| F3 | Builds and is tested: `npm ci`, `tsc` exit 0; `npm test` 25 unit tests pass; ESLint 0 errors, 10 warnings. Integration test runner exists (`test:integration`), not run here. | `src\test\` |
| F4 | Build, Run, Build and Run commands; compiler errors go to Problems; build diagnostics clear on edit; Build/Run refused in untrusted workspaces; status bar shows which tools were found. | `src\extension.ts:139`, `:160`, `:745-829` |
| F5 | Tool discovery: setting, then `PATH`, then a sibling `QB64Fresh\target\`. | `src\config.ts:128-245` |
| F6 | Tied to QB64Fresh: the compiler is called as `qb64fresh file -o out --emit-c`, and errors are parsed from `line N: message` lines. The old compiler's `-c` output needs a different parser and arguments. | `src\extension.ts:793`, `src\utils.ts:75-93` |
| F7 | Own ids (`qb64fresh` language, `qb64fresh.*` commands and settings), so it coexists with `qb64pe-vscode` apart from both claiming `.bas`. Aliases `BASIC` and `QBasic` are broad. | `package.json` |
| F8 | No help system; grammar 156 lines; 663 lines of snippets. | `syntaxes\`, `snippets\` |

**DX:** This is the extension `study\06` describes, already written, with tests and the right boundaries. Every
language feature is the server's job; the extension only finds tools, starts them and shows what they return.
Nothing to delete when our language server arrives. The server just takes the place of `qb64fresh-lsp`.

**PRAG:** And M1 doesn't need the language server at all. What M1 needs is: grammar, build/run, diagnostics from
the old compiler, formatting through `-y`. This extension already has the build/run/diagnostics/format plumbing;
only the commands it runs and the output it parses are QB64Fresh-specific (F6). That's a backend switch, a few
hundred lines, not a rewrite.

**TEST:** The parser is a pure function with unit tests (`parseCompilerOutput`). Adding an old-compiler parser next
to it, with fixtures recorded from `..\QB64pe\qb64pe.exe`, fits the existing test setup exactly.

**REF:** Two things I'd change on import. `extension.ts` at 1,150 lines holds commands, formatter, linter, build and
debug wiring; split it by concern before it grows. And the naming: "QB64Fresh" is the name of a different project.
Pick this project's name once and rename ids, settings and binaries together.

**QB64:** Keep the distinct language id (F7), but drop the `BASIC` / `QBasic` aliases; they claim more than we are.
Help is still missing (F8); that's where Session 3's help pipeline comes in.

**MOD:** Ownership is clean: it's the user's code, MIT, no other contributors to coordinate with. Bringing it into
this repo with its history (`git subtree add` or a history-preserving import) costs little and keeps the record of
why things are the way they are.

**BLD:** One dependency to settle: the extension expects separate `-fmt`, `-lint`, `-debug` binaries. Whether our
Rust tools ship as one binary with subcommands or several is an M2 decision; the extension's tool discovery
(F5) handles either with small changes.

**Consensus:** `vscode-qb64fresh` is the base for M1. `qb64pe-vscode` contributes help, language configuration and
snippets only.

## Recommendations (final)

1. **Use `vscode-qb64fresh` as the M1 extension.** Import it into this repo (`vscode\`), preferably with history.
2. **Add an old-compiler backend:** arguments for `..\QB64pe\qb64pe.exe` (`-c`, `-x`, `-y`), a parser for its
   error output with recorded fixtures, selected by a setting. Keep the QB64Fresh-style path for our own tools later.
3. **Rename once** (project name, language id, settings, commands) and drop the `BASIC` / `QBasic` aliases.
4. **Split `extension.ts`** by concern (build, format, lint, debug, client) as part of the import.
5. From `qb64pe-vscode`: the help pipeline (after the licence check), `language-configuration.json` and snippets
   where they add something, and its feature list as a parity checklist. Nothing else.
6. Unchanged from above: check whether the old compiler reports BASIC errors without a C++ build (E3).
7. Review `QB64Fresh` itself before M2; its `src\lsp` (about 2,000 lines plus 730 lines of tests) is the server
   this extension was built for.
