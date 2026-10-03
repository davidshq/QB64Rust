# M1 extension walkthrough

Record for OpenSpec change `m1-vscode-extension-v0`, tasks 8.1 and 8.2.

## Automated part (done 2026-10-03)

All in `vscode\` (commands in `vscode\DEVELOPMENT.md`), against `..\QB64pe\qb64pe.exe` 4.7.0 on Windows:

| Suite | Result |
|---|---|
| `npm run test:unit` (parser against recorded fixtures, discovery, run queue) | 31 passing |
| `npm run test:grammar` (4 annotated fixtures) | 4 passing |
| `npm run test:integration`, VS Code 1.140 (stable) | 37 passing |
| `npm run test:integration:min`, VS Code 1.100.0 (oldest supported since 2026-10-03) | 37 passing |
| `scripts/record-fixtures.sh` run twice | no difference |
| `npm run package` | `qb64rust-0.1.0.vsix`, 49 files, 229 KB |

The integration tests cover every spec scenario except these, which need a person: untrusted workspace (the test
instance runs with trust disabled), what Problems and the language status look like, the editor-title run menu, and
Toggle Line Comment/folding as seen in the UI. Arguments reaching `COMMAND$` are tested automatically (a program
writes `COMMAND$` to a file).

Behaviour learned while testing:
- The compiler's output uses LF when piped, puts a `\x01` byte before " in line m of … included", and with `-x`
  prints a banner and progress bar inside the error block (`design.md` updated).
- `-y` changes a name's case to match its declaration, even in an include file (`print k` → `Print K` when
  `inc.bi` has `CONST K`).
- VS Code turns a full-document formatting edit into several minimal edits; the text result is the same.
- `onDidCloseTextDocument` fires minutes after the editor closes, so diagnostics are also cleared when the file's
  last tab closes.
- CP437 round trip: see `study\09`, last section (0x00 makes VS Code refuse the file as binary; a lone 0x0D is
  rewritten as the document's line ending; every other byte survives).

## By hand (to do)

Install: `code --install-extension vscode\qb64rust-0.1.0.vsix`, reload, and set `qb64rust.compilerPath` to
`..\QB64pe\qb64pe.exe` (user settings). Copy `..\QB64pe\tests\qbasic_testcases\misc\wumpus.bas` to a scratch
folder first: building writes an `.exe` next to the source, and `..\QB64pe\` is read-only for this project.

| # | Step | Expected | Result |
|---|---|---|---|
| 1 | Open `wumpus.bas` | Language mode "QB64"; language status (`{}` next to "QB64" in the status bar) shows `qb64pe`; comments, strings, keywords, numbers coloured | |
| 2 | Toggle Line Comment on a line; fold a `SUB` | `'` added/removed; the `SUB` folds | |
| 3 | Add a line `x = 1 +`, save | Within about a second, Problems shows "Expected variable/value after '+'" on that line; the whole line is underlined | |
| 4 | Type anything | The problem disappears | |
| 5 | Remove the line, save | Problems empty | |
| 6 | Add `DIM unusedvar AS INTEGER`, save | Warning "Unused variable: unusedvar% INTEGER" | |
| 7 | Format Document (Shift+Alt+F) | Indentation and keyword case as in the QB64pe IDE; document dirty, file on disk unchanged; no `.qb64rust-fmt-*` file in the folder | |
| 8 | Run "QB64: Build and Run" (or the run button in the editor title) | "QB64" output channel shows the build; the game starts in terminal "QB64"; no "built" notification (Build alone shows one) | |
| 9 | Set `qb64rust.compilerPath` to a wrong path | Language status shows "compiler not found" as an error without reload; Build shows an error with "Open Setting" | |
| 10 | Open the folder in Restricted Mode (Manage Workspace Trust → don't trust), run Build | Message that building needs a trusted workspace; nothing runs; highlighting still works | |
| 11 | (study 09) Open `verification\cp437_all_bytes.bin` copied as a `.bas`, choose "Open Anyway", type and delete a character, save, compare `sha256sum` with `cp437_all_bytes.sha256` | Records whether 0x00 survives through "Open Anyway" (0x0D is expected to change, see `study\09`) | |
