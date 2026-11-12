# COM

`ON COM(n) GOSUB` and `COM(n) ON`, QuickBASIC's events of a serial port, are not in QB64pe. Both are compile errors.

## Syntax

```text
ON COM(n) GOSUB label
COM(n) {ON|OFF|STOP}
```

## Behavior

`ON COM(1) GOSUB h` is the compile error "Name already in use (COM)" and `COM(1) ON` a syntax error, as in QB64pe.
<!-- src: language/events "Events the old compiler does not have"; verification\v42_x18_on_com; verification\v42_x22_com_on -->

`COM` may still name a member of a `TYPE`.
<!-- src: verification\v61_m_com -->

## Availability

<!-- begin generated: availability -->
Rejected, as in QB64pe. The compiler gives:

- `COM` is not an event QB64pe traps, and no statement
<!-- end generated -->

<!-- probe: COM(1) ON -->

## See also

`ON`, `OPEN`.
