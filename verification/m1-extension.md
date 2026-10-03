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

## Walkthrough (done 2026-10-03)

All steps were done by Claude, not by hand: the packaged `.vsix` was installed into a separate VS Code 1.140
(own `--extensions-dir` and `--user-data-dir`), a small driver ran each step through the same commands the keys and
buttons run (`editor.action.commentLine`, `editor.fold`, `editor.action.formatDocument`, `qb64rust.buildAndRun`,
…), and the window was screenshotted and checked after each step. The run menu, the language status hover and the
"Open Setting" and "Open Anyway" buttons were clicked with the mouse. Step 10 ran in a second VS Code start
with workspace trust on (`security.workspace.trust.startupPrompt: never`, so the folder opened in Restricted
Mode). Screenshots were kept in the session scratchpad only.

Install: `code --install-extension vscode\qb64rust-0.1.0.vsix`, reload, and set `qb64rust.compilerPath` to
`..\QB64pe\qb64pe.exe` (user settings). Copy `..\QB64pe\tests\qbasic_testcases\misc\wumpus.bas` to a scratch
folder first: building writes an `.exe` next to the source, and `..\QB64pe\` is read-only for this project.

| # | Step | Expected | Result |
|---|---|---|---|
| 1 | Open `wumpus.bas` | Language mode "QB64"; language status (`{}` next to "QB64" in the status bar) shows `qb64pe`; comments, strings, keywords, numbers coloured | Pass. Status bar "QB64", "CP437"; hover shows `qb64pe — C:/code/qb64-new/QB64pe/qb64pe.exe (setting)`. `REM` keyword and comment text, `DIM`/`READ`, `PRINT`/`INPUT`, strings, numbers each coloured |
| 2 | Toggle Line Comment on a line; fold a `SUB` | `'` added/removed; the `SUB` folds | Pass. `DIM p(5)` → `' DIM p(5)` → back. `wumpus.bas` has no `SUB`, so one was appended to the copy; it folds to `SUB Demo ⋯` |
| 3 | Add a line `x = 1 +`, save | Within about a second, Problems shows "Expected variable/value after '+'" on that line; the whole line is underlined | Pass. 237 ms after save; Problems: "Expected variable/value after '+' qb64pe [Ln 3, Col 1]", error, columns 0–7 underlined |
| 4 | Type anything | The problem disappears | Pass (0 diagnostics 300 ms after typing a space) |
| 5 | Remove the line, save | Problems empty | Pass |
| 6 | Add `DIM unusedvar AS INTEGER`, save | Warning "Unused variable: unusedvar% INTEGER" | Pass. Warning on line 3, whole statement underlined in yellow |
| 7 | Format Document (Shift+Alt+F) | Indentation and keyword case as in the QB64pe IDE; document dirty, file on disk unchanged; no `.qb64rust-fmt-*` file in the folder | Pass. `Dim`, `Print`, `GoSub`, `End Sub`; `For`/`Select Case` bodies indented 4; dirty; SHA-256 of the file unchanged; folder holds only the two test files |
| 8 | Run "QB64: Build and Run" (or the run button in the editor title) | "QB64" output channel shows the build; the game starts in terminal "QB64"; no "built" notification (Build alone shows one) | Pass, with a corrected expectation: the terminal "QB64" runs `wumpus.exe`, but a program without `$CONSOLE` opens its own QB64 window ("INSTRUCTIONS (Y-N)?"), not the terminal. Build 6.4 s; output channel ends with `Built …\wumpus.exe`; no notification. Run menu lists Build and Run, Run, Build. Cosmetic: the `-x` progress bar fills about 50 output lines |
| 9 | Set `qb64rust.compilerPath` to a wrong path | Language status shows "compiler not found" as an error without reload; Build shows an error with "Open Setting" | Pass. `{}` gets an error badge; hover "compiler not found — qb64rust.compilerPath points to …"; Build: "QB64: qb64rust.compilerPath points to "C:/does/not/exist/qb64pe.exe", which does not exist." with "Open Setting", which opens Settings filtered to the setting. Back to the right path: status recovers without reload. Cosmetic: Settings shows the heading "Qb64rust: Compiler Path" |
| 10 | Open the folder in Restricted Mode (Manage Workspace Trust → don't trust), run Build | Message that building needs a trusted workspace; nothing runs; highlighting still works | Pass. Restricted Mode banner; Build: "QB64: building requires a trusted workspace. Nothing was executed."; the editor-title run button is greyed out; saving `x = 1 +` gave no diagnostic (check on save off); `wumpus.exe` unchanged; no terminal; highlighting as in step 1 |
| 11 | (study 09) Open `verification\cp437_all_bytes.bin` copied as a `.bas`, choose "Open Anyway", type and delete a character, save, compare `sha256sum` with `cp437_all_bytes.sha256` | Records whether 0x00 survives through "Open Anyway" (0x0D is expected to change, see `study\09`) | Done: **0x00 cannot be opened as text at all** in VS Code 1.140. The file shows "not displayed … binary or unsupported text encoding"; "Open Anyway" opens an editor picker; choosing "Text Editor" (by mouse, and again by Enter) brings the same notice back. A `.txt` copy (UTF-8) behaves the same, so this is VS Code, not the `cp437` default. The file stays unchanged (SHA-256 `40aff2e9…944880`) |
