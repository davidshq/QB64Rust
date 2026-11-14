# _DEVICES

`_DEVICES` gives the number of input devices: the keyboard is device 1, the mouse device 2, and game controllers come
after them.

## Syntax

<!-- begin generated: syntax -->
```text
_DEVICES
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG, at least 2.

## Behavior

The keyboard and the mouse are always devices, so `_DEVICES >= 2` is true.
<!-- src: language/device-input "The device functions"; verification\v47_b_devices; tests\reference\examples\_devices_1 -->

A program that uses it is built with libqb's device-input part, as QB64pe builds it.
<!-- src: language/device-input "The device-input runtime part" -->

## Errors

- `_DEVICES(1)` is a compile error, "Incorrect number of arguments".
  <!-- src: language/device-input "Errors of the device functions"; verification\v47_x20 -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_devices_1 -->
```qb64
PRINT "At least the keyboard and the mouse:"; _DEVICES >= 2
END
```

Output:

```text
At least the keyboard and the mouse:-1 

Press any key to continue
```
<!-- end generated -->

## See also

[`_DEVICE$`](_device-str.md), [`_DEVICEINPUT`](_deviceinput.md), [`_LASTBUTTON`](_lastbutton.md).
