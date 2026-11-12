# _ASSERT

`_ASSERT condition` checks that a condition holds while the program runs. With `$ASSERTS`, a condition that is 0 stops
the program; without it, `_ASSERT` does nothing.

## Syntax

```text
_ASSERT condition[, message$]
```

## Behavior

Without `$ASSERTS`, `_ASSERT` does nothing, though its condition and message are still evaluated.
<!-- src: language/asserts-and-debug "_ASSERT"; tests\reference\examples\_assert_1 -->

With `$ASSERTS`, a condition that is 0 raises run-time error 314. In a program with a console and with a message, the
message is written to the console and error 315 is raised instead.
<!-- src: language/asserts-and-debug "_ASSERT" -->

`ON ERROR` does not trap these errors: the program stops at the `_ASSERT`'s line.
<!-- src: language/asserts-and-debug "_ASSERT" -->

## Errors

- `_ASSERT` with no condition, or with other arguments: "this `_ASSERT` statement matches none of its forms: `_ASSERT
  ?[,?]`".
  <!-- src: language/asserts-and-debug "_ASSERT" -->

## Differences

A program stopped by a failed assertion exits with code 1; QB64pe's exits with 0 (`DIVERGENCES-QB64PE.md` D-012).
<!-- src: D-012 -->

## Availability

<!-- begin generated: availability -->
Supported. All 19 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_assert_1 -->
```qb64
x = 0
_ASSERT x > 0
PRINT "Without $ASSERTS, _ASSERT does nothing."
END
```

Output:

```text
Without $ASSERTS, _ASSERT does nothing.

Press any key to continue
```
<!-- end generated -->

## See also

`$ASSERTS`, `ON ERROR`.
