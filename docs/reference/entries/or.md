# OR

`OR` combines two numbers bit by bit: a bit of the result is 1 where either operand has a 1. With comparisons,
which give -1 or 0, it means "at least one is true".

## Syntax

```text
a OR b
```

## Arguments

- `a`, `b`: numbers of any numeric type. A floating-point number is first rounded to a whole number.

## Result

An integer. It is a LONG when both operands are INTEGER or LONG, and an `_INTEGER64` when one is an
`_INTEGER64`.

## Behavior

Each bit of the result is 1 where a bit of either operand is 1: `1 OR 4` is 5. With comparisons,
`k$ = "q" OR k$ = "Q"` is -1 when either holds.
<!-- src: language/numeric-semantics "Logical operators"; tests\reference\examples\or_1 -->

INTEGER and LONG operands are worked in 32 bits, and an `_INTEGER64` operand makes it 64 bits. A floating-point
operand is first rounded half to even: `2.5 OR 0` is 2.
<!-- src: language/numeric-semantics "Logical operators"; verification\v17_g_ops -->

Both operands are always evaluated, also when the left one is not 0. `_ORELSE` skips the right operand instead.
<!-- src: language/numeric-semantics "Logical operators"; verification\qb45\q45_and_short_circuit -->

`OR` binds less tightly than `AND` and more tightly than `XOR`, `EQV` and `IMP`: `1 OR 2 AND 0` is
`1 OR (2 AND 0)`, which is 1, and `1 OR 2 XOR 3` is `(1 OR 2) XOR 3`, which is 0.
<!-- src: verification\v17_g_ops; verification\v17_g_precedence; read: crates\syntax\src\parser\expr.rs:12-62 -->

## Errors

These are compile errors:

- A string operand: "this operator cannot be used on strings".
  <!-- src: language/numeric-semantics "Logical operators" -->
- `OR` with nothing before it, as in `x = OR`: "expected a value before the operator `OR`".
  <!-- src: compiler/pipeline "Syntax errors the old compiler reports"; verification\v51_x50_assign_or_alone -->

## Differences

From BASIC PDS 7.1: in an `IF`, `ELSEIF`, `WHILE` or `DO` condition whose two sides are comparisons, PDS 7.1 skips
the right side when the left is true. QB64Rust evaluates both sides, as QB 4.5 and QB64pe do
(`DIVERGENCES-QB45.md` Q-026).
<!-- src: Q-026; verification\qb45\q45_and_short_circuit -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:or_1 -->
```qb64
bold = 1: italic = 2: underline = 4
style = bold OR underline
PRINT "The style is"; style
key$ = "Q"
IF key$ = "q" OR key$ = "Q" THEN PRINT "Quit."
END
```

Output:

```text
The style is 5 
Quit.

Press any key to continue
```
<!-- end generated -->

## See also

`AND`, `XOR`, `NOT`, `_ORELSE`. `PUT` (graphics) takes `OR` as an action word, not as this operator.
