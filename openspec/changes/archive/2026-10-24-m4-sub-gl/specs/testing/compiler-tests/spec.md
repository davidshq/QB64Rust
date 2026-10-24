# Spec Delta

## ADDED Requirements

### Requirement: Coverage of the OpenGL table
The OpenGL routines and constants (`language/opengl`) SHALL be held apart from the built-in coverage rule: tier 1
SHALL check that the compiler's OpenGL table equals what is read from `libqb\`'s `gl.h` and that each routine's
call matches its helper in `gl_helper_code.h`, and every routine SHALL have a program in `tests/callsite/`.
`cargo test` SHALL fail and name the routine or constant when one of these is missing or differs.

#### Scenario: The table no longer equals gl.h
- **WHEN** a routine or a constant is added to, dropped from or changed in the compiler's OpenGL table by hand
- **THEN** `cargo test` fails and names it

#### Scenario: A routine without a call-site program
- **WHEN** the call-site program of `_glViewport` is taken out of `tests/callsite/`
- **THEN** `cargo test` fails and names `_glViewport`

#### Scenario: OpenGL is run
- **WHEN** tier 2 runs the verification pass list
- **THEN** the `v59` programs that draw in `SUB _GL` and read the result back run, and their output is compared
  with the old compiler's
