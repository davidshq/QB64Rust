$Console:Only
$Unstable:TypeFields
' Slice (m3-measured-copies): what the old compiler copies after an error in PUT, GET and a whole-array assignment
' of types with _DYNAMIC member arrays (verification\v38_a, v38_b, v38_d). GET unpacks its own buffer after a
' read that raised an error; a member with other bounds or another number
' of dimensions gets its declared bytes from GET when its block holds them; arrays of such a type with other
' bounds copy the target's number of elements. Left out, as the new compiler differs on purpose: a member block
' with fewer elements (DIVERGENCES.md D-019), a source with fewer elements or not allocated (D-022), a failed
' GET whose buffer was never written (D-021: the old program reads memory nobody wrote, other values each run).
Type t
    a As Integer
    v(1) _Dynamic As Long
End Type
Type u
    a As Integer
    w(1, 1) _Dynamic As Long
End Type
Type own
    v(1) _Dynamic As Long
    k As Long
End Type
On Error GoTo h
Dim i As Long, j As Long, pass As Long

' --- GET after a failed read (v38_a)
ReDim p(1) As t
ReDim q(1) As t
For i = 0 To 1
    q(i).a = 10 + i: q(i).v(0) = 100 + i: q(i).v(1) = 200 + i
Next
Open "s126.bin" For Binary As #1
Put #1, 1, q()
Close #1
For pass = 1 To 2
    If pass = 1 Then Open "s126.bin" For Binary As #3
    Print "2."; pass; ":";
    GoSub fill
    Get #3, 1, p()
    GoSub show
    If pass = 1 Then Close #3
Next
Open "s126.bin" For Binary As #5
Print "4. past the end:";
GoSub fill
Get #5, 15, p()
GoSub show
Close #5

' --- a member with other bounds or dimensions (v38_b)
ReDim r(1) As u
For i = 0 To 1
    r(i).a = 20 + i: r(i).w(0, 0) = 300 + i: r(i).w(1, 0) = 310 + i: r(i).w(0, 1) = 320 + i: r(i).w(1, 1) = 330 + i
Next
Open "s126u.bin" For Binary As #1: Put #1, 1, r(): Close #1
ReDim p(1) As t
ReDim p(1).v(0 TO 1, 0 TO 0) AS LONG
p(1).v(0, 0) = 7: p(1).v(1, 0) = 8
Print "5. PUT:";
Open "s126x.bin" For Output As #1: Close #1: Open "s126x.bin" For Binary As #1: Put #1, 1, p(): Print LOF(1): Close #1
Print "5. GET:";
Open "s126.bin" For Binary As #1: Get #1, 1, p(): Close #1
Print p(0).a; p(0).v(0); p(0).v(1); p(1).a; p(1).v(0, 0); p(1).v(1, 0); UBound(p(1).v, 1); UBound(p(1).v, 2)
ReDim p(1) As t
ReDim p(1).v(0 TO 1, 0 TO 1) AS LONG
p(1).v(0, 1) = 9: p(1).v(1, 1) = 6
Print "6. GET:";
Open "s126.bin" For Binary As #1: Get #1, 1, p(): Close #1
Print p(1).a; p(1).v(0, 0); p(1).v(1, 0); p(1).v(0, 1); p(1).v(1, 1)
ReDim s(1) As u
ReDim s(1).w(0 TO 3) AS LONG
Print "7. GET:";
Open "s126u.bin" For Binary As #1: Get #1, 1, s(): Close #1
Print s(0).a; s(0).w(0, 0); s(0).w(1, 1); s(1).a;
For j = 0 To 3: Print s(1).w(j);: Next
Print UBound(s(1).w, 1)
ReDim p(1) As t
ReDim p(1).v(5 TO 6) AS LONG
Print "8. GET:";
Open "s126.bin" For Binary As #1: Get #1, 1, p(): Close #1
Print p(1).a; p(1).v(5); p(1).v(6); LBound(p(1).v)
ReDim p(1) As t
ReDim p(1).v(0 TO 2) AS LONG
p(1).v(2) = 9
Print "9. GET:";
Open "s126.bin" For Binary As #1: Get #1, 1, p(): Close #1
Print p(1).a; p(1).v(0); p(1).v(1); p(1).v(2)
Kill "s126.bin": Kill "s126u.bin": Kill "s126x.bin"

' --- arrays of an owner type with other bounds (v38_d)
Dim so(1 To 2) As own
so(1).k = 4: so(2).k = 5: so(1).v(0) = 40: so(2).v(1) = 51
Dim d1(0 To 1) As own, d2(0 To 0) As own
Print "10.";: d1() = so(): Print d1(0).k; d1(1).k
Print "11.";: d2() = so(): Print d2(0).k
ReDim r4(5 To 6) As own, r5(0 To 0) As own
Print "12.";: r4() = so(): Print r4(5).k; r4(6).k; LBound(r4); UBound(r4)
Print "13.";: r5() = so(): Print r5(0).k
ReDim m(1 To 4, 0 To 0) As own, n(0 To 1, 0 To 1) As own
For i = 1 To 4: m(i, 0).k = 60 + i: Next
Print "14.";: n() = m(): Print n(0, 0).k; n(1, 0).k; n(0, 1).k; n(1, 1).k
Print "15.";: d1(0).v(0) = 99: Print d1(0).v(0); so(1).v(0); d1(1).v(1)
Print "ok"
System

fill:
For i = 0 To 1
    p(i).a = -1: p(i).v(0) = -2: p(i).v(1) = -3
Next
Return

show:
For i = 0 To 1
    Print p(i).a; p(i).v(0); p(i).v(1);
Next
Print
Return

h:
Print " [error"; Err; "]";
Resume Next
