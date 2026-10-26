$Console:Only
' Runtime comparison: DIM with 0 lower bound
Dim a(0 To 2) As Long
a(0) = 0
a(1) = 1
a(2) = 2
Print "lb:"; LBound(a); "ub:"; UBound(a)
Print a(0); a(1); a(2)
End
