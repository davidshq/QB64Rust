# _ROUND

`_ROUND` rounds a number to the nearest whole number: `_ROUND(2.6)` is 3. A value exactly halfway goes to the even
number.

## Syntax

<!-- begin generated: syntax -->
```text
_ROUND(number)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number of any numeric type.

## Result

An `_INTEGER64`.

## Behavior

`_ROUND(2.4)` is 2 and `_ROUND(-2.6)` is -3. Halves go to the even number: `_ROUND(0.5)` is 0, `_ROUND(1.5)` is 2 and
`_ROUND(2.5)` is 2.
<!-- src: verification\v20_f_math_edges; tests\reference\examples\_round_1 -->

A value beyond the `_INTEGER64` range, such as `_ROUND(1E30)`, gives -9223372036854775808, with no error.
<!-- src: verification\v20_f_math_edges -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:_round_1 -->
```qb64
PRINT _ROUND(2.4); _ROUND(2.6); _ROUND(-2.6)
PRINT "Halves go to the even number:"; _ROUND(0.5); _ROUND(1.5); _ROUND(2.5)
END
```

Output:

```text
 2  3 -3 
Halves go to the even number: 0  2  2 

Press any key to continue
```
<!-- end generated -->

## See also

`CINT`, `CLNG`, `INT`, `_CEIL`.
