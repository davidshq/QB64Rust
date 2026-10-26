$Console:Only
$Unstable:TypeFields
' Slice (m3-typefields task 5.3): PUT and GET of a whole array whose type has _DYNAMIC member arrays
' (verification\v35_e 5.-6b, v35_x55, v35_x65): the declared layout in the file, a member with other bounds (PUT
' raises error 9 and writes nothing), a file open FOR RANDOM (the whole array is one record), nested owner types,
' a two-dimensional array with a lower bound that is not 0. Where the new compiler differs on purpose
' (DIVERGENCES.md D-019: GET into a member with other bounds raises error 9 and keeps the member as it was; the old
' compiler writes the declared bytes into its smaller block) the lines were recorded from a program that, in place
' of that GET, gives the member its declared bounds, reads, raises ERROR 9 and puts the member's bounds and element
' back.
Type t
    a As Integer
    v(1) _Dynamic As Long
    b As Integer
End Type
Type r
    n As Integer
    v(1) _Dynamic As Long
End Type
Type pt
    x As Integer
    y As Integer
End Type
Type inner
    c As _Byte
    w(0 TO 1) _Dynamic As Integer
End Type
Type outer
    a As Integer
    i As inner
    j(1) As inner
    d(1 TO 2) _Dynamic As inner
    q(1) _Dynamic As pt
    b As _Byte
End Type
On Error GoTo h
Dim n As Long, i As Long, k As Long
Dim bytes As String
ReDim p(0 To 1) As t
p(0).a = 1: p(0).b = 2: ReDim p(0).v(0 TO 2) AS LONG: p(0).v(0) = &H11111111: p(0).v(1) = &H22222222: p(0).v(2) = &H33333333
p(1).a = 3: p(1).b = 4: p(1).v(1) = &H44444444
Open "s118.bin" For Binary As #1
Put #1, 1, p()
n = LOF(1)
Close #1
Print "1. PUT p() with p(0).v 0 TO 2, declared 0 TO 1: LOF ="; n
ReDim _Preserve p(0).v(0 TO 1) AS LONG
Open "s118.bin" For Binary As #1
Put #1, 1, p()
n = LOF(1)
Close #1
Print "2. PUT p() with the declared bounds: LOF ="; n
Open "s118.bin" For Binary As #1
bytes = Space$(n): Get #1, 1, bytes
Close #1
GoSub showbytes
ReDim q(0 To 1) As t
ReDim q(0).v(5 TO 5) AS LONG: q(0).v(5) = 5
Open "s118.bin" For Binary As #1
Get #1, 1, q()
Close #1
Print "3. GET q() with q(0).v 5 TO 5:"
GoSub showq
ReDim q(0).v(0 TO 1) AS LONG
Open "s118.bin" For Binary As #1
Get #1, 1, q()
Close #1
Print "4. GET q() after q(0).v has its declared bounds again:"
GoSub showq
Kill "s118.bin"
ReDim rr(1) As r
rr(1).n = 5: rr(1).v(1) = 7
Open "s118.tmp" For Random As #1 Len = 10
Put #1, 1, rr()
Print "5. RANDOM, LEN = 10:"; LOF(1)
Close #1
Open "s118.tmp" For Random As #1 Len = 20
Put #1, 1, rr()
Print "6. RANDOM, LEN = 20:"; LOF(1)
Close #1
ReDim rr(1) As r
Open "s118.tmp" For Random As #1 Len = 20
Get #1, 1, rr()
Close #1
Print "7. GET from it:"; rr(1).n; rr(1).v(0); rr(1).v(1)
Kill "s118.tmp"
ReDim o(1 To 2, 0 To 0) As outer
For i = 1 To 2
    o(i, 0).a = i * 100: o(i, 0).b = i
    o(i, 0).i.c = i + 10: o(i, 0).i.w(0) = i * 1000 + 1: o(i, 0).i.w(1) = i * 1000 + 2
    For k = 0 To 1
        o(i, 0).j(k).c = i * 20 + k: o(i, 0).j(k).w(0) = i * 300 + k: o(i, 0).j(k).w(1) = i * 300 + k + 10
    Next
    For k = 1 To 2
        o(i, 0).d(k).c = i * 40 + k: o(i, 0).d(k).w(1) = i * 500 + k
    Next
    o(i, 0).q(0).x = i * 7: o(i, 0).q(1).y = i * 9
Next
Open "s118.bin" For Binary As #1
Put #1, 1, o()
n = LOF(1)
Close #1
Print "8. PUT o() of nested owner types: LOF ="; n
Open "s118.bin" For Binary As #1
bytes = Space$(n): Get #1, 1, bytes
Close #1
GoSub showbytes
ReDim g(1 To 2, 0 To 0) As outer
Open "s118.bin" For Binary As #1
Get #1, 1, g()
Close #1
Print "9. GET g():"
GoSub showg
ReDim g(1 To 2, 0 To 0) As outer
ReDim g(2, 0).d(1).w(0 TO 0) AS INTEGER
g(2, 0).d(1).w(0) = 77
Open "s118.bin" For Binary As #1
Get #1, 1, g()
Close #1
Print "10. GET with g(2,0).d(1).w 0 TO 0:"; LBound(g(2, 0).d(1).w); UBound(g(2, 0).d(1).w); g(2, 0).d(1).w(0)
Print "    g(2,0).a, d(1).c, d(2).w(1), q(1).y, b:"; g(2, 0).a; g(2, 0).d(1).c; g(2, 0).d(2).w(1); g(2, 0).q(1).y; g(2, 0).b
Kill "s118.bin"
ReDim _Preserve o(1, 0).j(1).w(0 TO 2) AS INTEGER
Open "s118.bin" For Binary As #1
Put #1, 1, o()
n = LOF(1)
Close #1
Print "11. PUT with o(1,0).j(1).w 0 TO 2: LOF ="; n
Kill "s118.bin"
System
showbytes:
Print "   bytes:";
For k = 1 To Len(bytes): Print Right$("0" + Hex$(Asc(bytes, k)), 2);: Next: Print
Return
showq:
For i = 0 To 1
    Print "   q("; i; "): a, b ="; q(i).a; q(i).b; " v:"; LBound(q(i).v); UBound(q(i).v);
    For n = LBound(q(i).v) To UBound(q(i).v): Print q(i).v(n);: Next: Print
Next
Return
showg:
For i = 1 To 2
    Print g(i, 0).a; g(i, 0).b; g(i, 0).i.c; g(i, 0).i.w(0); g(i, 0).i.w(1);
    For k = 0 To 1: Print g(i, 0).j(k).c; g(i, 0).j(k).w(0); g(i, 0).j(k).w(1);: Next
    For k = 1 To 2: Print g(i, 0).d(k).c; g(i, 0).d(k).w(0); g(i, 0).d(k).w(1);: Next
    Print g(i, 0).q(0).x; g(i, 0).q(0).y; g(i, 0).q(1).x; g(i, 0).q(1).y
Next
Return
h:
Print "[error"; Err; "]";
Resume Next
