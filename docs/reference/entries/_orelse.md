# _ORELSE

`_ORELSE` is true when either operand is true, and it evaluates the right operand only when the left one is
false.

## Syntax

```text
a _ORELSE b
```

## Arguments

- `a`, `b`: numbers of any numeric type. 0 is false and any other value is true.

## Result

-1 when either operand is true, 0 otherwise.

## Behavior

When `a` is not 0, the result is -1 and `b` is not evaluated: a FUNCTION called in `b` does not run. When `a` is
0, `b` is evaluated, and the result is -1 if it is not 0.
<!-- src: language/numeric-semantics "Logical operators"; verification\v17_g_ops; tests\reference\examples\_orelse_1 -->

The result is -1 or 0, not a combination of bits: `0 _ORELSE 2.5` is -1.
<!-- src: language/numeric-semantics "Logical operators"; verification\v17_g_ops -->

A floating-point operand is first rounded half to even, so a value below 0.5 counts as false.
<!-- src: language/numeric-semantics "Logical operators"; verification\v17_g_ops -->

`_ORELSE` binds less tightly than every other operator: `0 _ORELSE 0 OR 2` is `0 _ORELSE (0 OR 2)`, which is -1.
<!-- src: verification\v17_g_precedence; read: crates\syntax\src\parser\expr.rs:12-49 -->

## Errors

- A string operand is a compile error: "this operator cannot be used on strings".
  <!-- src: language/numeric-semantics "Logical operators" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:_orelse_1 -->
```qb64
PRINT "0 _ORELSE 5 is"; 0 _ORELSE 5
PRINT "First check:"; 1 _ORELSE Check(1)
PRINT "Second check:"; 0 _ORELSE Check(2)
END

FUNCTION Check (n)
    PRINT "(Check"; n; "runs)";
    Check = -1
END FUNCTION
```

Output:

```text
0 _ORELSE 5 is-1 
First check:-1 
Second check:(Check 2 runs)-1 

Press any key to continue
```
<!-- end generated -->

## See also

`_ANDALSO`, `OR`, `_NEGATE`.
