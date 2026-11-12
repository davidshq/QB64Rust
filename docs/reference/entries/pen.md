# PEN

`PEN`, QuickBASIC's light pen function and event, is not in QB64pe. Each form is a compile error.

## Syntax

```text
x = PEN(n)
ON PEN GOSUB label
PEN {ON|OFF|STOP}
```

## Behavior

`ON PEN GOSUB h` and `PEN(1) = 2` are the compile error "Name already in use (PEN)", and `PEN ON` a syntax error, as
in QB64pe.
<!-- src: language/events "Events the old compiler does not have"; verification\v42_x17_on_pen; verification\v42_x20_pen_on; verification\v55_x03_pen_place -->

## Availability

<!-- begin generated: availability -->
Rejected, as in QB64pe. The compiler gives:

- `PEN` is not an event QB64pe traps, and no statement
- name already in use: `PEN`
<!-- end generated -->

<!-- probe: x = PEN(0) -->

<!-- probe: PEN ON -->

## See also

`ON`, `_MOUSEINPUT`.
