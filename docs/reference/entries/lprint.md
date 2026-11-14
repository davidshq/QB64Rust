# LPRINT

`LPRINT` prints to the printer, as `PRINT` prints to the screen. `LPOS(n)` gives the column of the printer's print
head.

## Syntax

```text
LPRINT [items] [;|,]
LPRINT USING format$; values
value = LPOS(n)
```

## Behavior

`LPRINT` takes every form console `PRINT` takes: items, `;` and `,`, `TAB`, `SPC` and `USING`. It compiles to QB64pe's
printer calls.
<!-- src: language/builtin-statements "LPRINT"; language/print-using "LPRINT USING" -->

`LPOS(n)` compiles to QB64pe's call. The examples do not run it, since the tests have no printer.
<!-- src: language/builtin-functions "System information functions" -->

## Availability

<!-- begin generated: availability -->
Supported. All 19 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: LPRINT "x" -->

<!-- probe: n = LPOS(0) -->

## See also

[`PRINT`](print.md), [`PRINT USING`](print-using.md).
