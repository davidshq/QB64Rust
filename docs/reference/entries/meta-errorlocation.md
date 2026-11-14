# $ERRORLOCATION

`$ERRORLOCATION:ON` makes the message of a critical run-time error name the line and file where it happened. A
critical error is one that no `ON ERROR` handler can catch, such as running out of stack space.

## Syntax

```text
$ERRORLOCATION:ON
$ERRORLOCATION:OFF
```

The line holds nothing else: no blank around the colon and no comment after it. Letter case does not count.

## Behavior

With `$ERRORLOCATION:ON`, the program records the line of each statement before it runs it. A critical error's
message then names that line and its file: "Line: 11 (in main module)" or "Line: 3 (in e_sub.bi)".
<!-- src: language/error-handling "$ERRORLOCATION"; verification\v58_e01_on_stack_main; verification\v58_e04_on_stack_include -->

With `$ERRORLOCATION:OFF`, or with neither, a critical error's message has no line. It ends with "Enable
$ErrorLocation:ON for source location details."
<!-- src: language/error-handling "$ERRORLOCATION"; verification\v58_e02_off_stack_main -->

The setting holds for the whole program, wherever its line stands, also when it stands only in an included file.
The same setting may be given more than once.
<!-- src: language/error-handling "$ERRORLOCATION"; language/error-handling "$ERRORLOCATION set twice"; verification\v58_e07_on_twice -->

Other run-time errors are not changed. `ERL` and `_ERRORLINE` in a handler, and the message of an error with no
handler, are the same with or without the setting.
<!-- src: language/error-handling "$ERRORLOCATION"; verification\v58_e05_on_other_errors -->

A `$ERRORLOCATION` line in a `$IF` branch that is not compiled changes nothing.
<!-- src: language/error-handling "$ERRORLOCATION set twice"; verification\v58_e13_off_in_if_not_taken -->

## Errors

These are compile errors:

- An `ON` and an `OFF` in one program, in any order, in any file: "`$ERRORLOCATION` is a whole-program option and
  cannot be changed within the program". It stands at the first line whose setting differs from the last one's.
  <!-- src: language/error-handling "$ERRORLOCATION set twice"; verification\v58_e08_on_then_off; verification\v58_e10_on_main_off_include; verification\v58_e12_off_in_if_taken -->
- Any other form, such as `$ERRORLOCATION` alone, `$ERRORLOCATION:MAYBE`, `$ERRORLOCATION ON`,
  `$ERRORLOCATION: ON` or a comment after the line: "unknown metacommand `$ERRORLOCATION`", with QB64pe's "Syntax
  error" as a note.
  <!-- src: compiler/pipeline "Metacommands the old compiler does not know"; verification\v58_e14_bare; verification\v58_e16_blank; verification\v58_e17_lowercase_comment; verification\v58_e18_blank_after_colon -->

## Differences

From QB64pe: an integer division by zero is no critical error in QB64Rust. A handler can catch it, and with no
handler its message names the line whether or not `$ERRORLOCATION:ON` is given. In QB64pe its message names a
line only with `$ERRORLOCATION:ON` (`DIVERGENCES-QB64PE.md` D-052).
<!-- src: D-052; verification\v58_e20_on_div_main; verification\v58_e21_off_div_main -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

<!-- probe: $ERRORLOCATION:ON : PRINT "ok" -->

## See also

[`ON ERROR`](on-error.md), [`_ERRORLINE`](_errorline.md), [`_INCLERRORLINE`](_inclerrorline.md), [`$CHECKING`](meta-checking.md).
