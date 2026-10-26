# Spec Delta

## MODIFIED Requirements

### Requirement: Server start
`qb64rust lsp` SHALL run a Language Server Protocol server over stdin and stdout until `shutdown` and `exit`, and
SHALL advertise exactly these capabilities: full text synchronisation, document symbols, folding ranges, go to
definition, document formatting and range formatting, hover, references, rename with prepare, and completion.

#### Scenario: Initialize and exit
- **WHEN** a client sends `initialize`, `initialized`, `shutdown` and `exit`
- **THEN** the server answers the two requests and the process ends with exit code 0

### Requirement: Go to definition
The server SHALL answer `textDocument/definition` for a procedure name (in a call with or without `CALL`, in an
expression, in a `DECLARE`) with the location of the first `SUB`/`FUNCTION` header of that name in file order,
and for a label name in a jump (`GOTO`, `GOSUB`, `ON … GOTO/GOSUB`, `ON ERROR GOTO`, `RESUME`, `RETURN`) with the
label's definition in the same body. For a variable, a constant, a user type or a member it SHALL answer with the
definition the front end records: the name in its `DIM`, `CONST`, parameter list or `TYPE` block, or the first use
of a variable that is not declared. A name with no definition SHALL give an empty result.

#### Scenario: Call before the definition
- **WHEN** line 1 is `bump 3` and `SUB bump (n)` is on line 10
- **THEN** definition at `bump` on line 1 is line 10

#### Scenario: Label in the right body
- **WHEN** the main module and `SUB a` both have a label `again:` and the cursor is on `GOTO again` inside `a`
- **THEN** the result is the label inside `a`

#### Scenario: Variable declared in an include
- **WHEN** `lib.bi` has `DIM SHARED total AS LONG` on line 2 and the cursor is on `total` in the main file
- **THEN** the result is line 2 of `lib.bi`

#### Scenario: Local variable
- **WHEN** both the main module and `SUB a` use `n`, and the cursor is on `n` inside `a`
- **THEN** the result is the first use of `n` inside `a`

## ADDED Requirements

### Requirement: Semantic index until the next change
Hover, references, rename, completion and definition beyond procedures and labels SHALL be answered from the
program's whole front end, read as a check reads it (open documents first, then the disk). The server SHALL run the
front end at most once per text of the program, at the first such request after a change, and SHALL answer a
request for a text that changed before its answer with the `ContentModified` error.

#### Scenario: Two hovers, one run
- **WHEN** the client asks for a hover twice with no edit between
- **THEN** the front end runs once

#### Scenario: Edit while waiting
- **WHEN** the document changes while a references request waits for the front end
- **THEN** the request is answered with `ContentModified`

### Requirement: Hover
The server SHALL answer `textDocument/hover` on a name with what the front end knows about it, as Markdown: a
variable's type, shape and storage; a constant's type and value; a procedure's header; a user type's members; a
member's type; a label's body. On a built-in it SHALL give the built-in table's arguments and result. On anything
else it SHALL give no hover.

#### Scenario: Variable
- **WHEN** `SUB a` has `DIM n AS _UNSIGNED LONG` and the cursor is on a later `n` inside `a`
- **THEN** the hover reads that `n` is an `_UNSIGNED LONG`, local to `SUB a`

#### Scenario: Constant
- **WHEN** the program has `CONST LIMIT = 10` and the cursor is on `LIMIT`
- **THEN** the hover gives its type and the value 10

#### Scenario: Built-in
- **WHEN** the cursor is on `LEFT$` in `x$ = LEFT$(a$, 2)`
- **THEN** the hover gives `LEFT$`'s arguments and its result type `STRING` from the built-in table

#### Scenario: A keyword
- **WHEN** the cursor is on `FOR`
- **THEN** there is no hover

### Requirement: Find references
The server SHALL answer `textDocument/references` on a name with every use of its symbol in the program, in every
file of it, and with the definition as well when the client's `includeDeclaration` is true. Two symbols with the same
name in different bodies, or with different suffixes, SHALL be kept apart, as the front end keeps them.

#### Scenario: Across an include
- **WHEN** `total` is declared in `lib.bi` and used twice in the main file
- **THEN** the references with the declaration are the three uses, under their two uris

#### Scenario: Two variables that differ by suffix
- **WHEN** a program uses both `a$` and `a%`, and the cursor is on `a$`
- **THEN** no use of `a%` is among the references

### Requirement: Rename
The server SHALL answer `textDocument/prepareRename` with the range of the name without its suffix, and
`textDocument/rename` with a workspace edit that writes the new name at every use, the definition included, in every
file of the program. Each use SHALL keep the suffix written there, and the new name SHALL be given without one.

#### Scenario: Rename a variable
- **WHEN** `count%` is used in four places, one of them written `COUNT%`, and is renamed to `total`
- **THEN** all four read `total%`, and no other text changes

#### Scenario: Rename a procedure across files
- **WHEN** `SUB bump` is defined in `lib.bm` and called in the main file, and is renamed to `step_up`
- **THEN** the edit changes the header in `lib.bm` and each call in the main file

### Requirement: Rename refused
The server SHALL refuse a rename with an error and a message that says why, and SHALL change nothing, in these cases:
the program has an error; the new name is not a name, has a suffix, is a reserved word or names a built-in; the new
name is already a name that one of the uses can see; a use lies in a file under the compiler root; or the cursor is
on a line number or on no symbol.

#### Scenario: Program with an error
- **WHEN** the program has `x$ = 1` on line 3 and a rename is asked on another name
- **THEN** the server answers with an error that names line 3, and no edit is made

#### Scenario: Clash
- **WHEN** `SUB a` has locals `i` and `j`, and `i` is renamed to `j`
- **THEN** the rename is refused, and the message names `j`

#### Scenario: Reserved word
- **WHEN** a variable is renamed to `print`
- **THEN** the rename is refused

### Requirement: Completion
The server SHALL answer `textDocument/completion` with the names visible at the cursor: the variables of the
procedure there, or of the main module, together with the shared ones; the procedures, constants, user types and
labels; the built-ins, with the old table's spelling; and the keywords. Each item SHALL say its kind. When no index
is ready, the last index of the program SHALL be used.

#### Scenario: Local names in a SUB
- **WHEN** the cursor is inside `SUB a`, which has a local `inner`, and the main module has a non-shared `outer`
- **THEN** `inner` is offered and `outer` is not

#### Scenario: Built-in spelling
- **WHEN** the user has typed `_rg`
- **THEN** `_RGB32` is among the items, spelled as the built-in table spells it
