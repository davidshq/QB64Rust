$Console:Only
' Runtime comparison: multi-dimensional array
Dim m(1 To 2, 1 To 3) As Long
Dim i As Long, j As Long
m(1, 1) = 1
m(1, 2) = 2
m(1, 3) = 3
m(2, 1) = 4
m(2, 2) = 5
m(2, 3) = 6
For i = 1 To 2
    For j = 1 To 3
        Print m(i, j);
    Next j
Next i
Print
Print "lbound1:"; LBound(m, 1); "ubound1:"; UBound(m, 1)
Print "lbound2:"; LBound(m, 2); "ubound2:"; UBound(m, 2)
End
