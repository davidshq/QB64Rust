## ADDED Requirements

### Requirement: Listing a folder with _FILES$
`_FILES$(spec$)` SHALL start a listing of the entries that `spec$` names (a folder, a pattern, or "" for the
current folder) and give the first, a folder's name ending in a backslash, and `_FILES$` without an argument SHALL
give the next entry, then "", as the old compiler's runtime does. A bare call with no listing started, or after
the listing has given "", SHALL raise error 5.

#### Scenario: Counting entries
- **WHEN** a folder holds 11 files and a program counts `_FILES$("")` and the bare calls after it until ""
- **THEN** it counts 13 (the files, `.` and `..`), and 12 with `_FILES$("*.*")`, as with the old compiler

#### Scenario: No listing
- **WHEN** a program's first `_FILES$` is a bare call, under a handler that prints `ERR`
- **THEN** the handler prints 5, as with the old compiler

#### Scenario: Wrong forms
- **WHEN** a program has `_FILES$("a", "b")` or `_FILES$(5)`
- **THEN** each is a compile error, as in the old compiler
