$Console:Only
' Runtime comparison: array of UDT
Type T
    v As Long
End Type
Dim a(1 To 3) As T
a(1).v = 1
a(2).v = 2
a(3).v = 3
Print a(1).v; a(2).v; a(3).v
End
