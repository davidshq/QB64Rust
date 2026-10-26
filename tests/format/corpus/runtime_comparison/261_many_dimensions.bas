$Console:Only
' Runtime comparison: Array with 4 dimensions (we have 72_three_dim for 3D)
Dim a(1 To 2, 1 To 2, 1 To 2, 1 To 2) As Long
a(1, 1, 1, 1) = 1
a(2, 2, 2, 2) = 16
Print a(1, 1, 1, 1); a(2, 2, 2, 2)
Print LBound(a, 1); UBound(a, 1)
Print LBound(a, 4); UBound(a, 4)
End
