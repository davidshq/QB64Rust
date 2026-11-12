# _BLINK

`_BLINK OFF` makes the text colors 16 to 31 bright backgrounds instead of blinking, and `_BLINK ON` makes them blink
again. The function `_BLINK` gives -1 while they blink.

## Syntax

```text
_BLINK {ON|OFF}
value = _BLINK
```

## Behavior

After `_BLINK OFF` the function gives 0, and after `_BLINK ON` -1.
<!-- src: language/screen-output "_BLINK and the function _DISPLAY"; tests\reference\examples\_blink_1 -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/screen-output "_BLINK and the function _DISPLAY" -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_blink_1 -->
```qb64
img& = _NEWIMAGE(80, 25, 0)
_DEST img&
_BLINK OFF
whenOff = _BLINK
_BLINK ON
whenOn = _BLINK
_DEST _CONSOLE
PRINT "_BLINK after OFF:"; whenOff; "and after ON:"; whenOn
END
```

Output:

```text
_BLINK after OFF: 0 and after ON:-1 

Press any key to continue
```
<!-- end generated -->

## See also

`COLOR`.
