# Spec Delta

## MODIFIED Requirements

### Requirement: Hover
The server SHALL answer `textDocument/hover` on a name with what the front end knows about it, as Markdown: a
variable's type, shape and storage; a constant's type and value; a procedure's header; a user type's members; a
member's type; a label's body. On a built-in it SHALL give the built-in table's arguments and result, and after
them the first sentence of the built-in's reference page when it has one. On a keyword that has a reference page
it SHALL give that page's first sentence. The sentence SHALL be given only when it is about the word: the page is
the word's own, or a code span of the sentence starts with the word. On anything else it SHALL give no hover.

#### Scenario: Variable
- **WHEN** `SUB a` has `DIM n AS _UNSIGNED LONG` and the cursor is on a later `n` inside `a`
- **THEN** the hover reads that `n` is an `_UNSIGNED LONG`, local to `SUB a`

#### Scenario: Constant
- **WHEN** the program has `CONST LIMIT = 10` and the cursor is on `LIMIT`
- **THEN** the hover gives its type and the value 10

#### Scenario: Built-in
- **WHEN** the cursor is on `LEFT$` in `x$ = LEFT$(a$, 2)`
- **THEN** the hover gives `LEFT$`'s arguments and its result type `STRING` from the built-in table

#### Scenario: A built-in with a reference page
- **WHEN** the cursor is on `_CLAMP` in `health = _CLAMP(health, 0, 100)`
- **THEN** the hover gives `_CLAMP`'s arguments, and then the first sentence of its reference page, which says
  that it holds a value between two bounds

#### Scenario: A keyword
- **WHEN** the cursor is on `FOR`
- **THEN** the hover gives the first sentence of the reference page of `FOR`

#### Scenario: A keyword without a page
- **WHEN** the cursor is on a reserved word that has no reference page
- **THEN** there is no hover

#### Scenario: A keyword covered on another word's page
- **WHEN** the cursor is on `TO` in `FOR i = 1 TO 10`, a word covered on the page of `FOR`, whose first sentence
  is not about `TO`
- **THEN** there is no hover

## ADDED Requirements

### Requirement: Procedures of a program
The server SHALL answer the request `qb64rust/procedures` for an open document with the main file of the
document's program and every SUB and FUNCTION of that program, in the program's order: each with its name as
declared, its kind, its parameters as written, its file and the range of its first line. The answer SHALL also
list every file of the program, the main file first, whether the file is open in the editor or not. It SHALL
answer from the text as edited, unsaved text included.

#### Scenario: An include file's procedure
- **WHEN** `bounce.bas` includes `rules.bm`, which holds `FUNCTION Speed! (level)`, and the request names
  `bounce.bas`
- **THEN** the answer holds `Speed!`, a FUNCTION, with the file `rules.bm` and the line of its header

#### Scenario: Asked from the include file
- **WHEN** the request names `rules.bm`, whose program is `bounce.bas`
- **THEN** the answer names `bounce.bas` as the main file and holds the same procedures

#### Scenario: A program of many files
- **WHEN** `game.bas` includes `game.bi`, `rules/army.bm` and `ui/menu.bm`, and only `game.bas` is open
- **THEN** the answer lists the four files, `game.bas` first, each once

#### Scenario: A syntax error elsewhere
- **WHEN** the program has a syntax error inside one SUB
- **THEN** the answer still holds the procedures that parsed

### Requirement: Help topic at a position
The server SHALL answer the request `qb64rust/helpTopic` for a position in an open document with one of three
results: the reference page of the built-in or keyword there, when it has one; "definition", when the name
there is one the program defines; or nothing.

#### Scenario: A built-in
- **WHEN** the position is on `_CLAMP`
- **THEN** the answer names the page `_clamp`

#### Scenario: A two-word statement
- **WHEN** the position is on `CASE` in `SELECT CASE n`
- **THEN** the answer names the page of `SELECT CASE`

#### Scenario: The program's own SUB
- **WHEN** the position is on a call of the program's `DrawBall`
- **THEN** the answer is "definition"

#### Scenario: A comment
- **WHEN** the position is inside a comment
- **THEN** the answer is nothing
