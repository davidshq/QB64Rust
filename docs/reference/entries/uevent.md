# UEVENT

`ON UEVENT GOSUB` and `UEVENT ON`, QuickBASIC's user-defined event, are not in QB64pe. Both are compile errors.

## Syntax

```text
ON UEVENT GOSUB label
UEVENT {ON|OFF|STOP}
```

## Behavior

`ON UEVENT GOSUB h` is the compile error "Name already in use (UEVENT)" and `UEVENT ON` a syntax error, as in QB64pe.
<!-- src: language/events "Events the old compiler does not have"; verification\v42_x19_on_uevent; verification\v42_x21_uevent_on -->

`UEVENT` may still name a member of a `TYPE`.
<!-- src: verification\v61_m_uevent -->

## Availability

<!-- begin generated: availability -->
Rejected, as in QB64pe. The compiler gives:

- `UEVENT` is not an event QB64pe traps, and no statement
<!-- end generated -->

<!-- probe: UEVENT ON -->

## See also

`ON`, `TIMER`.
