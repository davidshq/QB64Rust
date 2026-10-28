# Spec Delta

## ADDED Requirements

### Requirement: Fixes in a check
A check, the semantic index and formatting SHALL use the fix set the compiler would build the program with (the
`qb64rust.toml` found from the main file on disk, spec `compiler/fixes`); a document that is not a file SHALL be
checked with every fix off. An error in `qb64rust.toml` SHALL be published on the main file's first line.

#### Scenario: A fix's compile error shows
- **WHEN** `qb64rust.toml` beside `p.bas` holds `[fixes]`, `all = true`, and the open `p.bas` has `PRINT a()`
- **THEN** a check publishes the compile error of `whole-array-value` at `a()`

#### Scenario: A bad settings file
- **WHEN** `qb64rust.toml` beside `p.bas` holds `[fixes]` and `nonsense = true`
- **THEN** a check publishes an error on line 1 of `p.bas` naming `qb64rust.toml`

### Requirement: Settings file changed
When a `qb64rust.toml` is created, changed or deleted on disk, the server SHALL drop its check results and semantic
index for the open documents and check them again as after an edit. The client SHALL report the changes of the
files named `qb64rust.toml` in the workspace's folders only: a change to one above them, which the compiler also
finds, is seen at the next edit of the program or when it is opened again.

#### Scenario: The fixes turned off
- **WHEN** the check above shows the error of `whole-array-value`, and `all = true` becomes `all = false` on disk
- **THEN** the error is no longer published, with no edit of `p.bas`
