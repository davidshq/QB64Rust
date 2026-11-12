# NOT

`NOT` turns every bit of a number over: `NOT 0` is -1 and `NOT -1` is 0, so it turns true into false and false
into true.

## Syntax

```text
NOT a
```

## Arguments

- `a`: a number of any numeric type. A floating-point number is first rounded to a whole number.

## Result

An integer. It is a LONG for an INTEGER or LONG operand, and an `_INTEGER64` for an `_INTEGER64`.

## Behavior

Each bit of the result is the opposite of the operand's bit, so `NOT a` is `-a - 1`: `NOT 5` is -6. Only 0 and -1
turn into each other. `NOT` of another true value, such as 5, is not 0.
<!-- src: language/numeric-semantics "Logical operators"; verification\v17_g_ops; tests\reference\examples\not_1 -->

INTEGER and LONG operands are worked in 32 bits: `NOT i` with `i% = -32768` is 32767. An `_INTEGER64` operand is
worked in 64 bits. A floating-point operand is first rounded half to even: `NOT 1.5` is -3.
<!-- src: language/numeric-semantics "Logical operators"; verification\v17_g_ops -->

`NOT` takes everything up to the comparisons as its operand: `NOT 1 = 2` is `NOT (1 = 2)`, which is -1, and
`NOT 0 + 5` is `NOT 5`, which is -6. It binds more tightly than `AND` and the other logical operators: `NOT 0 AND
1` is `(NOT 0) AND 1`, which is 1.
<!-- src: verification\v17_g_precedence; verification\v17_g_ops; read: crates\syntax\src\parser\expr.rs:51-62 -->

`NOT` may follow `^`: `2 ^ NOT x` is `2 ^ (NOT x)`.
<!-- src: compiler/pipeline "Syntax errors the old compiler reports"; verification\v57_e09_power_not; verification\v24_j_power_unary -->

## Errors

- A string operand is a compile error: "`NOT` needs a number".
  <!-- src: language/numeric-semantics "Logical operators" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:not_1 -->
```qb64
PRINT "NOT 0 is"; NOT 0; "and NOT -1 is"; NOT -1
PRINT "NOT 5 is"; NOT 5
done = 0
IF NOT done THEN PRINT "Not done yet."
PRINT "NOT 1 = 2 is"; NOT 1 = 2
END
```

Output:

```text
NOT 0 is-1 and NOT -1 is 0 
NOT 5 is-6 
Not done yet.
NOT 1 = 2 is-1 

Press any key to continue
```
<!-- end generated -->

## See also

`_NEGATE`, `AND`, `OR`, `XOR`.
