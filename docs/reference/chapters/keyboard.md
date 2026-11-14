# The keyboard

A program reads keys as characters with `INKEY$`, as codes of keys going down and up with `_KEYHIT`, or by asking
whether a key is held with `_KEYDOWN`.

The pages of this area: [INKEY$](../entries/inkey-str.md), [_KEYHIT](../entries/_keyhit.md),
[_KEYDOWN](../entries/_keydown.md), [_KEYCLEAR](../entries/_keyclear.md), [_CAPSLOCK](../entries/_capslock.md).

## Three buffers

`_KEYHIT` gives a key's code when it goes down, the code negated when it goes up, and 0 when its buffer is empty.
`_KEYCLEAR` empties the `INKEY$` buffer, the `_KEYHIT` buffer and the keyboard port's, or one of them.
<!-- src: language/keyboard "_KEYHIT"; language/keyboard "_KEYCLEAR" -->

`_KEYDOWN(code)` gives -1 while a key is held; a code of 0 or less raises error 5.
<!-- src: language/keyboard "_KEYDOWN" -->

## The lock keys

`_CAPSLOCK`, `_NUMLOCK` and `_SCROLLLOCK` give and set the state of the lock keys. Setting it changes the machine's
keyboard, so no test does.
<!-- src: language/keyboard "The lock keys" -->
