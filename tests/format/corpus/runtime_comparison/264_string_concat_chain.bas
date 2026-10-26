$Console:Only
' Runtime comparison: A$ = B$ + C$ + ... many times
Dim a As String, b As String, c As String, d As String
b = "b"
c = "c"
d = "d"
a = b + c + d + b + c + d + b + c
Print a
Print Len(a)
End
