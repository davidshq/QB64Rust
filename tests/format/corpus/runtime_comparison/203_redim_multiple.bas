$Console:Only
' Runtime comparison: REDIM multiple arrays in one statement
Dim a As Long, b As Long
ReDim a(1 To 2) As Long, b(1 To 2) As Long
a(1) = 1
a(2) = 2
b(1) = 10
b(2) = 20
Print "a:"; a(1); a(2)
Print "b:"; b(1); b(2)
End
