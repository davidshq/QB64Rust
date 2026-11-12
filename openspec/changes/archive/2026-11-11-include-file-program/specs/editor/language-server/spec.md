# Spec Delta

## MODIFIED Requirements

### Requirement: Format failure
When the parser reports an error in the program, the server SHALL answer the formatting request with no edits,
SHALL publish the diagnostics as a check publishes them, and SHALL send a `window/showMessage` error that names
the first error's file and line. An error of a later stage SHALL NOT stop formatting (spec `compiler/format`,
"Program with an error"): the server SHALL answer with the edits, publish nothing and send no message.

#### Scenario: Syntax error
- **WHEN** the document's line 2 is `x = 1 +` and the client asks to format it
- **THEN** the answer is an empty list of edits, an error on line 2 is published, and a message names line 2

#### Scenario: A SUB that is not defined
- **WHEN** the document's only line is `call   missing` and the client asks to format it
- **THEN** the answer replaces the line with `Call missing`, and no message is sent

## ADDED Requirements

### Requirement: Program of an included file
The lookup SHALL be the same for every open file, whatever its extension. An open document that is not a file
SHALL be its own program. For any other the server SHALL take as its program the first of these whose program
includes it, directly or through other files: the `main` entry of the settings file; the open `.bas` document
that is its own program, the one opened first; a `.bas` file found on disk. A document with none SHALL be its
own program.

#### Scenario: An open includer
- **WHEN** `main.bas`, which includes `lib.bm`, and `lib.bm` are both open
- **THEN** the program of `lib.bm` is `main.bas`

#### Scenario: Through another include
- **WHEN** open `main.bas` includes `all.bi`, which includes `lib.bm`, and `lib.bm` is open
- **THEN** the program of `lib.bm` is `main.bas`

#### Scenario: A .bas file that another program includes
- **WHEN** `other.bas` is open and open `main.bas` includes it
- **THEN** the program of `other.bas` is `main.bas`

#### Scenario: A chain of .bas files
- **WHEN** open `main.bas` includes open `mid.bas`, which includes open `lib.bm`
- **THEN** the program of `mid.bas` and of `lib.bm` is `main.bas`

#### Scenario: A .bas file that nothing includes
- **WHEN** `main.bas` is open and no file open or on disk includes it
- **THEN** `main.bas` is its own program, and its check reports the errors of every stage as today

#### Scenario: The includer no longer includes it
- **WHEN** the `$INCLUDE` line for `lib.bm` is removed from open `main.bas`, and no other program includes it
- **THEN** after the next parse `lib.bm` is its own program

### Requirement: Main program in the settings file
The nearest `qb64rust.toml` in the document's folder or a folder above it MAY hold a `[project]` table with
`main = "<path>"`, a `.bas` file relative to the settings file's folder. When that file exists and its program
includes the document, it SHALL be the document's program, before any other. A `main` that is missing, is not a
string, or names a program that does not include the document SHALL be passed over, and the lookup SHALL go on.

#### Scenario: Two programs include the file
- **WHEN** `game.bas` and `tests\rules_test.bas` both include `lib.bm`, both are open, `rules_test.bas` was
  opened first, and `qb64rust.toml` holds `[project]` and `main = "game.bas"`
- **THEN** the program of `lib.bm` is `game.bas`

#### Scenario: Main does not include the document
- **WHEN** `main = "game.bas"` is set, `game.bas` does not include `tools.bm`, and open `tool.bas` does
- **THEN** the program of `tools.bm` is `tool.bas`

#### Scenario: The settings file changes
- **WHEN** `main` is changed on disk from `game.bas` to `editor.bas`, which also includes open `lib.bm`
- **THEN** the program of `lib.bm` becomes `editor.bas`, with no edit of `lib.bm`

### Requirement: The named main file is its own program
The file that the `main` entry names SHALL be its own program, even when another program includes it.

#### Scenario: Main is included by another program
- **WHEN** `main = "game.bas"` is set, `game_test.bas` includes `game.bas`, and both are open
- **THEN** `game.bas` is its own program

### Requirement: Order of programs found in one folder
Among several `.bas` files in one folder whose programs include the document, a file that none of the others
includes SHALL come first, then a name that does not end in `_test.bas` before one that does, and then the first
by name without regard to letter case.

#### Scenario: A test file beside the program
- **WHEN** `game.bas` and `game_test.bas` in one folder both include open `lib.bm`, and neither is open
- **THEN** the program of `lib.bm` is `game.bas`

#### Scenario: The top of a chain on disk
- **WHEN** `main.bas` includes `mid.bas`, which includes open `lib.bm`, and neither `.bas` file is open
- **THEN** the program of `lib.bm` is `main.bas`

### Requirement: Program found on disk
When neither the settings file nor an open document gives a program, the server SHALL look at the `.bas` files in
the document's folder, then in each folder above it, up to the folder of the nearest `qb64rust.toml` or the
workspace folder that holds the document. It SHALL stop at the first folder with a program that includes the
document.

#### Scenario: The program is not open
- **WHEN** `turn.bm` is open, and `CWSTRAT.BAS` beside it includes it and is not open
- **THEN** the program of `turn.bm` is `CWSTRAT.BAS`, read from disk

#### Scenario: The program is one folder up
- **WHEN** `src\lib\draw.bm` is open and `src\game.bas` includes `lib\draw.bm`
- **THEN** the program of `draw.bm` is `src\game.bas`

#### Scenario: A .bas file whose includer is not open
- **WHEN** `other.bas` is open, and `main.bas` beside it includes it and is not open
- **THEN** the program of `other.bas` is `main.bas`, read from disk

#### Scenario: No program
- **WHEN** no `.bas` file in the folders searched includes open `lib.bm`
- **THEN** `lib.bm` is its own program

### Requirement: Answers from the document's program
For a document whose program is another file, the server SHALL answer from that program, read as a check reads
it (open documents first, then the disk): the parser's diagnostics after a change, `qb64rust/check`, hover,
definition, references, rename, completion, document symbols and folding ranges. Diagnostics SHALL be published
for the program's open documents only when its main file is not open. An edit of the document SHALL parse its
program again.

#### Scenario: Check of an include file
- **WHEN** `lib.bm` calls `draw`, which `main.bas` defines, and `qb64rust/check` names `lib.bm`
- **THEN** no error is published for the call, and the answer counts the errors of `main.bas`'s program

#### Scenario: Definition in the main file
- **WHEN** the cursor is on `draw` in `lib.bm`, and `SUB draw` is on line 40 of `main.bas`, which is not open
- **THEN** the definition is line 40 of `main.bas`

#### Scenario: Shared variable
- **WHEN** `main.bas` has `DIM SHARED total AS LONG` and the cursor is on `total` in a SUB of `lib.bm`
- **THEN** the hover says `total` is a shared `LONG`

#### Scenario: Error elsewhere in a program that is not open
- **WHEN** `lib.bm`'s program `main.bas` is not open and has a syntax error on its line 3
- **THEN** no diagnostic is published under `main.bas`'s uri

### Requirement: Formatting an included file
For a document whose program is another file, `textDocument/formatting` and `textDocument/rangeFormatting` SHALL
lay out the document's own text, with the names spelled as that program declares them. The program's main file
SHALL NOT be edited.

#### Scenario: Case from the program
- **WHEN** `main.bas` has `DIM SHARED Total AS LONG`, includes `lib.bm`, and `lib.bm` holds `total=1`
- **THEN** formatting `lib.bm` gives `Total = 1`, and the answer holds no edit of `main.bas`

### Requirement: Included file without a program
For a document that is its own program and whose name does not end in `.bas`, `qb64rust/check` SHALL publish and
count the parser's diagnostics only. Formatting SHALL follow spec `compiler/format`, "Names in an incomplete
program". The other requests SHALL be answered from the file alone.

#### Scenario: Check of a lone include file
- **WHEN** `lib.bm` calls a SUB that no file defines, no program includes it, and `qb64rust/check` names it
- **THEN** the answer is `{ "errors": 0, "warnings": 0 }` and no diagnostic is published

#### Scenario: Syntax error in a lone include file
- **WHEN** line 2 of that `lib.bm` is `x = 1 +` and it is checked
- **THEN** the answer counts one error, published on line 2

### Requirement: Program notification
The server SHALL send the notification `qb64rust/program` with `{ "uri", "program", "source" }` when a document
is opened, and again each time its program changes. `program` SHALL be the uri of the
program's main file, or null when the document is its own program. `source` SHALL be `settings`, `open`, `disk`
or `none`.

#### Scenario: Found on disk
- **WHEN** `turn.bm` is opened and its program is `CWSTRAT.BAS`, found on disk
- **THEN** the notification carries `CWSTRAT.BAS`'s uri and `disk`

### Requirement: Program status item
The client SHALL show the program's file name for the active editor in a language status item. For `none` it
SHALL show "no program found", except for a `.bas` document, where it SHALL hide the item.

#### Scenario: Program shown
- **WHEN** `turn.bm` is the active editor and its program is `CWSTRAT.BAS`
- **THEN** the status item reads `CWSTRAT.BAS`

#### Scenario: No program
- **WHEN** `lib.bm` is opened and no program includes it
- **THEN** the notification carries a null program and `none`, and the status item reads "no program found"

#### Scenario: A .bas file inside another program
- **WHEN** `other.bas` is opened and its program is open `main.bas`
- **THEN** the notification carries `main.bas`'s uri and `open`, and the status item reads `main.bas`

#### Scenario: A .bas file that is its own program
- **WHEN** `main.bas` is opened and nothing includes it
- **THEN** the notification carries a null program and `none`, and no status item is shown
