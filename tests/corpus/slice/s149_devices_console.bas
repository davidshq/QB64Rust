$CONSOLE:ONLY
' Device input in a console program (m4-devices-continue-paths, design D4; verification\v47_b_devices, v47_x30):
' the keyboard and the mouse as devices 1 and 2 (libqb's fixed counts), the joystick functions with no game
' controller, the error 5 cases, ON STRIG with a label and a SUB, the STRIG switch. Nothing that counts game
' controllers (CLAUDE.md working rule 12).
ON ERROR GOTO trap
PRINT _DEVICES > 1
PRINT _DEVICE$(1)
PRINT _DEVICE$(2)
PRINT _LASTBUTTON(1); _LASTAXIS(1); _LASTWHEEL(1)
PRINT _LASTBUTTON(2); _LASTAXIS(2); _LASTWHEEL(2)
PRINT STICK(0); STICK(1); STICK(2, 1)
PRINT STRIG(0); STRIG(1); STRIG(4, 1)
x = STRIG(70000)
x = STICK(0, 0)
d$ = _DEVICE$(0)
x = _BUTTON(1)
y# = _AXIS
y# = _WHEEL(1)
x = _BUTTONCHANGE
x = _DEVICEINPUT(99)
x = _DEVICEINPUT(2)
PRINT _DEVICE$
PRINT _LASTAXIS
y# = _AXIS(99)
x = _LASTBUTTON
ON STRIG(0) GOSUB h
ON STRIG(4, 1) s 3
STRIG(0) ON
STRIG OFF
ON STRIG(1) GOSUB h
PRINT "end"
SYSTEM
trap:
PRINT "error"; ERR
RESUME NEXT
h:
RETURN

SUB s (k AS INTEGER)
    PRINT k
END SUB
