# editor/language-server Specification

## Purpose
A thin language server, part of the `qb64rust` binary, that gives the editor syntax errors as the user types,
the document outline, folding ranges and go to definition for procedures and labels, from the new compiler's
parser alone; and, on the client's request, a check of a program by the compiler's whole front end, which the
driver gives the server (change `extension-diagnostics`).

## Requirements

### Requirement: Server start
`qb64rust lsp` SHALL run a Language Server Protocol server over stdin and stdout until `shutdown` and `exit`, and
SHALL advertise exactly these capabilities: full text synchronisation, document symbols, folding ranges and go to
definition.

#### Scenario: Initialize and exit
- **WHEN** a client sends `initialize`, `initialized`, `shutdown` and `exit`
- **THEN** the server answers the two requests and the process ends with exit code 0

### Requirement: Syntax diagnostics
For every open document the server SHALL publish the parser's diagnostics of the program the document is, after
every change, under the uri of the file each diagnostic is in, with severity error and source `qb64rust`. It SHALL
NOT publish anything from the later stages (no "not supported yet" of `sema`, no type errors) after a change; the
later stages run only for a check (requirement "Check request"). A diagnostic the
parser itself marks "not supported yet" (a syntax error at a BASIC word, an expression nested too deep) is
published with the marker in its message. An error with the old compiler's wording (requirement "Diagnostics
output" of `compiler/cli`) SHALL have it as a second line of its message, `QB64pe: <wording>`.

#### Scenario: Error appears and goes
- **WHEN** the user types `x = 1 +` on a line and then completes it to `x = 1 + 2`
- **THEN** a diagnostic appears on that line after the first edit and is gone after the second, without a save

#### Scenario: Unsupported statement is no diagnostic
- **WHEN** an open document contains a statement `sema` rejects and the parser accepts, and no check has run
- **THEN** the server publishes no diagnostic for that line

#### Scenario: Error in an included file
- **WHEN** the open main file includes `lib.bi` and `lib.bi` has a syntax error on line 3
- **THEN** the diagnostic is published under `lib.bi`'s uri at line 3, and none under the main file for it

#### Scenario: The old compiler's wording
- **WHEN** a parser error the table holds is published
- **THEN** its message is the error's, a line end, and `QB64pe: ` with the old compiler's wording

### Requirement: Document text in its encoding
The server SHALL parse the bytes of the document's text in the document's encoding, told by the client (a default
in the initialization options and a `qb64rust/documentEncoding` notification per document), CP437 when nothing is
told. A character the encoding cannot represent SHALL be parsed as one byte `?` per UTF-16 unit, the bytes VS Code
writes for it on save (a character outside the Basic Multilingual Plane gives `??`).

#### Scenario: CP437 string literal
- **WHEN** a CP437 document contains `PRINT "║═╗"` (three bytes above 127 in the file)
- **THEN** the parse sees a three-byte string literal and reports no diagnostic

### Requirement: Positions
Diagnostic ranges and symbol ranges SHALL be reported in UTF-16 code units, the only position encoding VS Code's
language client accepts (`vscode-languageclient` 10.1.2 offers `utf-16` alone), mapped from the parser's byte
offsets; positions the client sends SHALL be read the same way. For a single-byte encoding one byte is one
column.

#### Scenario: Column after a multi-byte character
- **WHEN** a UTF-8 document's line is `PRINT "é" +` with the error after `+`
- **THEN** the diagnostic's column is the character column of the editor, not the byte column

### Requirement: Included files
When the parser meets `$INCLUDE`, the server SHALL look the file up as the compiler does (the including file's
folder, then the include root from the initialization options, never the workspace folder) and SHALL use the open
document's current text when the file is open in the editor, the file on disk otherwise.

#### Scenario: Unsaved include
- **WHEN** `lib.bi` is open with an unsaved syntax error and the main file that includes it is edited
- **THEN** the main program's parse reports the error under `lib.bi`'s uri

#### Scenario: Include edited reparses the includer
- **WHEN** `lib.bi` is edited while `main.bas`, which includes it, is also open
- **THEN** `main.bas`'s program is parsed again and its diagnostics updated

### Requirement: Document symbols
The server SHALL answer `textDocument/documentSymbol` with every `SUB` and `FUNCTION` (name, the whole block as
range, the name as selection range), every `TYPE` with its members as children, and every label, nested in the
procedure it stands in.

#### Scenario: Outline of a program
- **WHEN** a document has `SUB a`, a label `top:` in the main module and a label `again:` inside `a`
- **THEN** the symbols are `a` (with child `again`) and `top`

### Requirement: Folding ranges
The server SHALL answer `textDocument/foldingRange` with a range for every multi-line block (`SUB`, `FUNCTION`,
`TYPE`, multi-line `IF`, `FOR`, `DO`, `WHILE`, `SELECT` and each `CASE`), for every `$IF` region, and for every
run of three or more comment lines.

#### Scenario: DO loop folds
- **WHEN** a document has `DO` on line 2 and `LOOP` on line 5
- **THEN** a folding range from line 2 to line 4 is offered

### Requirement: Go to definition
The server SHALL answer `textDocument/definition` for a procedure name (in a call with or without `CALL`, in an
expression, in a `DECLARE`) with the location of the first `SUB`/`FUNCTION` header of that name in file order,
and for a label name in a jump (`GOTO`, `GOSUB`, `ON … GOTO/GOSUB`, `ON ERROR GOTO`, `RESUME`, `RETURN`) with the
label's definition in the same body. A name with no definition, or a variable, SHALL give an empty result.

#### Scenario: Call before the definition
- **WHEN** line 1 is `bump 3` and `SUB bump (n)` is on line 10
- **THEN** definition at `bump` on line 1 is line 10

#### Scenario: Label in the right body
- **WHEN** the main module and `SUB a` both have a label `again:` and the cursor is on `GOTO again` inside `a`
- **THEN** the result is the label inside `a`

### Requirement: Reparse and cancellation
The server SHALL parse a program again after each change, dropping a pending parse when a newer change arrives,
and SHALL answer a cancelled request with the `RequestCancelled` error.

#### Scenario: Burst of edits
- **WHEN** three changes arrive within the debounce interval
- **THEN** one parse runs and its diagnostics are published

### Requirement: Check request
The server SHALL answer the request `qb64rust/check`, whose parameter names a document by `uri`, by running the
compiler's whole front end over that document's program and publishing what it finds. The program is read as
the live parse reads it (open documents first, then the disk, in their encodings). The answer SHALL be the
numbers of errors and warnings found, `{ "errors": n, "warnings": n }`.

#### Scenario: Type error on check
- **WHEN** an open document's line 2 is `x$ = 1` and the client sends `qb64rust/check` for it
- **THEN** the answer is `{ "errors": 1, "warnings": 0 }` and an error on line 2 is published, which the live
  parse alone does not report

#### Scenario: Not an open document
- **WHEN** `qb64rust/check` names a uri that is not open
- **THEN** the server answers with the `InvalidParams` error and publishes nothing

### Requirement: Check diagnostics
A check SHALL publish, under the uri of the file each is in, every error and warning the CLI reports for the same
files with `-w`: the parser's, `sema`'s (with "not supported yet" in the message where the CLI marks it) and the
missing files of `$EXEICON` and `$EMBED`. Each SHALL have the CLI's message, line and column, the old compiler's
wording as the second line where the CLI gives it a note, source `qb64rust`, and severity error or warning.

#### Scenario: Several errors in one run
- **WHEN** a program has an undefined `GOTO` label on line 3 and `x$ = 1` on line 7 and is checked
- **THEN** both errors are published, each at the column of the CLI's report

#### Scenario: Same as the CLI
- **WHEN** a program that the CLI rejects is checked
- **THEN** the published diagnostics are those `qb64rust -w` prints for it, file, line, column and message alike

#### Scenario: Warning
- **WHEN** a checked program has `$DEBUG` on line 1
- **THEN** a diagnostic with severity warning is published on line 1

#### Scenario: Missing icon
- **WHEN** a checked program has `$EXEICON:'./game.ico'` and no `game.ico` lies beside it
- **THEN** an error on the metacommand's line names `./game.ico`

#### Scenario: Error in an included file
- **WHEN** `main.bas` includes `lib.bi`, line 4 of `lib.bi` is `y$ = 2`, and `main.bas` is checked
- **THEN** the error is published under `lib.bi`'s uri at line 4

### Requirement: Check result until the next parse
The diagnostics of a check SHALL stand until the next parse of the same program after a change, which SHALL
publish the parser's diagnostics alone again for the files it covers. A newer check of the same program SHALL
replace an older one; an older check that ends later SHALL publish nothing and SHALL be answered with the
`ContentModified` error.

#### Scenario: Edit after a check
- **WHEN** a check has published a type error and the user then types in the document
- **THEN** after the parse that follows, only the parser's diagnostics are published for it

#### Scenario: Two checks in a row
- **WHEN** two `qb64rust/check` requests for one program arrive before the first has ended
- **THEN** only the second one's diagnostics are published
