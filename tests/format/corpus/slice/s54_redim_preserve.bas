$Console:Only
' REDIM _PRESERVE (spec language/arrays-and-types, "REDIM _PRESERVE"): the elements kept by flat position, as many
' as the smaller of the old and the new number: growing and shrinking, strings, fixed-length strings, a user type,
' _BIT elements, two dimensions (positions, not coordinates), another lower bound; of an array that is not
' allocated (as REDIM), as the first statement that names the array, of a static array (error 10); with SHARED;
' of a shared array in a SUB; in a loop that grows an array. Error cases go to a handler that prints ERR and
' resumes next.
Type rec
    n As Long
    t As String * 2
End Type
On Error GoTo h
Dim i As Long, n As Long
ReDim p(3) As Long
For i = 0 To 3: p(i) = i + 1: Next
ReDim _Preserve p(5) As Long
Print "1. 0 to 3 holding 1 2 3 4, REDIM _PRESERVE p(5): p(3), p(5), UBOUND"; p(3); p(5); UBound(p)
ReDim _Preserve p(1) As Long
Print "   then REDIM _PRESERVE p(1): UBOUND, p(0), p(1)"; UBound(p); p(0); p(1)
ReDim _Preserve p(3) As Long
Print "   then REDIM _PRESERVE p(3): p(1), p(2), p(3)"; p(1); p(2); p(3)
ReDim t(1) As String
t(0) = "zero": t(1) = "keep"
ReDim _Preserve t(4) As String
Print "2. strings: "; t(0); " "; t(1); Len(t(4))
ReDim _Preserve t(0) As String
ReDim _Preserve t(2) As String
Print "   shrunk to one and grown again: "; t(0); " ["; t(1); "]"
ReDim g(1, 1) As Long
g(0, 0) = 1: g(1, 0) = 2: g(0, 1) = 3: g(1, 1) = 4
ReDim _Preserve g(2, 1) As Long
Print "3. two dimensions, (1, 1) holding 1 2 3 4 to (2, 1):"; g(0, 0); g(1, 0); g(2, 0); g(0, 1); g(1, 1); g(2, 1)
ReDim b(3) As Long
For i = 0 To 3: b(i) = i + 1: Next
ReDim _Preserve b(10 To 15) As Long
Print "4. another lower bound, 0 to 3 to 10 to 15:";
For i = 10 To 15: Print b(i);: Next
Print
Dim s(5) As Long
s(1) = 3
Print "5. of a static array:";
ReDim _Preserve s(9) As Long
Print UBound(s); s(1)
ReDim _Preserve nw(3) As Long
nw(3) = 5
Print "6. as the first statement that names the array:"; UBound(nw); nw(3)
Erase nw
ReDim _Preserve nw(2) As Long
Print "   of an array that is not allocated:"; UBound(nw); nw(2)
ReDim fx(1) As String * 3, ud(1) As rec
fx(1) = "abc": ud(1).n = 7: ud(1).t = "xy"
ReDim _Preserve fx(3) As String * 3, ud(3) As rec
Print "7. STRING * 3 and a user type: "; fx(1); Asc(fx(3)); ud(1).n; ud(1).t; ud(3).n; Asc(ud(3).t)
ReDim bt(4) As _Bit * 5
For i = 0 To 4: bt(i) = i + 1: Next
ReDim _Preserve bt(9) As _Bit * 5
Print "8. _BIT * 5 from 4 to 9:";
For i = 0 To 9: Print bt(i);: Next
Print
ReDim _Preserve bt(1) As _Bit * 5
ReDim _Preserve bt(3) As _Bit * 5
Print "   shrunk to 1 and grown to 3:"; bt(0); bt(1); bt(2); bt(3)
ReDim _Preserve Shared sh(2) As Long
sh(2) = 6
Print "9. REDIM _PRESERVE SHARED sh(2), grown in a SUB:";
grow
Print UBound(sh); sh(2); sh(4)
ReDim lst(0) As String
n = 0
For i = 1 To 5
    n = n + 1
    ReDim _Preserve lst(n) As String
    lst(n) = Str$(i * i)
Next
Print "10. a list grown in a loop:"; UBound(lst); lst(1); lst(3); lst(5); " ["; lst(0); "]"
n = 2
ReDim _Preserve lst(n To n + 2) As String
Print "11. the same, cut to three elements from 2:"; LBound(lst); UBound(lst); " ["; lst(2); "]"; lst(3); lst(4)
System
h:
Print " [error"; Err; "]";
Resume Next

Sub grow
    ReDim _Preserve sh(4)
    sh(4) = sh(2) + 1
End Sub
