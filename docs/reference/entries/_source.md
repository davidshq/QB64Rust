# _SOURCE

`_SOURCE handle` makes an image the source that `POINT` and `SCREEN` read. `_SOURCE` as a function gives its handle.

## Syntax

```text
_SOURCE handle
handle = _SOURCE
```

## Behavior

After `_SOURCE img&`, `POINT` reads the pixels of `img&`.
<!-- src: language/console-and-window "_DEST and _SOURCE"; tests\reference\examples\_source_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 12 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_source_1 -->
```qb64
img& = _NEWIMAGE(20, 20, 256)
_DEST img&: PSET (2, 2), 11
_DEST _CONSOLE
_SOURCE img&
PRINT "POINT reads the source:"; POINT(2, 2); "and _SOURCE is img&:"; _SOURCE = img&
END
```

Output:

```text
POINT reads the source: 11 and _SOURCE is img&:-1 

Press any key to continue
```
<!-- end generated -->

## See also

`_DEST`, `POINT`.
