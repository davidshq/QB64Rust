# Spec Delta

## ADDED Requirements

### Requirement: An omitted argument of a built-in function
An argument left out of a call of `POINT`, `_RGB32`, `_RGB`, `_RGBA32`, `INSTR`, `MID$` or `LEFT$` SHALL be the
old compiler's error where a recording shows that it rejects the form, with its message as the note (measured,
`verification\v27_x50`, `v28_x49`, `v57_o01`–`o17`, `o21`). A form the old compiler accepts, and a form of any
other function, SHALL stay "not supported yet".

#### Scenario: The first argument of POINT
- **WHEN** a program has `d = POINT(, 2)`
- **THEN** the compile fails with an error, not marked "not supported yet" (`verification\v27_x50`)

#### Scenario: The middle argument of _RGB32
- **WHEN** a program has `u = _RGB32(1, , 3)`
- **THEN** the compile fails with an error, not marked "not supported yet" (`verification\v28_x49`)

#### Scenario: The last argument
- **WHEN** a program has `u$ = MID$("abc", 1, )`, or leaves out the last argument of `POINT`, `_RGB32`, `_RGBA32`,
  `MID$` or `LEFT$`
- **THEN** the compile fails with an error whose note is "Invalid expression" (`verification\v57_o12`)

#### Scenario: Another argument
- **WHEN** another argument is left out, or any argument of `_RGB` or `INSTR` but `INSTR`'s first
- **THEN** the old compiler's message is "Incorrect number of arguments" for `POINT`, `_RGB`, `INSTR` and `MID$`, and
  "Syntax error" for `_RGB32`, `_RGBA32` and `LEFT$`

#### Scenario: The first argument of INSTR
- **WHEN** a program has `u = INSTR(, "ab", "b")`, which the old compiler compiles (`verification\v57_o07`)
- **THEN** it is reported "not supported yet"

### Requirement: The old compiler's stubs
A name the old compiler's table lists as a stub and compiles by no rule of its own SHALL be a compile error,
with its "Command not implemented" as the note, also where the line is never reached (measured,
`verification\v57_s01`–`s20`). A stub with a rule of its own SHALL follow that rule.

#### Scenario: FRE of a number
- **WHEN** a program has `f0 = FRE(0)`
- **THEN** the compile fails with an error at `FRE`, not marked "not supported yet"
  (`tests\corpus\runtime_comparison\218_fre`)

#### Scenario: The stubs
- **WHEN** a program has `TRON`, `TROFF`, `LIST`, `IOCTL 1, "x"`, `IOCTL$(1)`, `SETMEM(0)`, `FILEATTR(1, 1)` or
  `FRE` of any argument
- **THEN** the compile fails with an error, with "Command not implemented" as its note (`verification\v57_s01`–
  `s10`)

#### Scenario: Stubs after CALL
- **WHEN** a program has `CALL TRON`, `CALL TROFF`, `CALL MID$(a$, 1, 1, "x")` or `CALL _MEMGET(…)`
- **THEN** the compile fails with an error, with "Command not implemented" as its note (`verification\v57_s11`,
  `s14`, `s15`, `s17`); `CALL FRE(0)` is "Expected CALL sub-name [(...)]", as `CALL` of any function (`s19`)

#### Scenario: RESET
- **WHEN** a program has `RESET` or `CALL RESET`
- **THEN** it compiles and closes every open file, as `CLOSE` does (`verification\v57_s03`, `s18`)
