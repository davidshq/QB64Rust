$Console:Only
' Runtime comparison: Whole TYPE assignment (a = b)
Type P
    x As Long
    y As Long
End Type
Dim a As P, b As P
a.x = 1
a.y = 2
b = a
Print b.x; b.y
b.x = 99
Print a.x; b.x
End
