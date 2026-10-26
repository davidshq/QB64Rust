$Console:Only
' Runtime comparison 236: OPTION BASE 1 sets array lower bound
Option Base 1
Dim a(5) As Long
a(1) = 1
a(5) = 5
Print "lbound"; LBound(a); "ubound"; UBound(a); "a(1)"; a(1); "a(5)"; a(5)
End
