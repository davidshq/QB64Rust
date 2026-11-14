# EQV

`EQV` combines two numbers bit by bit: a bit of the result is 1 where the operands' bits are the same. With
comparisons, it means "both true or both false".

## Syntax

```text
a EQV b
```

## Arguments

- `a`, `b`: numbers of any numeric type. A floating-point number is first rounded to a whole number.

## Result

An integer. It is a LONG when both operands are INTEGER or LONG, and an `_INTEGER64` when one is an
`_INTEGER64`.

## Behavior

`a EQV b` is `NOT (a XOR b)`: `5 EQV 3` is -7, `-1 EQV -1` is -1 and `-1 EQV 0` is 0.
<!-- src: language/numeric-semantics "Logical operators"; verification\v17_g_imp_eqv; tests\reference\examples\eqv_1 -->

INTEGER and LONG operands are worked in 32 bits, and an `_INTEGER64` operand makes it 64 bits. A floating-point
operand is first rounded half to even: `2.5 EQV 2` is -1.
<!-- src: language/numeric-semantics "Logical operators"; verification\v17_g_imp_eqv -->

`EQV` binds less tightly than `XOR` and more tightly than `IMP`: `1 XOR 3 EQV 0` is `(1 XOR 3) EQV 0`, and
`1 IMP 0 EQV 0` is `1 IMP (0 EQV 0)`. A chain of `EQV`s is worked from left to right.
<!-- src: verification\v17_g_imp_eqv; verification\v17_g_precedence; read: crates\syntax\src\parser\expr.rs:12-62 -->

## Errors

- A string operand is a compile error: "this operator cannot be used on strings".
  <!-- src: language/numeric-semantics "Logical operators" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:eqv_1 -->
```qb64
PRINT "-1 EQV -1 is"; -1 EQV -1; "and -1 EQV 0 is"; -1 EQV 0
a = 3 > 2: b = 1 > 0
IF a EQV b THEN PRINT "Both comparisons agree."
END
```

Output:

```text
-1 EQV -1 is-1 and -1 EQV 0 is 0 
Both comparisons agree.

Press any key to continue
```
<!-- end generated -->

## See also

[`IMP`](imp.md), [`XOR`](xor.md), [`NOT`](not.md).
