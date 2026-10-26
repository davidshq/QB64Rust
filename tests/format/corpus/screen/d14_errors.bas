' Runtime errors of the screen statements and functions under a handler: the number of each.
Dim n As Integer, d As Double, l As Long
On Error GoTo handler
n = 1: Screen 3
n = 2: PSet (1, 1)
n = 3: Line (1, 1)-(2, 2)
n = 4: Circle (5, 5), 2
n = 5: Paint (1, 1)
n = 6: Draw "R5"
n = 7: Locate 26, 1
n = 8: Locate 1, 81
n = 9: View Print 20 To 30
n = 10: View (1, 1)-(5, 5)
n = 11: Window (0, 0)-(1, 1)
n = 12: d = Point(1, 1)
n = 13: l = Screen(26, 1)
n = 14: PCopy 0, 9
n = 15: Color 32
n = 16: Width -1
n = 17: Palette -1, 0
n = 18: Cls 9
n = 19: Screen 0, , -1
n = 20: Locate 0
Print "done";
System
handler:
Print "error"; Err; "at"; n
Resume Next
