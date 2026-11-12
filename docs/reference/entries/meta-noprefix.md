# $NOPREFIX

`$NOPREFIX` is not available. In older versions of QB64 it let a program write QB64's own names without their
leading underscore, such as `BYTE` for `_BYTE`. QB64pe rejects it as deprecated, and QB64Rust does the same.

## Syntax

```text
$NOPREFIX
```

The form older programs use. It does not compile.

## Behavior

A program with `$NOPREFIX` does not compile. The names without their underscore are not known either: `DIM t AS
BYTE` is a second error, "no type is named `BYTE`".
<!-- src: compiler/pipeline "Metacommands the old compiler does not know"; verification\v21_x06_noprefix; verification\v43_x32 -->

## Errors

- `$NOPREFIX`: "`$NOPREFIX` is deprecated: QB64pe's own names must have their underscore (the IDE can convert the
  program)".
  <!-- src: compiler/pipeline "Metacommands the old compiler does not know"; verification\v21_x06_noprefix -->

## Differences

A program that uses `$NOPREFIX` must be converted: remove the line and write each of QB64's own names with its
underscore. The QB64pe IDE can do it, as its message says.
<!-- src: verification\v21_x06_noprefix -->

## Availability

<!-- begin generated: availability -->
Rejected, as in QB64pe. The compiler gives:

- `$NOPREFIX` is deprecated: QB64pe's own names must have their underscore (the IDE can convert the program)
<!-- end generated -->

<!-- probe: $NOPREFIX -->

## See also

`_BYTE`, `_UNSIGNED`, `_ERRORLINE`: names that `$NOPREFIX` programs write without the underscore.
