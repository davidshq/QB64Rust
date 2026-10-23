' TEST: ir
$CONSOLE:ONLY
' The STRIG switch and ON STRIG in the IR (m4-devices-continue-paths tasks 5.1, 5.2, design D4; verification\v47_x30):
' the switch with no number, a button and a button and a controller, each LONG with its "passed" flags; the setup of
' ON STRIG with a controller left out and given; the program's runtime part deviceinput.
DIM d AS DOUBLE
d = 4
STRIG ON
STRIG(0) OFF
STRIG(d, 2) STOP
ON STRIG(0) GOSUB h
ON STRIG(d, 1) GOSUB h
SYSTEM
h:
RETURN
