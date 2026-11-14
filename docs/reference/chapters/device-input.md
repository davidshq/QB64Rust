# Game controllers and input devices

The device functions see the keyboard, the mouse and each game controller as numbered devices with buttons, axes and
wheels. `STICK` and `STRIG` are QuickBASIC's older view of a joystick.

The pages of this area: [_DEVICES](../entries/_devices.md), [_DEVICE$](../entries/_device-str.md),
[_DEVICEINPUT](../entries/_deviceinput.md), [_LASTBUTTON](../entries/_lastbutton.md),
[_BUTTON](../entries/_button.md), [_AXIS](../entries/_axis.md), [STICK](../entries/stick.md),
[STRIG](../entries/strig.md).

## Devices

Device 1 is the keyboard and device 2 the mouse, so `_DEVICES` is at least 2; game controllers follow. `_DEVICE$(n)`
describes one: `[KEYBOARD][BUTTON]` and `[MOUSE][BUTTON][AXIS][WHEEL]`.
<!-- src: language/device-input "The device functions"; verification\v47_b_devices -->

`_DEVICEINPUT(n)` selects a device, and `_BUTTON`, `_BUTTONCHANGE`, `_AXIS` and `_WHEEL` then read it. Before a device
is selected, or for a button, axis or wheel it does not have, they raise error 5.
<!-- src: verification\v47_b_devices; tests\reference\examples\_button_1 -->

`_LASTBUTTON`, `_LASTAXIS` and `_LASTWHEEL` count what a device has: the mouse has 8 buttons, 2 axes and 4 wheels.
<!-- src: verification\v47_b_devices; tests\reference\examples\_lastbutton_1 -->

## STICK and STRIG

`STICK(n)` gives an axis from 1 to 254, and 128 with no controller attached. `STRIG(n)` tells whether a button is down
or was pressed, and `ON STRIG` traps a press as an event.
<!-- src: language/device-input "STICK and STRIG"; language/events "Joystick-button handlers" -->

## The runtime part

A program that uses any of these words is built with libqb's device-input part, as QB64pe builds it.
<!-- src: language/device-input "The device-input runtime part" -->
