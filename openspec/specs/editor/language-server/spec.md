# editor/language-server Specification

## Purpose
A thin language server, part of the `qb64rust` binary, that gives the editor syntax errors as the user types,
the document outline, folding ranges and go to definition for procedures and labels, from the new compiler's
parser alone.

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
NOT publish anything from the later stages (no "not supported yet" of `sema`, no type errors). A diagnostic the
parser itself marks "not supported yet" (a syntax error at a BASIC word, an expression nested too deep) is
published with the marker in its message.

#### Scenario: Error appears and goes
- **WHEN** the user types `x = 1 +` on a line and then completes it to `x = 1 + 2`
- **THEN** a diagnostic appears on that line after the first edit and is gone after the second, without a save

#### Scenario: Unsupported statement is no diagnostic
- **WHEN** an open document contains `OPEN "f" FOR OUTPUT AS #1`, which the compiler does not compile yet
- **THEN** the server publishes no diagnostic for that line

#### Scenario: Error in an included file
- **WHEN** the open main file includes `lib.bi` and `lib.bi` has a syntax error on line 3
- **THEN** the diagnostic is published under `lib.bi`'s uri at line 3, and none under the main file for it

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
