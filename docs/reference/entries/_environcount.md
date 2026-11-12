# _ENVIRONCOUNT

`_ENVIRONCOUNT` gives the number of the program's environment variables, for `ENVIRON$(n)`.

## Syntax

<!-- begin generated: syntax -->
```text
_ENVIRONCOUNT
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG.

## Behavior

`ENVIRON$(_ENVIRONCOUNT)` is the last entry, and the next one gives "".
<!-- src: language/builtin-functions "System information functions"; tests\corpus\slice\s159_builtins_last; tests\reference\examples\_environcount_1 -->

## Errors

- An argument, use in a `CONST`, and `_ENVIRONCOUNT` as a statement are compile errors.
  <!-- src: language/builtin-functions "System information functions"; verification\v58_x66_environcount_arg; verification\v58_x111_environcount_const; verification\v58_x86_environcount_statement -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_environcount_1 -->
```qb64
n = _ENVIRONCOUNT
PRINT "Some variables:"; n > 0
PRINT "The last one has an = in it:"; INSTR(ENVIRON$(n), "=") > 0
PRINT "Past the last: ["; ENVIRON$(n + 1); "]"
END
```

Output:

```text
Some variables:-1 
The last one has an = in it:-1 
Past the last: []

Press any key to continue
```
<!-- end generated -->

## See also

`ENVIRON`.
