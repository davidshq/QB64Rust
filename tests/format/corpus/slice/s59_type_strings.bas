$Console:Only
' STRING members (spec language/arrays-and-types, "User types": the two "String member" scenarios; "ERASE":
' "Array of a user type with a string member"): a member AS STRING is a string of its own in every value of the
' type: a variable, a nested member, each element of a static and of a dynamic array, a local (empty on each
' call) and a STATIC variable (kept). ERASE of a static array empties it, REDIM _PRESERVE keeps it, REDIM and
' ERASE of a dynamic array give it up. It is stored, read, compared and passed like any string. Error cases go
' to a handler that prints ERR and resumes next.
Type ts
    n As String
    v As Long
    f As String * 2
End Type
Type outer
    k As Integer
    i As ts
    t As String
End Type
On Error GoTo h
Dim p As ts, q As ts, o As outer, a(2) As ts, i As Long
p.n = "abc": p.n = p.n + "!": Print "1. "; p.n; Len(p.n)
Print "2. the members beside it:"; p.v; Asc(p.f); " ["; q.n; "]"
o.i.n = "in": o.t = "out": o.k = 7
Print "3. nested: "; o.i.n; " "; o.t; o.k; Len(o.i.n)
a(1).n = "one": a(2).n = "two": a(1).v = 1
Print "4. a static array: ["; a(0).n; "] "; a(1).n; " "; a(2).n
i = 2
a(i).n = a(i - 1).n + "+"
Print "5. from element to element: "; a(2).n; " "; a(1).n
Print "6. a read with an index outside:";
Print " ["; a(9).n; "]";
a(0).n = a(1).n + a(2).n
Print " "; a(0).n
Erase a
Print "7. after ERASE: ["; a(1).n; "]"; a(1).v; UBound(a)
ReDim d(1) As outer
d(1).i.n = "keep": d(1).t = "too": d(1).k = 5
ReDim _Preserve d(3) As outer
Print "8. REDIM _PRESERVE: "; d(1).i.n; " "; d(1).t; d(1).k; " ["; d(3).t; "]"
ReDim _Preserve d(0) As outer
Print "9. REDIM _PRESERVE to fewer elements: ["; d(0).t; "]"; UBound(d)
ReDim d(2) As outer
Print "10. REDIM: ["; d(1).i.n; "]"; d(1).k
d(1).t = "x"
Erase d
Print "11. ERASE of the dynamic array:";
Print UBound(d);
Print
Print "12. a local, twice: ";
fresh
fresh
Print
Print "13. a STATIC one, twice: ";
keep
keep
Print
change p.n
Print "14. passed by reference: "; p.n
p.n = String$(1000, "z"): Print "15. a long string and an empty one:"; Len(p.n);
p.n = "": Print Len(p.n)
Read p.n, q.n$
Print "16. READ, and the member named with its suffix: "; p.n; " "; q.n$
Mid$(p.n, 2, 1) = "#"
Swap p.n, q.n
Print "17. MID$ and SWAP: "; p.n; " "; q.n
If q.n = "d#ta" And p.n > q.n Then Print "18. compared"
Print "19. a local array, twice: ";
locals 2
locals 3
Print
System
Data data,more
h:
Print " [error"; Err; "]";
Resume Next

Sub fresh
    Dim l As ts
    Print "["; l.n; "]";
    l.n = "x"
End Sub
Sub keep
    Static k As ts
    k.n = k.n + "k"
    Print k.n; " ";
End Sub
Sub change (s As String)
    s = s + "?"
End Sub
Sub locals (n As Long)
    Dim la(n) As ts
    Print "["; la(n).n; "]";
    la(n).n = "y"
    Print la(n).n; UBound(la);
End Sub
