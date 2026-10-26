$Console:Only
' Runtime comparison: array element assignment in expression context
Dim a(1 To 3) As Long
a(1) = 1
a(2) = 2
a(3) = 3
Print "a(2):"; a(2)
a(2) = a(1) + a(3)
Print "after a(2)=a(1)+a(3):"; a(2)
End
