# _ANDALSO

`_ANDALSO` is true when both operands are true, and it evaluates the right operand only when the left one is
true. It is the safe way to write a test that is valid only after another, such as checking an index before
reading the array.

## Syntax

```text
a _ANDALSO b
```

## Arguments

- `a`, `b`: numbers of any numeric type. 0 is false and any other value is true.

## Result

-1 when both operands are true, 0 otherwise.

## Behavior

When `a` is 0, the result is 0 and `b` is not evaluated: a FUNCTION called in `b` does not run. When `a` is not 0,
`b` is evaluated, and the result is -1 if it is not 0.
<!-- src: language/numeric-semantics "Logical operators"; verification\v17_g_ops -->

The result is -1 or 0, not a combination of bits: `2 _ANDALSO 4` is -1, where `2 AND 4` is 0.
<!-- src: language/numeric-semantics "Logical operators"; tests\reference\examples\_andalso_1 -->

A floating-point operand is first rounded half to even, so a value below 0.5 counts as false: `0.4 _ANDALSO 1` is
0.
<!-- src: language/numeric-semantics "Logical operators"; verification\v17_g_ops -->

`_ANDALSO` binds less tightly than every other operator but `_ORELSE`: `1 _ANDALSO 2 AND 4` is
`1 _ANDALSO (2 AND 4)`, which is 0.
<!-- src: verification\v17_g_precedence; read: crates\syntax\src\parser\expr.rs:12-49 -->

## Errors

- A string operand is a compile error: "this operator cannot be used on strings".
  <!-- src: language/numeric-semantics "Logical operators" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:_andalso_1 -->
```qb64
DIM a(1 TO 3) AS INTEGER
a(1) = 10: a(2) = 20: a(3) = 30
i = 4
IF i <= 3 _ANDALSO a(i) > 15 THEN PRINT "Found." ELSE PRINT "Index"; i; "is out of range; a(i) was not read."
PRINT "2 _ANDALSO 4 is"; 2 _ANDALSO 4; "while 2 AND 4 is"; 2 AND 4
END
```

Output:

```text
Index 4 is out of range; a(i) was not read.
2 _ANDALSO 4 is-1 while 2 AND 4 is 0 

Press any key to continue
```
<!-- end generated -->

## See also

`_ORELSE`, `AND`, `_NEGATE`, `IF`.
