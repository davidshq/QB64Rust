$Console:Only
$Unstable:TypeFields
' Slice (m3-arraycopy): _ARRAYCOPY of member arrays (verification\v41_d, v41_e, v41_k): of a plain TYPE, of a
' variable, of an element, nested, of numbers, STRING * n and STRING, the members beside them unchanged, the order
' of the indexes on the way; _DYNAMIC member arrays of one and two dimensions and STRING, after a REDIM of the
' member, an erased one. Left out, as the new compiler differs on purpose: an index outside its bounds on the way
' (DIVERGENCES.md D-004), a CLI test.
Type t
    k As Integer
    b(1 TO 6) As Long
    s(0 TO 2) As String * 3
    v(0 TO 3) As String
End Type
Type o
    tag As Long
    inner As t
End Type
Type dy
    _Dynamic n(1 TO 4) As Long
    _Dynamic g(1 TO 2, 1 TO 3) As Integer
    _Dynamic s(0 TO 2) As String
End Type
Dim i As Long, j As Long
Dim v As t, tt(1 To 2) As t, oo As o, oa(0 To 1) As o
Dim src(1 To 4) As Long, dst(0 To 7) As Long
For i = 1 To 4: src(i) = i * 11: Next

Print "-- plain members"
_ArrayCopy src() To v.b(2)
For i = 1 To 6: Print v.b(i);: Next: Print
_ArrayCopy src(2 To 3) To tt(2).b(5)
For i = 1 To 6: Print tt(2).b(i);: Next: Print
For i = 1 To 6: Print tt(1).b(i);: Next: Print
_ArrayCopy v.b() To dst()
For i = 0 To 7: Print dst(i);: Next: Print
_ArrayCopy v.b(2 To 4) To oo.inner.b()
For i = 1 To 6: Print oo.inner.b(i);: Next: Print
_ArrayCopy oo.inner.b() To oa(1).inner.b(1)
For i = 1 To 6: Print oa(1).inner.b(i);: Next: Print
Print oa(1).tag; oa(1).inner.k; oa(0).inner.b(1)
v.s(0) = "abc": v.s(1) = "def": v.s(2) = "ghi"
_ArrayCopy v.s(1 To 2) To tt(1).s(0)
Print tt(1).s(0); tt(1).s(1); "["; tt(1).s(2); "]"
Dim fs(0 To 2) As String * 3
_ArrayCopy v.s() To fs()
Print fs(0); fs(1); fs(2)
For i = 1 To 6: v.b(i) = i: Next
_ArrayCopy v.b(1 To 4) To v.b(3)
For i = 1 To 6: Print v.b(i);: Next: Print
_ArrayCopy src() To tt(ix(2)).b(ix(1))
For i = 1 To 6: Print tt(2).b(i);: Next: Print
_ArrayCopy tt(ix(1)).b(ix(2) To ix(3)) To dst(ix(0))
For i = 0 To 7: Print dst(i);: Next: Print

Print "-- STRING members"
Dim w(1 To 2) As t, a(1 To 3) As String
a(1) = "one": a(2) = "two": a(3) = "three"
_ArrayCopy a() To v.v(1)
For i = 0 To 3: Print " ["; v.v(i); "]";: Next: Print
a(2) = "changed"
Print " ["; v.v(2); "]"
_ArrayCopy v.v(1 To 2) To w(2).v()
For i = 0 To 3: Print " ["; w(2).v(i); "]";: Next: Print
_ArrayCopy w(2).v() To v.v()
For i = 0 To 3: Print " ["; v.v(i); "]";: Next: Print
_ArrayCopy v.v(0 To 2) To v.v(1)
For i = 0 To 3: Print " ["; v.v(i); "]";: Next: Print
Print v.k; w(1).k

Print "-- _DYNAMIC members"
Dim x As dy, y(1 To 2) As dy
Dim src2(1 To 4) As Long, back(1 To 10) As Long
For i = 1 To 4: src2(i) = i * 100: Next
_ArrayCopy src2() To x.n()
For i = 1 To 4: Print x.n(i);: Next: Print
ReDim x.n(1 TO 10) AS LONG
_ArrayCopy src2() To x.n(7)
For i = 1 To 10: Print x.n(i);: Next: Print
_ArrayCopy x.n() To back()
For i = 1 To 10: Print back(i);: Next: Print
_ArrayCopy x.n(7 To 9) To y(2).n(1)
For i = 1 To 4: Print y(2).n(i);: Next: Print
Dim gs(1 To 2, 1 To 3) As Integer
For i = 1 To 2: For j = 1 To 3: gs(i, j) = i * 10 + j: Next: Next
_ArrayCopy gs() To y(1).g()
For i = 1 To 2: For j = 1 To 3: Print y(1).g(i, j);: Next: Next: Print
_ArrayCopy y(1).g(1 To 2, 2 To 3) To x.g(1, 1)
For i = 1 To 2: For j = 1 To 3: Print x.g(i, j);: Next: Next: Print
Dim ss(0 To 2) As String
ss(0) = "x": ss(1) = "yy": ss(2) = "zzz"
_ArrayCopy ss() To x.s()
For i = 0 To 2: Print " ["; x.s(i); "]";: Next: Print
_ArrayCopy x.s(1 To 2) To y(2).s(0)
For i = 0 To 2: Print " ["; y(2).s(i); "]";: Next: Print
ss(1) = "changed"
_ArrayCopy x.s() To ss()
For i = 0 To 2: Print " ["; ss(i); "]";: Next: Print
On Error GoTo h
Erase x.n
Print "erased member as source:";
_ArrayCopy x.n() To back()
Print " done"
System
h:
Print " error"; Err;
Resume Next

Function ix& (n As Long)
    Print "(ix"; n; ")";
    ix& = n
End Function
