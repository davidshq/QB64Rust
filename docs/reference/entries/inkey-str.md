# INKEY$

`INKEY$` gives the next key waiting in the keyboard buffer, or `""` when none is. It does not wait, so a program calls
it in a loop.

## Syntax

<!-- begin generated: syntax -->
```text
INKEY$
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A STRING: `""`, one character, or two for a key with no character, the first being `CHR$(0)`.

## Behavior

With no key waiting `INKEY$` gives `""`.
<!-- src: language/screen-output "Functions that read the screen and the keyboard"; tests\reference\examples\inkey-str_1 -->

`INKEY$` is written without parentheses.
<!-- src: language/screen-output "Functions that read the screen and the keyboard" -->

`_KEYCLEAR 1` empties its buffer.
<!-- src: language/keyboard "_KEYCLEAR" -->

## Errors

- `INKEY$()` with parentheses is a compile error.
  <!-- src: language/screen-output "Functions that read the screen and the keyboard" -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:inkey-str_1 -->
```qb64
k$ = INKEY$
PRINT "With no key waiting, INKEY$ gives a string of length"; LEN(k$)
END
```

Output:

```text
With no key waiting, INKEY$ gives a string of length 0 

Press any key to continue
```
<!-- end generated -->

## See also

[`_KEYHIT`](_keyhit.md), [`INPUT`](input.md), [`_KEYCLEAR`](_keyclear.md).
