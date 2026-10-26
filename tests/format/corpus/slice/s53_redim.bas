$Console:Only
' REDIM (spec language/arrays-and-types, "REDIM"): the first REDIM as the declaration; new bounds and new elements
' each time; the type kept without AS, of an array made by REDIM and by DIM; the same array by its suffix and
' another one by another suffix; REDIM of a static array (error 10); several arrays in one statement and the type
' before the names; a dimension with no element and a bound that raises; REDIM of a name without bounds (a
' scalar); a scalar of the array's name; strings, fixed-length strings, a user type; REDIM SHARED, and REDIM of a
' shared array in a SUB. Error cases go to a handler that prints ERR and resumes next.
Type rec
    n As Long
    t As String * 2
End Type
On Error GoTo h
Dim i As Long, n As Long, m As Long
n = 3
ReDim r(5) As Long
r(2) = 7
ReDim r(1 To 20) As Long
Print "1. REDIM r(5), r(2) = 7, REDIM r(1 TO 20): LBOUND, UBOUND, r(2)"; LBound(r); UBound(r); r(2)
ReDim x(5) As Long
ReDim x(10)
x(1) = 100000
Print "2. REDIM x(5) AS LONG, REDIM x(10), x(1) = 100000:"; x(1); UBound(x)
Dim s(5) As Long
s(1) = 3
Print "3. REDIM of a static array:";
ReDim s(9) As Long
Print UBound(s); s(1)
ReDim x&(7)
ReDim x!(2)
x!(1) = 1.5
Print "4. REDIM x&(7) and REDIM x!(2): UBOUND(x), UBOUND(x&), UBOUND(x!), x!(1)"; UBound(x); UBound(x&); UBound(x!); x!(1)
Dim dm(n) As Long
ReDim dm(7)
dm(1) = 100000
Print "5. DIM dm(n) AS LONG, REDIM dm(7), dm(1) = 100000:"; UBound(dm); dm(1)
ReDim p1(2) As Long, p2(n, 1 To 2) As String, p3%(4)
p2(3, 2) = "last"
Print "6. three arrays in one statement:"; UBound(p1); UBound(p2, 1); UBound(p2, 2); UBound(p3%); " "; p2(3, 2); Len(p2(0, 1))
ReDim As Double q1(2), q2(1 To n)
q2(n) = .5
Print "7. the type before the names:"; UBound(q1); LBound(q2); q2(n); Len(q1(0))
m = -1
r(1) = 4
Print "8. no element, REDIM r(m) with m = -1:";
ReDim r(m) As Long
Print LBound(r); UBound(r);
Print r(1)
Print
Print "   and REDIM r(2) after it:";
ReDim r(2) As Long
Print UBound(r); r(1)
r(1) = 4
Print "9. a bound that raises, REDIM r(ASC of an empty string TO 5):";
ReDim r(Asc("") To 5) As Long
Print LBound(r); UBound(r); r(1)
ReDim e1 As Long
ReDim e2 As Long
e1 = 100000.5: e2 = 2.5
Print "10. REDIM e1 AS LONG and REDIM e2() AS LONG are scalars:"; e1; e2; Len(e1)
sc& = 5
ReDim sc&(3)
sc&(1) = 2
ReDim sc&(4)
Print "11. a scalar of the array's name:"; sc&; sc&(1); UBound(sc&)
ReDim st(n) As String, fx(n) As String * 3, ud(n) As rec
st(1) = "one": fx(1) = "abcdef": ud(2).n = 7: ud(2).t = "xy"
Print "12. STRING, STRING * 3, a user type: "; st(1); "["; st(2); "] "; fx(1); ud(2).n; ud(2).t
ReDim st(n + 1) As String, fx(n + 1) As String * 3, ud(n + 1) As rec
Print "    after REDIM: ["; st(1); "]"; Len(fx(1)); Asc(fx(1)); ud(2).n; Asc(ud(2).t); UBound(st)
For i = 1 To 3
    ReDim lp(i * 2) As Long
    lp(i) = lp(i) + i
Next
Print "13. one REDIM in a loop that runs three times:"; UBound(lp); lp(3); lp(2)
ReDim Shared g(2) As Long
g(1) = 8
Print "14. REDIM SHARED g(2), in a SUB:";
grow
Print "    after the SUB's REDIM g(5):"; UBound(g); g(1); g(5)
Dim Shared fixed(3) As Long
fixed(1) = 9
Print "15. REDIM of a shared static array in a SUB:";
nogrow
Print UBound(fixed); fixed(1)
Option Base 1
ReDim ob(n) As Long, ob0(0) As Long
Print "16. OPTION BASE 1, REDIM ob(n), ob0(0):"; LBound(ob); UBound(ob); LBound(ob0); UBound(ob0)
System
h:
Print " [error"; Err; "]";
Resume Next

Sub grow
    Print g(1); UBound(g)
    ReDim g(5)
    g(5) = 1
End Sub

Sub nogrow
    ReDim fixed(7) As Long
End Sub
