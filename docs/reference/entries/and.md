# AND

`AND` combines two numbers bit by bit: a bit of the result is 1 where both operands have a 1. With comparisons,
which give -1 or 0, it means "both are true".

## Syntax

```text
a AND b
```

## Arguments

- `a`, `b`: numbers of any numeric type. A floating-point number is first rounded to a whole number.

## Result

An integer. It is a LONG when both operands are INTEGER or LONG, and an `_INTEGER64` when one is an
`_INTEGER64`.

## Behavior

Each bit of the result is 1 where the bits of both operands are 1: `6 AND 4` is 4 and `6 AND 1` is 0. A
comparison gives -1 (every bit 1) when true and 0 when false, so `(a > 1) AND (a < 9)` is -1 only when both hold.
<!-- src: language/numeric-semantics "Logical operators"; language/numeric-semantics "Comparisons"; verification\v17_g_ops -->

INTEGER and LONG operands are worked in 32 bits, and an `_INTEGER64` operand makes it 64 bits. A floating-point
operand is first rounded half to even: `1.5 AND 3` is 2, and `2.5 AND 7` is 2.
<!-- src: language/numeric-semantics "Logical operators"; verification\v17_g_ops -->

Both operands are always evaluated, also when the left one is 0. `_ANDALSO` skips the right operand instead.
<!-- src: language/numeric-semantics "Logical operators"; verification\qb45\q45_and_short_circuit -->

`AND` binds less tightly than the comparisons and `NOT`, and more tightly than `OR`, `XOR`, `EQV` and `IMP`:
`6 AND 3 = 2` is `6 AND (3 = 2)`, which is 0, and `NOT 0 AND 1` is `(NOT 0) AND 1`, which is 1.
<!-- src: verification\v17_g_precedence; read: crates\syntax\src\parser\expr.rs:12-62 -->

## Errors

These are compile errors:

- A string operand, as in `"a" AND "b"`: "this operator cannot be used on strings".
  <!-- src: language/numeric-semantics "Logical operators"; verification\v17_g_err_string_and -->
- `AND` with nothing before it, as in `x = AND 1`: "expected a value before the operator `AND`".
  <!-- src: compiler/pipeline "Syntax errors the old compiler reports"; verification\v57_e08_assign_and -->

## Differences

From BASIC PDS 7.1: in an `IF`, `ELSEIF`, `WHILE` or `DO` condition whose two sides are comparisons, PDS 7.1 skips
the right side when the left is false. QB64Rust evaluates both sides, as QB 4.5 and QB64pe do. A ported program
that calls `RND` or `INKEY$` on the right side then behaves differently (`DIVERGENCES-QB45.md` Q-026).
<!-- src: Q-026; verification\qb45\q45_and_short_circuit -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:and_1 -->
```qb64
flags = 6
PRINT "6 AND 4 is"; flags AND 4; "and 6 AND 1 is"; flags AND 1
age = 30
IF age >= 18 AND age < 65 THEN PRINT "Of working age."
PRINT "A true comparison is"; (age > 18); "and AND keeps it:"; (age > 18) AND (age < 65)
END
```

Output:

```text
6 AND 4 is 4 and 6 AND 1 is 0 
Of working age.
A true comparison is-1 and AND keeps it:-1 

Press any key to continue
```
<!-- end generated -->

## See also

[`OR`](or.md), [`NOT`](not.md), [`XOR`](xor.md), [`_ANDALSO`](_andalso.md). [`PUT`](put.md) (graphics) takes `AND` as an action word, not as this operator.
