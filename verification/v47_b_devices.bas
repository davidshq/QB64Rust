$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): the device functions with no game controller.
' Nothing here counts controllers (CLAUDE.md working rule 12): devices 1 and 2 are the keyboard and the mouse.
ON ERROR GOTO handler
PRINT _DEVICES > 1
PRINT _DEVICE$(1)
PRINT _DEVICE$(2)
PRINT _LASTBUTTON(1); _LASTAXIS(1); _LASTWHEEL(1)
PRINT _LASTBUTTON(2); _LASTAXIS(2); _LASTWHEEL(2)
PRINT STICK(0); STICK(1); STICK(2, 1); STICK(3, 2)
PRINT STRIG(0); STRIG(1); STRIG(4); STRIG(1, 1); STRIG(5, 2)
PRINT "errors:"
e = 0: x = STRIG(70000): PRINT "strig 70000"; e
e = 0: x = STRIG(-1): PRINT "strig -1"; e
e = 0: x = STRIG(1, 0): PRINT "strig 1,0"; e
e = 0: x = STICK(0, 0): PRINT "stick 0,0"; e
e = 0: x = STICK(70000): PRINT "stick 70000"; e
e = 0: x = STICK(0, 70000): PRINT "stick 0,70000"; e
e = 0: d$ = _DEVICE$(0): PRINT "device$ 0"; e; LEN(d$)
e = 0: d$ = _DEVICE$(99): PRINT "device$ 99"; e; LEN(d$)
e = 0: d$ = _DEVICE$: PRINT "device$"; e; LEN(d$)
e = 0: x = _BUTTON(1): PRINT "button, none selected"; e
e = 0: x = _BUTTONCHANGE: PRINT "buttonchange, none selected"; e
e = 0: y# = _AXIS(1): PRINT "axis, none selected"; e
e = 0: y# = _WHEEL: PRINT "wheel, none selected"; e
e = 0: x = _DEVICEINPUT(99): PRINT "deviceinput 99"; e
e = 0: x = _DEVICEINPUT(2): PRINT "deviceinput 2"; e
e = 0: d$ = _DEVICE$: PRINT "device$ selected"; e; d$
e = 0: x = _LASTAXIS: PRINT "lastaxis selected"; e; x
e = 0: y# = _AXIS(99): PRINT "axis 99"; e
e = 0: y# = _WHEEL(99): PRINT "wheel 99"; e
e = 0: x = _BUTTON(0): PRINT "button 0"; e
e = 0: x = _BUTTONCHANGE(99): PRINT "buttonchange 99"; e
SYSTEM
handler:
e = ERR
RESUME NEXT
