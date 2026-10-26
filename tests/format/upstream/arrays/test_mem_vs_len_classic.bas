Option _Explicit
$Console:Only

Dim z(1 To 50) As Double
Dim m As _MEM
m = _Mem(z())
If m.SIZE = 400 And m.ELEMENTSIZE = 8 And Len(z()) = 400 And Len(z(1)) = 8 Then Print "Test mem classic pass"
System
