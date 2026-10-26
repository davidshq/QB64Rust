$Console:Only
' A DIM or REDIM of several dynamic arrays is one statement (spec language/arrays-and-types, "Dynamic arrays"):
' run a second time it raises error 10 once, an array of the list that is not allocated is still allocated after
' the error, and RESUME NEXT goes on after the whole list. Error cases go to a handler that prints ERR and
' resumes next.
On Error GoTo h
Dim i As Long, n As Long
Print "1. DIM du(i + 1), dv(i + 3), run twice:";
For i = 1 To 2
    Dim du(i + 1), dv(i + 3)
Next
Print UBound(du); UBound(dv)
Print "2. the second is new in the second round:";
ReDim dy(1)
For i = 1 To 2
    Erase dy
    Dim dx(i + 1), dy(i + 3)
Next
Print UBound(dx); UBound(dy)
Print "3. a statement after it on the line:";
For i = 1 To 2
    Dim dz(i + 1), ea(i + 3): n = n + 1
Next
Print n
Print "4. REDIM of a static and a dynamic array:";
Dim st(3)
ReDim st(5), eb(4)
Print UBound(st); UBound(eb)
Print "5. three arrays, the second allocated before:";
ReDim m2(1)
Dim m1(i), m2(i + 1), m3(i + 2)
Print UBound(m1); UBound(m2); UBound(m3)
Print "6. in a SUB, DIM a(3), b(n) run twice in a call:";
local 4
Print "7. with strings:";
For i = 1 To 2
    Dim s1(i) As String, s2(i) As String
    s1(1) = s1(1) + "a": s2(1) = s2(1) + "b"
Next
Print " "; s1(1); s2(1)
System
h:
Print " [error"; Err; "]";
Resume Next

Sub local (n As Long)
    Dim r As Long, k As Long
    For r = 1 To 2
        Dim a(3) As Long, b(n + r) As Long: k = k + 1
    Next
    Print UBound(a); UBound(b); k
End Sub
