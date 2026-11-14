# _FREEIMAGE

`_FREEIMAGE handle` frees an image made by `_NEWIMAGE`, `_LOADIMAGE` or `_COPYIMAGE`.

## Syntax

```text
_FREEIMAGE handle
```

## Behavior

After `_FREEIMAGE img&`, the handle names no image: `_WIDTH(img&)` raises error 258.
<!-- src: language/images "Image handles"; tests\reference\examples\_freeimage_1 -->

## Errors

- A handle that is no image raises error 258, "Invalid handle".
  <!-- src: language/images "Image handles"; tests\reference\examples\_freeimage_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 12 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_freeimage_1 -->
```qb64
img& = _NEWIMAGE(10, 10, 32)
_FREEIMAGE img&
ON ERROR GOTO handler
PRINT _WIDTH(img&)
END

handler:
PRINT "The freed handle is no image: error"; ERR
RESUME NEXT
```

Output:

```text
The freed handle is no image: error 258 

Press any key to continue
```
<!-- end generated -->

## See also

[`_NEWIMAGE`](_newimage.md), [`_COPYIMAGE`](_copyimage.md).
