$CONSOLE:ONLY
' Whole values and parameters of a user type (spec language/arrays-and-types, "Whole user-type values"; spec
' language/procedures, "User-type parameters"): q = p copies every member, a STRING member as a string of its
' own, between variables, elements and members, and to itself. The copy is not guarded: an index outside raises
' error 9 and the array's first element stands in. For a type with a STRING member the two places are found again
' for every member, the target first. SWAP exchanges two places of one type. A parameter AS a TYPE is the place
' passed: a variable, an element, a member, the procedure's own parameter; an element with an index outside
' raises 9 and the procedure is not entered. Error cases go to a handler that prints ERR and resumes next.
TYPE num
    x AS LONG
    y AS INTEGER
END TYPE
TYPE pt
    x AS LONG
    n AS STRING
    f AS STRING * 2
END TYPE
TYPE box
    k AS num
    a AS pt
    b AS pt
END TYPE
ON ERROR GOTO h
DIM p AS pt, q AS pt, a(3) AS pt, bx AS box, bs(2) AS box, m AS num, ma(2) AS num
p.x = 5: p.n = "a": p.f = "ff"
q = p
p.x = 6: p.n = "b"
PRINT "1. q = p, then p changed:"; q.x; q.n; " "; q.f
a(2).x = 9: a(2).n = "el"
q = a(2)
PRINT "2. q = a(2):"; q.x; q.n
a(1) = p
PRINT "3. a(1) = p:"; a(1).x; a(1).n
a(3) = a(1)
a(1).n = "changed"
PRINT "4. a(3) = a(1), then a(1) changed:"; a(3).x; a(3).n
p = p
PRINT "5. p = p:"; p.x; p.n
bx.a = p
bx.b = bx.a
bx.a.n = "first"
PRINT "6. members: "; bx.a.n; " "; bx.b.n; bx.b.x
bs(1) = bx
bs(2).a = bs(1).b
PRINT "7. an element of a nested type, and a member of one: "; bs(1).a.n; " "; bs(2).a.n
a(0).x = 100: a(0).n = "zero"
PRINT "8. a(9) = p:";
a(9) = p
PRINT a(0).x; a(0).n
a(0).x = 100: a(0).n = "zero"
PRINT "9. q = a(9):";
q = a(9)
PRINT q.x; q.n
m.x = 1: m.y = 2
ma(1) = m
m.x = 3
PRINT "10. a type without a STRING:"; ma(1).x; ma(1).y; m.x
PRINT "11. ma(one(1)) = ma(one(2)):";
ma(one(1)) = ma(one(2))
PRINT
PRINT "12. a(one(3)) = a(one(4)), three members:";
a(one(3)) = a(one(4))
PRINT
PRINT "13. bs(one(5)) = bs(one(6)), seven parts:";
bs(one(5)) = bs(one(6))
PRINT
SWAP p, q
PRINT "14. SWAP p, q:"; p.x; p.n; q.x; q.n
SWAP a(1), a(2)
PRINT "15. SWAP a(1), a(2): "; a(1).n; " "; a(2).n
q.x = 0: q.n = "": a(2).x = 0: a(2).n = "": bx.a.x = 0: bx.a.n = "": a(0).x = 0
move q
move a(2)
move bx.a
move bs(2).b
PRINT "16. a variable, an element, a member:"; q.x; q.n; a(2).x; a(2).n; bx.a.x; bx.a.n; bs(2).b.x
PRINT "17. a FUNCTION:"; twice(q); q.x
PRINT "18. move a(9):";
move a(9)
PRINT a(0).x
passon q
PRINT "19. passed on to a second SUB:"; q.x; q.n
copies q
PRINT "20. copied out of and into a parameter:"; q.x; q.n
bump bs(1).k
bump ma(2)
PRINT "21. a parameter of a type without a STRING:"; bs(1).k.x; ma(2).x
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB move (p AS pt)
    p.x = p.x + 1
    p.n = p.n + "m"
END SUB
FUNCTION twice (p AS pt)
    twice = p.x * 2
    p.x = p.x + 10
END FUNCTION
SUB passon (p AS pt)
    move p
END SUB
SUB copies (p AS pt)
    DIM l AS pt
    l = p
    l.x = l.x + 100: l.n = l.n + "c"
    p = l
END SUB
SUB bump (m AS num)
    m.x = m.x + 1
END SUB
FUNCTION one (n AS LONG)
    PRINT n;
    one = 1
END FUNCTION
