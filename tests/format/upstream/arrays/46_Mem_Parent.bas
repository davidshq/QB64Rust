Option _Explicit
$Console:Only

Type test
    a(1 to 50) As Long
End Type
Dim Parent(1) As test

Dim e As _MEM, f As _MEM, g As _MEM

e = _Mem(Parent(0).a)
f = _Mem(Parent(1).a)
g = _Mem(Parent())

If g.SIZE = 400 And f.SIZE = 200 And e.SIZE = 200 Then Print "PASS" Else Print "Fail"; e.SIZE; f.SIZE; g.SIZE

_MemFree e
_MemFree f
_MemFree g

System
