$Console:Only
' Runtime comparison: REDIM without PRESERVE
Dim a As Long
ReDim a(1 To 3) As Long
a(1) = 1
a(2) = 2
a(3) = 3
ReDim a(1 To 2) As Long
a(1) = 10
a(2) = 20
Print "after redim:"; a(1); a(2)
End
