' TEST: cpp
$CONSOLE:ONLY
' ON STRIG in the C++ (m4-devices-continue-paths task 5.2, design D4; verification\v47_x30, x31): the setup
' onstrig_setup(button,controller,passed,id,pass) with the controller 0 and passed 0 when it is left out, a case
' in onstrig.txt per handler and, for a label, the jump in onstrigj.txt; a label handler, a SUB handler with and
' without an argument, a controller given and left out, a button that is no LONG; the program links libqb's
' device-input part.
DIM b AS SINGLE
b = 4.4
ON STRIG(0) GOSUB h
ON STRIG(b, 2) GOSUB h
ON STRIG(8) s 3
ON STRIG(12, 1) s0
STRIG(0) ON
SYSTEM
h:
RETURN
SUB s (n AS INTEGER)
    PRINT n
END SUB
SUB s0
END SUB
