# 55. Making the editor feel like BASIC: a panel review, with mockups

Written 2026-11-25. It answers the user's request of that day: "I think I want to extend the existing vsc
extension or to add another extension. Part of BASIC is how easy and intuitive it is to use, and VSC doesn't feel
100% easy and intuitive. I feel like maybe we need a side panel or a main panel or something? I'd like a panel of
experts to consider what we might need and what it might look like. I'd like some mockups as well."

It is a review, so it stays active until the user has answered §7 (`CLAUDE.md` working rule 7). Nothing here is
decided and nothing is built. The recommendations are the panel's, which Claude role-played; none is the user's
ruling.

**The mockups are in `study\55-mockups\index.html`.** Open the file in a browser. It has five screens, each a
drawing of a VS Code window with numbered notes. They are drawings in HTML and CSS, not the extension.

**Sources.** The extension as it is: `vscode\package.json`, `vscode\README.md`, `vscode\src\`. The open editor
rows: `SOMEDAY.md`, "VS Code parity left". The old IDE: `..\QB64pe\source\ide\ide_methods.bas`. VS Code's
documents, read on 2026-11-25 through a web search's extracts (`CLAUDE.md` working rule 17); §8 lists what was
not checked.

- [Contribution Points](https://code.visualstudio.com/api/references/contribution-points): view containers go in
  the activity bar or the panel; `viewsWelcome` fills an empty tree view.
- [Release notes of version 1.106](https://code.visualstudio.com/updates/v1_106) (October 2025): a view
  container can be registered in the secondary side bar. The extension asks for VS Code 1.100.
- UX guidelines: [Webviews](https://code.visualstudio.com/api/ux-guidelines/webviews),
  [Views](https://code.visualstudio.com/api/ux-guidelines/views),
  [Walkthroughs](https://code.visualstudio.com/api/ux-guidelines/walkthroughs), and the
  [Webview API guide](https://code.visualstudio.com/api/extension-guides/webview).
- Cross-origin isolation in VS Code: [issue 137884](https://github.com/microsoft/vscode/issues/137884) and the
  team's discussion "Towards cross origin isolation" (2022).
- The Webview UI Toolkit: its repository, archived on 2025-01-06.

## 1. What is hard today

These are read from the extension's code and manual, not from watching a beginner. No beginner has used it yet.

1. **Nothing is visible.** The extension adds no icon, no view and no page. Its seven commands are in the Command
   Palette under "QB64". One small run button shows at the top right of the editor, and only when a `.bas` file
   is open.
2. **An empty start.** After the install nothing happens until a `.bas` file is opened. "QB64: New Project" is a
   palette command that asks for a folder and writes a `main.bas` and a `qb64rust.toml`. QBasic started with an
   empty editor and a cursor.
3. **The first run surprises.** The first build shows a dialog that offers a 190 MB download, and then builds the
   runtime for about 40 seconds. What is happening is told only in the "QB64" output channel.
4. **F5 does not mean "run".** F5 builds with `--debug` and starts a debug session. VS Code then shows its debug
   toolbar and may switch the side bar. In the QB64pe IDE the menu item is "Start  F5".
5. **No help.** A hover on `_CLAMP` shows `_CLAMP(value, minimum, maximum)` and no word of what it does. F1 does
   nothing for BASIC. Yet `docs\reference` holds 401 keyword pages and 39 chapters of our own, and
   `tests\reference\examples` holds 333 example programs with their recorded output. None of it ships in the
   extension.
6. **No examples** to open, run and change.
7. **"Is my program right?" has no one place.** The old IDE prints "OK" under the editor, or the error. Here the
   answer is a count in the status bar and a list in the Problems panel.
8. **VS Code's own parts.** Source control, extensions, workspace trust and the rest surround the editor. An
   extension cannot remove them (§6).

Rows 1 to 7 are ours to fix. Row 8 is VS Code's.

## 2. What it might look like

The five mockups, and what each is built from.

| Mockup | What it is | Built from | The panel |
|---|---|---|---|
| 1. The QB64 side panel | One activity bar icon. Views: Program (a large Run button, the program's name, "OK" or the first error), SUBs and FUNCTIONs of the whole program, Examples, Help. A Run button and the same status in the status bar | One small webview view (Program), three native tree views, a status bar item | First |
| 2. Help beside the code | F1 opens the word's reference page in a tab to the right. Its example has "Run this example" and "Open as a new program". The hover gains the page's first sentence | One webview panel. Pages made from `docs\reference` when the extension is packaged | First |
| 3. First run: a walkthrough | VS Code's own "Get Started" checklist: write a program, run it, look a word up, try an example, fix a mistake, put it on the web | `contributes.walkthroughs`. No webview | First |
| 4. A home page | A page of our own in the editor area: New program, Open, Examples, Help, recent programs, setup state | A webview panel | Not first |
| 5. Run beside the code | The program's screen in a tab beside its source, from the web build | A webview panel that shows the wasm page | Later, after a measurement (§5) |

## 3. The panel

The six standing roles, and three more. A **user-experience designer**, because the request is about how the
editor feels. A **teacher of beginners**, because "easy and intuitive" is judged by someone who has never
programmed. A **VS Code extension engineer**, because what VS Code allows decides what can be built.

**The user-experience designer.** The extension is not short of features. It is short of places. A beginner has
three moments, and each needs one obvious place. The first five minutes: the walkthrough. Every minute after: the
side panel, with one Run button and one status line. Being stuck: F1. Do not build a second user interface on top
of VS Code. One icon, one panel, one help tab.

**The teacher.** A beginner needs four things: one button that runs the program, examples to change, help that
shows an example with its output, and errors in plain words. A beginner does not need a project, a folder, a
settings file or the word "toolchain". "New program" must give one file and ask nothing. The error line of
mockup 1 says "THEN is missing after the condition of this IF". Whether the compiler's messages read that
plainly today should be checked on the twenty most common mistakes.

**The QB64 engineer.** People who come from the old IDE will look for four things: F5 to run, F1 for help, the
F2 list of SUBs, and "OK" under the editor. All four are in mockups 1 and 2. The ASCII chart (`SOMEDAY.md`, group
`editor-previews`) fits the side panel as one more view later. The old IDE's SUBs list covers the whole program;
VS Code's Outline covers one file, so the view is not a duplicate.

**The VS Code extension engineer.** Build with native parts wherever they do the job. Tree views, the walkthrough
and status bar items follow the user's theme, work with a keyboard and a screen reader, and can be tested
through the API. A webview has none of that for free: it needs a content security policy, VS Code's color
variables, saved state and a message protocol. VS Code's guidelines say to use webviews sparingly, to limit
webview views, and not to repeat the Welcome page in one. Microsoft's Webview UI Toolkit was archived in January
2025, so a webview here is plain HTML and CSS. That leaves two webviews worth their cost: the Program view,
because a tree view cannot hold a large button and a status box, and the help page, because a tree cannot show
a page. A native fallback exists for the Program view: `viewsWelcome` draws buttons, but only in an empty tree
view. The home page (mockup 4) is the one the guidelines argue against.

**The compiler and languages engineer.** The language server already knows what the views need. It finds the
program of an include file, it holds the symbols of every file, and it has the diagnostics. The side panel
needs one new request: the program's main file and its SUBs and FUNCTIONs across all its files. F1 should ask
the server what the word under the cursor is, as hover does. Then F1 on a user's SUB goes to its definition, and
F1 on `_CLAMP` opens the page.

**The Rust engineer.** Nothing changes in the compiler for mockups 1 to 3. The help pages are data:
`tools\reference\reference.py` already renders the reference, so it gains an output as HTML fragments, made when
the extension is packaged. The 333 examples ship as they are. `reference.py --check` and the examples' step
already run at each merge, so what ships is checked.

**The test engineer.** Each view gets a model free of VS Code, as `buildPlan.ts` and `formatPlan.ts` are today,
with unit tests. The integration suite checks that each command exists, that each walkthrough step names a real
command, and what the trees hold. A webview's HTML is a string that a unit test can compare. What no test can
judge is whether it feels easy. That needs a person who has never used it, watched for ten minutes.

**The pragmatic engineer.** One extension (§6). Build mockups 1 to 3 as one change, because they share the
examples and the help pages. Do not build the home page until someone has tried the walkthrough and missed it.
Do not build mockup 5 until its question is measured. Four small rows of `SOMEDAY.md` make the editor easier
with no panel at all and belong in the same change: snippets for the blocks, signature help, a click on an
`$INCLUDE` path, and F1.

## 4. The recommendation

Build mockups 1, 2 and 3 as one OpenSpec change. It adds a mechanism and spans the extension, the language
server and the reference tool, so it needs a design (`CLAUDE.md` working rule 11).

1. The QB64 view container with four views: Program, SUBs and FUNCTIONs, Examples, Help.
2. A Run button and the "OK" or first-error status in the status bar.
3. The help tab, F1, and a hover with the page's first sentence. The reference pages and the 333 examples ship
   in the extension.
4. The walkthrough, and "QB64: New Program", which makes one file and asks nothing.
5. A short set of new example programs: first steps, graphics, sound, small games. Our own, as rule 3 asks.

**Effort, in Claude's time:** about two sessions to write and test. What stays after that:

- Two webviews to maintain and to check by eye in a light, a dark and a high-contrast theme.
- A longer integration suite in CI.
- A larger extension package. The reference pages and examples are text; their size is not measured yet.
- The user's time. Only a person can say whether it feels easy, so each screen needs a look and likely a second
  round.

## 5. Running the program beside the code

Mockup 5 is the one closest to the feel of BASIC, and the only one with an open technical question.

The web build uses threads. A browser gives a page threads only when the page is cross-origin isolated, which is
why `qb64rust serve` sends two headers. In 2022 the VS Code team wrote that a webview is isolated only when VS
Code is started with `--enable-coi`. What today's VS Code does was not found in its documents. A page inside a
frame is isolated only when the page around it is, so serving our page from `127.0.0.1` into a webview does not
get around this.

Two ways forward:

- **A one-thread web build.** It is already an open question for other reasons (question 5 of `study\51` §10).
  It would run in any webview.
- **A measurement.** Open a webview in the current VS Code and read `crossOriginIsolated`. About an hour, once
  the OpenSpec change `wasm-publish` is built.

A desktop program's own window cannot be placed inside VS Code. So mockup 5 always shows the web build, and a
program can behave differently there. The tab must say so.

## 6. One extension or two

**The panel: one extension.** A second extension would need the compiler's discovery, the language server and
the build queue of the first, through an exported API. It would be a second package to release and a second
suite to run. A beginner would have to install two things, which is the opposite of easy. Everything in mockups
1 to 5 is something the existing extension may contribute.

What a second extension could not do either is remove VS Code's own parts (§1 row 8). An extension cannot hide
another extension's views or VS Code's activity bar items. A VS Code profile can, and the user can export one as
a file. Whether an extension may ship or apply a profile was not checked (§8).

The decision of 2026-10-02 stands under this review: the text-mode IDE is not ported, and parity comes through a
VS Code extension (`DECISIONS.md`).

## 7. For the user to decide

**Answered on 2026-11-25** (`DECISIONS.md`): question 1, one extension; question 2, yes; question 4, F5 keeps
debugging. Question 3: a save dialog each time, as VS Code asks for any new file, and no folder of our own.
Help gets no key, so F1 in mockup 2 is not built. The plan is the OpenSpec change `editor-side-panel`.

1. **One extension or two?** The panel: one (§6). *Built*: one extension, `editor-side-panel` (2026-11-25).
2. **Is the first step mockups 1, 2 and 3, as one OpenSpec change?** The panel: yes (§4). *Built*: the OpenSpec
   change `editor-side-panel` (2026-11-25), not archived until its integration tests have run in CI.
3. **Where does "New program" put its file?** A beginner should be asked nothing, but a build needs a file on
   disk. The panel: a folder `QB64 Programs` in the user's documents, opened as the workspace, with a setting to
   change it. The other ways: ask for a folder each time, as "New Project" does now, or keep the text unsaved
   and ask at the first run.
4. **What does F5 do?** Today it debugs. The teacher and the QB64 engineer: F5 runs, as in QBasic and the QB64pe
   IDE. The VS Code extension engineer: VS Code's users expect F5 to debug and Ctrl+F5 to run. The panel: F5 runs
   for QB64 files, Debug keeps its button, and a setting gives VS Code's way back. *Built* as the user answered:
   F5 still debugs, and Run is the Program view's button and the status bar's item (2026-11-25).
5. **The home page (mockup 4): later, or never?** The panel: not now. Ask again after a beginner has tried the
   walkthrough. *Built* at the user's request after trying the side panel (2026-11-26): "QB64 Home", with a
   checkbox that stops it opening by itself (`editor-side-panel` design D11).
6. **Running beside the code (mockup 5): measure it after `wasm-publish`?** The panel: yes, an hour's
   measurement, then decide with question 5 of `study\51`.
7. **A simpler VS Code?** Should the project offer a profile that hides what a BASIC beginner does not need? The
   panel: later, after checking what a profile can carry.
8. **Who tries it?** The panel's test engineer asks for one person new to it, watched for ten minutes, before
   the look is called done. Is there such a person?

## 8. Not checked

- No beginner was watched. §1 is a reading of the code and the manual.
- The current state of cross-origin isolation in VS Code's webviews. The sources are from 2022.
- Whether an extension can ship or apply a VS Code profile.
- The exact manifest form of a webview view. The search's extract of the contribution-points page did not
  include it. (Checked since in VS Code 1.100's own schema: `"type": "webview"`; `editor-side-panel` design,
  "Context".)
- The size the reference pages and examples add to the extension package. (Measured since: 0.7 MB compressed;
  `editor-side-panel` design, "Risks".)
- How plainly the compiler's messages read for a beginner's common mistakes. (Listed since, §9.)
- The UX guideline pages were dated about five years old by the search. They may have changed.

## 9. The compiler's messages for twenty beginner's mistakes

Each program is two or three lines with one mistake, checked with `qb64rust -q -z` (the release build of
2026-11-25, at the OpenSpec change `editor-side-panel`). The message is as the compiler prints it, after the
line and column. A message marked *poor* would puzzle a beginner; this list is the input of a later change of
the wording (`SOMEDAY.md`, group `beginner-messages`). Nothing here is changed yet.

| # | Mistake | The program | The message |
|---|---|---|---|
| 1 | A missing `THEN` | `IF x > 1 PRINT "big"` | `not supported yet: word PRINT (expected THEN or GOTO)`. *Poor*: it is a mistake, not a missing feature |
| 2 | A string never closed | `PRINT "Hello` | None. QB64 ends the string at the end of the line, as QBasic does |
| 3 | `NEXT` without `FOR` | `NEXT i` | `` `NEXT` without `FOR` `` |
| 4 | `FOR` without `NEXT` | `FOR i = 1 TO 3` and no `NEXT` | `` `FOR` without `NEXT` ``, at the `FOR` |
| 5 | `IF` without `END IF` | A block `IF` and no `END IF` | `` `IF` without `END IF` `` |
| 6 | `END IF` without `IF` | `END IF` alone | `` `END IF` without `IF` `` |
| 7 | `LOOP` without `DO` | `LOOP` alone | `` `LOOP` without `DO` `` |
| 8 | `DO` without `LOOP` | `DO` and no `LOOP` | `` `DO` without `LOOP` `` |
| 9 | A misspelled keyword | `PRNT "Hello"` | `` `PRNT` is no statement and no SUB ``, with `QB64pe: Syntax error`. Only from the whole check, on save: the editor shows nothing as you type |
| 10 | A missing `)` | `PRINT (2 + 3` | `` expected `)` `` |
| 11 | Text put in a number variable | `x = "five"` | `cannot store a string in a number variable` |
| 12 | A number put in a string variable | `a$ = 5` | `cannot store a number in a string variable` |
| 13 | A missing operator | `x = 2 3` | `expected the end of the statement`. *Poor*: it does not say that an operator is missing between `2` and `3` |
| 14 | A SUB called with too many arguments | `Greet 1, 2` for `SUB Greet (n)` | `` `Greet` takes 1 argument, not 2 `` |
| 15 | A FUNCTION used but never written | `PRINT Twice(3)` | None. *Poor* for a beginner: the program compiles. What it does when run was not measured here |
| 16 | An array used past its size | `DIM a(5)` then `a(10) = 1` | None at compile time. What it does when run was not measured here |
| 17 | A keyword as a variable name | `print = 3` | `not supported yet: operator = (expected an expression)`. *Poor*: it should say that `PRINT` is a keyword |
| 18 | `GOTO` a label that is not there | `GOTO finish` | `` label `finish` is not defined in the main module `` |
| 19 | `SELECT CASE` without `END SELECT` | `SELECT CASE x` … and no `END SELECT` | `` `SELECT CASE` without `END SELECT` `` |
| 20 | `END SUB` without `SUB` | `END SUB` alone | `` `END SUB` without a `SUB` `` |
