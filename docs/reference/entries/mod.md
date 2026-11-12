# MOD

`MOD` gives the remainder of a whole-number division: `17 MOD 5` is 2. `n MOD 2 = 0` tests whether `n` is even.

## Syntax

```text
a MOD b
```

## Arguments

- `a`: the number divided, of any numeric type.
- `b`: the number it is divided by, of any numeric type.

A floating-point operand is first rounded to a whole number.

## Result

An integer: the remainder of `a \ b`.

## Behavior

Both operands are rounded half to even to whole numbers first. Then `a` is divided by `b`, cut toward zero, and
the remainder is the result: `7.5 MOD 2` is `8 MOD 2`, which is 0.
<!-- src: language/numeric-semantics "Integer division and MOD"; verification\v17_g_ops -->

The remainder has the sign of `a`, whatever the sign of `b`: `-7 MOD 3` is -1, `7 MOD -3` is 1 and `-7 MOD -3` is
-1.
<!-- src: language/numeric-semantics "Integer division and MOD"; verification\v17_g_ops; verification\v17_g_precedence -->

The smallest value of a signed type `MOD -1` is 0.
<!-- src: language/numeric-semantics "Integer division and MOD"; D-006 -->

`MOD` binds less tightly than `*`, `/` and `\`, and more tightly than `+` and `-`: `2 * 3 MOD 4` is
`(2 * 3) MOD 4`, which is 2, and `5 MOD 3 \ 2` is `5 MOD (3 \ 2)`, which is 0.
<!-- src: verification\v17_g_ops; read: crates\syntax\src\parser\expr.rs:12-49 -->

## Errors

- A divisor that is 0 after rounding raises run-time error 11, "Division by zero". An `ON ERROR` handler can catch
  it, and the result is then 0.
  <!-- src: language/numeric-semantics "Integer division and MOD"; D-052; verification\v63_e1_error_11_handler -->

## Differences

From QB64pe:

- A `MOD` by 0 is a critical error there: no handler catches it, and the program ends. QB64Rust raises error 11,
  which a handler can catch, as QB 4.5 does (`DIVERGENCES-QB64PE.md` D-052, `DIVERGENCES-QB45.md` Q-003).
  <!-- src: D-052; Q-003; verification\v17_g_mod_zero -->
- The smallest LONG or `_INTEGER64` `MOD -1` makes the program crash there. QB64Rust gives 0
  (`DIVERGENCES-QB64PE.md` D-006).
  <!-- src: D-006; verification\v17_g_long_min_div -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:mod_1 -->
```qb64
PRINT "17 MOD 5 is"; 17 MOD 5
PRINT "-17 MOD 5 is"; -17 MOD 5
PRINT "7.5 MOD 2 is"; 7.5 MOD 2
FOR n = 1 TO 6
    IF n MOD 2 = 0 THEN PRINT n; "is even"
NEXT n
END
```

Output:

```text
17 MOD 5 is 2 
-17 MOD 5 is-2 
7.5 MOD 2 is 0 
 2 is even
 4 is even
 6 is even

Press any key to continue
```
<!-- end generated -->

## See also

`\` (integer division), [`INT`](int.md), [`FIX`](fix.md).
