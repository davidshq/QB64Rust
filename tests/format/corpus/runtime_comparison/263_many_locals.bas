$Console:Only
' Runtime comparison: SUB with many local variables
Call manyVars
End

Sub manyVars
    Dim a As Long, b As Long, c As Long, d As Long, e As Long
    Dim f As Long, g As Long, h As Long, i As Long, j As Long
    a = 1: b = 2: c = 3: d = 4: e = 5
    f = 6: g = 7: h = 8: i = 9: j = 10
    Print a + b + c + d + e + f + g + h + i + j
End Sub
