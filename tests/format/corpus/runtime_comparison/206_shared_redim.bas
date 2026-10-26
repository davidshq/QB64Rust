$Console:Only
' Runtime comparison: SHARED array then REDIM in SUB (blocked if REDIM not supported)
Dim a(1 To 2) As Long
a(1) = 10
a(2) = 20
Call showA
Print a(1); a(2)
End

Sub showA
    Shared a() As Long
    Print "in sub:"; a(1); a(2)
End Sub
