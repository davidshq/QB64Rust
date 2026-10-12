$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 9): LBOUND, UBOUND and LEN of a member array whose base has
' indexes: whether the indexes are evaluated, and what one outside its bounds does.
TYPE t1
    id AS LONG
    v(2 TO 5) AS LONG
END TYPE
TYPE t2
    AS INTEGER q(1 TO 2, 3), r
END TYPE
TYPE t3
    AS t2 n(1), m
END TYPE
ON ERROR GOTO h
DIM a(3) AS t1, x(1) AS t3, t AS t1
PRINT "1. UBOUND(a(one(1)).v):";
PRINT UBOUND(a(one(1)).v)
PRINT "2. LBOUND(a(9).v):";
PRINT LBOUND(a(9).v)
PRINT "3. UBOUND(x(one(1)).n(one(2)).q, one(2)):";
PRINT UBOUND(x(one(1)).n(one(2)).q, one(2))
PRINT "4. UBOUND(x(1).n(9).q):";
PRINT UBOUND(x(1).n(9).q)
PRINT "5. LBOUND(t.v, 0), LBOUND(t.v, 2), UBOUND(x(0).n(0).q, 3):";
PRINT LBOUND(t.v, 0);
PRINT LBOUND(t.v, 2);
PRINT UBOUND(x(0).n(0).q, 3)
PRINT "6. LBOUND(t.v, 1.5), UBOUND(x(0).n(0).q, 2.5), LBOUND(x(0).n(0).q, 1):";
PRINT LBOUND(t.v, 1.5); UBOUND(x(0).n(0).q, 2.5); LBOUND(x(0).n(0).q, 1)
PRINT "7. LEN(a(one(1)).v(one(2))), LEN(a(9).v(9)):";
PRINT LEN(a(one(1)).v(one(2))); LEN(a(9).v(9))
PRINT "8. UBOUND(t.v()), UBOUND(a(1).v()), UBOUND(t.v) / 2:";
PRINT UBOUND(t.v()); UBOUND(a(1).v()); UBOUND(t.v) / 2
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

FUNCTION one (n AS LONG)
    PRINT n;
    one = 1
END FUNCTION
