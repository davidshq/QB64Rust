$Console:Only
' Slice (m3-redim-retain task 3.2): REDIM _RETAIN of numbers (verification\v34_a). The elements whose indexes lie
' in both the old and the new bounds keep their values at the same indexes: one dimension moved, grown, shrunk and
' moved away, two and three dimensions, the first REDIM _RETAIN of an array and one after ERASE, the type kept
' without AS and by a suffix, _UNSIGNED _BYTE, a dimension with no element (error 5), the same bounds.
On Error GoTo h
Dim i As Long, j As Long, k As Long
Print "moved:";
ReDim a(1 To 5) As Long
For i = 1 To 5: a(i) = i * 10: Next
ReDim _Retain a(3 To 7) As Long
For i = 3 To 7: Print a(i);: Next: Print
Print "grown:";
ReDim _Retain a(0 To 9) As Long
For i = 0 To 9: Print a(i);: Next: Print
Print "shrunk:";
ReDim _Retain a(4 To 5) As Long
For i = 4 To 5: Print a(i);: Next: Print
Print "moved away:";
ReDim _Retain a(100 To 102) As Long
For i = 100 To 102: Print a(i);: Next: Print
Print "two dimensions:"
ReDim m(1 To 3, 1 To 4) As Integer
For i = 1 To 3: For j = 1 To 4: m(i, j) = 10 * i + j: Next: Next
ReDim _Retain m(2 To 5, 0 To 3) As Integer
For i = 2 To 5: For j = 0 To 3: Print m(i, j);: Next: Print: Next
Print "three dimensions:"
ReDim c(2, 2, 2) As Double
For i = 0 To 2: For j = 0 To 2: For k = 0 To 2: c(i, j, k) = 100 * i + 10 * j + k + 0.5: Next: Next: Next
ReDim _Retain c(1 To 3, 0 To 1, 1 To 2) As Double
For i = 1 To 3: For j = 0 To 1: For k = 1 To 2: Print c(i, j, k);: Next: Next: Print: Next
Print "first:";
ReDim _Retain f(2 To 4) As Long
Print LBound(f); UBound(f); f(2); f(4)
Print "after ERASE:";
f(3) = 9
Erase f
ReDim _Retain f(1 To 5) As Long
Print LBound(f); UBound(f); f(3)
Print "type kept:";
ReDim q(5) As _Integer64
q(5) = 123456789012345
ReDim _Retain q(3 To 8)
Print q(5); Len(q(5));
ReDim s&(3)
s&(2) = 77
ReDim _Retain s&(2 To 6)
Print s&(2)
Print "bytes:";
ReDim b(0 To 7) As _Unsigned _Byte
For i = 0 To 7: b(i) = i: Next
ReDim _Retain b(-2 To 3) As _Unsigned _Byte
For i = -2 To 3: Print b(i);: Next: Print
Print "no element:";
ReDim a(1 To 3) As Long: a(2) = 4
j = 4
ReDim _Retain a(5 To j) As Long
Print LBound(a); UBound(a);
ReDim _Retain a(1 To 3) As Long
Print a(1); a(2); a(3)
Print "same bounds:";
a(2) = 6
ReDim _Retain a(1 To 3) As Long
Print a(2)
Print "done"
System
h:
Print " [error"; Err; "]";
Resume Next
