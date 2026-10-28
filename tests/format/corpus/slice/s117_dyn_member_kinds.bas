$Console:Only
$Unstable:TypeFields
' Slice (m3-typefields tasks 4.2 to 4.4): _DYNAMIC member arrays of every kind of element, whole-value copies and
' the statements that read into a place through a member (verification\v35_c, v35_x56, x57, x61, x62).
' 1.-10.: each numeric type, STRING * n, STRING, user types without and with a string and with a _DYNAMIC member
' of their own; _RETAIN, _PRESERVE, plain REDIM and ERASE of them, and a copy of a type that holds them all.
' 11.-16.: how often a copy finds its places (an index FUNCTION that prints), for a type without and with a
' STRING, for owner types nested as a member and as an inline member array; an index outside; what a copy does
' when the source's member is not allocated (a type without a STRING keeps the target's member, one with a STRING
' leaves it unallocated); a copy to itself.
' 17.-48.: READ, INPUT # and LINE INPUT # from a file, console INPUT and LINE INPUT, and the MID$ statement into a
' member of an element and into an element of a member array (fixed and _DYNAMIC): a number is read before the
' indexes, a string after them; an index outside raises error 9. Where the new compiler differs on purpose
' (QB64PE-DIVERGENCES.md D-004: a number read through an index outside is stored nowhere; the old compiler stores it into
' position 0) items 23, 38 and 39 were recorded from a program that reads the item into q$ and raises ERROR 9.
Type inner
    k As Integer
    f As String * 2
End Type
Type innerstr
    k As Integer
    s As String
End Type
Type innerdyn
    k As Integer
    d(1) _Dynamic As Integer
End Type
Type kinds
    b(1) _Dynamic As _Byte
    ub(1) _Dynamic As _Unsigned _Byte
    i(1) _Dynamic As Integer
    l(1) _Dynamic As Long
    q(1) _Dynamic As _Integer64
    uq(1) _Dynamic As _Unsigned _Integer64
    s(1) _Dynamic As Single
    d(1) _Dynamic As Double
    f(1) _Dynamic As _Float
    o(1) _Dynamic As _Offset
    fs(1) _Dynamic As String * 3
    vs(1) _Dynamic As String
    u(1) _Dynamic As inner
    us(1) _Dynamic As innerstr
    ud(1) _Dynamic As innerdyn
End Type
Type cflat
    n As Long
    v(1) _Dynamic As Long
    m As Long
End Type
Type cstr
    n As Long
    s As String
    v(1) _Dynamic As Long
    w(1) _Dynamic As String
End Type
Type nin
    k As Long
    d(1) _Dynamic As Long
End Type
Type nout
    a As Long
    i As nin
    r(1) As nin
End Type
Type nins
    k As Long
    s As String
    d(1) _Dynamic As Long
End Type
Type nouts
    a As Long
    i As nins
    r(1) As nins
    t(1) As String
End Type
Type flat
    d(1) _Dynamic As Long
End Type
Type owner
    s As String
    d(1) _Dynamic As Long
End Type
Type tg
    m As Long
    s As String
    v(1) As Long
    w(1) As String
    d(1) _Dynamic As Long
    e(1) _Dynamic As String
End Type
Dim Shared q$, z$
Dim Shared tgt(2) As tg
On Error GoTo h
Dim x As kinds, y As kinds
x.b(1) = -5: x.ub(1) = 250: x.i(1) = -30000: x.l(1) = 100000: x.q(1) = 2 ^ 40: x.uq(1) = 18446744073709551614~&&
x.s(1) = 1.5: x.d(1) = 2.25: x.f(1) = 3.125: x.o(1) = 77
x.fs(0) = "abc": x.fs(1) = "xy"
x.vs(0) = "first": x.vs(1) = "second string"
x.u(1).k = 9: x.u(1).f = "zz"
x.us(1).k = 8: x.us(1).s = "inner string"
x.ud(1).k = 7: x.ud(1).d(1) = 70
ReDim x.ud(1).d(2 TO 4) AS INTEGER: x.ud(1).d(4) = 44
Print "1."; x.b(1); x.ub(1); x.i(1); x.l(1); x.q(1); x.uq(1); x.s(1); x.d(1); x.f(1); x.o(1)
Print "2. ["; x.fs(0); "] ["; x.fs(1); "] ["; x.vs(0); "] ["; x.vs(1); "]"
Print "3."; x.u(1).k; x.u(1).f; x.us(1).k; x.us(1).s; x.ud(1).k; LBound(x.ud(1).d); UBound(x.ud(1).d); x.ud(1).d(4)
y = x
x.vs(1) = "changed": x.us(1).s = "changed": x.ud(1).d(4) = -1: x.fs(0) = "QQQ"
Print "4. copy: ["; y.vs(1); "] ["; y.us(1).s; "]"; y.ud(1).d(4); "["; y.fs(0); "]"
ReDim _Retain x.vs(1 TO 3) AS STRING
Print "5. _RETAIN vs(1 TO 3): ["; x.vs(1); "] ["; x.vs(2); "] ["; x.vs(3); "]"
ReDim _Retain x.fs(-1 TO 1) AS STRING * 3
Print "6. _RETAIN fs(-1 TO 1): ["; x.fs(-1); "] ["; x.fs(0); "] ["; x.fs(1); "]"
ReDim _Preserve x.us(0 TO 2) AS INNERSTR
Print "7. _PRESERVE us(0 TO 2):"; x.us(1).k; "["; x.us(1).s; "]"; x.us(2).k; "["; x.us(2).s; "]"
ReDim _Retain x.ud(1 TO 2) AS INNERDYN
Print "8. _RETAIN ud(1 TO 2):"; x.ud(1).k; LBound(x.ud(1).d); UBound(x.ud(1).d); x.ud(1).d(4); LBound(x.ud(2).d); UBound(x.ud(2).d)
ReDim x.vs(0 TO 1) AS STRING
Print "9. plain REDIM vs: ["; x.vs(0); "] ["; x.vs(1); "]"; Len(x.vs(1))
Erase x.us
Print "10. ERASE us, then y.us(1).s: ["; y.us(1).s; "]"
ReDim ca(2) As cflat, cb(2) As cflat, cc(2) As cstr, cd(2) As cstr
cb(1).v(1) = 5: cd(1).s = "s": cd(1).w(1) = "w"
Print "11.";
ca(one(1)) = cb(one(2))
Print
Print "12.";
cc(one(1)) = cd(one(2))
Print
Print "   "; ca(1).v(1); cc(1).s; cc(1).w(1)
Dim cp As cflat
ReDim cp.v(4 TO 5) AS LONG: cp.v(5) = 55: cp.n = 6
ca(9) = cp
Print "13."; ca(0).n; LBound(ca(0).v); UBound(ca(0).v)
cp = ca(9)
Print "   "; cp.n
ReDim na(2) As nout, nb(2) As nout, nc(2) As nouts, ne(2) As nouts
nb(1).a = 1: nb(1).i.k = 2: nb(1).i.d(1) = 3: nb(1).r(1).k = 4: nb(1).r(1).d(0) = 5
ne(1).a = 1: ne(1).i.s = "s": ne(1).i.d(1) = 3: ne(1).r(1).s = "r": ne(1).r(1).d(0) = 5: ne(1).t(1) = "t"
Print "14.";
na(one(1)) = nb(one(2))
Print
Print "   "; na(1).a; na(1).i.k; na(1).i.d(1); na(1).r(1).k; na(1).r(1).d(0)
nb(1).i.d(1) = 30: nb(1).r(1).d(0) = 50
Print "   after a change of nb(1):"; na(1).i.d(1); na(1).r(1).d(0)
Print "15.";
nc(one(1)) = ne(one(2))
Print
Print "   "; nc(1).a; " "; nc(1).i.s; nc(1).i.d(1); " "; nc(1).r(1).s; nc(1).r(1).d(0); " "; nc(1).t(1)
Dim ff As flat, fg As flat, oo As owner, op As owner
ff.d(1) = 7: oo.d(1) = 7
Erase fg.d, op.d
ff = fg
Print "16. flat, source erased:";
Print LBound(ff.d); UBound(ff.d);
Print ff.d(1)
oo = op
Print "    owner, source erased:";
Print LBound(oo.d); UBound(oo.d);
Print oo.d(1)
ReDim op.d(2 TO 3) AS LONG: op.d(3) = 9
oo = op
Print "    owner, source redimmed:"; LBound(oo.d); UBound(oo.d); oo.d(3)
oo = oo
Print "    copied to itself:"; LBound(oo.d); UBound(oo.d); oo.d(3)
Read tgt(rd(1)).m: Print "17."; tgt(1).m
Read tgt(rd(1)).s: Print "18. "; tgt(1).s
Read tgt(1).v(rd(1)): Print "19."; tgt(1).v(1)
Read tgt(1).w(rd(1)): Print "20. "; tgt(1).w(1)
Read tgt(1).d(rd(1)): Print "21."; tgt(1).d(1)
Read tgt(1).e(rd(1)): Print "22. "; tgt(1).e(1)
Read tgt(9).m: Print "23."; tgt(0).m
Read tgt(9).s: Print "24. "; tgt(0).s
Read tgt(1).v(9): Print "25."; tgt(1).v(0)
Read tgt(1).d(9): Print "26."; tgt(1).d(0)
Read tgt(1).e(9): Print "27. "; tgt(1).e(0)
Read q$: Print "28. next item: "; q$
Open "s117.tmp" For Output As #1
For i = 1 To 20: Print #1, "L"; LTrim$(Str$(100 + i)): Next
Close #1
Open "s117.tmp" For Input As #1
Input #1, tgt(rf(1)).s: Print "29. "; tgt(1).s
Input #1, tgt(1).e(rf(1)): Print "30. "; tgt(1).e(1)
Line Input #1, tgt(rf(1)).s: Print "31. "; tgt(1).s
Line Input #1, tgt(1).w(rf(1)): Print "32. "; tgt(1).w(1)
Input #1, tgt(9).s: Print "33. "; tgt(0).s
Line Input #1, tgt(1).e(9): Print "34. "; tgt(1).e(0)
Input #1, q$: Print "35. next line: "; q$
Close #1
Open "s117.tmp" For Output As #1
For i = 1 To 20: Print #1, 200 + i: Next
Close #1
Open "s117.tmp" For Input As #1
Input #1, tgt(rf(1)).m: Print "36."; tgt(1).m
Input #1, tgt(1).d(rf(1)): Print "37."; tgt(1).d(1)
Input #1, tgt(9).m: Print "38."; tgt(0).m
Input #1, tgt(1).v(9): Print "39."; tgt(1).v(0)
Input #1, q$: Print "40. next line: "; q$
Close #1
Kill "s117.tmp"
Input tgt(ci(1)).m, tgt(1).d(ci(1)): Print "41."; tgt(1).m; tgt(1).d(1)
Line Input tgt(1).e(ci(1)): Print "42. "; tgt(1).e(1)
Input tgt(9).m: Print "43."; tgt(0).m
Line Input tgt(1).e(9): Print "44. "; tgt(1).e(0)
tgt(1).s = "abcdef": tgt(1).e(1) = "uvwxyz": tgt(0).s = "000000": tgt(1).w(0) = "nnnnnn"
z$ = "A": Mid$(tgt(mf(1)).s, 1, 1) = z$: Print "45. "; tgt(1).s
z$ = "A": Mid$(tgt(1).e(mf(1)), 2) = z$: Print "46. "; tgt(1).e(1)
Mid$(tgt(9).s, 3, 1) = "Z": Print "47. "; tgt(0).s
Mid$(tgt(1).w(9), 4, 1) = "Z": Print "48. "; tgt(1).w(0)
System
h:
Print "[error"; Err; "]";
Resume Next
Data 10,11,x,y,20,21,p,q,30,31,r,s,40,u,50,v,60,w,70,z,80,y,90
Function one& (k As Long)
    Print k;
    one& = 1
End Function
Function rd (k As Long)
    Read q$
    Print "[rd "; q$; "]";
    rd = k
End Function
Function rf (k As Long)
    Input #1, q$
    Print "[rf "; q$; "]";
    rf = k
End Function
Function ci (k As Long)
    Print "[ci]";
    ci = k
End Function
Function mf (k As Long)
    z$ = "B"
    Print "[mf]";
    mf = k
End Function
