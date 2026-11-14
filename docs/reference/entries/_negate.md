# _NEGATE

`_NEGATE` turns true into false and false into true: it gives -1 for 0 and 0 for any other value. Unlike `NOT`,
it treats every value but 0 as true.

## Syntax

```text
_NEGATE a
```

## Arguments

- `a`: a number of any numeric type.

## Result

-1 when `a` is 0, 0 otherwise.

## Behavior

`_NEGATE 0` is -1 and `_NEGATE 5` is 0, where `NOT 5` is -6.
<!-- src: language/numeric-semantics "Logical operators"; tests\reference\examples\_negate_1 -->

A floating-point operand is first rounded half to even, so a value below 0.5 counts as 0: `_NEGATE 0.4` is -1.
<!-- src: language/numeric-semantics "Logical operators"; verification\v17_g_ops -->

`_NEGATE` takes everything up to the comparisons as its operand, as `NOT` does: `_NEGATE 0 + 5` is `_NEGATE 5`,
which is 0, and `_NEGATE 0 = 5` is `_NEGATE (0 = 5)`, which is -1. It binds more tightly than `AND`:
`_NEGATE 0 AND 2` is 2.
<!-- src: verification\v17_g_precedence; read: crates\syntax\src\parser\expr.rs:51-62 -->

## Errors

These are compile errors:

- A string operand: "`_NEGATE` needs a number".
  <!-- src: language/numeric-semantics "Logical operators" -->
- `_NEGATE` right after `^`, as in `2 ^ _NEGATE x`: "expected a value after `^`, not `_NEGATE`".
  <!-- src: compiler/pipeline "Syntax errors the old compiler reports"; verification\v24_x238_power_negate -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:_negate_1 -->
```qb64
PRINT "_NEGATE 0 is"; _NEGATE 0; "and _NEGATE 5 is"; _NEGATE 5
PRINT "NOT 5 is"; NOT 5
found = 2
IF _NEGATE found THEN PRINT "Not found." ELSE PRINT "Found at"; found
END
```

Output:

```text
_NEGATE 0 is-1 and _NEGATE 5 is 0 
NOT 5 is-6 
Found at 2 

Press any key to continue
```
<!-- end generated -->

## See also

[`NOT`](not.md), [`_ANDALSO`](_andalso.md), [`_ORELSE`](_orelse.md).
