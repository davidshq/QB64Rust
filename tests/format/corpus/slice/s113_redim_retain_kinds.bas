$Console:Only
' Slice (m3-redim-retain task 3.2): REDIM _RETAIN of the other kinds of element and in other places
' (verification\v34_b, v34_c): strings in one and two dimensions, STRING * 3 (NUL bytes where nothing is kept),
' _UNSIGNED _BIT * 5 and _BIT, a user type with strings, one with a member array, REDIM _RETAIN SHARED, a local
' array of a SUB, a SHARED array in a SUB, an array parameter, a static array (error 10), a member array (nothing
' changes), conventional memory, and a _MEM block of the array, no longer valid after it.
Type rec
    n As Long
    s As String
    f As String * 3
End Type
Type box
    id As Integer
    v(1 TO 3) As Long
    t As String * 2
End Type
DECLARE SUB grow (p() AS LONG)
On Error GoTo h
Dim i As Long, j As Long
ReDim _Retain Shared g(1 To 3) As Long
g(2) = 22
Print "strings:";
ReDim s(1 To 4) As String
For i = 1 To 4: s(i) = Chr$(64 + i): Next
ReDim _Retain s(3 To 6) As String
For i = 3 To 6: Print "["; s(i); "]";: Next: Print
Print "strings, two dimensions:"
ReDim t(2, 2) As String
For i = 0 To 2: For j = 0 To 2: t(i, j) = LTrim$(Str$(i)) + LTrim$(Str$(j)): Next: Next
ReDim _Retain t(1 To 3, 1 To 2) As String
For i = 1 To 3: For j = 1 To 2: Print "["; t(i, j); "]";: Next: Print: Next
Print "STRING * 3:";
ReDim x(1 To 3) As String * 3
For i = 1 To 3: x(i) = String$(3, 96 + i): Next
ReDim _Retain x(2 To 5) As String * 3
For i = 2 To 5: Print Asc(x(i), 1); Asc(x(i), 3);: Next: Print
Print "_UNSIGNED _BIT * 5:";
ReDim p(0 To 9) As _Unsigned _Bit * 5
For i = 0 To 9: p(i) = 3 * i: Next
ReDim _Retain p(5 To 14) As _Unsigned _Bit * 5
For i = 5 To 14: Print p(i);: Next: Print
Print "_BIT:"
ReDim q(3, 3) As _Bit
For i = 0 To 3: For j = 0 To 3: q(i, j) = -((i + j) Mod 2): Next: Next
ReDim _Retain q(1 To 4, 2 To 5) As _Bit
For i = 1 To 4: For j = 2 To 5: Print q(i, j);: Next: Print: Next
Print "a user type with strings:";
ReDim r(1 To 3) As rec
For i = 1 To 3: r(i).n = i: r(i).s = "s" + LTrim$(Str$(i)): r(i).f = "f" + LTrim$(Str$(i)): Next
ReDim _Retain r(2 To 4) As rec
For i = 2 To 4: Print r(i).n; "["; r(i).s; "]"; Asc(r(i).f, 1); Asc(r(i).f, 3);: Next: Print
Print "a user type with a member array:"
ReDim b(1, 1) As box
For i = 0 To 1: For j = 0 To 1
        b(i, j).id = 10 * i + j: b(i, j).v(2) = 100 + 10 * i + j: b(i, j).t = "t" + LTrim$(Str$(j))
Next: Next
ReDim _Retain b(0 To 1, 1 To 2) As box
For i = 0 To 1: For j = 1 To 2: Print b(i, j).id; b(i, j).v(2); Asc(b(i, j).t, 1);: Next: Print: Next
Print "a member array:";
Dim one As box
one.v(2) = 5
ReDim _Retain one.v(1 TO 3) AS LONG
Print one.v(2)
Print "a local array:";
localsub
Print "SHARED:";
gsub
Print UBound(g); g(2);
ReDim k(1 To 3) As Long
k(2) = 33
sharedsub
Print UBound(k); k(2)
Print "a parameter:";
ReDim w(1 To 3) As Long
For i = 1 To 3: w(i) = i: Next
grow w()
For i = 0 To 5: Print w(i);: Next: Print
Print "a static array:";
Dim st(5) As Long
st(1) = 3
ReDim _Retain st(9) As Long
Print UBound(st); st(1)
Print "conventional memory:";
ReDim cm(1 To 4) As Integer
For i = 1 To 4: cm(i) = i * 11: Next
Print VarPtr(cm(1)) >= 0;
ReDim _Retain cm(2 To 6) As Integer
For i = 2 To 6: Print cm(i);: Next: Print
Print "a _MEM block:";
ReDim mm(1 To 3) As Long
Dim blk As _MEM
blk = _Mem(mm())
Print _MemExists(blk);
ReDim _Retain mm(1 To 5) As Long
Print _MemExists(blk)
Print "done"
System
h:
Print " [error"; Err; "]";
Resume Next

Sub localsub
    ReDim l(1 To 3) As String
    l(3) = "x"
    ReDim _Retain l(3 To 4) As String
    Print "["; l(3); "]"; LBound(l)
End Sub

Sub gsub
    ReDim _Retain g(2 To 8) As Long
End Sub

Sub sharedsub
    Shared k() As Long
    ReDim _Retain k(2 To 7) As Long
End Sub

Sub grow (p() As Long)
    ReDim _Retain p(0 To 5) As Long
End Sub
