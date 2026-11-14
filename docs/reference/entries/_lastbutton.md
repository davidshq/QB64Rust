# _LASTBUTTON

`_LASTBUTTON(n)` gives how many buttons input device `n` has, `_LASTAXIS(n)` how many axes and `_LASTWHEEL(n)` how
many wheels.

## Syntax

<!-- begin generated: syntax -->
```text
_LASTAXIS(deviceNumber)
_LASTBUTTON(deviceNumber)
_LASTWHEEL(deviceNumber)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG.

## Behavior

The keyboard has 512 buttons, no axis and no wheel. The mouse has 8 buttons, 2 axes and 4 wheels.
<!-- src: language/device-input "The device functions"; verification\v47_b_devices; tests\reference\examples\_lastbutton_1 -->

With no argument they count the device selected by the last `_DEVICEINPUT(n)`: after `_DEVICEINPUT(2)`, `_LASTAXIS` is
2.
<!-- src: verification\v47_b_devices; tests\reference\examples\_lastbutton_1 -->

A program that uses it is built with libqb's device-input part, as QB64pe builds it.
<!-- src: language/device-input "The device-input runtime part" -->

## Errors

- A number that is no device raises error 5.
  <!-- src: language/device-input "The device functions"; verification\v47_b_devices -->
- More arguments than its form takes, or a string argument, is a compile error.
  <!-- src: language/device-input "Errors of the device functions" -->

## Availability

<!-- begin generated: availability -->
Supported. All 25 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_lastbutton_1 -->
```qb64
PRINT "Keyboard:"; _LASTBUTTON(1); _LASTAXIS(1); _LASTWHEEL(1)
PRINT "Mouse:   "; _LASTBUTTON(2); _LASTAXIS(2); _LASTWHEEL(2)
x = _DEVICEINPUT(2)
PRINT "Axes of the selected device:"; _LASTAXIS
END
```

Output:

```text
Keyboard: 512  0  0 
Mouse:    8  2  4 
Axes of the selected device: 2 

Press any key to continue
```
<!-- end generated -->

## See also

[`_DEVICES`](_devices.md), [`_BUTTON`](_button.md), [`_AXIS`](_axis.md), [`_WHEEL`](_axis.md).
