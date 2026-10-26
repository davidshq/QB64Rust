$Console:Only
' Runtime comparison: multiple assignments
Dim a As Long, b As Long, c As Long
a = 1
b = 2
c = 3
a = b
b = c
Print "a:"; a; "b:"; b; "c:"; c
End
