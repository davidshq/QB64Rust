$Console:Only
' Dynamic arrays in the main module (spec language/arrays-and-types, "Dynamic arrays", and the scenarios of
' "Static arrays" that came with them): DIM with a bound that is not constant, also by its spelling only; DIM run
' twice; $DYNAMIC and $STATIC; a dimension with no element; a bound that raises; an array used before its DIM has
' run; every element type, _BIT included (also as READ targets); float bounds; OPTION BASE; DIM SHARED seen in a
' SUB and an element passed by reference; DIM with empty parentheses. Error cases go to a handler that prints ERR and resumes next.
Type rec
    n As Long
End Type
On Error GoTo h
Dim i As Long, n As Long, m As Long
n = 4
Dim d(n) As Long: d(4) = 9
Print "1. a bound from a variable, DIM d(n): LBOUND, UBOUND, d(4), d(0)"; LBound(d); UBound(d); d(4); d(0)
d(1) = 5
Print "2. another DIM of it, DIM d(n + 1):";
Dim d(n + 1) As Long
Print UBound(d); d(1)
'$Dynamic
Dim a(5) As Long
'$Static
Dim a0(5) As Long
ReDim a(8) As Long
Print "3. $DYNAMIC above DIM a(5), then REDIM a(8):"; UBound(a)
Print "   $STATIC above DIM a0(5), then REDIM a0(8):";
ReDim a0(8) As Long
Print UBound(a0)
m = -1
Print "4. no element, DIM e(m) with m = -1:";
Dim e(m) As Long
Print LBound(e); UBound(e);
e(0) = 1
Print
Print "   and DIM e(n) after it:";
Dim e(n) As Long
Print UBound(e)
Print "5. a bound with an operator, DIM a3(2 + 3) then REDIM a3(20):";
Dim a3(2 + 3) As Long
ReDim a3(20) As Long
Print UBound(a3)
Print "   one number, DIM a1(5) then REDIM a1(20):";
Dim a1(5) As Long
ReDim a1(20) As Long
Print UBound(a1)
Print "6. two DIM statements, DIM s(5) then DIM s(n):";
Dim s(5) As Long
Dim s(n) As Long
Print UBound(s)
Print "7. a bound that raises, DIM r(ASC of an empty string):";
Dim r(Asc("")) As Long
Print UBound(r)
Print "8. a DIM that GOTO skips: late(1) = 1";
GoTo skip
Dim late(n) As Long
skip:
late(1) = 1
Print "   LBOUND(late)";
Print LBound(late)
Print "   UBOUND(late, 1)";
Print UBound(late, 1)
Print "   PRINT late(1)";
Print late(1)
Print "9. one DIM in a loop that runs twice:";
For i = 1 To 2
    Dim lp(i + 2) As Long
Next
Print UBound(lp)
Dim st(n) As String, fx(n) As String * 3, ud(n) As rec, db(1 To n, n) As Double
st(1) = "one": fx(1) = "abcdef": ud(2).n = 7: db(2, 3) = 1.5
Print "10. STRING, STRING * 3, a user type, DOUBLE in two dimensions: "; st(1); "["; st(2); "] "; fx(1); Len(fx(2)); Asc(fx(2)); ud(2).n; ud(1).n; db(2, 3); db(1, 0); LBound(db, 1); UBound(db, 2)
x = 2.5
Dim q(x), q2(1.5 To x + 1)
Print "11. float bounds, DIM q(x) with x = 2.5, DIM q2(1.5 TO x + 1):"; UBound(q); LBound(q2); UBound(q2)
Print "12. an index outside, d(5) = 1 and PRINT d(-1):";
d(5) = 1
Print d(-1)
Dim bt(n) As _Bit * 4, bu(1 To n) As _Unsigned _Bit * 3
bt(0) = 7: bt(1) = 8: bt(3) = 15: bu(1) = 9: bu(4) = -1
Print "13. dynamic _BIT * 4 and _UNSIGNED _BIT * 3:"; bt(0); bt(1); bt(2); bt(3); bu(1); bu(2); bu(4)
Dim b4(3) As _Bit * 4
b4(0) = 7: b4(1) = 8: b4(3) = 15
Print "14. static _BIT * 4, stores 7, 8, none, 15:"; b4(0); b4(1); b4(2); b4(3)
Dim Shared sh(n) As Long
sh(2) = 6
Print "15. DIM SHARED sh(n) in a SUB:";
show
bump sh(2)
bump d(4)
Print "    elements passed by reference:"; sh(2); d(4)
Dim ep As Long
ep = 100000.5
Print "16. DIM ep() AS LONG, ep = 100000.5: ep, LEN(ep)"; ep; Len(ep)
Option Base 1
Dim ob(n) As Long, ob0(0, n) As Long, ob2(0 To n) As Long
Print "17. OPTION BASE 1, DIM ob(n), ob0(0, n), ob2(0 TO n):"; LBound(ob); UBound(ob); LBound(ob0, 1); UBound(ob0, 1); LBound(ob0, 2); LBound(ob2)
m = 0
Print "18. OPTION BASE 1, DIM z(m) with m = 0:";
Dim z(m) As Long
Print LBound(z); UBound(z)
Print "    DIM z2(0 + 0):";
Dim z2(0 + 0) As Long
Print LBound(z2); UBound(z2)
Read bt(2), bu(2), b4(2)
Print "19. READ into _BIT elements, dynamic and static:"; bt(2); bu(2); b4(2)
Data 5,7,-8
System
h:
Print " [error"; Err; "]";
Resume Next

Sub show
    Print sh(2); UBound(sh)
End Sub

Sub bump (v As Long)
    v = v + 1
End Sub
