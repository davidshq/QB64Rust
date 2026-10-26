$Console:Only
' Runtime comparison: LBOUND/UBOUND for each dimension (2D)
Dim m(1 To 2, 3 To 5) As Long
Print "dim1:"; LBound(m, 1); UBound(m, 1)
Print "dim2:"; LBound(m, 2); UBound(m, 2)
End
