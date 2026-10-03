# Design

## Context

See `proposal.md` for motivation and `specs/editor/*` for requirements. The extension is new code in `vscode\`;
it is not built on `vscode-qb64fresh` (on `<share>`) or `<qb64contain>\qb64pe-vscode` (decision 2026-10-02).
Under `CLAUDE.md` rule 5, nothing is copied from `qb64pe-vscode`; small self-contained pieces of `vscode-qb64fresh`
may be taken when read in full and checked against `qb64pe.exe`.

### Old-compiler behaviour (measured 2026-10-02, QB64pe 4.7.0, `..\QB64pe\qb64pe.exe`, Windows)

| Invocation | Result |
|---|---|
| `qb64pe -z -q -w <src> -o <exe>` | Generates C++ only (no C++ build, no exe). 0.3 s for a clean file, 1.3 s with an error. Exit 1 on error, 0 otherwise. |
| `-x` instead of `-q` | Same, plus a banner and a `[....] n%` progress bar using `\r`. Don't parse that; use `-q`. |
| Works from any working directory | Yes; the compiler finds its `internal\` folder itself. |
| Error output (main file) | Optional blank line, `<message>`, `Caused by (or after):<upper-cased statement>`, `LINE <n>:<source line>`. Line ends are LF when stdout is a pipe (re-measured 2026-10-03 for the fixtures; first noted as CRLF). |
| Error inside `$INCLUDE` | `<message>\x01 in line <m> of <absolute path> included` (a `\x01` byte before the suffix; the path mixes `\` and `/`), then `Caused by…`, then `LINE <n>:…` where **n is the main-file line of the `$INCLUDE`**, not the error line. The location to report is `<path>:<m>`. |
| `-x` output with an error | Banner (`QB64-PE Compiler V…`, `Beginning C++ output…`) and the `\r`-separated progress bar come before the message, in the same block. |
| Missing include | `File inc.bi not found`, `Caused by (or after):`, `LINE 1:` (n = the `$INCLUDE` line). |
| Warning (`-w`) | `<file name>:<line>: warning: <message>` then an indented detail line (`    unusedvar%   INTEGER`). Exit 0. File name only, not a full path. |
| `qb64pe -y -q <src> -o <out>` | Writes the formatted program to `<out>` (CRLF, keyword case `Print`, 4-space indent). CP437 bytes in strings unchanged. Exit 0. |
| `-y` on a program with an error | Same error output as `-z`, exit 1, `<out>` not written. `-y` runs the whole front end. |
| `-y` on a copy in another folder | Fails with `File inc.bi not found`: includes resolve relative to the source file's folder. |
| Build: `-c -x -w <src> -o <exe>` | Full build (seconds). Error output as above; C++ errors go to `internal\temp\compilelog.txt`. |

All invocations share `<qb64pe>\internal\temp\` (`study\archive\11` E3, `study\05`), so two compiler runs at once can
corrupt each other's files.

## Goals / Non-Goals

**Goals:**
- A thin client: the extension finds a tool, runs it, and shows what it returns. The old compiler's arguments and
  output format are known only to `compiler/qb64pe.ts` and the parser. No abstraction is built for M2: there the
  language server replaces check and format outright (diagnostics and formatting over LSP), and only build and run
  stay as process calls (`study
` §4).
- The output parser is a pure function, unit-tested against recorded outputs.

**Non-Goals:**
- Column information (the old compiler has none).
- Multiple errors per check (the old compiler stops at the first).
- Mapping C++ compile errors back to BASIC lines. A failed C++ step is reported as one error pointing to the
  output channel and `compilelog.txt`.
- Publishing to the Marketplace (packaging as `.vsix` only).
- macOS/Linux testing. Code uses `path` and platform checks so it should work there, but M1 is verified on Windows.

## Decisions

### D1. Layout and tooling
`vscode\` holds a standalone npm project: TypeScript compiled with `tsc` to `out\` (no bundler in M1; a few
files don't need one). Tests: mocha for pure unit tests (run in Node, fast, no VS Code), and `@vscode/test-electron`
for a small integration suite. ESLint with the TypeScript recommended rules. Minimum VS Code 1.100 (decision
2026-10-03): its extension host can `require` ESM-only mocha 12, and it has `TextDocument.encoding` (D10). The
integration suite runs on stable and on 1.100.

Source split by concern from the start (`study\archive\11` noted that `vscode-qb64fresh`'s single 1,150-line
`extension.ts` mixed everything):

| File | Role |
|---|---|
| `src/extension.ts` | `activate`: wires the parts together, nothing else |
| `src/config.ts` | Reads `qb64rust.*` settings |
| `src/compiler/discovery.ts` | Finds the compiler (setting, then `PATH`) |
| `src/compiler/qb64pe.ts` | Old-compiler calls: plain functions `check`, `build`, `format` (arguments, process handling); no interface type |
| `src/compiler/parseQb64peOutput.ts` | Pure parser: output text → `CompilerMessage[]` (no `vscode` import) |
| `src/compiler/runQueue.ts` | Serialises compiler runs, cancellation, timeout |
| `src/diagnostics.ts` | `CompilerMessage[]` → `DiagnosticCollection`, clearing on edit/close |
| `src/build.ts` | Build / Run / Build and Run commands, terminal, output channel |
| `src/tasks.ts` | Task provider for type `qb64rust` |
| `src/format.ts` | Document formatting provider |
| `src/statusBar.ts` | Compiler status: a language status item (`languages.createLanguageStatusItem`), `study\18` |

*Alternative:* import `vscode-qb64fresh` and adapt it. Rejected by the user on 2026-10-02 (start fresh).

### D2. One parser data type
`CompilerMessage = { severity: 'error' | 'warning'; file: string | undefined; line: number /* 1-based */;
message: string }`. `file` undefined means "the file that was compiled". The parser:
1. Accepts CRLF and LF, removes control bytes other than `\r` (the `\x01` before the include suffix), drops
   progress-bar lines (a line that still contains `\r` after removing a trailing CR, or `[....] n%`) and blank lines.
2. Warning lines: `^(.+?):(\d+): warning: (.*)$`; a following line that starts with whitespace is the detail,
   whitespace-collapsed and appended as `message: detail`. The file name is resolved against the compiled
   file's folder, then against the include paths seen in errors; else the main file.
3. Error block: a line `LINE <n>:…` closes the block. The message is the last non-empty line before the
   `Caused by` line (or before `LINE`, if there is no `Caused by`), so that `-x` banners are skipped. If it
   ends with ` in line <m> of <path> included`, the location is `<path>:<m>` and that suffix is removed; otherwise
   the location is the main file, line n. The `Caused by` line is dropped from the message.
4. If the exit code is non-zero and no error was found, one error at line 1 with the trimmed output
   (`compiler-diagnostics` spec, "Unrecognised output").

*Why structure, not a list of known messages:* `qb64pe-vscode` recognises 37 message prefixes, so
messages outside that list are not shown (`study\archive\11` E4); parsing by structure covers every message.

### D3. Checking uses `-z`, on the real file
On save, the file on disk equals the document, so `check` runs `-z -q -w <file> -o <tempdir>\check.exe` directly
on it. No temp copy, includes resolve naturally. The `-o` target is in a per-session temp folder, so nothing lands
next to the source (`-z` writes no exe, but `-o` must be given; measured).
*Alternative:* full `-c` build on save, as `qb64pe-vscode` does. Rejected: seconds per save, and an exe written.

### D4. All compiler runs go through one queue
Because of the shared `internal\temp`, `runQueue` runs one compiler process at a time across check, format and
build. A new check for a file cancels a queued or running check of the same file (kill the process tree with
`taskkill /T /F` on Windows, `kill` elsewhere). Build and format are never cancelled by checks; a check queued
behind a build simply waits. Each run has a timeout (`qb64rust.checkTimeoutSeconds`, default 30, for check and
format; builds have none, as they show progress in the output channel).
*Alternative:* copy the compiler per run. Rejected: large, and still one `internal\` per copy.

### D5. Formatting writes a sibling temp file
`-y` needs a file on disk in the document's folder (includes). Steps: encode the document text with the
document's encoding (default CP437) → write `<dir>\.qb64rust-fmt-<random>.bas` → `-y -q <tmp> -o <tmpdir>\out.bas`
→ read `out.bas` with the same encoding → convert line endings to the document's EOL → one full-range `TextEdit`.
The sibling file is deleted in `finally`. For untitled documents the temp file goes in the session temp folder
(relative includes can't work there anyway).
Error output is parsed with D2; locations pointing at the temp file are mapped back to the document.
Encoding conversion uses `iconv-lite` (`cp437`), the only runtime dependency, because Node's `TextDecoder` has
no CP437.
*Alternative:* format only saved, clean documents and pass the real file. Rejected: Format Document must work on
unsaved text (spec), and formatting mid-edit is the normal case.

### D6. Build and run
Build runs `-c -x -w <file> -o <dir>\<base>[.exe]` with the output streamed to the "QB64" output channel (the
`-x` progress bar is shown as is, `\r` replaced). Errors are parsed with D2 and published like a check. Run
uses a new VS Code terminal named "QB64" each time (the previous one the extension opened is closed, so a program
still running or waiting for a key never receives the command line as input), with `cwd` = source folder and
`qb64rust.runArguments` appended. Run sets `QB64PE_NOPROMPT` only if `qb64rust.noPrompt` is true (default false: users expect the
runtime error dialog; tests set it).
Tasks (D1 `tasks.ts`) use `ProcessExecution`/`ShellExecution` with the same arguments, so they appear in
the terminal like any task.

### D7. Encoding default
`package.json` `contributes.configurationDefaults`: `"[qb64rust]": { "files.encoding": "cp437",
"files.autoGuessEncoding": false }`. VS Code supports `cp437`. Language-specific defaults lose to user and
workspace settings, which gives the override in the spec.

### D8. Grammar
Hand-written `syntaxes/qb64rust.tmLanguage.json`, case-insensitive (`(?i)`). Order of patterns: strings first,
then `'$meta` / `REM $meta`, then comments (`'`, `REM` at statement start), metacommands (`$CONSOLE`, `$IF`…),
numbers with suffixes and `&H`/`&O`/`&B`, line numbers and labels at line start, type-suffixed identifiers, then
keyword lists. Keyword lists are hand-maintained in M1 (generation from `builtins.json` is later, decision
2026-10-02); they come from `tools\builtins\builtins.json` names once, by a copy. That table lacks the names
defined in the auto-included BASIC files (`study\10` §3.4), so `_TRUE`, `_FALSE` and the color constants are added
by hand from `..\QB64pe\internal\support\include\` (`beforefirstline.bi` and the color-constant includes).
Grammar tests use `vscode-tmgrammar-test` with small annotated `.bas` fixtures.

### D9. Test fixtures from the real compiler
`vscode\test-fixtures\compiler\` holds `.bas` inputs and their recorded `qb64pe` outputs (`.out.txt`, exit code
in `.exit`), produced by a script `vscode\scripts\record-fixtures.sh` that runs `..\QB64pe\qb64pe.exe`. Parser unit
tests read these, so they run without the compiler. Integration tests that need the compiler skip when
`..\QB64pe\qb64pe.exe` is missing.

### D10. Encoding is per document, read from VS Code
Every place that turns document text into bytes (D5) uses the document's own encoding (`files.encoding` for its
resource), never a fixed CP437. The M2 language server needs the same information to map the compiler's byte
offsets to LSP UTF-16 positions (`study\15` §1); M1 only keeps that lookup in one function in `config.ts`, which
returns `TextDocument.encoding` (VS Code 1.100+), so an encoding chosen by hand is respected too.

## Risks / Trade-offs

- [Check and build share one queue, so a long build delays on-save diagnostics] → acceptable in M1; the status
  bar shows "checking…/building…".
- [Sibling temp file in the user's folder can be picked up by file watchers or git for a moment] → dot-prefixed
  name, deleted in `finally`, and also removed on activation if a stale one is found.
- [Output format of the old compiler changes in a later QB64pe] → fixtures pin 4.7.0; the "Unrecognised output"
  fallback guarantees the user still sees the failure.
- [The warning line gives a bare file name; two included files with the same name in different folders are
  ambiguous] → resolve against the main file's folder first; rare.
- [`-y` changes more than layout, e.g. keyword case `PRINT` → `Print`] → that is the QB64pe IDE's behaviour,
  which is the reference for M1. Keyword-case options come with the new formatter.
- [Other BASIC extensions also claim `.bas`] → we don't add `BASIC` aliases; the README explains
  `files.associations`.

## Migration Plan

New extension, no migration. Install locally with `npx @vscode/vsce package` and "Install from VSIX".

## Open Questions

- Should Run default to `QB64PE_NOPROMPT=y`? Decided default false in D6; revisit after the user has tried it.
