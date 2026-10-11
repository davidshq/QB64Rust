$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a whole user-type value assigned: between variables, an element
' and a variable both ways, between elements, to itself, a nested member, with STRING members; the copy is its
' own; an index outside on either side; SWAP.
TYPE pt
    x AS LONG
    n AS STRING
    f AS STRING * 2
END TYPE
TYPE box
    a AS pt
    b AS pt
END TYPE
ON ERROR GOTO h
DIM p AS pt, q AS pt, a(3) AS pt, bx AS box
p.x = 5: p.n = "a": p.f = "ff"
q = p
p.x = 6: p.n = "b"
PRINT "1. q = p, then p changed: q.x, q.n, q.f"; q.x; q.n; " "; q.f
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
PRINT "6. members of a user type: bx.a.n, bx.b.n, bx.b.x "; bx.a.n; " "; bx.b.n; bx.b.x
q = bx.b
PRINT "7. q = bx.b:"; q.x; q.n
a(0).x = 100: a(0).n = "zero"
PRINT "8. a(9) = p (index outside):";
a(9) = p
PRINT " a(0).x, a(0).n"; a(0).x; a(0).n
a(0).x = 100: a(0).n = "zero"
PRINT "9. q = a(9):";
q = a(9)
PRINT " q.x, q.n"; q.x; q.n
SWAP p, q
PRINT "10. SWAP p, q: p.x, p.n, q.x, q.n"; p.x; p.n; q.x; q.n
SWAP a(1), a(2)
PRINT "11. SWAP a(1), a(2): "; a(1).n; " "; a(2).n

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
