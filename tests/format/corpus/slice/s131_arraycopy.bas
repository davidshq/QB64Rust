$Console:Only
' Slice (m3-arraycopy): _ARRAYCOPY of arrays (verification\v41_a, v41_b, v41_c, v41_f, v41_g, v41_h): the six
' forms, overlap and an array copied to itself, every element type but _FLOAT, several dimensions with numbers and
' strings, the places of an array, the run-time errors (error 9, the destination unchanged), the order in which the
' bounds and starts are evaluated and their rounding. Left out, as the new compiler differs on purpose: _FLOAT
' arrays (QB64PE-DIVERGENCES.md D-025) and an index outside its bounds on the way to a member array (D-004), each a CLI
' test.
Option Base 1
Common Shared cs() As Long
Dim Shared sh(5) As Long
Dim i As Long, j As Long, k As Long
Dim s(3 To 8) As Long, d(-2 To 12) As Long
For i = 3 To 8: s(i) = i * 10: Next
Print "-- forms"
GoSub clr: _ArrayCopy s() To d(): GoSub show
GoSub clr: _ArrayCopy s(4 To 6) To d(-1): GoSub show
GoSub clr: _ArrayCopy s(4 To 6) To d(): GoSub show
GoSub clr: _ArrayCopy s() To d(0): GoSub show
GoSub clr: _ArrayCopy s(5 To 5) To d(12): GoSub show
GoSub fill: _ArrayCopy s(3 To 6) To s(5): GoSub shows
GoSub fill: _ArrayCopy s(5 To 8) To s(3): GoSub shows
GoSub fill: _ArrayCopy s() To s(): GoSub shows
GoSub fill: _ArrayCopy s(3 To 4) To s(7): GoSub shows

Print "-- kinds"
Dim b1(1 To 3) As _Byte, b2(0 To 3) As _Byte
For i = 1 To 3: b1(i) = -i: Next: _ArrayCopy b1(2 To 3) To b2(1)
For i = 0 To 3: Print b2(i);: Next: Print
Dim ub1(1 To 3) As _Unsigned _Byte, ub2(0 To 3) As _Unsigned _Byte
For i = 1 To 3: ub1(i) = 250 + i: Next: _ArrayCopy ub1(2 To 3) To ub2(1)
For i = 0 To 3: Print ub2(i);: Next: Print
Dim n1(1 To 3) As Integer, n2(0 To 3) As Integer
For i = 1 To 3: n1(i) = -1000 * i: Next: _ArrayCopy n1(2 To 3) To n2(1)
For i = 0 To 3: Print n2(i);: Next: Print
Dim un1(1 To 3) As _Unsigned Integer, un2(0 To 3) As _Unsigned Integer
For i = 1 To 3: un1(i) = 65530 + i: Next: _ArrayCopy un1(2 To 3) To un2(1)
For i = 0 To 3: Print un2(i);: Next: Print
Dim ul1(1 To 3) As _Unsigned Long, ul2(0 To 3) As _Unsigned Long
For i = 1 To 3: ul1(i) = 4294967290 + i: Next: _ArrayCopy ul1(2 To 3) To ul2(1)
For i = 0 To 3: Print ul2(i);: Next: Print
Dim q1(1 To 3) As _Integer64, q2(0 To 3) As _Integer64
For i = 1 To 3: q1(i) = -9000000000000 * i: Next: _ArrayCopy q1(2 To 3) To q2(1)
For i = 0 To 3: Print q2(i);: Next: Print
Dim uq1(1 To 3) As _Unsigned _Integer64, uq2(0 To 3) As _Unsigned _Integer64
For i = 1 To 3: uq1(i) = 18446744073709551610~&& + i: Next: _ArrayCopy uq1(2 To 3) To uq2(1)
For i = 0 To 3: Print uq2(i);: Next: Print
Dim f1(1 To 3) As Single, f2(0 To 3) As Single
For i = 1 To 3: f1(i) = i / 3: Next: _ArrayCopy f1(2 To 3) To f2(1)
For i = 0 To 3: Print f2(i);: Next: Print
Dim d1(1 To 3) As Double, d2(0 To 3) As Double
For i = 1 To 3: d1(i) = i / 3: Next: _ArrayCopy d1(2 To 3) To d2(1)
For i = 0 To 3: Print d2(i);: Next: Print
Dim o1(1 To 3) As _Offset, o2(0 To 3) As _Offset
For i = 1 To 3: o1(i) = i * 7: Next: _ArrayCopy o1(2 To 3) To o2(1)
For i = 0 To 3: Print o2(i);: Next: Print
Dim uo1(1 To 3) As _Unsigned _Offset, uo2(0 To 3) As _Unsigned _Offset
For i = 1 To 3: uo1(i) = i * 9: Next: _ArrayCopy uo1() To uo2()
For i = 0 To 3: Print uo2(i);: Next: Print
Dim fs1(1 To 3) As String * 3, fs2(0 To 3) As String * 3
For i = 1 To 3: fs1(i) = Chr$(64 + i) + "x": Next: _ArrayCopy fs1(2 To 3) To fs2(1)
For i = 0 To 3: Print "["; fs2(i); "]";: Next: Print
Dim vs1(1 To 3) As String, vs2(0 To 3) As String
For i = 1 To 3: vs1(i) = String$(i, 96 + i): Next: vs2(0) = "keep": _ArrayCopy vs1(2 To 3) To vs2(1)
For i = 0 To 3: Print "["; vs2(i); "]";: Next: Print
vs1(2) = "changed": Print "["; vs2(1); "]"
For i = 1 To 3: vs1(i) = String$(i, 65): Next: _ArrayCopy vs1(1 To 2) To vs1(2)
For i = 1 To 3: Print "["; vs1(i); "]";: Next: Print

Print "-- dimensions"
Dim m(1 To 2, 0 To 2) As Long, n(1 To 2, 0 To 2) As Long, big(0 To 4, 0 To 5) As Long
For i = 1 To 2: For j = 0 To 2: m(i, j) = i * 10 + j: Next: Next
_ArrayCopy m() To n()
For i = 1 To 2: For j = 0 To 2: Print n(i, j);: Next: Next: Print
GoSub bclr: _ArrayCopy m() To big(1, 2): GoSub bshow
For i = 0 To 4: For j = 0 To 5: big(i, j) = i * 10 + j: Next: Next
Dim small(0 To 2, 0 To 3) As Long, other(0 To 4, 0 To 5) As Long
For i = 0 To 2: For j = 0 To 3: small(i, j) = -1: Next: Next
_ArrayCopy big(1 To 2, 2 To 4) To small(1, 0)
For i = 0 To 2: For j = 0 To 3: Print small(i, j);: Next: Print: Next
_ArrayCopy big(1 To 2, 2 To 4) To other()
For i = 0 To 4: For j = 0 To 5: Print other(i, j);: Next: Print: Next
Dim c3(1 To 2, 1 To 2, 1 To 2) As Integer, e3(1 To 2, 1 To 2, 1 To 2) As Integer
For i = 1 To 2: For j = 1 To 2: For k = 1 To 2: c3(i, j, k) = i * 100 + j * 10 + k: Next: Next: Next
_ArrayCopy c3(2 To 2, 1 To 2, 2 To 2) To e3(1, 1, 1)
For i = 1 To 2: For j = 1 To 2: For k = 1 To 2: Print e3(i, j, k);: Next: Next: Next: Print
Dim sm(1 To 2, 1 To 3) As String, sb(0 To 3, 0 To 4) As String
For i = 1 To 2: For j = 1 To 3: sm(i, j) = Chr$(64 + i) + Chr$(48 + j): Next: Next
GoSub sclr: _ArrayCopy sm() To sb(1, 1): GoSub sshow
GoSub sclr: _ArrayCopy sm(1 To 2, 2 To 3) To sb(0, 0): GoSub sshow
GoSub sclr: _ArrayCopy sm(2 To 2, 1 To 3) To sb(): GoSub sshow
Dim fx(1 To 2, 1 To 2) As String * 2, fy(1 To 2, 1 To 2) As String * 2
fx(1, 1) = "aa": fx(2, 1) = "bb": fx(1, 2) = "cc": fx(2, 2) = "dd"
_ArrayCopy fx() To fy(): Print fy(1, 1); fy(2, 1); fy(1, 2); fy(2, 2)

Print "-- places"
ReDim cs(5) As Long
ReDim r(3) As Long
For i = 1 To 3: r(i) = i: Next
_ArrayCopy r() To sh(2)
For i = 1 To 5: Print sh(i);: Next: Print
ReDim _Preserve r(6) As Long
_ArrayCopy sh() To r(2)
For i = 1 To 6: Print r(i);: Next: Print
'$Dynamic
Dim dz(4) As Long
'$Static
dz(2) = 22: _ArrayCopy dz() To cs()
For i = 1 To 5: Print cs(i);: Next: Print
inside
For i = 1 To 5: Print sh(i);: Next: Print
Dim p(1 To 3) As Long, q(1 To 5) As Long, p2(1 To 2, 1 To 2) As Long, r2(1 To 2, 1 To 2) As Long
p(1) = 7: p(2) = 8: p(3) = 9
param p(), q()
For i = 1 To 5: Print q(i);: Next: Print
p2(1, 1) = 1: p2(2, 1) = 2: p2(1, 2) = 3: p2(2, 2) = 4
param2 p2(), r2()
Print r2(1, 1); r2(2, 1); r2(1, 2); r2(2, 2)
u(1) = 5: w(1) = 6
_ArrayCopy u() To w(): Print w(1); w(10)

Print "-- errors"
On Error GoTo h
Dim a(1 To 5) As Long, e(0 To 5) As Long, z5(2 To 4) As Long
For i = 1 To 5: a(i) = i: Next
GoSub eclr: _ArrayCopy a(3 To 2) To e(1): GoSub eshow
GoSub eclr: _ArrayCopy a(0 To 2) To e(1): GoSub eshow
GoSub eclr: _ArrayCopy a(4 To 6) To e(1): GoSub eshow
GoSub eclr: _ArrayCopy a(1 To 2) To e(-1): GoSub eshow
GoSub eclr: _ArrayCopy a(1 To 2) To e(6): GoSub eshow
GoSub eclr: _ArrayCopy a(1 To 3) To e(4): GoSub eshow
GoSub eclr: _ArrayCopy a() To e(2): GoSub eshow
GoSub eclr: _ArrayCopy a() To e(): GoSub eshow
_ArrayCopy a() To z5(): Print z5(2); z5(3); z5(4)
_ArrayCopy a(1 To 2) To z5(): Print z5(2); z5(3); z5(4)
ReDim zz(3) As Long
Erase zz
GoSub eclr: _ArrayCopy zz() To e(): GoSub eshow
_ArrayCopy a() To zz(): Print " erased destination"
'$Dynamic
Dim nv(1 To 5) As Long
'$Static
Erase nv
_ArrayCopy a(1 To 2) To nv(1): Print " erased $DYNAMIC"
Dim st(1 To 5) As Long
For i = 1 To 5: st(i) = 9: Next
Erase st
_ArrayCopy a() To st(): Print st(1); st(5)
Dim mm(1 To 2, 1 To 3) As Long, nn(1 To 3, 1 To 2) As Long, kk(0 To 2, 1 To 3) As Long
_ArrayCopy mm() To nn(): Print " other shape"
_ArrayCopy mm() To kk(): Print " other lower"
_ArrayCopy mm() To nn(2, 1): Print " does not fit"
_ArrayCopy mm(1 To 3, 1 To 1) To nn(1, 1): Print " range outside"
_ArrayCopy mm(1 To 2, 2 To 3) To nn(): Print " range to whole outside"
Dim t1(1 To 3) As String, t2(1 To 2) As String
t1(1) = "a": t1(2) = "b": t1(3) = "c": t2(1) = "keep"
_ArrayCopy t1() To t2(1): Print " "; t2(1)
_ArrayCopy t1(2 To 1) To t2(1): Print " "; t2(1)
_ArrayCopy a(1 To 9223372036854775807) To e(0): Print " huge"
GoSub eclr: _ArrayCopy a(1 To bad&) To e(0): GoSub eshow

Print "-- order"
Dim g(0 To 9) As Long, gd(0 To 9) As Long, gm(0 To 3, 0 To 3) As Long, gn(0 To 3, 0 To 3) As Long
For i = 0 To 9: g(i) = i: Next
_ArrayCopy g(f(2) To f(4)) To gd(f(6)): Print
For i = 0 To 9: Print gd(i);: Next: Print
_ArrayCopy gm(f(1) To f(2), f(3) To f(3)) To gn(f(0), f(1)): Print
GoSub gclr: _ArrayCopy g(1.5E+0,1.5 To 3.5E+0,3.5) To gd(0.5E+0,0.5): GoSub gshow
GoSub gclr: _ArrayCopy g(2.4E+0,2.4 To 2.6E+0,2.6) To gd(4.49E+0,4.49): GoSub gshow
Dim b8 As _Byte, u64 As _Unsigned _Integer64, sg As Single
b8 = 2: u64 = 3: sg = 4.5
GoSub gclr: _ArrayCopy g(b8 To u64) To gd(sg): GoSub gshow
GoSub gclr: _ArrayCopy g(-1 + 2 To 2 ^ 2) To gd(): GoSub gshow
GoSub gclr: _ArrayCopy g(1 To 2) To gd(bad&): GoSub gshow
System

clr:
For i = -2 To 12: d(i) = -1: Next
Return
show:
For i = -2 To 12: Print d(i);: Next: Print
Return
fill:
For i = 3 To 8: s(i) = i * 10: Next
Return
shows:
For i = 3 To 8: Print s(i);: Next: Print
Return
bclr:
For i = 0 To 4: For j = 0 To 5: big(i, j) = -1: Next: Next
Return
bshow:
For i = 0 To 4: For j = 0 To 5: Print big(i, j);: Next: Print: Next
Return
sclr:
For i = 0 To 3: For j = 0 To 4: sb(i, j) = "..": Next: Next
Return
sshow:
For i = 0 To 3: For j = 0 To 4: Print " "; sb(i, j);: Next: Print: Next
Return
eclr:
For i = 0 To 5: e(i) = -1: Next
Return
eshow:
For i = 0 To 5: Print e(i);: Next: Print
Return
gclr:
For i = 0 To 9: gd(i) = -1: Next
Return
gshow:
For i = 0 To 9: Print gd(i);: Next: Print
Return
h:
Print " error"; Err;
Resume Next

Sub inside
    Dim lc(3) As Long
    Static st(3) As Long
    lc(1) = 31: lc(2) = 32: lc(3) = 33
    _ArrayCopy lc() To st()
    _ArrayCopy st(1 To 2) To sh(4)
End Sub

Sub param (x() As Long, y() As Long)
    _ArrayCopy x() To y(LBOUND(y) + 1)
End Sub

Sub param2 (x() As Long, y() As Long)
    _ArrayCopy x(1 To 2, 1 To 2) To y(1, 1)
End Sub

Function f& (v As Long)
    Print " f"; LTrim$(Str$(v));
    f& = v
End Function

Function bad&
    Error 5
    bad& = 2
End Function
