# _CEIL

`_CEIL` gives the smallest whole number not below a number: `_CEIL(2.1)` is 3. It rounds up, toward plus infinity.

## Syntax

<!-- begin generated: syntax -->
```text
_CEIL(expression)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number of any numeric type.

## Result

A floating-point number with no fraction.

## Behavior

`_CEIL(2)` is 2 and `_CEIL(-2.9)` is -2. It is the usual way to count how many groups a number of items needs:
`_CEIL(23 / 10)` is 3.
<!-- src: verification\v22_f_functions; tests\reference\examples\_ceil_1 -->

A SINGLE argument is computed as a SINGLE.
<!-- src: language/builtin-functions "Supported built-in functions" -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_ceil_1 -->
```qb64
PRINT _CEIL(2.1); _CEIL(2); _CEIL(-2.9)
items = 23: perPage = 10
PRINT "Pages needed:"; _CEIL(items / perPage)
END
```

Output:

```text
 3  2 -2 
Pages needed: 3 

Press any key to continue
```
<!-- end generated -->

## See also

[`INT`](int.md), [`FIX`](fix.md), [`_ROUND`](_round.md).
