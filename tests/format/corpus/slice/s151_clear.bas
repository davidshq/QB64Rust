$Console:Only
' Slice (m4-builtins-param-types task 4.2): what CLEAR resets (verification/v49_e): variables of each kind, STATIC
' and SHARED ones, static arrays cleared and dynamic ones released, the DATA position, open files; CONST kept.
' CALL CLEAR and CLEAR with arguments; RETURN after CLEAR in a GOSUB is error 3.
Type ut
    s As String
    n As Long
    f As String * 3
End Type
Common Shared cs As Long
Dim Shared sh As Long, shs As String
Dim a As Integer, s As String, fs As String * 4, u As ut, d As Double, q As _Unsigned _Integer64
Dim st(3) As Long, sts(2) As String, stu(1) As ut, b(1 To 2) As _Byte
ReDim dy(4) As Long, dys(2) As String
Const K = 7
a = 5: s = "x": fs = "abcd": u.s = "q": u.n = 3: u.f = "zz": cs = 1: sh = 2: shs = "y": d = 1.5: q = 9
st(1) = 9: sts(1) = "w": stu(1).s = "v": stu(1).n = 8: dy(2) = 4: dys(1) = "d": b(2) = 3: implicit = 6
counter
counter
Read r: Print "read"; r
Open "s151.tmp" For Output As #1
On Error GoTo h
Clear
Print a; Len(s); Len(fs); Asc(fs); Len(u.s); u.n; Asc(u.f); cs; sh; Len(shs); d; q; K
Print st(1); Len(sts(1)); Len(stu(1).s); stu(1).n; b(2); UBound(st); implicit
Print "freefile"; FreeFile
Read r: Print "read"; r
counter
On Error GoTo h
Print "dynamic:"; UBound(dy)
Print "strings:"; UBound(dys)
ReDim dy(2) As Long: Print "again:"; UBound(dy); dy(1)
Kill "s151.tmp"
z = 1: Call Clear: Print "z"; z
z = 2: Clear z + 1, , 100: Print "z"; z
z = 3: Clear , 2: Print "z"; z
On Error GoTo h
GoSub g
Print "done"
System
g:
Clear
On Error GoTo h
Return
Print "after return"
System
h:
Print "error"; Err
Resume Next
Data 1,2
Sub counter
    Static c
    c = c + 1
    Print "c="; c
End Sub
