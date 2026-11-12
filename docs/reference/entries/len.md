# LEN

`LEN` gives the length of a string in bytes: `LEN("Wellington")` is 10. Of a variable of another type it gives the
bytes the variable takes.

## Syntax

<!-- begin generated: syntax -->
```text
LEN(literalTextOrVariable$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A string, or a variable, an element or a member of any type but `_BIT`.

## Result

A LONG.

## Behavior

`LEN` of a string is its number of bytes. `LEN("")` is 0.
<!-- src: language/builtin-functions "Supported built-in functions"; tests\reference\examples\len_1 -->

`LEN` of a numeric variable is the size of its type: 2 for an INTEGER, 4 for a LONG or SINGLE, 8 for a DOUBLE. Of a
`STRING * n` it is `n`, and of a user-type variable the size of the type.
<!-- src: verification\v24_a_define; language/fixed-length-strings "Fixed-length strings in arrays and types"; tests\reference\examples\len_1 -->

The index of an element of a numeric or `STRING * n` array is not evaluated: `LEN(a(9))` on `DIM a(3)` raises no
error.
<!-- src: language/fixed-length-strings "Fixed-length strings in arrays and types"; Q-029 -->

`LEN` of a whole array, `LEN(a())`, is its size in bytes.
<!-- src: language/arrays-and-types "LEN of a whole array" -->

## Errors

These are compile errors:

- A number that is no variable, as `LEN(5)`: "`LEN` needs a string or a variable, element or member".
  <!-- src: language/builtin-functions "Names and arity" -->
- A `_BIT` variable: "`LEN` of a `_BIT` variable".
  <!-- src: language/numeric-semantics "Numeric types"; verification\v21_x18_len_bit -->
- A whole array of variable-length strings, or of a type with a `STRING` member.
  <!-- src: language/arrays-and-types "Arrays LEN does not take" -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:len_1 -->
```qb64
PRINT LEN("Wellington"); LEN("")
DIM n AS LONG, d AS DOUBLE, code AS STRING * 6
PRINT LEN(n); LEN(d); LEN(code)
END
```

Output:

```text
 10  0 
 4  8  6 

Press any key to continue
```
<!-- end generated -->

## See also

`STRING`, `TYPE`, `LEFT$`.
