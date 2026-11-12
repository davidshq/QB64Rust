# Assertions and $DEBUG

`_ASSERT` checks a condition in a program built with `$ASSERTS`, and does nothing in one without it. `$DEBUG` marks a
program for QB64pe's IDE debugger.

The pages of this area: [_ASSERT](../entries/_assert.md), [$ASSERTS](../entries/meta-asserts.md),
[$DEBUG](../entries/meta-debug.md).

## Assertions

With `$ASSERTS`, an `_ASSERT` whose condition is 0 raises error 314, or 315 with its message in a program with a
console. `ON ERROR` does not trap either.
<!-- src: language/asserts-and-debug "_ASSERT"; language/asserts-and-debug "$ASSERTS" -->

## $DEBUG

`$DEBUG` sets `_DEBUG_` and `_SOCKETS_` to 1, and the program behaves as QB64pe's does outside the IDE: no debugger
code is written.
<!-- src: language/asserts-and-debug "$DEBUG"; D-017 -->
