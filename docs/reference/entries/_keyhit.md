# _KEYHIT

`_KEYHIT` gives the next code from the key-hit buffer: a key's code when it goes down, the code made negative when it
goes up, and 0 when nothing is waiting.

## Syntax

<!-- begin generated: syntax -->
```text
_KEYHIT
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG.

## Behavior

Pressing and releasing `a` gives 97 and then -97. The codes and their order are those QB64pe's program reads for the
same keys.
<!-- src: language/keyboard "_KEYHIT" -->

With nothing waiting `_KEYHIT` is 0.
<!-- src: language/keyboard "_KEYHIT"; tests\reference\examples\_keyhit_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_keyhit_1 -->
```qb64
PRINT "With no key waiting, _KEYHIT is"; _KEYHIT
END
```

Output:

```text
With no key waiting, _KEYHIT is 0 

Press any key to continue
```
<!-- end generated -->

## See also

`_KEYDOWN`, `INKEY$`, `_KEYCLEAR`.
