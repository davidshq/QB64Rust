# $ERROR

`$ERROR` stops the compile with a message of the program's own. It stands in a `$IF` branch, to refuse a platform
or a setting the program does not support.

## Syntax

```text
$ERROR message
```

## Arguments

- `message`: any text, up to the end of the line.

## Behavior

A `$ERROR` in code that is compiled is a compile error at its line. The message shows the text in upper case.
<!-- src: compiler/pipeline "Preprocessor"; verification\v16_m6_error_active -->

A `$ERROR` in a `$IF` branch that is not compiled does nothing.
<!-- src: compiler/pipeline "Preprocessor"; verification\v16_m6_error_inactive -->

## Errors

- A `$ERROR` that is compiled: "`$ERROR`: STOP HERE" for `$ERROR stop here`, with QB64pe's "Compilation check
  failed: STOP HERE" as a note.
  <!-- src: verification\v16_m6_error_active -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

<!-- probe: $IF 0 THEN : $ERROR never : $END IF -->

## See also

`$IF`, `$LET`.
