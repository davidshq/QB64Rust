# Proposal

## Why

M1 of the roadmap (`study\07`, R4) is the risk reducer. A VS Code extension that works with the **old** QB64pe
compiler gives users a modern editor now, even if the Rust rewrite stalls. It also fixes the client side
(ids, settings, tool discovery, diagnostics plumbing) that the M2+ language server will plug into. The first M1
check is answered: `qb64pe -z -q -w` reports BASIC-level errors and warnings in about 0.3–1.3 s without a C++
build (see `design.md`), so diagnostics on save are affordable.

## What Changes

- New VS Code extension in `vscode\`, written fresh in TypeScript (decision 2026-10-02). It is not built on any
  existing extension. Per `CLAUDE.md` rule 5: nothing is copied from `qb64pe-vscode`; small self-contained pieces of
  the user's `vscode-qb64fresh` (a grammar fragment, a helper, a fixture) may be taken after reading them in full,
  and anything they assume about QB64pe is checked against `qb64pe.exe`.
- Language id `qb64rust` for `.bas`, `.bi`, `.bm`; grammar scope `source.qb64rust`; commands and settings under
  `qb64rust.*`. No `BASIC` / `QBasic` aliases, so it can coexist with other BASIC extensions.
- A hand-written TextMate grammar and a language configuration (comments, brackets, `SUB`/`FUNCTION` folding).
- Build, Run, and Build and Run commands that call the old compiler (`qb64pe.exe -c -x -w … -o …`), plus a task
  provider so the same actions work as VS Code tasks. Disabled in untrusted workspaces.
- Diagnostics: on save (and on command), run `qb64pe -z -q -w` and turn its output into Problems entries,
  including errors located in `$INCLUDE` files and `file:line: warning:` lines. Build failures feed the same parser.
- Formatting: Format Document runs `qb64pe -y` on a temporary copy and replaces the document with the result; on a
  compile error it shows the error and leaves the document unchanged.
- CP437 is the default file encoding for the `qb64rust` language.
- Tool discovery: setting `qb64rust.compilerPath`, then `PATH`. A language status item shows whether the compiler was found.
- Unit tests for the output parser with fixtures recorded from `..\QB64pe\qb64pe.exe`, and a small integration test
  run in VS Code.
- Not in M1: snippets, keyword lists generated from `builtins.json`, help, language server, debugger.

## Capabilities

### New Capabilities

- `editor/language-support`: language registration, file associations, grammar, language configuration, default
  encoding.
- `editor/build-run`: compiler discovery and the build/run commands and tasks.
- `editor/compiler-diagnostics`: running the compiler for checking and mapping its output to diagnostics.
- `editor/formatting`: document formatting through the compiler's `-y` mode.

### Modified Capabilities

None (no specs exist yet).

## Impact

- New folder `vscode\` with its own `package.json`, `node_modules` (ignored), and build output `out\` (ignored).
  Dev dependencies: `typescript`, `@types/vscode`, `@types/node`, `mocha`, `@vscode/test-electron`, ESLint.
- Depends at run time on an installed QB64pe (`qb64pe.exe`); tests that need the compiler use
  `..\QB64pe\qb64pe.exe` and are skipped if it is missing.
- `.gitignore` gains `vscode/node_modules/` and `vscode/out/`.
- `STATUS.md` and `study\00` updated when M1 is done.
