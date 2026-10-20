' TEST: typed
$CONSOLE:ONLY
' The device functions and the STRIG switch in the typed tree (m4-devices-continue-paths task 5.1, design D4;
' verification\v47_b_devices, v47_x30): each function with and without its optional argument, an argument of
' another numeric type converted to LONG, _DEVICE$ a string, _AXIS and _WHEEL DOUBLE, the others LONG; STICK and
' STRIG with one and two arguments; the STRIG switch with no number, one and two.
DIM x AS LONG, d AS DOUBLE, s AS SINGLE
s = 1.5
x = _DEVICES
PRINT _DEVICE$; _DEVICE$(1); _DEVICE$(s)
x = _DEVICEINPUT
x = _DEVICEINPUT(2)
x = _LASTBUTTON + _LASTBUTTON(1)
x = _LASTAXIS + _LASTAXIS(2)
x = _LASTWHEEL + _LASTWHEEL(2)
x = _BUTTON + _BUTTON(1)
x = _BUTTONCHANGE + _BUTTONCHANGE(1)
d = _AXIS + _AXIS(s)
d = _WHEEL + _WHEEL(1)
x = STICK(0) + STICK(1, 2)
x = STRIG(0) + STRIG(s, 2)
STRIG ON
STRIG(0) OFF
STRIG(4, 1) STOP
