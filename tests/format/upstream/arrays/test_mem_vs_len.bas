Option _Explicit
$Console:Only

Type test
    a(1 TO 50) As Long
End Type

Dim Parent(1) As test
Dim e As _MEM, f As _MEM, g As _MEM

e = _Mem(Parent(0).a)
f = _Mem(Parent(1).a)
g = _Mem(Parent())

if g.size = 400 and f.size = 200 and e.size = 200 and g.elementsize = 200 and f.elementsize = 4 and e.elementsize = 4 and len(parent(0).a()) = 200 _
and len(parent(0).a()) = 200 and len (parent())= 400 and len(parent(0).a(1)) = 4 then print "LEN/MEM PASS"

'PRINT g.SIZE
'PRINT f.SIZE
'PRINT e.SIZE
'PRINT "-----------"
'PRINT g.ELEMENTSIZE
'PRINT f.ELEMENTSIZE
'PRINT e.ELEMENTSIZE
'PRINT "-----------"
'PRINT LEN(Parent(0).a())
'PRINT LEN(Parent(1).a())
'PRINT LEN(Parent())
'PRINT LEN(Parent(0).a(1))

_MemFree e
_MemFree f
_MemFree g
'SLEEP
System

