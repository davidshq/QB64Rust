# _BUTTON

`_BUTTON(n)` tells whether button `n` of the device selected by `_DEVICEINPUT` is down. `_BUTTONCHANGE(n)` tells
whether it went down or up since the last read.

## Syntax

<!-- begin generated: syntax -->
```text
_BUTTON(button_number%)
_BUTTONCHANGE(button_number%)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG, as libqb gives it.

## Behavior

They read the device selected by the last `_DEVICEINPUT(d)`; its buttons count from 1 to `_LASTBUTTON`.
<!-- src: language/device-input "The device functions"; verification\v47_b_devices -->

A program that uses it is built with libqb's device-input part, as QB64pe builds it.
<!-- src: language/device-input "The device-input runtime part" -->

## Errors

- Before a device is selected, either raises error 5. So does a button the device does not have, such as `_BUTTON(0)`
  or `_BUTTONCHANGE(99)` of the mouse.
  <!-- src: verification\v47_b_devices; tests\reference\examples\_button_1 -->
- `_BUTTON(1, 2)` is a compile error, "Incorrect number of arguments", as is a string argument.
  <!-- src: language/device-input "Errors of the device functions"; verification\v47_x14 -->

## Availability

<!-- begin generated: availability -->
Supported. All 17 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_button_1 -->
```qb64
ON ERROR GOTO trap
PRINT "Before a device is selected:"
x = _BUTTON(1)
x = _BUTTONCHANGE(1)
x = _DEVICEINPUT(2)
PRINT "After selecting the mouse:"
x = _BUTTON(0)
x = _BUTTONCHANGE(99)
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

[`_DEVICEINPUT`](_deviceinput.md), [`_LASTBUTTON`](_lastbutton.md), [`_AXIS`](_axis.md).
