# _DEVICE$

`_DEVICE$(n)` describes input device `n`: its kind and what it has, such as `[MOUSE][BUTTON][AXIS][WHEEL]`.

## Syntax

<!-- begin generated: syntax -->
```text
_DEVICE$(device_number)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A STRING.

## Behavior

`_DEVICE$(1)` is `[KEYBOARD][BUTTON]` and `_DEVICE$(2)` is `[MOUSE][BUTTON][AXIS][WHEEL]`. A game controller's text
comes from the machine.
<!-- src: language/device-input "The device functions"; verification\v47_b_devices; tests\reference\examples\_device-str_1 -->

With no argument it describes the device selected by the last `_DEVICEINPUT(n)`.
<!-- src: verification\v47_b_devices; tests\reference\examples\_deviceinput_1 -->

A program that uses it is built with libqb's device-input part, as QB64pe builds it.
<!-- src: language/device-input "The device-input runtime part" -->

## Errors

- A number that is no device, such as 0 or 99, raises error 5, as does the bare form before a device is selected.
  <!-- src: verification\v47_b_devices; tests\reference\examples\_device-str_1 -->
- More arguments than its form takes, or a string argument, is a compile error.
  <!-- src: language/device-input "Errors of the device functions" -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_device-str_1 -->
```qb64
ON ERROR GOTO trap
PRINT _DEVICE$(1)
PRINT _DEVICE$(2)
d$ = _DEVICE$(0)
END
trap:
PRINT "_DEVICE$(0) raised error"; ERR
RESUME NEXT
```

Output:

```text
[KEYBOARD][BUTTON]
[MOUSE][BUTTON][AXIS][WHEEL]
_DEVICE$(0) raised error 5 

Press any key to continue
```
<!-- end generated -->

## See also

[`_DEVICES`](_devices.md), [`_DEVICEINPUT`](_deviceinput.md).
