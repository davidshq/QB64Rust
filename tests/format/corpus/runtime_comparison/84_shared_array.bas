$Console:Only
' Runtime comparison: SHARED in SUB
Dim g As Long
g = 10
Call useShared
Print "after:"; g
End

Sub useShared
    Shared g As Long
    Print "in sub g:"; g
    g = 20
End Sub
