$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 9): elements of member arrays as arguments: the outer index
' outside, a STRING and a STRING * n element to a STRING parameter, an element of a user type to a parameter of
' that type, whole values between elements of a member array, and SWAP of elements.
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
    s(0 TO 1) AS STRING
    f(0 TO 1) AS STRING * 3
END TYPE
TYPE t3
    AS t1 n(1), m
END TYPE
ON ERROR GOTO h
DIM a(3) AS t1, x AS t3, t AS t1, q AS t1
a(0).v(1) = 100: a(1).v(0) = 200
PRINT "1. bump a(9).v(1):";
bump a(9).v(1)
PRINT ":"; a(0).v(1)
PRINT "2. bump a(1).v(9):";
bump a(1).v(9)
PRINT ":"; a(1).v(0)
t.s(1) = "s": t.f(1) = "f"
adds t.s(1)
adds t.f(1)
PRINT "3. a STRING and a STRING * 3 element to a STRING parameter: ["; t.s(1); "]["; t.f(1); "]"
x.n(1).id = 5: x.n(1).v(2) = 6: x.n(1).s(1) = "n1"
show x.n(1)
PRINT "4. an element of a user type to a parameter of the type:"; x.n(1).id
x.n(0) = x.n(1)
q = x.n(0)
PRINT "5. whole values, x.n(0) = x.n(1) and q = x.n(0):"; x.n(0).v(2); q.id; q.v(2); "["; q.s(1); "]"
PRINT "6. show x.n(9):";
show x.n(9)
PRINT
t.v(0) = 1: t.v(1) = 2: a(1).v(0) = 3: a(2).v(1) = 4
SWAP t.v(0), t.v(1)
SWAP a(1).v(0), a(2).v(1)
PRINT "7. SWAP of two elements, of one variable and of two array elements:"; t.v(0); t.v(1); a(1).v(0); a(2).v(1)
PRINT "8. SWAP t.v(9), t.v(1) (t.v(0) is 2, t.v(1) is 1):";
SWAP t.v(9), t.v(1)
PRINT ":"; t.v(0); t.v(1)
t.s(0) = "a": t.s(1) = "b": x.n(0).id = 1: x.n(1).id = 2
SWAP t.s(0), t.s(1)
SWAP x.n(0), x.n(1)
PRINT "9. SWAP of two STRING elements and of two elements of a user type: "; t.s(0); t.s(1); x.n(0).id; x.n(1).id
a(1).id = 7: a(2).id = 8
SWAP a(1).id, a(2).id
PRINT "10. SWAP of a member of two array elements:"; a(1).id; a(2).id
PRINT "11. the order in SWAP a(one(1)).v(one(2)), a(one(3)).v(one(4)):";
SWAP a(one(1)).v(one(2)), a(one(3)).v(one(4))
PRINT
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB bump (n AS LONG)
    PRINT " in bump";
    n = n + 1
END SUB

SUB adds (s AS STRING)
    s = s + "!"
END SUB

SUB show (p AS t1)
    PRINT " in show"; p.id; p.v(2); "["; p.s(1); "]";
    p.id = p.id + 1
END SUB

FUNCTION one (n AS LONG)
    PRINT n;
    one = 1
END FUNCTION
