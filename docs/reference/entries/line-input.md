# LINE INPUT

`LINE INPUT` reads a whole line, commas and quotes included, into a string variable: `LINE INPUT "Motto: "; m$`.

## Syntax

```text
LINE INPUT [;] ["prompt";] variable$
LINE INPUT #filenumber, variable$
```

## Arguments

- `prompt`: a string literal, printed as it is, with no `? `.
- `variable$`: one string variable.

## Behavior

The whole line is stored, with no split at commas: the line `Courage, and again courage` is one value.
<!-- src: language/builtin-statements "Console INPUT and LINE INPUT"; tests\reference\examples\line-input_1 -->

The prompt is printed as it is, never with `? `.
<!-- src: language/builtin-statements "Console INPUT and LINE INPUT" -->

When the input has ended it raises error 62 and does not wait. A last line with no line end is read as a line.
<!-- src: language/console-and-window "Console input at the end of the input" -->

`LINE INPUT #n` reads a line of file `n`.
<!-- src: language/file-io "INPUT and LINE INPUT from a file" -->

## Errors

- A target that is no string variable, or more than one target, is a compile error.
  <!-- src: language/builtin-statements "Console INPUT and LINE INPUT" -->

## Differences

From QB64pe: `LINE INPUT` at the end of redirected input waits there for ever. QB64Rust raises error 62
(`DIVERGENCES-QB64PE.md` D-048).
<!-- src: D-048 -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:line-input_1 -->
```qb64
LINE INPUT "Motto: "; m$
PRINT
PRINT "["; m$; "]"
SYSTEM
```

Output:

```text
Motto: 
[Courage, and again courage]
```
<!-- end generated -->

## See also

`INPUT`, `INPUT$`.
