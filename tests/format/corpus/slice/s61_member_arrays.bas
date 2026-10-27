$Console:Only
' Member arrays (spec language/arrays-and-types, "Member arrays", and the member-array scenario of "LBOUND and
' UBOUND"): a TYPE member declared as an array with constant bounds, in both declaration forms; its elements are
' part of each value of the type. An element is named through any chain of members and elements. A store
' evaluates a number before the indexes and a string after them. The bounds never change: REDIM with the
' declared bounds and ERASE clear, REDIM _PRESERVE does nothing. LBOUND and UBOUND give the member's bounds
' without finding the place. A whole member array is an array argument. Error cases go to a handler that prints
' ERR and resumes next. A store with an index outside its bounds stores nothing here and writes position 0 of
' that dimension in the old compiler (QB64PE-DIVERGENCES.md D-004): no line prints such a position after such a store.
Type below0
    w(3) As Long
End Type
Option Base 1
Type below1
    w(3) As Long
    e(0 TO 2) As Long
End Type
Option Base 0
Type t1
    id As Long
    v(0 TO 3) As Long
    tail As Integer
End Type
Type inner
    As Integer q(2), r
End Type
Type outer
    As inner n(1), m
End Type
Type wide
    g(1 TO 2, 3) As _Byte
    f(2) As String * 3
    d(1) As Double
    s(0 TO 1) As String
    n(1 TO 4) As Long
End Type
Type holder
    k As Long
    Values(0 TO 3) As Long
End Type
On Error GoTo h
Dim Shared z$
Dim t As t1, a(2) As t1, u As wide, ua(1) As wide, b0 As below0, b1 As below1, i As Long, k As Long
Dim item(1) As holder, p As wide, q As wide
ReDim x(0) As outer
t.v(2) = 7
Print "1. t.v(2) = 7:"; t.v(2); t.v(3)
x(0).n(1).q(2) = 77: x(0).m.r = 88
Print "2. a chain:"; x(0).n(1).q(2); x(0).m.r; x(0).n(0).q(2)
a(0).v(1) = 5
Print "3. each element has its own:"; a(0).v(1); a(1).v(1)
Print "4. t.v(4) = 1:";
t.v(4) = 1
Print
t.v(1) = 100: t.v(2) = 200: a(1).v(1) = 211: a(2).v(1) = 221
Print "5. stores with an index outside:";
t.v(9) = 5
a(9).v(1) = 6
a(1).v(9) = 7
Print t.v(1); t.v(2); a(1).v(1); a(2).v(1)
Print "6. a(one(1)).v(one(2)) = one(3):";
a(one(1)).v(one(2)) = one(3)
Print ":"; a(1).v(1)
Print "7. LEN of the variable, an element, a nested element and a type of two:"; Len(t); Len(t.v(1)); Len(x(0).n(1)); Len(x(0))
Print "8. OPTION BASE at the TYPE block:"; LBound(b0.w); UBound(b0.w); LBound(b1.w); UBound(b1.w); LBound(b1.e); Len(b0); Len(b1)
For i = 0 To 3: item(0).Values(i) = i + 1: item(1).Values(i) = i + 11: Next
item(0).k = 9
ReDim item(0).Values(0 TO 3)
Print "9. REDIM item(0).Values(0 TO 3):"; item(0).Values(0); item(0).Values(1); item(0).Values(2); item(0).Values(3); item(1).Values(0); item(1).Values(3); item(0).k
t.id = 1: t.tail = 9
For i = 0 To 3: t.v(i) = i + 1: Next
Erase t.v
Print "10. ERASE t.v:"; t.v(0); t.v(3); UBound(t.v); t.id; t.tail
Print "11. LBOUND and UBOUND:"; LBound(u.n); UBound(u.n); LBound(x(0).n); UBound(x(0).n(1).q); LBound(u.g, 1); UBound(u.g, 1); UBound(u.g, 2); UBound(t.v()); UBound(t.v) / 2
Print "12. the indexes of the place are not evaluated: UBOUND(a(one(1)).v), LBOUND(a(9).v), LEN(a(one(2)).v(9)):"; UBound(a(one(1)).v); LBound(a(9).v); Len(a(one(2)).v(9))
Print "13. a dimension outside: LBOUND(t.v, 0), UBOUND(t.v, 2), UBOUND(u.g, i) for i = 3:";
k = LBound(t.v, 0)
k = UBound(t.v, 2)
i = 3
k = UBound(u.g, i)
i = 2
Print UBound(u.g, i)
u.g(2, 3) = 100: u.g(1, 0) = -1: u.f(1) = "abcdef": u.d(1) = 1# / 3: u.s(1) = "str"
Print "14. two dimensions, STRING * 3, DOUBLE, STRING:"; u.g(2, 3); u.g(1, 0); u.g(2, 0); "["; u.f(1); "]"; Asc(u.f(2)); u.d(1); u.s(1); Len(u.s(0))
Print "15. u.g(one(1), one(2)) = one(3):";
u.g(one(1), one(2)) = one(3)
Print ":"; u.g(1, 1)
Print "16. x(zero(1)).n(one(2)).q(one(3)) = one(4):";
x(zero(1)).n(one(2)).q(one(3)) = one(4)
Print ":"; x(0).n(1).q(1)
z$ = "before"
u.s(setz(1)) = z$
z$ = "before"
ua(setz(1)).s(setz(0)) = z$ + "!"
Print "17. a string is evaluated after the indexes: "; u.s(1); " "; ua(1).s(0)
t.v(1) = 11: a(1).v(0) = 10
Print "18. reads with an index outside:";
k = t.v(9)
k = a(9).v(1)
Print u.s(9);
Print
Print "19. a float index and a float stored:";
t.v(1.5) = 2.5
Print t.v(2)
Print "20. a local variable, twice:";
fresh
fresh
Print
t.v(2) = 1: a(1).v(2) = 1
bump t.v(2)
bump a(1).v(2)
bump (t.v(2))
bumpi t.v(2)
Print "21. by reference, in parentheses, to an INTEGER parameter:"; t.v(2); a(1).v(2)
Print "22. bump a(9).v(1) and bump t.v(9):";
bump a(9).v(1)
bump t.v(9)
Print
u.s(1) = "s": u.f(1) = "f"
adds u.s(1)
adds u.f(1)
Print "23. a STRING and a STRING * 3 element to a STRING parameter: ["; u.s(1); "]["; u.f(1); "]"
x(0).n(1).r = 5: x(0).n(1).q(2) = 6
show x(0).n(1)
x(0).n(0) = x(0).n(1)
Print "24. an element of a user type as an argument and as a whole value:"; x(0).n(1).r; x(0).n(0).r; x(0).n(0).q(2)
t.v(0) = 1: t.v(1) = 2: a(1).v(0) = 3: a(2).v(1) = 4: u.s(0) = "a": u.s(1) = "b": x(0).n(0).r = 1
Swap t.v(0), t.v(1)
Swap a(1).v(0), a(2).v(1)
Swap u.s(0), u.s(1)
Swap x(0).n(0), x(0).n(1)
Print "25. SWAP:"; t.v(0); t.v(1); a(1).v(0); a(2).v(1); u.s(0); u.s(1); x(0).n(0).r; x(0).n(1).r
Print "26. SWAP a(one(1)).v(one(2)), a(one(3)).v(one(4)):";
Swap a(one(1)).v(one(2)), a(one(3)).v(one(4))
Print
t.id = 1: t.tail = 9
fill t.v()
fill a(1).v()
Print "27. a whole member array as an argument:"; t.v(0); t.v(3); t.id; t.tail; a(1).v(3); a(0).v(3); a(2).v(3)
Print "28. two dimensions, the bounds in the SUB:";
grid u.g()
Print ":"; u.g(2, 2)
u.s(0) = "": u.s(1) = "x"
names u.s()
Print "29. a STRING member array: ["; u.s(0); "]["; u.s(1); "]"
Print "30. ERASE and REDIM in the SUB:";
wipe t.v()
again a(1).v()
Print ":"; t.v(0); t.v(3); t.id; t.tail; a(1).v(3); UBound(a(1).v)
Print "31. fill a(9).v() and fill a(zero(1)).v():";
a(0).v(3) = 0
fill a(9).v()
Print a(0).v(3);
fill a(zero(1)).v()
Print a(0).v(3)
p.g(2, 3) = 1: p.f(2) = "fff": p.d(1) = 2.5: p.s(0) = "s0": p.s(1) = "s1": p.n(4) = 44
q = p
p.s(1) = "changed": p.n(4) = 0
Print "32. q = p with member arrays:"; q.g(2, 3); q.f(2); q.d(1); q.s(0); q.s(1); q.n(4)
Print "33. ua(one(1)) = ua(one(2)), six parts:";
ua(one(1)) = ua(one(2))
Print
ReDim a(2).v(3) AS LONG
ReDim x(0).n(1).q(0 TO 2)
Print "34. REDIM with AS and through a chain:"; a(2).v(1); x(0).n(1).q(2); x(0).n(1).r
t.v(1) = 8
ReDim _Preserve t.v(0 TO 3)
ReDim _Preserve a(one(1)).v(0 TO 3)
ReDim _Preserve a(9).v(0 TO 3)
Print "35. REDIM _PRESERVE does nothing:"; t.v(1); UBound(t.v)
a(0).v(1) = 1: a(1).v(1) = 2: a(2).v(1) = 3
Print "36. ERASE a(one(1)).v, then a(9).v:";
Erase a(one(1)).v
Erase a(9).v
Print ":"; a(0).v(1); a(1).v(1); a(2).v(1)
u.s(1) = "keep": ua(1).s(1) = "gone": u.f(1) = "fff": u.d(1) = 1
Erase ua(1).s
Erase u.f, u.d
Print "37. ERASE of STRING, STRING * 3 and DOUBLE member arrays: ["; ua(1).s(1); "]["; u.s(1); "]"; Asc(u.f(1)); u.d(1)
System
h:
Print " [error"; Err; "]";
Resume Next

Sub fresh
    Dim l As t1
    Print l.v(2);
    l.v(2) = 5
End Sub
Sub bump (n As Long)
    Print " in bump";
    n = n + 1
End Sub
Sub bumpi (n As Integer)
    n = n + 1
End Sub
Sub adds (s As String)
    s = s + "!"
End Sub
Sub show (p As inner)
    p.r = p.r + 1
End Sub
Sub fill (b() As Long)
    Dim i As Long
    For i = LBound(b) To UBound(b): b(i) = i * 2 + 1: Next
End Sub
Sub grid (b() As _Byte)
    Print LBound(b, 1); UBound(b, 1); LBound(b, 2); UBound(b, 2);
    b(2, 2) = 22
End Sub
Sub names (b() As String)
    b(0) = "zero": b(1) = b(1) + "!"
End Sub
Sub wipe (b() As Long)
    Erase b
End Sub
Sub again (b() As Long)
    ReDim b(0 To 3) As Long
    Print " after REDIM";
End Sub
Function one (n As Long)
    Print n;
    one = 1
End Function
Function zero (n As Long)
    Print n;
    zero = 0
End Function
Function setz (n As Long)
    z$ = "after"
    setz = n
End Function
