$Console:Only
$Unstable:TypeFields
' Slice (m3-arraycopy): _ARRAYCOPY of _DYNAMIC member arrays that a REDIM gave another number of dimensions than
' they were declared with (verification\v41_l): error 9 and nothing copied, as source and as destination, whole,
' to a start and with ranges; a member REDIMmed back to its own number copies again. A plain array beside each is
' printed: -1 where nothing was copied.
Type dy
    _Dynamic n(1 TO 4) As Long
    _Dynamic g(1 TO 2, 1 TO 3) As Long
End Type
Dim i As Long, j As Long
Dim v As dy, w(1 To 2) As dy
Dim src(1 To 4) As Long, d(1 To 10) As Long, m(1 To 2, 1 To 3) As Long
On Error GoTo h
For i = 1 To 4: src(i) = i * 100: Next
ReDim v.n(1 TO 2, 1 TO 3) AS LONG
ReDim w(2).g(1 TO 6) AS LONG
Print "1:";
For i = 1 To 10: d(i) = -1: Next
_ArrayCopy v.n() To d()
For i = 1 To 10: Print d(i);: Next: Print
Print "2:";
_ArrayCopy v.n(1 To 2) To d(5)
For i = 1 To 10: Print d(i);: Next: Print
Print "3:";
_ArrayCopy src() To v.n()
_ArrayCopy src() To v.n(1)
_ArrayCopy src(2 To 3) To v.n()
Print
Print "4:";
For i = 1 To 2: For j = 1 To 3: m(i, j) = -1: Next: Next
_ArrayCopy w(2).g() To m()
_ArrayCopy w(2).g(1 To 1, 1 To 2) To m(1, 1)
For i = 1 To 2: For j = 1 To 3: Print m(i, j);: Next: Next: Print
Print "5:";
_ArrayCopy m() To w(2).g()
_ArrayCopy m() To w(2).g(1, 1)
_ArrayCopy m(1 To 2, 1 To 2) To w(2).g()
Print
Print "6:";
_ArrayCopy v.n() To w(1).n()
Print
ReDim v.n(1 TO 4) AS LONG
_ArrayCopy src() To v.n()
Print "7:";: For i = 1 To 4: Print v.n(i);: Next: Print
System
h:
Print " error"; Err;
Resume Next
