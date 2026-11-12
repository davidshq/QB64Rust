# TRON

`TRON` and `TROFF` are not available. In QuickBASIC they turned line tracing on and off; QB64pe lists them but rejects
them, and QB64Rust does the same.

## Syntax

```text
TRON
TROFF
```

## Behavior

`TRON` and `TROFF` are compile errors, also where the line can never run, and after `CALL`: "`TRON` is not implemented
in QB64pe".
<!-- src: language/builtin-functions "The old compiler's stubs"; verification\v57_s01_tron -->

## Availability

<!-- begin generated: availability -->
Rejected, as in QB64pe. The compiler gives:

- `TRON` is not implemented in QB64pe
<!-- end generated -->

<!-- probe: TRON -->

## See also

`$DEBUG`.
