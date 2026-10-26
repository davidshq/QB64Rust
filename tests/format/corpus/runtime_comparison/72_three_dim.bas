$Console:Only
' Runtime comparison: 3D array
Dim a(1 To 2, 1 To 2, 1 To 2) As Long
a(1, 1, 1) = 1
a(1, 1, 2) = 2
a(1, 2, 1) = 3
a(1, 2, 2) = 4
a(2, 1, 1) = 5
a(2, 1, 2) = 6
a(2, 2, 1) = 7
a(2, 2, 2) = 8
Print a(1, 1, 1); a(2, 2, 2)
Print "lb1:"; LBound(a, 1); "ub3:"; UBound(a, 3)
End
