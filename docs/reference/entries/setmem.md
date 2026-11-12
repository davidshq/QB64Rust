# SETMEM

`SETMEM`, QuickBASIC's function that changed the size of the far heap, is a stub in QB64pe: a compile error.

## Syntax

```text
x = SETMEM(bytes)
```

## Behavior

`x = SETMEM(0)` is the compile error "Command not implemented", also where the line is never reached.
<!-- src: language/builtin-functions "The old compiler's stubs"; verification\v57_s07_setmem -->

## Availability

<!-- begin generated: availability -->
Rejected, as in QB64pe. The compiler gives:

- `SETMEM` is not implemented in QB64pe
<!-- end generated -->

<!-- probe: x = SETMEM(0) -->

## See also

`FRE`.
