# XOR

`XOR` combines two numbers bit by bit: a bit of the result is 1 where exactly one operand has a 1. It is often
used to switch a value between two states.

## Syntax

```text
a XOR b
```

## Arguments

- `a`, `b`: numbers of any numeric type. A floating-point number is first rounded to a whole number.

## Result

An integer. It is a LONG when both operands are INTEGER or LONG, and an `_INTEGER64` when one is an
`_INTEGER64`.

## Behavior

Each bit of the result is 1 where the bits of the operands differ: `5 XOR 3` is 6. `x = x XOR 1` switches `x`
between 0 and 1.
<!-- src: language/numeric-semantics "Logical operators"; tests\reference\examples\xor_1 -->

INTEGER and LONG operands are worked in 32 bits, and an `_INTEGER64` operand makes it 64 bits. A floating-point
operand is first rounded half to even.
<!-- src: language/numeric-semantics "Logical operators"; verification\v17_g_ops -->

`XOR` binds less tightly than `OR` and more tightly than `EQV` and `IMP`: `1 OR 2 XOR 3` is `(1 OR 2) XOR 3`,
which is 0, and `1 XOR 3 EQV 0` is `(1 XOR 3) EQV 0`, which is -3.
<!-- src: verification\v17_g_precedence; read: crates\syntax\src\parser\expr.rs:12-62 -->

## Errors

- A string operand is a compile error: "this operator cannot be used on strings".
  <!-- src: language/numeric-semantics "Logical operators" -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:xor_1 -->
```qb64
PRINT "5 XOR 3 is"; 5 XOR 3
light = 0
FOR i = 1 TO 3
    light = light XOR 1
    PRINT "Switch"; i; ": the light is"; light
NEXT i
END
```

Output:

```text
5 XOR 3 is 6 
Switch 1 : the light is 1 
Switch 2 : the light is 0 
Switch 3 : the light is 1 

Press any key to continue
```
<!-- end generated -->

## See also

`AND`, `OR`, `EQV`, `NOT`. `PUT` (graphics) takes `XOR` as an action word, not as this operator.
