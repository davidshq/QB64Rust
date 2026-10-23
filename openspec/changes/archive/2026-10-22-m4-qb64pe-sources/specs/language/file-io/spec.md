## ADDED Requirements

### Requirement: Whole files and full paths
`_READFILE$(name$)` SHALL return the whole content of a file as a string, `_WRITEFILE name$, data$` SHALL write
`data$` as the whole content of a file (creating it or replacing it), and `_FULLPATH$(path$)` SHALL return the full
path of an existing file or folder, as the old compiler's runtime does. A file or path that cannot be found, read
or written SHALL raise the old compiler's error (`verification\v52_c_files`), and a function SHALL then return "".

#### Scenario: A round trip
- **WHEN** `_WRITEFILE "rt.tmp", "a" + CHR$(0) + "b": d$ = _READFILE$("rt.tmp"): PRINT LEN(d$); ASC(d$, 2)` runs
- **THEN** it prints ` 3  0 `

#### Scenario: A missing file
- **WHEN** `d$ = _READFILE$("nope.tmp")` runs under a handler that prints `ERR`
- **THEN** the handler prints 53, and `d$` is empty after `RESUME NEXT`

#### Scenario: A full path
- **WHEN** `PRINT RIGHT$(_FULLPATH$("."), 1)` runs
- **THEN** it prints `\`

#### Scenario: A path that does not exist
- **WHEN** `p$ = _FULLPATH$("no\such\dir")` runs under a handler that prints `ERR`
- **THEN** the handler prints 76, and `p$` is empty after `RESUME NEXT`

#### Scenario: A folder read, a file in a missing folder written
- **WHEN** `d$ = _READFILE$(".")` and `_WRITEFILE "no_dir\x.tmp", "a"` run under a handler that prints `ERR`
- **THEN** the handler prints 53, then 76

#### Scenario: An empty path
- **WHEN** `p$ = _FULLPATH$("")` runs
- **THEN** `p$` is empty and no error is raised
