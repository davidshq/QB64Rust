$Console:Only
' Runtime comparison: simple TYPE (LONG, DOUBLE, STRING) - no _BYTE/_UNSIGNED
Type num_t
    i As Long
    d As Double
    s As String
End Type
Dim n As num_t
n.i = -12345
n.d = 3.14
n.s = "hello"
Print n.i; n.d; n.s
Dim n2 As num_t
n2 = n
Print n2.i; n2.d; n2.s
End
