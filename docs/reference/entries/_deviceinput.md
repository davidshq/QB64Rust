# _DEVICEINPUT

`_DEVICEINPUT(n)` selects input device `n`, whose buttons, axes and wheels `_BUTTON`, `_AXIS` and `_WHEEL` then read.

## Syntax

<!-- begin generated: syntax -->
```text
_DEVICEINPUT[(device_number&)]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG, as libqb gives it.

## Behavior

After `_DEVICEINPUT(2)`, the forms with no device number read the mouse: `_DEVICE$` is `[MOUSE][BUTTON][AXIS][WHEEL]`,
`_LASTAXIS` is 2.
<!-- src: language/device-input "The device functions"; verification\v47_b_devices; tests\reference\examples\_deviceinput_1 -->

A program that uses it is built with libqb's device-input part, as QB64pe builds it.
<!-- src: language/device-input "The device-input runtime part" -->

## Errors

- A number that is no device, such as 99, raises error 5.
  <!-- src: verification\v47_b_devices; tests\reference\examples\_deviceinput_1 -->
- More arguments than its form takes, or a string argument, is a compile error.
  <!-- src: language/device-input "Errors of the device functions" -->

## Availability

<!-- begin generated: availability -->
Supported. All 9 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_deviceinput_1 -->
```qb64
ON ERROR GOTO trap
x = _DEVICEINPUT(99)
x = _DEVICEINPUT(2)
PRINT "Selected: "; _DEVICE$
END
trap:
PRINT "_DEVICEINPUT(99) raised error"; ERR
RESUME NEXT
```

Output:

```text
_DEVICEINPUT(99) raised error 5 
Selected: [MOUSE][BUTTON][AXIS][WHEEL]

Press any key to continue
```
<!-- end generated -->

## See also

`_DEVICES`, `_BUTTON`, `_AXIS`, `_LASTBUTTON`.
