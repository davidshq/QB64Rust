$Console:Only
' Slice (m3-events, task 3.3): event trapping in a console program, recorded with qb64pe.exe. Timers at 0.6 s or
' more, each count read at least 0.3 s away from an event, so that load does not change the output: off until
' TIMER ON, STOP holding one event until ON, OFF forgetting it; SUB handlers with their argument; a handler set
' up in a SUB; numbered timers, FREE and the reuse of a number; the run-time errors; KEY in a console.
Dim Shared n As Long, a As Long, b As Long
Dim t As Long, u As Long, v As Long, x As Long
On Error GoTo oops
On Timer(0.6) GoSub tick
_Delay 0.9
Print "1. before TIMER ON:"; n
Timer On
_Delay 1.5
Timer Off
Print "2. 1.5 s at 0.6 s:"; n
n = 0
Timer On
Timer Stop
_Delay 0.9
Print "3. stopped for 0.9 s:"; n
Timer On
Print "4. right after TIMER ON:"; n
Timer Off
n = 0
_Delay 0.9
Print "5. off for 0.9 s:"; n
x = 3
On Timer(0.6) show x * 2
x = 100
GoSub once
On Timer(0.6) small 300
GoSub once
On Timer(0.6) wide 2.5
GoSub once
On Timer(0.6) u16 -5
GoSub once
setup
GoSub once
t = _FreeTimer
u = _FreeTimer
Print "6. two timers:"; t; u
On Timer(t, 0.6) GoSub ta
On Timer(u, 1) GoSub tb
Timer(t) On
Timer(u) On
_Delay 1.5
Timer(t) Off
Timer(u) Off
Print "7. a, b after 1.5 s:"; a; b
Timer(t) Free
v = _FreeTimer
Print "8. _FREETIMER after FREE:"; v
Print "9. ON TIMER(9, 1):";: On Timer(9, 1) GoSub ta: Print " done"
Print "10. TIMER FREE:";: Timer Free: Print " done"
Print "11. TIMER(u + 0.6) ON:";: Timer(u + 0.6) On: Print " done"
Print "12. TIMER(u + 0.4) ON:";: Timer(u + 0.4) On: Print " done"
Timer(u) Off
Print "13. ON KEY(0.7):";: On Key(0.7) GoSub h: Print " done"
Print "14. ON KEY(31.6):";: On Key(31.6) GoSub h: Print " done"
Print "15. KEY(31.6) ON:";: Key(31.6) On: Print " done"
Print "16. KEY 40, x:";: Key 40, "x": Print " done"
Key(0) On
Key(0) Off
Key On
Key Off
Key 1, "abc"
Key 15, Chr$(0) + Chr$(30)
Print "17. KEY LIST:"
Key List
System

once:
Timer On
_Delay 0.9
Timer Off
Return

tick:
n = n + 1
Return

ta:
a = a + 1
Return

tb:
b = b + 1
Return

h:
Print "main label h"
Return

oops:
Print " error"; Err;
Resume Next

Sub show (k As Integer)
    Print "show"; k
    k = k + 1
End Sub

Sub small (k As _Byte)
    Print "small"; k
End Sub

Sub wide (k As _Integer64)
    Print "wide"; k
End Sub

Sub u16 (k As _Unsigned Integer)
    Print "u16"; k
End Sub

Sub setup
    On Timer(0.6) GoSub h
End Sub
