$Console:Only
' Slice (m3-whole-array-assign task 3.2): whole-array assignment of array variables (verification\v36_a, v36_b,
' v36_c, v36_f, v36_x14, x17, x20, x36): every numeric type, two and three dimensions, a REDIM array on either
' side, an array assigned to itself, LET, the source in parentheses, a one-line IF; STRING, STRING * n, _BIT, user
' types of numbers, with strings, nested and with member arrays, _MEM; arrays of every place (SHARED, COMMON
' SHARED, local, STATIC, parameters, $DYNAMIC, used without DIM, a FUNCTION); the run-time errors (bounds that
' differ, arrays not allocated), after which nothing is copied.
Common Shared cm() As Integer
Dim Shared cm(1 To 3) As Integer
Dim Shared g(1 To 3) As Integer
Dim Shared gs(1 To 2) As String
Type pair
    a As Long
    b As Single
End Type
Type txt
    n As Integer
    s As String
    f As String * 3
End Type
Type outer
    p As pair
    t As txt
End Type
Type holder
    v(2) As Integer
    w(1) As String
End Type
On Error GoTo h
Dim i As Long, j As Long, k As Long
Dim sb(1 To 3) As _Byte, db(1 To 3) As _Byte
Dim sbu(1 To 3) As _Unsigned _Byte, dub(1 To 3) As _Unsigned _Byte
Dim si(1 To 3) As Integer, di(1 To 3) As Integer
Dim sui(1 To 3) As _Unsigned Integer, dui(1 To 3) As _Unsigned Integer
Dim sl(1 To 3) As Long, dl(1 To 3) As Long
Dim sul(1 To 3) As _Unsigned Long, dul(1 To 3) As _Unsigned Long
Dim s64(1 To 3) As _Integer64, d64(1 To 3) As _Integer64
Dim su64(1 To 3) As _Unsigned _Integer64, du64(1 To 3) As _Unsigned _Integer64
Dim ss(1 To 3) As Single, ds(1 To 3) As Single
Dim sd(1 To 3) As Double, dd(1 To 3) As Double
Dim sf(1 To 3) As _Float, df(1 To 3) As _Float
Dim so(1 To 3) As _Offset, dof(1 To 3) As _Offset
Dim sx(1 To 3) As _Unsigned _Offset, dx(1 To 3) As _Unsigned _Offset
For i = 1 To 3
    sb(i) = -i: sbu(i) = 250 + i: si(i) = -1000 * i: sui(i) = 65530 + i: sl(i) = -100000 * i
    sul(i) = 4294967290 + i: s64(i) = -10000000000 * i: su64(i) = 1844674407370955161 + i
    ss(i) = i + 0.25: sd(i) = i + 0.125: sf(i) = i + 0.0625: so(i) = 1000 + i: dx(i) = 7: sx(i) = 2000 + i
    dl(i) = 99
Next
db() = sb(): dub() = sbu(): di() = si(): dui() = sui(): dl() = sl(): dul() = sul(): d64() = s64()
du64() = su64(): ds() = ss(): dd() = sd(): df() = sf(): dof() = so(): dx() = sx()
Print "1. every numeric type:"
For i = 1 To 3
    Print db(i); dub(i); di(i); dui(i); dl(i); dul(i); d64(i); du64(i); ds(i); dd(i); df(i); dof(i); dx(i)
Next
Print "2. independent of its source:";
sl(2) = 5: Print dl(2); sl(2)
Print "3. two dimensions:"
Dim m(-1 To 1, 2 To 4) As Integer, n(-1 To 1, 2 To 4) As Integer
For i = -1 To 1: For j = 2 To 4: m(i, j) = 10 * i + j: Next: Next
n() = m()
For i = -1 To 1: For j = 2 To 4: Print n(i, j);: Next: Print: Next
Print "4. three dimensions:"
Dim c(1, 1, 1) As Double, e(1, 1, 1) As Double
For i = 0 To 1: For j = 0 To 1: For k = 0 To 1: c(i, j, k) = 100 * i + 10 * j + k + 0.5: Next: Next: Next
e() = c()
For i = 0 To 1: For j = 0 To 1: For k = 0 To 1: Print e(i, j, k);: Next: Next: Next: Print
Print "5. REDIM arrays, either side:";
ReDim r(2 To 4) As Long, q(2 To 4) As Long
Dim f(2 To 4) As Long
For i = 2 To 4: r(i) = i * 7: Next
q() = r(): f() = q()
For i = 2 To 4: Print q(i); f(i);: Next: Print
Print "6. itself:";
r() = r()
For i = 2 To 4: Print r(i);: Next: Print
Print "7. LET, parentheses, blanks, a one-line IF:";
Dim x(2), y(2)
x(2) = 2.5
Let y() = (x())
Print y(2);
x(1) = 1.5
y() = x()
Print y(1);
x(0) = 3
If i > 0 Then y() = x() Else Print "no"
Print y(0)
Print "8. STRING:";
Dim s(2) As String, d(2) As String
s(0) = "zero": s(1) = "one": s(2) = "two": d(1) = "old"
d() = s()
s(1) = "changed"
Print " "; d(0); " "; d(1); " "; d(2); " "; s(1)
Print "9. STRING * 4:";
Dim sfx(2) As String * 4, dfx(2) As String * 4
sfx(0) = "A": sfx(1) = "BBBB": sfx(2) = "CCCCCC"
dfx() = sfx()
sfx(0) = "Z"
Print " ["; dfx(0); "]["; dfx(1); "]["; dfx(2); "]"
Print "10. _BIT:";
Dim sbit(0 To 19) As _Bit, dbit(0 To 19) As _Bit
For i = 0 To 19: If i Mod 3 = 0 Then sbit(i) = -1
Next
dbit() = sbit()
For i = 0 To 19: Print dbit(i);: Next: Print
Print "11. _UNSIGNED _BIT * 3:";
Dim sb3(0 To 9) As _Unsigned _Bit * 3, db3(0 To 9) As _Unsigned _Bit * 3
For i = 0 To 9: sb3(i) = i Mod 8: Next
db3() = sb3()
For i = 0 To 9: Print db3(i);: Next: Print
Print "12. TYPE of numbers:";
Dim sp(1 To 2) As pair, dp(1 To 2) As pair
sp(1).a = 1024: sp(2).b = 3.5
dp() = sp()
Print dp(1).a; dp(2).b
Print "13. TYPE with STRING and STRING * 3:";
Dim st(1) As txt, dt(1) As txt
st(0).n = 10: st(0).s = "first": st(0).f = "abc": st(1).s = "second"
dt() = st()
st(0).s = "changed"
Print dt(0).n; dt(0).s; " "; dt(0).f; " "; dt(1).s; " "; st(0).s
Print "14. nested TYPE:";
Dim sn(1) As outer, dn(1) As outer
sn(1).p.a = 7: sn(1).t.s = "inner": sn(1).t.f = "xyz"
dn() = sn()
sn(1).t.s = "gone"
Print dn(1).p.a; dn(1).t.s; " "; dn(1).t.f
Print "15. TYPE with member arrays:";
Dim sh(1) As holder, dh(1) As holder
sh(0).v(2) = 42: sh(1).w(1) = "deep"
dh() = sh()
sh(1).w(1) = "shallow"
Print dh(0).v(2); dh(1).w(1)
Print "16. _MEM:";
Dim sm(1) As _MEM, dm(1) As _MEM
Dim z As Long
sm(0) = _Mem(z)
dm() = sm()
Print dm(0).SIZE; dm(0).OFFSET = _Offset(z)
Dim a(1 To 3) As Integer, b(1 To 3) As Integer
For i = 1 To 3: g(i) = i: a(i) = 10 * i: Next
gs(1) = "g1": gs(2) = "g2"
Print "17. SHARED into COMMON (conventional memory), from a SUB:";
FromShared
For i = 1 To 3: Print cm(i);: Next: Print
Print "18. local and STATIC arrays of a SUB:";
Locals: Locals: Print
Print "19. array parameters, both sides:";
Params a(), b()
For i = 1 To 3: Print b(i);: Next: Print
Print "20. a parameter and a SHARED array:";
ToParam b()
For i = 1 To 3: Print b(i);: Next: Print
Print "21. a parameter of strings:";
Dim t(1 To 2) As String
StrParam t()
Print " "; t(1); " "; t(2)
Print "22. a parameter whose dimensions only the copy gives:";
OnlyCopy a()
Print "23. a SUB declared STATIC:";
StaticSub: StaticSub: Print
'$Dynamic
Dim dy(1 To 3) As Integer
'$Static
Print "24. $DYNAMIC:";
dy() = a()
For i = 1 To 3: Print dy(i);: Next: Print
Print "25. used without DIM:";
u(4) = 44
Dim w(10) As Single
w() = u()
Print w(4); UBound(w)
Print "26. in a FUNCTION:"; Fn(2)
Dim ea(0 To 5) As Long, eb(1 To 6) As Long, ec(0 To 4) As Long, ed(0 To 6) As Long
Dim em(2, 3) As Long, en(2, 4) As Long
For i = 0 To 5: ea(i) = i + 1: Next
eb(1) = 99: ec(0) = 98: ed(0) = 97
Print "27. same count, moved:";: eb() = ea(): Print eb(1)
Print "28. fewer:";: ec() = ea(): Print ec(0)
Print "29. more:";: ed() = ea(): Print ed(0)
Print "30. second dimension:";: en() = em(): Print " done"
ReDim er(0 To 5) As Long
Erase er
Print "31. target erased:";: er() = ea(): Print " done"
Print "32. source erased:";: ea() = er(): Print ea(0)
'$Dynamic
Dim nv(0 To 5) As Long
'$Static
Erase nv
Print "33. target never allocated again:";: nv() = ea(): Print " done"
Dim es(0 To 5) As Long
Erase es
Print "34. static after ERASE:";: es() = ea(): Print es(5)
Dim s1(2) As String, s2(3) As String
s2(0) = "keep"
Print "35. strings:";: s2() = s1(): Print " "; s2(0)
System
h:
Print " error"; Err;
Resume Next

Sub FromShared
    cm() = g()
End Sub

Sub Locals
    Static nn As Integer
    Dim l(2) As Long
    Static sl2(2) As Long
    nn = nn + 1
    l(1) = 100 + nn
    Print sl2(1);
    sl2() = l()
    Print sl2(1);
End Sub

Sub Params (p() As Integer, pq() As Integer)
    pq() = p()
End Sub

Sub ToParam (pq() As Integer)
    g(2) = -2
    pq() = g()
End Sub

Sub StrParam (pq() As String)
    pq() = gs()
End Sub

Sub OnlyCopy (p() As Integer)
    Dim pq(1 To 3) As Integer
    pq(2) = 6
    p() = pq()
    Print p(2)
End Sub

Sub StaticSub Static
    Dim sa(2) As Long, sbb(2) As Long
    sa(1) = sa(1) + 1
    sbb() = sa()
    Print sbb(1);
End Sub

Function Fn (kk As Long)
    Dim fx(3) As Long, fy(3) As Long
    fx(kk) = kk * 3
    fy() = fx()
    Fn = fy(kk)
End Function
