' TEST: check-fail
$CONSOLE:ONLY
' The device functions with arguments the old compiler rejects (m4-devices-continue-paths task 5.1, design D4;
' verification\v47_x14-x16, x19, x20, x22, x23): too many arguments, none where one is needed, a string. Each is an
' error that is not "not supported yet"
DIM x AS LONG, d AS DOUBLE
x = _BUTTON(1, 2)
x = STICK(1, 2, 3)
x = STICK
x = _DEVICES(1)
d = _AXIS("1")
x = STRIG("a")
x$ = _DEVICE$(1, 2)
STRIG(1, 2, 3) ON
