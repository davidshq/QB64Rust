$Console:Only
' Whole values and parameters of a user type (spec language/arrays-and-types, "Whole user-type values"; spec
' language/procedures, "User-type parameters"): q = p copies every member, a STRING member as a string of its
' own, between variables, elements and members, and to itself. The copy is not guarded: an index outside raises
' error 9 and the array's first element stands in. For a type with a STRING member the two places are found again
' for every member, the target first. SWAP exchanges two places of one type. A parameter AS a TYPE is the place
' passed: a variable, an element, a member, the procedure's own parameter; an element with an index outside
' raises 9 and the procedure is not entered. Error cases go to a handler that prints ERR and resumes next.
Type num
    x As Long
    y As Integer
End Type
Type pt
    x As Long
    n As String
    f As String * 2
End Type
Type box
    k As num
    a As pt
    b As pt
End Type
On Error GoTo h
Dim p As pt, q As pt, a(3) As pt, bx As box, bs(2) As box, m As num, ma(2) As num
p.x = 5: p.n = "a": p.f = "ff"
q = p
p.x = 6: p.n = "b"
Print "1. q = p, then p changed:"; q.x; q.n; " "; q.f
a(2).x = 9: a(2).n = "el"
q = a(2)
Print "2. q = a(2):"; q.x; q.n
a(1) = p
Print "3. a(1) = p:"; a(1).x; a(1).n
a(3) = a(1)
a(1).n = "changed"
Print "4. a(3) = a(1), then a(1) changed:"; a(3).x; a(3).n
p = p
Print "5. p = p:"; p.x; p.n
bx.a = p
bx.b = bx.a
bx.a.n = "first"
Print "6. members: "; bx.a.n; " "; bx.b.n; bx.b.x
bs(1) = bx
bs(2).a = bs(1).b
Print "7. an element of a nested type, and a member of one: "; bs(1).a.n; " "; bs(2).a.n
a(0).x = 100: a(0).n = "zero"
Print "8. a(9) = p:";
a(9) = p
Print a(0).x; a(0).n
a(0).x = 100: a(0).n = "zero"
Print "9. q = a(9):";
q = a(9)
Print q.x; q.n
m.x = 1: m.y = 2
ma(1) = m
m.x = 3
Print "10. a type without a STRING:"; ma(1).x; ma(1).y; m.x
Print "11. ma(one(1)) = ma(one(2)):";
ma(one(1)) = ma(one(2))
Print
Print "12. a(one(3)) = a(one(4)), three members:";
a(one(3)) = a(one(4))
Print
Print "13. bs(one(5)) = bs(one(6)), seven parts:";
bs(one(5)) = bs(one(6))
Print
Swap p, q
Print "14. SWAP p, q:"; p.x; p.n; q.x; q.n
Swap a(1), a(2)
Print "15. SWAP a(1), a(2): "; a(1).n; " "; a(2).n
q.x = 0: q.n = "": a(2).x = 0: a(2).n = "": bx.a.x = 0: bx.a.n = "": a(0).x = 0
move q
move a(2)
move bx.a
move bs(2).b
Print "16. a variable, an element, a member:"; q.x; q.n; a(2).x; a(2).n; bx.a.x; bx.a.n; bs(2).b.x
Print "17. a FUNCTION:"; twice(q); q.x
Print "18. move a(9):";
move a(9)
Print a(0).x
passon q
Print "19. passed on to a second SUB:"; q.x; q.n
copies q
Print "20. copied out of and into a parameter:"; q.x; q.n
bump bs(1).k
bump ma(2)
Print "21. a parameter of a type without a STRING:"; bs(1).k.x; ma(2).x
System
h:
Print " [error"; Err; "]";
Resume Next

Sub move (p As pt)
    p.x = p.x + 1
    p.n = p.n + "m"
End Sub
Function twice (p As pt)
    twice = p.x * 2
    p.x = p.x + 10
End Function
Sub passon (p As pt)
    move p
End Sub
Sub copies (p As pt)
    Dim l As pt
    l = p
    l.x = l.x + 100: l.n = l.n + "c"
    p = l
End Sub
Sub bump (m As num)
    m.x = m.x + 1
End Sub
Function one (n As Long)
    Print n;
    one = 1
End Function
