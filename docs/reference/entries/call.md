# CALL

`CALL` runs a SUB: `CALL Show(1, 2)` is the same as `Show 1, 2`. With `CALL`, the arguments stand in parentheses.

## Syntax

```text
CALL name [(argument [, argument]...)]
name [argument [, argument]...]
```

## Behavior

`CALL name(args)` and `name args` both call the SUB with the arguments.
<!-- src: language/procedures "Calls"; tests\reference\examples\call_1 -->

The parentheses of `CALL` belong to the statement, so a variable written plainly inside them is still passed by
reference: `CALL bump(n)` changes `n`, as `bump n` does.
<!-- src: language/procedures "Argument passing" -->

A built-in statement may follow `CALL` too: `CALL SWAP(s, t)`.
<!-- src: language/builtin-statements "A statement with a template after CALL" -->

## Errors

These are compile errors:

- `CALL` of a name that is no SUB: "`CALL nosuch`: no SUB of this name".
  <!-- src: language/procedures "A statement that names no SUB" -->
- `CALL` of a FUNCTION.
  <!-- src: language/procedures "A program FUNCTION as a statement" -->

## Availability

<!-- begin generated: availability -->
Supported. All 114 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:call_1 -->
```qb64
CALL Show(1, 2)
Show 3, 4
END

SUB Show (a AS INTEGER, b AS INTEGER)
    PRINT "a ="; a; "b ="; b
END SUB
```

Output:

```text
a = 1 b = 2 
a = 3 b = 4 

Press any key to continue
```
<!-- end generated -->

## See also

[`SUB`](sub.md), [`FUNCTION`](function.md), [`DECLARE`](declare.md).
