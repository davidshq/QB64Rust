# Spec Delta

## ADDED Requirements

### Requirement: Names that call no built-in
A name with one leading underscore and parentheses that calls no built-in function with that spelling, a
built-in function's name used as a variable, and a built-in function used as a statement SHALL be compile errors
that are not "not supported yet", as the old compiler's are. A form `verification\v55` does not record SHALL
stay "not supported yet".

#### Scenario: A name with one leading underscore
- **WHEN** `_foo(1) = 2`, `n = _TRUE(1)`, `PRINT _CRC32$("x")`, `x = _CAST&(LONG, 1)`, `PRINT _OPENCLIENT&("x")`
  or `s = _IIF$(-1, "yes", "no")` is compiled
- **THEN** each is a compile error that is not "not supported yet" (`verification\v24_x112`, `v26_x120`,
  `v33_x29`, `v49_x16`, `v44_x46`, `tests\snippets\qb64fresh\inline_conditional_functions__iif_string`)

#### Scenario: A built-in function's name as a variable
- **WHEN** `rnd(1) = 2`, `TAB(5) = 1`, `READ ABS(a)`, `INPUT ABS(a)`, `LINE INPUT CHR$(1)` or `SWAP ABS(a), b` is
  compiled
- **THEN** each is a compile error that is not "not supported yet" (the old compiler: "Expected variable =" for a
  store, "Expected variable" for the others; `verification\v24_x119`, `v28_x53`, `v22_x79`, `v55_x13`–`x15`)

#### Scenario: A built-in function as a statement
- **WHEN** `LEN("a")`, `LEN "a"` or `ABS(1)` stands alone on a line
- **THEN** each is a compile error that is not "not supported yet" (the old compiler: "Syntax error";
  `verification\v20_x06_function_as_statement`, `v55_x29`, `x31`)

#### Scenario: Every spelling of the table calls its function
- **WHEN** each function of the built-in table is called by each spelling the table gives it (its name with each
  suffix it may or must have), the functions of the files QB64pe includes by itself among them
- **THEN** none is "Invalid array name"
