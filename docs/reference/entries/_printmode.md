# _PRINTMODE

`_PRINTMODE` sets how text is drawn: `_KEEPBACKGROUND` draws only the characters, `_ONLYBACKGROUND` only their
background, `_FILLBACKGROUND` both. The function gives the mode, 1 to 3.

## Syntax

```text
_PRINTMODE {_KEEPBACKGROUND|_ONLYBACKGROUND|_FILLBACKGROUND}[, image]
mode = _PRINTMODE[(image)]
```

## Behavior

A new 32-bit image has mode 3; after `_PRINTMODE _KEEPBACKGROUND` it is 1.
<!-- src: language/fonts "Unicode text and the print mode"; tests\reference\examples\_printmode_1 -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_PRINTMODE`: "this `_PRINTMODE` statement matches none of its forms".
  - `_PRINTMODE 1`: "this `_PRINTMODE` statement matches none of its forms".
  - `PRINT _PRINTMODE(1, 2)`: "`_PRINTMODE` takes 0 or 1 arguments".
  <!-- src: language/fonts "Forms of the font built-ins"; verification\v31_x40_printmode_bare; verification\v31_x41_printmode_number; verification\v31_x42_printmode_fn_two -->

## Availability

<!-- begin generated: availability -->
Supported. All 9 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_printmode_1 -->
```qb64
img& = _NEWIMAGE(60, 20, 32)
_DEST img&
m1 = _PRINTMODE
_PRINTMODE _KEEPBACKGROUND
m2 = _PRINTMODE
_DEST _CONSOLE
PRINT "The print mode was"; m1; "and is now"; m2
END
```

Output:

```text
The print mode was 3 and is now 1 

Press any key to continue
```
<!-- end generated -->

## See also

[`_PRINTSTRING`](_printstring.md), [`COLOR`](color.md).
