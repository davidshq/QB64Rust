# _KEYDOWN

`_KEYDOWN(code)` tells whether a key is held down at this moment: -1 while it is, 0 otherwise. Games use it to read
several keys at once.

## Syntax

<!-- begin generated: syntax -->
```text
_KEYDOWN(code&)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `code`: the key's code, as `_KEYHIT` gives it, such as 100304 for the left Shift.

## Result

-1 or 0.

## Behavior

`_KEYDOWN(code)` is -1 while that key is held and 0 once it is lifted.
<!-- src: language/keyboard "_KEYDOWN" -->

## Errors

- A code of 0 or less raises run-time error 5.
  <!-- src: language/keyboard "_KEYDOWN" -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: k& = _KEYDOWN(100304) -->

## See also

[`_KEYHIT`](_keyhit.md), [`INKEY$`](inkey-str.md).
