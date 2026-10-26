$Console:Only
' Device input in a console program (m4-devices-continue-paths, design D4; verification\v47_b_devices, v47_x30):
' the keyboard and the mouse as devices 1 and 2 (libqb's fixed counts), the joystick functions with no game
' controller, the error 5 cases, ON STRIG with a label and a SUB, the STRIG switch. Nothing that counts game
' controllers (CLAUDE.md working rule 12).
On Error GoTo trap
Print _Devices > 1
Print _Device$(1)
Print _Device$(2)
Print _LastButton(1); _LastAxis(1); _LastWheel(1)
Print _LastButton(2); _LastAxis(2); _LastWheel(2)
Print Stick(0); Stick(1); Stick(2, 1)
Print Strig(0); Strig(1); Strig(4, 1)
x = Strig(70000)
x = Stick(0, 0)
d$ = _Device$(0)
x = _Button(1)
y# = _Axis
y# = _Wheel(1)
x = _ButtonChange
x = _DeviceInput(99)
x = _DeviceInput(2)
Print _Device$
Print _LastAxis
y# = _Axis(99)
x = _LastButton
On Strig(0) GoSub h
On Strig(4, 1) s 3
Strig (0)On
Strig Off
On Strig(1) GoSub h
Print "end"
System
trap:
Print "error"; Err
Resume Next
h:
Return

Sub s (k As Integer)
    Print k
End Sub
