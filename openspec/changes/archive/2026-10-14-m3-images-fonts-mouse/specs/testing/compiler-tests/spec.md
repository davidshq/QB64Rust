# Spec Delta

## MODIFIED Requirements

### Requirement: Built-in coverage
Every built-in function and every built-in statement that `sema` compiles SHALL be used in at least one program of
`tests/corpus/slice.list` (so its output is compared with the old compiler's in tier 2) and in at least one
front-end test that pins it in tier 1: a `typed` test for a function (its result type and argument conversions),
an `ir` test for a statement (its slots, places and items). `cargo test` SHALL fail and name the built-in when
either is missing. For a built-in that no test runs (a window built-in, `language/console-and-window`; `BEEP`,
which rings the machine's bell, `language/sound`; `CHAIN`, which starts another compiler, and `LPRINT`, which
prints a page, `language/builtin-statements`; `_SCREENIMAGE`, `_CLIPBOARDIMAGE`, `_PRINTIMAGE`, `_DEPTHBUFFER`
and the 3-D form of `_MAPTRIANGLE`, which read the desktop or the clipboard, print a page or draw through
OpenGL, `language/images`; `_MOUSEMOVE`, which moves the machine's cursor, `language/mouse`), the slice program
SHALL hold it as code that is compiled and not reached, and the built-in SHALL also have a call-site program for
each of its forms; the list of such built-ins SHALL be explicit, and `cargo test` SHALL fail when one of them has
no call-site program.

#### Scenario: Built-in added without a slice program
- **WHEN** `sema` starts compiling a built-in that no slice program calls
- **THEN** `cargo test` fails and names that built-in

#### Scenario: Built-in added without a typed test
- **WHEN** `sema` starts compiling a built-in that no `typed` front-end test calls
- **THEN** `cargo test` fails and names that built-in

#### Scenario: Statement added without an ir test
- **WHEN** `sema` starts compiling a built-in statement that no `ir` front-end test uses
- **THEN** `cargo test` fails and names that statement

#### Scenario: Window built-in without a call-site program
- **WHEN** `sema` starts compiling a window built-in that no program of `tests/callsite/` uses
- **THEN** `cargo test` fails and names that built-in

#### Scenario: LPRINT without a call-site program
- **WHEN** the call-site programs of `LPRINT` are taken out of `tests/callsite/`
- **THEN** `cargo test` fails and names `LPRINT`

#### Scenario: A desktop image without a call-site program
- **WHEN** the call-site programs of `_SCREENIMAGE` are taken out of `tests/callsite/`
- **THEN** `cargo test` fails and names `_SCREENIMAGE`
