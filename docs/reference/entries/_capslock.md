# _CAPSLOCK

`_CAPSLOCK`, `_NUMLOCK` and `_SCROLLLOCK` give the state of the lock keys, -1 when on and 0 when off. As statements
they set the state: `_CAPSLOCK ON`, `_NUMLOCK OFF`, `_SCROLLLOCK _TOGGLE`.

## Syntax

```text
value = _CAPSLOCK
_CAPSLOCK {ON|OFF|_TOGGLE}
value = _NUMLOCK
_NUMLOCK {ON|OFF|_TOGGLE}
value = _SCROLLLOCK
_SCROLLLOCK {ON|OFF|_TOGGLE}
```

## Behavior

The functions give the key's state, 0 or -1, as libqb reads it.
<!-- src: language/keyboard "The lock keys" -->

The statements change the machine's keyboard state, so no test runs them; they compile to QB64pe's calls.
<!-- src: language/keyboard "The lock keys" -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_CAPSLOCK (ON)`: "this `_CAPSLOCK` statement matches none of its forms".
  - `x = _CAPSLOCK(1)`: "`_CAPSLOCK` takes no arguments".
  - `_CAPSLOCK 1`: "this `_CAPSLOCK` statement matches none of its forms".
  <!-- src: language/keyboard "The lock keys"; verification\v58_x105_capslock_stmt_paren; verification\v58_x23_capslock_fn_arg; verification\v58_x24_capslock_number -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: c = _CAPSLOCK : n = _NUMLOCK : s = _SCROLLLOCK -->

## See also

[`_KEYDOWN`](_keydown.md), [`INKEY$`](inkey-str.md).
