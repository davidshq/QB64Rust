# _DEST

`_DEST handle` makes an image the destination of drawing and text output. `_DEST` as a function gives the
destination's handle.

## Syntax

```text
_DEST handle
handle = _DEST
```

## Behavior

After `_DEST a&`, `PSET` and `PRINT` go to image `a&`. `_DEST _CONSOLE` sends `PRINT` to the console.
<!-- src: language/console-and-window "_DEST and _SOURCE"; tests\reference\examples\_dest_1 -->

A destination set in a SUB stays after the SUB returns.
<!-- src: language/console-and-window "_DEST and _SOURCE" -->

## Availability

<!-- begin generated: availability -->
Supported. All 12 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_dest_1 -->
```qb64
a& = _NEWIMAGE(20, 20, 256)
b& = _NEWIMAGE(20, 20, 256)
_DEST a&: PSET (1, 1), 3
_DEST b&: PSET (1, 1), 9
_DEST _CONSOLE
_SOURCE a&: PRINT "a&:"; POINT(1, 1);
_SOURCE b&: PRINT " b&:"; POINT(1, 1)
PRINT "_DEST is the console:"; _DEST = _CONSOLE
END
```

Output:

```text
a&: 3  b&: 9 
_DEST is the console:-1 

Press any key to continue
```
<!-- end generated -->

## See also

[`_SOURCE`](_source.md), [`_NEWIMAGE`](_newimage.md), [`_CONSOLE`](_console.md).
