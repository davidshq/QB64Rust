# GOTO

`GOTO` continues the program at a label or a line number: `GOTO again`, `GOTO 100`.

## Syntax

```text
GOTO label
GOTO linenumber

label:
linenumber statement
```

## Behavior

The program goes on at the statement after the label. The label must be in the same body: the main module, or the
same SUB or FUNCTION. It may stand inside a block, such as the `THEN` branch of an `IF` whose condition is false.
<!-- src: language/control-flow "GOTO"; language/control-flow "Labels per body"; tests\reference\examples\goto_1 -->

A label is a name and a colon at the start of a line. A number at the start of a line is a line number, and is a
label too. Line numbers need not be in order, and they share one set of names with the labels of their body.
<!-- src: language/control-flow "Line numbers"; tests\reference\examples\goto_1 -->

A line number may have a decimal point or a type suffix, and may be beyond the LONG range. It is named by its
digits with leading zeros dropped, or, with a point or a suffix, by its text as written.
<!-- src: language/control-flow "The spelling of a line number" -->

The same label name in two bodies is two labels.
<!-- src: language/control-flow "Labels per body" -->

## Errors

These are compile errors:

- A label that does not exist in the body: "label `nowhere` is not defined in the main module", or "in this SUB or
  FUNCTION".
  <!-- src: language/control-flow "Labels per body"; language/control-flow "Line numbers as jump targets" -->
- The same label twice in one body: "duplicate label: `a`".
  <!-- src: language/control-flow "Labels per body" -->
- A jump to the name of a built-in, as in `GOTO BEEP`.
  <!-- src: language/control-flow "A jump to a built-in's name" -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:goto_1 -->
```qb64
n = 0
again:
n = n + 1
IF n < 3 THEN GOTO again
PRINT "n ="; n
GOTO 100
PRINT "This line is skipped."
100 PRINT "Line 100 is a line number."
END
```

Output:

```text
n = 3 
Line 100 is a line number.

Press any key to continue
```
<!-- end generated -->

## See also

`GOSUB`, `ON`, `IF`.
