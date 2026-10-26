$Console:Only
' Runtime comparison: ON n GOTO line1, line2, ...
Dim n As Long
n = 1
On n GOTO L1, L2
Print "fall1"
End
L1:
Print "L1"
n = 2
On n GOTO L1, L2
Print "fall2"
End
L2:
Print "L2"
End
