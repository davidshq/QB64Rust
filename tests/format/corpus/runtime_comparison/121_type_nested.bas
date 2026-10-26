$Console:Only
' Runtime comparison: nested TYPE
Type Inner
    v As Long
End Type
Type Outer
    i As Inner
    x As Long
End Type
Dim o As Outer
o.i.v = 5
o.x = 10
Print "i.v:"; o.i.v; "x:"; o.x
End
