# INPUT

`INPUT` reads values typed at the keyboard, or from a file with `#n`, into variables: `INPUT "How many men"; men&`.

## Syntax

```text
INPUT [;] ["prompt" {;|,}] variable [, variable]...
INPUT #filenumber, variable [, variable]...
```

## Arguments

- `prompt`: a string literal, printed first.
- `variable`: the variables that take the values, of any type.

## Behavior

A prompt followed by `;` is printed with `? ` after it; one followed by `,` is printed as it is: `INPUT "How many
men"; men&` prints `How many men? `.
<!-- src: language/builtin-statements "Console INPUT and LINE INPUT"; tests\reference\examples\input_1 -->

The line read is split at commas, one field for each variable. A field is converted to the variable's type. With fewer
fields than variables, the others are 0 or empty.
<!-- src: language/builtin-statements "Console INPUT and LINE INPUT" -->

A character that does not fit a numeric field is dropped, and the text so far is printed again: `INPUT "byte"; b` for
a `_BYTE` with the line `300` stores 30. The program never asks again.
<!-- src: language/builtin-statements "Console INPUT and LINE INPUT" -->

A `;` right after `INPUT` keeps the output on the same line after the answer.
<!-- src: language/builtin-statements "Console INPUT and LINE INPUT" -->

When the input has ended, as when it comes from a file or a pipe, `INPUT` raises error 62 and does not wait. Its
variables are then 0 or empty.
<!-- src: language/console-and-window "Console input at the end of the input" -->

`INPUT #n` reads the fields from file `n`.
<!-- src: language/file-io "INPUT and LINE INPUT from a file" -->

## Errors

- A prompt that is no string literal, as `INPUT p$; l`, or a target that is no variable, as `INPUT 5`, is a compile
  error.
  <!-- src: language/builtin-statements "Console INPUT and LINE INPUT" -->

## Differences

From QB64pe: `INPUT` at the end of redirected input waits there for ever. QB64Rust raises error 62
(`DIVERGENCES-QB64PE.md` D-048).
<!-- src: D-048 -->

## Availability

<!-- begin generated: availability -->
Supported. All 29 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:input_1 -->
```qb64
INPUT "How many men"; men&
INPUT "Leader", leader$
PRINT
PRINT leader$; " leads"; men&; "men."
SYSTEM
```

Output:

```text
How many men? Leader
Davout leads 26000 men.
```
<!-- end generated -->

## See also

`LINE INPUT`, `INKEY$`, `INPUT$`, `VAL`.
