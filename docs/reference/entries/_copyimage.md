# _COPYIMAGE

`_COPYIMAGE(handle)` makes a new image that is a copy of another, and gives its handle.

## Syntax

<!-- begin generated: syntax -->
```text
_COPYIMAGE(imageHandle&[, mode])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG handle.

## Behavior

The copy is an image of its own: drawing on the original afterwards does not change it.
<!-- src: language/images "Image handles"; tests\reference\examples\_copyimage_1 -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `l = _COPYIMAGE`: "`_COPYIMAGE` takes 1 or 2 arguments".
  - `l = _COPYIMAGE(1, 2, 3)`: "`_COPYIMAGE` takes 1 or 2 arguments".
  <!-- src: language/images "Forms of the image built-ins"; verification\v31_x02_copyimage_bare; verification\v31_x92_copyimage_three -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_copyimage_1 -->
```qb64
a& = _NEWIMAGE(10, 10, 32)
_DEST a&: PSET (2, 3), _RGB32(1, 2, 3)
b& = _COPYIMAGE(a&)
_DEST a&: PSET (2, 3), _RGB32(9, 9, 9)
_SOURCE b&
_DEST _CONSOLE
PRINT "The copy kept the old pixel: "; HEX$(POINT(2, 3)); _WIDTH(b&)
END
```

Output:

```text
The copy kept the old pixel: FF010203 10 

Press any key to continue
```
<!-- end generated -->

## See also

[`_NEWIMAGE`](_newimage.md), [`_PUTIMAGE`](_putimage.md), [`_FREEIMAGE`](_freeimage.md).
