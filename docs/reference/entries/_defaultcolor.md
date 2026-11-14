# _DEFAULTCOLOR

`_DEFAULTCOLOR` gives the foreground color of the destination, and `_BACKGROUNDCOLOR` its background color.

## Syntax

```text
color~& = _DEFAULTCOLOR[(image)]
color~& = _BACKGROUNDCOLOR[(image)]
```

## Result

An `_UNSIGNED LONG`.

## Behavior

After `COLOR 14, 2` they are 14 and 2.
<!-- src: language/screen-output "The default colours"; tests\reference\examples\_defaultcolor_1 -->

With an argument they give the colors of the image or page named.
<!-- src: language/screen-output "The default colours" -->

## Errors

- A handle of no image raises QB64pe's run-time error.
  <!-- src: language/screen-output "The default colours" -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_defaultcolor_1 -->
```qb64
img& = _NEWIMAGE(20, 20, 256)
_DEST img&
COLOR 14, 2
d = _DEFAULTCOLOR: b = _BACKGROUNDCOLOR
_DEST _CONSOLE
PRINT "The colors are"; d; "on"; b
END
```

Output:

```text
The colors are 14 on 2 

Press any key to continue
```
<!-- end generated -->

## See also

[`COLOR`](color.md), [`_DEST`](_dest.md).
