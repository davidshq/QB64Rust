$Console:Only
$Unstable:TypeFields
' Slice (m3-typefields task 3.4): _DYNAMIC member arrays of a variable and of array elements (verification\v35_a,
' v35_g, v35_x58). The first bounds and values, REDIM (plain, _PRESERVE, _RETAIN, another number of dimensions),
' ERASE and every use after it, LBOUND and UBOUND, a whole-value copy and its isolation, the index errors, and an
' index outside on the way to the member. Where the new compiler differs on purpose (DIVERGENCES.md D-004 widened:
' a store through a parent index outside its bounds stores nothing; the old compiler stores into the member of
' element 0) the line was recorded from a program with ERROR 9 in place of that store.
Type t
    a As Integer
    v(2) _Dynamic As Long
    b As Integer
End Type
Type u
    v(1) _Dynamic As Long
End Type
On Error GoTo h
Dim x As t, y As t
Dim i As Long
Print "1. first bounds:"; LBound(x.v); UBound(x.v); "values:"; x.v(0); x.v(1); x.v(2)
x.a = 1: x.b = 2
For i = 0 To 2: x.v(i) = 10 + i: Next
ReDim x.v(1 TO 5) AS LONG
Print "2. plain REDIM 1 TO 5:"; LBound(x.v); UBound(x.v); "values:";
For i = 1 To 5: Print x.v(i);: Next: Print
Print "3. a, b:"; x.a; x.b
For i = 1 To 5: x.v(i) = 20 + i: Next
ReDim _Preserve x.v(1 TO 7) AS LONG
Print "4. _PRESERVE 1 TO 7:";
For i = 1 To 7: Print x.v(i);: Next: Print
ReDim _Preserve x.v(3 TO 5) AS LONG
Print "5. _PRESERVE 3 TO 5:";
For i = 3 To 5: Print x.v(i);: Next: Print
ReDim _Retain x.v(4 TO 8) AS LONG
Print "6. _RETAIN 4 TO 8:";
For i = 4 To 8: Print x.v(i);: Next: Print
y = x
y.v(5) = 999
Print "7. copy, then y.v(5) = 999:"; x.v(5); y.v(5); LBound(y.v); UBound(y.v)
ReDim x.v(1 TO 2, 3 TO 4) AS LONG
Print "8. two dimensions:"; LBound(x.v, 1); UBound(x.v, 1); LBound(x.v, 2); UBound(x.v, 2)
x.v(2, 4) = 7: Print "   x.v(2, 4) ="; x.v(2, 4)
Print "9. one index after two dimensions:"; x.v(1)
Erase x.v
Print "10. after ERASE: UBOUND ="; UBound(x.v)
Print "11. after ERASE: x.v(1) ="; x.v(1)
Print "12. after ERASE: x.v(0) = 1:";: x.v(0) = 1: Print
ReDim x.v(0 TO 1) AS LONG
x.v(1) = 5
Print "13. REDIM after ERASE:"; LBound(x.v); UBound(x.v); x.v(1)
Print "14. out of range: x.v(2) ="; x.v(2)
Print "15. x.v(-1) = 3:";: x.v(-1) = 3: Print
Print "16. UBOUND(x.v, 2):"; UBound(x.v, 2)
Print "17. a, b after the REDIMs:"; x.a; x.b; "y.v(4):"; y.v(4)
ReDim p(2) As u
Print "18.";: ReDim p(9).v(4 TO 6) AS LONG: Print UBound(p(0).v); UBound(p(1).v)
Print "19.";: Print UBound(p(9).v)
Print "20.";: p(9).v(5) = 55: Print p(0).v(5)
Print "21.";: Erase p(9).v: Print UBound(p(0).v)
Print "22.";: Print LBound(p(1).v)
ReDim q(1) As u
Erase q
Print "23. REDIM q(1).v after ERASE q:";: ReDim q(1).v(0 TO 3) AS LONG: Print
Print "done"
System
h:
Print "[error"; Err; "]";
Resume Next
