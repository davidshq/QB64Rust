# language/opengl Specification

## Purpose
How a program draws with OpenGL: the routine `SUB _GL` that the runtime calls each frame, the OpenGL constants
and routines a program with it may use, and the errors of calling them out of place, all as the old compiler has
them.

## Requirements

### Requirement: SUB _GL
A SUB named `_GL` without parameters (`STATIC` or not) SHALL compile as the program's OpenGL routine: the runtime
SHALL call it once a frame while the program's window is shown, at the layer `_GLRENDER` sets, and the program
SHALL be built with the runtime's OpenGL part, as the old compiler builds it (measured, `verification\v59`).
`DECLARE SUB _GL ()` SHALL be accepted and change nothing (`v59_x34`).

#### Scenario: The upstream program
- **WHEN** `tests\upstream` program `opengl/test` (`SUB _GL STATIC` calling `_glViewport 0, 0, _WIDTH, _HEIGHT`)
  is compiled
- **THEN** it compiles with no diagnostic and builds

#### Scenario: Called each frame
- **WHEN** a program counts the calls of its `SUB _GL` in a shared variable, waits with `_LIMIT`
  until the count is at least 3, and prints a line
- **THEN** it prints what the old compiler's program prints, and ends

### Requirement: SUB _GL without a shown window
A program whose window is never shown (`$SCREENHIDE`, `$CONSOLE:ONLY`) SHALL compile and build with `SUB _GL`, and
the SUB SHALL never be called, as in the old compiler's program (`v59_a5`, `v59_a6`).

#### Scenario: A console program
- **WHEN** a `$CONSOLE:ONLY` program with a `SUB _GL` that counts its calls waits 1 s and prints the count
- **THEN** it prints 0, as the old compiler's program does

### Requirement: The forms of SUB _GL the old compiler rejects
A form of `SUB _GL` that the old compiler rejects SHALL be a compile error whose note is its message, each as
`verification\v59` records it: a call of the SUB (`_GL`, `CALL _GL`, also inside itself: "Cannot call SUB _GL
directly"), `SUB _GL` with a parameter, `FUNCTION _GL` and a second `SUB _GL` ("Invalid name", as any other
procedure's name with a leading underscore), and `SUB _GL$` ("Type symbols after a SUB name are invalid").

#### Scenario: A direct call
- **WHEN** a program with `SUB _GL` has the statement `_GL` or `CALL _GL`
- **THEN** the compile fails with an error whose note is "Cannot call SUB _GL directly"

#### Scenario: Another form
- **WHEN** a program has a form of `SUB _GL` that `v59` records the old compiler rejecting
- **THEN** the compile fails with an error whose note is the old compiler's message

### Requirement: The OpenGL names
A program with `SUB _GL` SHALL be able to use the constants and routines the old compiler reads from its copy of
`gl.h`: each `GL_…` constant as `_GL_…`, an `_INTEGER64`, and each routine `gl…` as `_gl…`, a SUB or a FUNCTION
by its return type, and `_gluPerspective`. The constants SHALL stand wherever a constant may but in a `CONST`.

#### Scenario: A constant and a statement
- **WHEN** a program with `SUB _GL` has `_glClear _GL_COLOR_BUFFER_BIT` in it and prints `_GL_COLOR_BUFFER_BIT`
  in the main module
- **THEN** it compiles, and prints what the old compiler's program prints

### Requirement: The OpenGL names where they do not exist
In a program without `SUB _GL` the OpenGL names SHALL be what the old compiler makes of them there
(`verification\v59_b*`): a statement is "Syntax error", a function or a constant "Invalid variable name". A
program's own procedure, variable or `CONST` named as one of them SHALL be rejected as any name with a leading
underscore is, and a `CONST` whose value names an OpenGL constant SHALL be rejected (`v59_x43`, `x44`).

#### Scenario: Without SUB _GL
- **WHEN** a program without `SUB _GL` calls `_glViewport 0, 0, 1, 1` or prints `_GL_COLOR_BUFFER_BIT`
- **THEN** the compile fails, as the old compiler's does (`v59_b1`–`b4`)

#### Scenario: A CONST of an OpenGL constant
- **WHEN** a program with `SUB _GL` has `CONST k = _GL_TRUE`
- **THEN** the compile fails with an error whose note is "Unexpected element '_GL_TRUE'"

### Requirement: The arguments of an OpenGL routine
Every argument of an OpenGL routine SHALL be passed by value, converted to the routine's type as the old compiler
converts it (`GLenum`, `GLbitfield`, `GLuint` to `_UNSIGNED LONG`, `GLint` and `GLsizei` to `LONG`, `GLfloat` to
`SINGLE`, `GLdouble` to `DOUBLE`, and so on), every pointer as an `_OFFSET`; the wrong number of arguments or a
string where a number goes SHALL be the old compiler's error. `_glGetString` SHALL return a `STRING`.

#### Scenario: The calls are the old compiler's
- **WHEN** the call-site check compiles the program of each OpenGL routine
- **THEN** each call is equal to the old compiler's

#### Scenario: A pointer argument
- **WHEN** `SUB _GL` reads the viewport with `_glGetIntegerv _GL_VIEWPORT, _OFFSET(v&(0))` into `DIM SHARED v&(3)`
  and the main module prints the four values
- **THEN** it prints what the old compiler's program prints

#### Scenario: A rejected call
- **WHEN** a program with `SUB _GL` has `_glViewport 0, 0, 1` or `_glClear "a"`
- **THEN** the compile fails with an error whose note is the old compiler's message (`verification\v59`)

### Requirement: Reading OpenGL back
What a program reads from OpenGL inside `SUB _GL` SHALL be what the old compiler's program reads, where the value
does not depend on how the driver rasterises or rounds: a colour the frame was cleared to, read with
`_glReadPixels`, the viewport, the state `_glIsEnabled` and `_glGetError` report.

#### Scenario: A cleared frame
- **WHEN** `SUB _GL` clears the frame with `_glClearColor 1, 0, 0, 1: _glClear _GL_COLOR_BUFFER_BIT`, reads one
  pixel with `_glReadPixels 0, 0, 1, 1, _GL_RGBA, _GL_UNSIGNED_BYTE, _OFFSET(p~%%(0))` into a shared array and
  the main module prints the four bytes after the next frame
- **THEN** it prints what the old compiler's program prints (`255 0 0 255`)

### Requirement: The errors of OpenGL out of place
An OpenGL routine that the old compiler calls through a helper (one with an argument, or `_glGetString`) called
while `SUB _GL` is not running SHALL raise run-time error 270, and `END` or `SYSTEM` while it runs SHALL raise error
271: each is libqb's critical error, which ends the program with its message whatever `ON ERROR` says, as in the
old compiler's program (`verification\v59_f*`). A routine without a helper (`_glFlush`, `_glGetError`) and
`_gluPerspective` SHALL raise nothing there.

#### Scenario: A routine in the main module
- **WHEN** a program with `SUB _GL` calls `_glViewport 0, 0, 1, 1` in its main module under `ON ERROR GOTO h`
- **THEN** it ends with the message "_GL command called outside of SUB _GL's scope", and `h` does not run, as in
  the old compiler's program

#### Scenario: A routine without a helper
- **WHEN** a program with `SUB _GL` calls `_glFlush` in its main module
- **THEN** it goes on with no error, as the old compiler's program does

#### Scenario: SYSTEM inside SUB _GL
- **WHEN** `SUB _GL` runs `SYSTEM`, under the main module's `ON ERROR` handler or none
- **THEN** the program ends with the message "END/SYSTEM called within SUB _GL's scope", as the old compiler's
  program does

### Requirement: STOP inside SUB _GL
`STOP` inside `SUB _GL` SHALL end the program with error 271, as `END` and `SYSTEM` do there; the old compiler's
program hangs instead (`v59_f9`; `QB64PE-DIVERGENCES.md` D-036, user, 2026-10-24).

#### Scenario: STOP
- **WHEN** `SUB _GL` prints a line and runs `STOP`, and the main module waits 20 s and prints another
- **THEN** only the first line is printed, and the program ends at once with the message "END/SYSTEM called within
  SUB _GL's scope", where the old compiler's program does not end
