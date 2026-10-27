$Console:Only
' Slice (m3-mem task 3.1 and 3.2): the block of every kind of place, from verification\v32_a_blocks without its
' array parameter whose first use is _MEM (QB64PE-DIVERGENCES.md D-016), and v32_x119 (members of an element). Each line
' prints the place, then OFFSET less the address _OFFSET gives for the variable or the array's first element,
' SIZE, TYPE, ELEMENTSIZE, IMAGE and SOUND; then _MEM(offset, size), _OFFSET against the block's OFFSET, and a block of a member array named without ().
Type t
    a As Integer
    s As String * 5
    v(0 TO 3) As Long
    d As Double
End Type
Dim Shared g As Long, ga(5) As Double
Dim a As Long, m As _MEM, m2 As _MEM, z(3) As _MEM
Dim f As String * 6, u As t, b(10) As Integer, ut(3) As t, fs(4) As String * 3, b2(3, 4) As Long
Dim s1 As Single, d1 As Double, i64 As _Integer64, by As _Byte, ub As _Unsigned _Byte, o As _Offset
Dim uo As _Unsigned _Offset, fl As _Float, ul As _Unsigned Long, i As Integer, ui As _Unsigned Integer
Dim ui64 As _Unsigned _Integer64
ReDim dy(5) As Long
m = _Mem(a): show "a", m, _Offset(a)
m = _Mem(f): show "f", m, _Offset(f)
m = _Mem(u): show "u", m, _Offset(u)
m = _Mem(u.a): show "u.a", m, _Offset(u)
m = _Mem(u.s): show "u.s", m, _Offset(u)
m = _Mem(u.d): show "u.d", m, _Offset(u)
m = _Mem(u.v(2)): show "u.v(2)", m, _Offset(u)
m = _Mem(u.v()): show "u.v()", m, _Offset(u)
m = _Mem(b()): show "b()", m, _Offset(b())
m = _Mem(b(4)): show "b(4)", m, _Offset(b())
m = _Mem(ut()): show "ut()", m, _Offset(ut())
m = _Mem(ut(1)): show "ut(1)", m, _Offset(ut())
m = _Mem(ut(1).d): show "ut(1).d", m, _Offset(ut())
m = _Mem(ut(1).v()): show "ut(1).v()", m, _Offset(ut())
m = _Mem(ut(1).v(2)): show "ut(1).v(2)", m, _Offset(ut())
m = _Mem(fs()): show "fs()", m, _Offset(fs())
m = _Mem(fs(2)): show "fs(2)", m, _Offset(fs())
m = _Mem(b2()): show "b2()", m, _Offset(b2())
m = _Mem(b2(1, 2)): show "b2(1, 2)", m, _Offset(b2())
m = _Mem(dy()): show "dy()", m, _Offset(dy())
m = _Mem(dy(2)): show "dy(2)", m, _Offset(dy())
m = _Mem(m2): show "m2", m, _Offset(m2)
m = _Mem(z(1)): show "z(1)", m, _Offset(z())
m = _Mem(s1): show "s1", m, _Offset(s1)
m = _Mem(d1): show "d1", m, _Offset(d1)
m = _Mem(i64): show "i64", m, _Offset(i64)
m = _Mem(by): show "by", m, _Offset(by)
m = _Mem(ub): show "ub", m, _Offset(ub)
m = _Mem(o): show "o", m, _Offset(o)
m = _Mem(uo): show "uo", m, _Offset(uo)
m = _Mem(fl): show "fl", m, _Offset(fl)
m = _Mem(ul): show "ul", m, _Offset(ul)
m = _Mem(i): show "i", m, _Offset(i)
m = _Mem(ui): show "ui", m, _Offset(ui)
m = _Mem(ui64): show "ui64", m, _Offset(ui64)
Print "_MEMELEMENT"
m = _MemElement(a): show "a", m, _Offset(a)
m = _MemElement(f): show "f", m, _Offset(f)
m = _MemElement(u): show "u", m, _Offset(u)
m = _MemElement(u.a): show "u.a", m, _Offset(u)
m = _MemElement(u.d): show "u.d", m, _Offset(u)
m = _MemElement(u.v(1)): show "u.v(1)", m, _Offset(u)
m = _MemElement(b()): show "b()", m, _Offset(b())
m = _MemElement(b(4)): show "b(4)", m, _Offset(b())
m = _MemElement(ut(1)): show "ut(1)", m, _Offset(ut())
m = _MemElement(ut(1).d): show "ut(1).d", m, _Offset(ut())
m = _MemElement(ut(1).a): show "ut(1).a", m, _Offset(ut())
m = _MemElement(ut(1).s): show "ut(1).s", m, _Offset(ut())
m = _MemElement(ut(1).v(2)): show "ut(1).v(2)", m, _Offset(ut())
m = _MemElement(fs(2)): show "fs(2)", m, _Offset(fs())
m = _MemElement(b2(1, 2)): show "b2(1, 2)", m, _Offset(b2())
Print "an address, new memory"
o = _Offset(a)
m = _Mem(o, 10): show "_MEM(o, 10)", m, o
m = _MemNew(16)
show "_MEMNEW(16)", m, m.OFFSET
_MemFree m
m = _MemNew(0)
show "_MEMNEW(0)", m, 0
Print "LEN"; Len(m); Len(u); Len(z(1))
Print "procedures"
Call x(ga())
Print "_OFFSET and OFFSET"
Dim k As Long
m = _Mem(k): Print m.OFFSET = _Offset(k)
m = _Mem(b(3)): Print m.OFFSET = _Offset(b(3)); _Offset(b(3)) - _Offset(b()); _Offset(b2(1, 0)) - _Offset(b2())
m = _Mem(ut(2).v(1)): Print m.OFFSET = _Offset(ut(2).v(1)); _Offset(ut(2).v(1)) - _Offset(ut())
m = _Mem(u.v): Print m.SIZE; m.TYPE; m.OFFSET = _Offset(u.v)
k = 9: o = _Offset(k): m = _Mem(o, 4): Print m.SIZE; m.TYPE; m.ELEMENTSIZE; m.IMAGE; m.OFFSET = o
m = _Mem(o, 2.5): Print m.SIZE
System

Sub show (l$, m As _MEM, bas As _Offset)
    Print l$; ":"; m.OFFSET - bas; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.IMAGE; m.SOUND
End Sub

Sub x (p() As Double)
    Dim l As Integer
    Static st As Long
    Dim m As _MEM
    m = _Mem(l): show "l", m, _Offset(l)
    m = _Mem(st): show "st", m, _Offset(st)
    m = _Mem(g): show "g", m, _Offset(g)
    m = _Mem(ga(1)): show "ga(1)", m, _Offset(ga())
    m = _Mem(p()): show "p()", m, _Offset(p())
    m = _Mem(p(2)): show "p(2)", m, _Offset(p())
    m = _MemElement(p(2)): show "p(2) element", m, _Offset(p())
End Sub

