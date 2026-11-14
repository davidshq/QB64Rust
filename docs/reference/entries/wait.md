# WAIT

`WAIT port, and[, xor]` waits until a port's byte, XORed with `xor` and ANDed with `and`, is not 0.

## Syntax

<!-- begin generated: syntax -->
```text
WAIT port%, andMask%[, xorMask%]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

`WAIT &H3DA, 8` waits for the display's vertical retrace, as libqb emulates it.
<!-- src: language/conventional-memory "INP, OUT and WAIT"; verification\v58_s01b_wait_window -->

A port libqb does not emulate returns at once.
<!-- src: language/conventional-memory "INP, OUT and WAIT"; tests\corpus\slice\s159_builtins_last; tests\reference\examples\wait_1 -->

## Errors

- A port above 65535 or below -65536 raises error 6.
  <!-- src: read: libqb\internal\c\libqb.cpp:16399-16412; tests\reference\examples\wait_1 -->
- `WAIT` with one argument, with four or none, and `WAIT` as a function, are compile errors.
  <!-- src: verification\v58_x18_wait_one_arg; verification\v58_x19_wait_four_args; verification\v58_x20_wait_bare; verification\v58_x99_wait_fn -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:wait_1 -->
```qb64
ON ERROR GOTO trap
WAIT &H378, 1
PRINT "A port libqb does not emulate: WAIT returns"
WAIT 70000, 1
PRINT "done"
END
trap:
PRINT "error"; ERR
RESUME NEXT
```

Output:

```text
A port libqb does not emulate: WAIT returns
error 6 
done

Press any key to continue
```
<!-- end generated -->

## See also

[`INP`](inp.md), [`OUT`](out.md).
