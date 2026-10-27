$Console:Only
$Unstable:TypeFields
' Slice (m3-whole-array-assign task 4.3): whole-array assignment with member arrays and owner types
' (verification\v36_d, v36_e, v36_x33, x46, x47, x49-x51): member to member, member to array and back, members
' of array elements, STRING and user-type member arrays, a member's bounds that differ (error 9), the target's
' place found before the source's, an error handled before the statement; arrays of a type with _DYNAMIC members
' (a DIM and a REDIM target, the members' own bounds, strings, a nested owner type), a _DYNAMIC member array to
' another and to a plain array, inline and _DYNAMIC member arrays of an owner type, _MEM of a member of an
' element after the copy. Where the new compiler differs on purpose (an index outside on the way, QB64PE-DIVERGENCES.md
' D-004; an owner type with other bounds, D-022) nothing is here: CLI tests hold those.
Type leaf
    k As Long
    s As String
End Type
Type box
    v(1 TO 3) As Long
    w(1) As String
    l(1) As leaf
End Type
Type t2
    v(1) As Long
End Type
Type own
    a As Integer
    v(1) _Dynamic As Long
    s As String
End Type
Type nest
    o As own
    w(0) _Dynamic As String
End Type
Type own2
    v(1) _Dynamic As Long
    k As Long
End Type
Type ibox
    l(1) As own2
End Type
Type dbox
    d(1) _Dynamic As own2
End Type
On Error GoTo h
Dim x As box, y As box, i As Long
Dim a(1 To 3) As Long
Dim b(0 To 2) As Long
Dim e(2) As box
For i = 1 To 3: x.v(i) = i * 5: Next
x.w(0) = "w0": x.w(1) = "w1": x.l(1).k = 9: x.l(1).s = "leaf"
Print "1. member to member:";
y.v() = x.v(): y.w() = x.w(): y.l() = x.l()
x.w(1) = "changed": x.l(1).s = "changed"
Print y.v(1); y.v(3); " "; y.w(0); " "; y.w(1); y.l(1).k; " "; y.l(1).s
Print "2. member to array:";
a() = x.v()
For i = 1 To 3: Print a(i);: Next: Print
Print "3. array to member:";
a(2) = -1
x.v() = a()
For i = 1 To 3: Print x.v(i);: Next: Print
Print "4. members of array elements:";
e(1).v(2) = 77: e(1).w(1) = "e1"
e(2).v() = e(1).v(): e(2).w() = e(1).w()
Print e(2).v(2); " "; e(2).w(1)
Print "5. other bounds (error):";
b() = x.v()
Print " after:"; b(0)
Print "6. a member of an element from an array:";
Dim e2(2) As t2, a2(1) As Long
a2(1) = 4
e2(1).v() = a2()
Print e2(1).v(1)
Print "7. the target's place first:";
e2(1).v(1) = 9
e2(f(2)).v() = e2(f(1)).v()
Print e2(2).v(1)
Print "8. an error handled before:";
a2(1) = 3
Error 7: e2(1).v() = a2()
Print e2(1).v(1)
Print "9. DIM target of an owner type:";
Dim src(1 To 2) As own, dst(1 To 2) As own
ReDim rsrc(1 To 2) As own, rdst(1 To 2) As own
src(1).a = 5: ReDim src(1).v(3 TO 5): src(1).v(4) = 44: src(1).s = "one"
src(2).v(1) = 21
dst(1).v(0) = 999: dst(2).s = "old"
dst() = src()
src(1).v(4) = -1: src(1).s = "gone"
Print dst(1).a; LBound(dst(1).v); UBound(dst(1).v); dst(1).v(4); " "; dst(1).s; dst(2).v(1); "["; dst(2).s; "]"
Print "10. REDIM target:";
rsrc(2).s = "r2": ReDim rsrc(2).v(0 TO 2): rsrc(2).v(2) = 22
rdst() = rsrc()
Print UBound(rdst(2).v); rdst(2).v(2); " "; rdst(2).s
Print "11. DIM target from REDIM source:";
dst() = rsrc()
Print UBound(dst(2).v); dst(2).v(2); " "; dst(2).s
Print "12. nested owner type:";
Dim ns(1) As nest, nd(1) As nest
ReDim ns(1).o.v(0 TO 2): ns(1).o.v(2) = 12: ns(1).w(0) = "w": ns(1).o.s = "os"
nd() = ns()
ns(1).w(0) = "x"
Print nd(1).o.v(2); " "; nd(1).w(0); " "; nd(1).o.s
Print "13. a _DYNAMIC member to another, other bounds:";
Dim p As own, q As own
ReDim p.v(1 TO 2): p.v(2) = 8
q.v() = p.v()
Print LBound(q.v); UBound(q.v)
Print "14. the same bounds:";
ReDim q.v(1 TO 2)
q.v() = p.v()
Print q.v(2)
Print "15. a plain array to a _DYNAMIC member and back:";
Dim pa(1 To 2) As Long
pa(1) = 31
q.v() = pa(): pa(2) = -9: pa() = p.v()
Print q.v(1); pa(2)
Print "16. an inline member array of an owner type:";
Dim ib As ibox, oa(1) As own2
oa(1).k = 5
ib.l() = oa()
Print ib.l(1).k;
oa(1).k = 0
oa() = ib.l()
Print oa(1).k
Print "17. a _DYNAMIC member array of an owner type:";
Dim db As dbox
oa(1).k = 6
db.d() = oa()
Print db.d(1).k;
oa(1).k = 0
oa() = db.d()
Print oa(1).k
Print "18. _MEM of a member of an element:";
ReDim ms(1) As own2, md(1) As own2
Dim md2(1) As own2
ms(1).k = 4
Dim m As _MEM, m2 As _MEM
m = _Mem(md(1).k): m2 = _Mem(md2(1).k)
md() = ms(): md2() = ms()
Print _MemExists(m); _MemExists(m2); md(1).k; md2(1).k
System
h:
Print " error"; Err;
Resume Next

Function f (n)
    Print " f"; n;
    f = n
End Function
