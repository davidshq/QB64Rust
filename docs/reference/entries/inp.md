# INP

`INP(port)` reads a byte from one of the I/O ports libqb emulates: the palette, the keyboard and the display status.

## Syntax

<!-- begin generated: syntax -->
```text
INP(address)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG.

## Behavior

The ports emulated are the palette's `&H3C7`, `&H3C8` and `&H3C9`, the keyboard's `&H60` and the display status
`&H3DA`. After `OUT &H3C7, 1`, three `INP(&H3C9)` give the red, green and blue of color 1, from 0 to 63.
<!-- src: language/conventional-memory "INP, OUT and WAIT"; verification\v58_s01_ports_window -->

Any other port gives 0.
<!-- src: language/conventional-memory "INP, OUT and WAIT"; tests\reference\examples\inp_1 -->

## Errors

- A port above 65535 or below -65536 raises error 6.
  <!-- src: language/conventional-memory "INP, OUT and WAIT"; read: libqb\internal\c\libqb.cpp:16331-16336; tests\reference\examples\inp_1 -->
- `INP` with no port, a string or two arguments, and `INP` as a statement, are compile errors.
  <!-- src: verification\v58_x12_inp_bare; verification\v58_x13_inp_string; verification\v58_x14_inp_two_args; verification\v58_x96_inp_statement -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:inp_1 -->
```qb64
ON ERROR GOTO trap
PRINT "A port libqb does not emulate:"; INP(&H379)
x = INP(70000)
END
trap:
PRINT "error"; ERR
RESUME NEXT
```

Output:

```text
A port libqb does not emulate: 0 
error 6 

Press any key to continue
```
<!-- end generated -->

## See also

[`OUT`](out.md), [`WAIT`](wait.md), [`PALETTE`](palette.md).
