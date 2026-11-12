# _KEYCLEAR

`_KEYCLEAR` empties the keyboard buffers, so that keys pressed earlier are not read later.

## Syntax

```text
_KEYCLEAR [buffer]
```

## Arguments

- `buffer`, optional: 1 for the `INKEY$` buffer, 2 for the `_KEYHIT` buffer, 3 for the `INP(&H60)` one. With none, all
  three.

## Behavior

`_KEYCLEAR` empties the `INKEY$`, `_KEYHIT` and `INP(&H60)` buffers, and `_KEYCLEAR n` only that one.
<!-- src: language/keyboard "_KEYCLEAR"; tests\reference\examples\_keyclear_1 -->

## Errors

- Any other buffer number raises run-time error 5.
  <!-- src: language/keyboard "_KEYCLEAR" -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_keyclear_1 -->
```qb64
_KEYCLEAR
PRINT "After _KEYCLEAR, INKEY$ has length"; LEN(INKEY$)
END
```

Output:

```text
After _KEYCLEAR, INKEY$ has length 0 

Press any key to continue
```
<!-- end generated -->

## See also

[`INKEY$`](inkey-str.md), [`_KEYHIT`](_keyhit.md).
