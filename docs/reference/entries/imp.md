# IMP

`IMP` combines two numbers bit by bit: a bit of the result is 0 only where the left operand has a 1 and the right
a 0. With comparisons, it means "if the left is true, the right is true too".

## Syntax

```text
a IMP b
```

## Arguments

- `a`, `b`: numbers of any numeric type. A floating-point number is first rounded to a whole number.

## Result

An integer. It is a LONG when both operands are INTEGER or LONG, and an `_INTEGER64` when one is an
`_INTEGER64`.

## Behavior

`a IMP b` is `(NOT a) OR b`: `5 IMP 3` is -5. With -1 and 0, it is 0 only for `-1 IMP 0`.
<!-- src: language/numeric-semantics "Logical operators"; verification\v17_g_imp_eqv; tests\reference\examples\imp_1 -->

INTEGER and LONG operands are worked in 32 bits, and an `_INTEGER64` operand makes it 64 bits. A floating-point
operand is first rounded half to even: `1.5 IMP 0` is -3.
<!-- src: language/numeric-semantics "Logical operators"; verification\v17_g_imp_eqv -->

`IMP` binds less tightly than every other bit-by-bit operator, and more tightly than `_ANDALSO` and `_ORELSE`:
`1 OR 2 IMP 0` is `(1 OR 2) IMP 0`, which is -4.
<!-- src: verification\v17_g_imp_eqv; read: crates\syntax\src\parser\expr.rs:12-62 -->

A chain of `IMP`s is worked from left to right: `5 IMP 3 IMP 0` is `(5 IMP 3) IMP 0`, which is 4.
<!-- src: language/numeric-semantics "Logical operators"; D-005 -->

## Errors

- A string operand is a compile error: "this operator cannot be used on strings".
  <!-- src: language/numeric-semantics "Logical operators" -->

## Differences

From QB64pe: it computes `a IMP b IMP c` as `a OR b OR c`, so `5 IMP 3 IMP 0` is 7 there. QB64Rust gives 4, the
value of `(5 IMP 3) IMP 0` (`DIVERGENCES-QB64PE.md` D-005).
<!-- src: D-005; verification\v17_g_precedence -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:imp_1 -->
```qb64
PRINT " a  b  a IMP b"
FOR a = -1 TO 0
    FOR b = -1 TO 0
        PRINT a; b; a IMP b
    NEXT b
NEXT a
END
```

Output:

```text
 a  b  a IMP b
-1 -1 -1 
-1  0  0 
 0 -1 -1 
 0  0 -1 

Press any key to continue
```
<!-- end generated -->

## See also

`EQV`, `NOT`, `OR`.
