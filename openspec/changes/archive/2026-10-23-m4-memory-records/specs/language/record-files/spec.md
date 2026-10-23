# Spec Delta

## MODIFIED Requirements

### Requirement: The forms of GET and PUT on a file
`GET [#]n[, [position][, variable]]` and `PUT` with the same list SHALL compile to the runtime call the old
compiler writes, also after `CALL`. The file number SHALL be any numeric expression. A first argument that starts
with `(` SHALL be read as the graphics form. The third argument SHALL be a variable; every form the old compiler
rejects SHALL be a compile error that is not "not supported yet". A whole array `und()` of a name that is no array
SHALL be an array used without `DIM`, and SHALL read and write what the old compiler's program does; `GET` of
`und$()`, an array of strings so made, SHALL read into its first element as into a string variable, and `PUT` of
it SHALL be the old compiler's error, as for a declared array of strings.

#### Scenario: With and without a position
- **WHEN** `GET #1, 3, c`, `GET #1, , c`, `PUT #1, , c` and `PUT #k + 1, 2, c` are compiled by both compilers
- **THEN** the runtime calls are the same

#### Scenario: Not a variable
- **WHEN** a program has `PUT #1, , 5`, `PUT #1, , a + 1`, `PUT #1, , LEN(s$)` or `PUT #1, , K` with `K` a `CONST`
- **THEN** each is a compile error

#### Scenario: A variable in parentheses
- **WHEN** `PUT #1, , (c)` is compiled
- **THEN** it compiles as `PUT #1, , c` does

#### Scenario: Rejected arguments
- **WHEN** a program has `GET #1, , b` with `b` a `_BIT` variable, `GET #1, , s$()` with `s$()` an array of
  variable-length strings, `GET #1, , t` with `t` of a `TYPE` that has a `STRING` member, `GET #1, , r.v()` with
  `v` a member array, `GET #1, ,`, `GET`, `GET #, , c` or `GET "a", , c`
- **THEN** each is a compile error

#### Scenario: An array that is not declared
- **WHEN** the programs `verification\v30_x176_get_undeclared_array`, `x177` and `x178` (`GET #1, , und()`,
  `PUT #1, , und()`, `GET #1, , und$()`) and `verification\v56_e_undeclared_arrays`, which has data in the file,
  run
- **THEN** each prints what the old compiler's program printed

#### Scenario: PUT of an array of strings not declared
- **WHEN** a program has `PUT #1, 1, uns$()` and no array `uns$` (`verification\v56_e2`)
- **THEN** it is a compile error that is not "not supported yet"
