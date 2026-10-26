$Console:Only
' Runtime comparison: Nested TYPE 3+ levels
Type A
    v As Long
End Type
Type B
    a As A
End Type
Type C
    b As B
End Type
Dim c As C
c.b.a.v = 42
Print c.b.a.v
End
