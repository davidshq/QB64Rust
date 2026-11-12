# _AXIS

`_AXIS(n)` gives the position of axis `n` of the device selected by `_DEVICEINPUT`, from -1 to 1. `_WHEEL(n)` gives
how far wheel `n` turned.

## Syntax

<!-- begin generated: syntax -->
```text
_AXIS(axis_number%)
_WHEEL(wheelNumber%)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A DOUBLE.

## Behavior

They read the device selected by the last `_DEVICEINPUT(d)`; its axes count from 1 to `_LASTAXIS`, its wheels to
`_LASTWHEEL`.
<!-- src: language/device-input "The device functions"; verification\v47_b_devices -->

A program that uses it is built with libqb's device-input part, as QB64pe builds it.
<!-- src: language/device-input "The device-input runtime part" -->

## Errors

- Before a device is selected, either raises error 5. So does an axis or wheel the device does not have, such as
  `_AXIS(99)` of the mouse.
  <!-- src: verification\v47_b_devices; tests\reference\examples\_axis_1 -->
- `_AXIS("1")` is a compile error, "Number required for function".
  <!-- src: language/device-input "Errors of the device functions"; verification\v47_x16 -->

## Availability

<!-- begin generated: availability -->
Supported. All 17 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_axis_1 -->
```qb64
ON ERROR GOTO trap
PRINT "Before a device is selected:"
y# = _AXIS(1)
y# = _WHEEL(1)
x = _DEVICEINPUT(2)
PRINT "After selecting the mouse:"
y# = _AXIS(99)
y# = _WHEEL(99)
END
trap:
PRINT " error"; ERR
RESUME NEXT
```

Output:

```text
Before a device is selected:
 error 5 
 error 5 
After selecting the mouse:
 error 5 
 error 5 

Press any key to continue
```
<!-- end generated -->

## See also

`_DEVICEINPUT`, `_LASTAXIS`, `STICK`, `_MOUSEX`.
