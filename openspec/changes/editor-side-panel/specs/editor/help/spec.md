# Spec Delta

## Purpose

Help inside the editor: the project's own language reference as a tree, a search, and a page shown beside the
code, with examples that run. It works offline and holds no QB64pe wiki text.

## ADDED Requirements

### Requirement: The Help view
The Help view SHALL show the reference as a tree: "First steps", "Topics" with one row per chapter, "Every
keyword, A to Z" with one row per keyword page, "Error messages and codes", "Coming from QuickBASIC 4.5" and
"Coming from QB64pe". A click on a page's row SHALL open that page in the help tab.

#### Scenario: A keyword
- **WHEN** the user opens "Every keyword, A to Z" and clicks `_CLAMP`
- **THEN** the help tab shows the page of `_CLAMP`

### Requirement: The help tab
A help page SHALL open in one editor tab, "QB64 Help", beside the active editor and never in place of it. A
second page SHALL replace the first in the same tab. The tab SHALL have Back and Forward. A link to another page
SHALL open that page in the tab. A link to a web address SHALL open in the browser.

#### Scenario: Beside the code
- **WHEN** `health.bas` is the only editor and a help page opens
- **THEN** `health.bas` stays visible at the left and the help tab is at the right

#### Scenario: A second page and Back
- **WHEN** the tab shows `_CLAMP`, the user clicks the "See also" link `_MIN`, and then Back
- **THEN** the same tab shows `_MIN`, and then `_CLAMP` again

### Requirement: What a help page shows
A page SHALL show the reference page's text as written: its headings, its syntax, its tables and its examples
with their recorded output. The page SHALL follow VS Code's color theme, a high-contrast theme too, and the
example's code SHALL be colored as QB64.

#### Scenario: The recorded output
- **WHEN** the page of `_CLAMP` is shown
- **THEN** under its example it shows the two output lines recorded for it

### Requirement: The buttons of an example
Each example of a help page that is a whole program SHALL have the buttons "Run this example", "Open as a new
program" and "Copy". "Run this example" SHALL build and run a copy kept in the extension's own storage, never
in the user's folders. "Open as a new program" SHALL give its text to "QB64: New Program".

#### Scenario: Run this example
- **WHEN** the user clicks "Run this example" on the page of `_CLAMP`
- **THEN** the example is built and run, and no file appears in the user's workspace or documents

#### Scenario: An untrusted workspace
- **WHEN** the workspace is not trusted
- **THEN** "Run this example" is disabled, and "Copy" works

### Requirement: Search of the help
The command "QB64: Search Help" SHALL list every page by its title and first sentence, narrowed as the user
types, and open the chosen page in the help tab. The Help view's title SHALL have a button for it.

#### Scenario: A word of the sentence
- **WHEN** the user runs "QB64: Search Help" and types `bounds`
- **THEN** the list holds `_CLAMP`, whose first sentence has the word

### Requirement: Help for the word under the cursor
The command "QB64: Help for This Word" SHALL act on the word at the cursor of a QB64 file. For a built-in or a
keyword that has a page, it SHALL open the page in the help tab. For a name the program defines, it SHALL go to
its definition. For anything else it SHALL open "QB64: Search Help" with the word typed in.

#### Scenario: A built-in
- **WHEN** the cursor is on `_CLAMP` and the user runs the command
- **THEN** the help tab shows the page of `_CLAMP`

#### Scenario: A SUB of the program
- **WHEN** the cursor is on a call of the program's `DrawBall`
- **THEN** the editor goes to `SUB DrawBall`

#### Scenario: Without the language server
- **WHEN** the language server does not run and the cursor is on `_CLAMP`
- **THEN** the page of `_CLAMP` opens all the same, found by the word's spelling

### Requirement: Help without a compiler
The Help view, the help tab and the search SHALL work in an untrusted workspace and without the `qb64rust`
binary.

#### Scenario: An untrusted workspace
- **WHEN** the workspace is not trusted and the user clicks a keyword in the Help view
- **THEN** its page opens
