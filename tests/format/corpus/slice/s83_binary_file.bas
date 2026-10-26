$Console:Only
' Slice (m3-console-files, task 5.3; from verification\v30_d_binary, v30_x39, x43-x46): GET and PUT on a file open
' FOR BINARY, 16 numbered lines. The size of each numeric type in the file and the values read back; the position
' given, left out and as a fraction (SEEK, LOC); a variable-length string (as many characters as it holds, an
' empty one reads nothing) and a fixed-length one; a TYPE variable, a member, an element, an element of a string
' array; a whole array of numbers, of fixed-length strings, of a TYPE, a dynamic one and one of two dimensions; a
' GET at and past the end (zeros, EOF, the file not grown); a PUT past the end (the gap is 0); the errors: position
' 0 and below (63), no variable (54), a file not open (52), a sequential file (54), ACCESS READ and WRITE (75);
' after CALL; an index outside (error 9, element 0 stands in). The file is s83_binary_file.tmp and is removed.
On Error GoTo handler
Type rec
    n As Long
    s As String * 6
    d As Double
End Type
Dim f As String, b As _Byte, ub As _Unsigned _Byte, i As Integer, l As Long, q As _Integer64, s As Single
Dim d As Double, fl As _Float, o As _Offset, v As String, x As String * 4, r As rec, r2 As rec, k As Long
Dim a(1 To 4) As Long, a2(1 To 4) As Long, e(1 To 2) As String, fa(1 To 2) As String * 3, ra(1 To 2) As rec
Dim t2(1 To 2, 1 To 3) As Integer, n As Long
f = "s83_binary_file.tmp"
Open f For Binary As #1
i = 258: l = 65537: d = 1.5: b = -2
Put #1, , i: Put #1, , l: Put #1, , d: Put #1, , b
Print "1:"; LOF(1); Seek(1); Loc(1)
i = 0: l = 0: d = 0: b = 0
Get #1, 1, i: Get #1, , l: Get #1, , d: Get #1, , b
Print "2:"; i; l; d; b; Seek(1); EOF(1)
q = 9: s = 2.5: fl = 3.5: o = 7
Put #1, 1, q: Print "3:"; Seek(1);: Put #1, 1, s: Print Seek(1);: Put #1, 1, fl: Print Seek(1);: Put #1, 1, o: Print Seek(1)
Close #1: Kill f
Open f For Binary As #1
v = "abcdef": Put #1, 1, v
Print "4:"; LOF(1);: v = "": Get #1, 1, v: Print "["; v; "]"; Seek(1);: v = Space$(3): Get #1, 1, v: Print "["; v; "]"; Seek(1);
v = Space$(9): Get #1, 2, v: Print "["; RTrim$(v); "]"; Asc(v, 6); EOF(1); LOF(1)
x = "wxyz": Put #1, 3, x: x = "": Get #1, 1, x
Print "5: ["; x; "]";: Get #1, 2.5, ub: Print ub;: Get #1, 3.5, ub: Print ub; Seek(1)
Close #1: Kill f
Open f For Binary As #1
r.n = 7: r.s = "abc": r.d = 2.25
Put #1, 1, r: Print "6:"; LOF(1);: Get #1, 1, r2: Print r2.n; "["; r2.s; "]"; r2.d;
r.n = 99: Put #1, 1, r.n: r.s = "member": Put #1, 5, r.s: Get #1, 1, r2: Print r2.n; "["; r2.s; "]"; LOF(1)
Close #1: Kill f
Open f For Binary As #1
a(1) = 11: a(2) = 22: a(3) = 33: a(4) = 44
Put #1, 1, a(): Print "7:"; LOF(1);: Get #1, 1, a2(): Print a2(1); a2(2); a2(3); a2(4);
k = 3: Put #1, 1, a(k): Get #1, 1, l: Print l;: Get #1, 5, a2(k + 1): Print a2(4)
Close #1: Kill f
Open f For Binary As #1
e(1) = "ab": e(2) = "cde"
Put #1, 1, e(2): Print "8:"; LOF(1);: v = "??": Get #1, 1, v: Print v;: Get #1, 2, e(1): Print " "; e(1)
Close #1: Kill f
Open f For Binary As #1
fa(1) = "ab": fa(2) = "cde"
Put #1, 1, fa(): Print "9:"; LOF(1);: fa(1) = "": Get #1, 4, fa(1): Print "["; fa(1); "]";
ra(1).n = 1: ra(2).n = 2: ra(2).s = "two"
Put #1, 1, ra(): Print LOF(1);: Get #1, 19, r2: Print r2.n; "["; r2.s; "]";: Put #1, 1, ra(2): Get #1, 1, l: Print l
Close #1: Kill f
Open f For Binary As #1
n = 5
Dim da(1 To n) As Long
da(5) = 55: Put #1, 1, da(): Print "10:"; LOF(1);: ReDim da(1 To 2) As Long: Get #1, 17, da(): Print da(1); Seek(1);
t2(2, 3) = 6: Put #1, 1, t2(): Get #1, 11, i: Print i
Close #1: Kill f
Open f For Binary As #1
l = 16909060: Put #1, 1, l
l = -1: Get #1, 3, l: Print "11:"; l; EOF(1); LOF(1); Seek(1);: l = -1: Get #1, 5, l: Print l; EOF(1); LOF(1);
l = -1: Get #1, 20, l: Print l; EOF(1); LOF(1); Seek(1)
ub = 9: Put #1, 10, ub: Print "12:"; LOF(1);: For k = 5 To 10: Get #1, k, ub: Print ub;: Next: Print
Print "13:";: Put #1, 0, ub: Print Seek(1);: Get #1, -1, ub: Print Seek(1);: Get #1: Put #1, 2: Print LOF(1)
Print "14:";: Get #3, , ub: Put #3, 1, ub: Print
Call Put( #1, 1, i): Call Get(1, 1, l)
Print "15:"; Seek(1);: Put #1, 1, a(9): Print Seek(1);: Get #1, 1, a2(0): Print Seek(1)
Close #1
Open f For Output As #1: Print "16:";: Put #1, , ub: Get #1, , ub: Close #1
Open f For Input As #1: Get #1, , ub: Close #1
Open f For Binary Access Read As #1: Put #1, 1, ub: Get #1, 1, ub: Close #1
Open f For Binary Access Write As #1: Get #1, 1, ub: Put #1, 1, ub: Close #1
Print
Kill f
System
handler:
Print " <error"; Err; ">";
Resume Next
