# OUT

`OUT port, value` writes a byte to one of the I/O ports libqb emulates: the palette's, and `&H3C0` for blinking.

## Syntax

<!-- begin generated: syntax -->
```text
OUT registerAddress%, value%
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

`OUT &H3C8, n` and then three `OUT &H3C9` of red, green and blue from 0 to 63 set color `n` of the palette. `OUT
&H3C0` sets what `_BLINK` gives.
<!-- src: language/conventional-memory "INP, OUT and WAIT"; verification\v58_s01_ports_window -->

Any other port does nothing, a port out of range included: `OUT 70000, 1` raises no error.
<!-- src: read: libqb\internal\c\libqb.cpp:12829-12840; tests\reference\examples\out_1 -->

## Errors

- `OUT` with one argument, with three or with a string, and `OUT` as a function, are compile errors.
  <!-- src: verification\v58_x15_out_one_arg; verification\v58_x16_out_three_args; verification\v58_x17_out_string; verification\v58_x98_out_fn -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:out_1 -->
```qb64
ON ERROR GOTO trap
OUT &H378, 1
PRINT "A port libqb does not emulate: no error"
OUT 70000, 1
PRINT "A port out of range: no error either"
END
trap:
PRINT "error"; ERR
RESUME NEXT
```

Output:

```text
A port libqb does not emulate: no error
A port out of range: no error either

Press any key to continue
```
<!-- end generated -->

## See also

[`INP`](inp.md), [`PALETTE`](palette.md), [`_BLINK`](_blink.md).
