$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): TYPE member arrays: both declaration forms, an element through a
' variable, through an array element and through a chain, LEN of the types, LBOUND and UBOUND, first values,
' the member after the array, two dimensions, a fixed-length string array, an index outside, a local variable
' on each call.
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
    tail AS INTEGER
END TYPE
TYPE t2
    AS INTEGER q(2), r
END TYPE
TYPE t3
    AS t2 n(1), m
END TYPE

TYPE t4
    g(1 TO 2, 3) AS _BYTE
    s(2) AS STRING * 3
    d(1) AS DOUBLE
END TYPE
ON ERROR GOTO h
DIM t AS t1
t.v(2) = 7
PRINT "1. t.v(2) = 7: t.v(2), t.v(3), LEN(t)"; t.v(2); t.v(3); LEN(t)
PRINT "2. LBOUND(t.v), UBOUND(t.v), UBOUND(t.v, 1):"; LBOUND(t.v); UBOUND(t.v); UBOUND(t.v, 1)
t.id = 1: t.tail = 9: t.v(0) = -1: t.v(3) = -1
PRINT "3. the members around the array after its ends are set to -1: id, tail"; t.id; t.tail
REDIM x(0) AS t3
x(0).n(1).q(2) = 77: x(0).m.r = 88
PRINT "4. a chain: x(0).n(1).q(2), x(0).m.r, x(0).n(0).q(2), LEN(x(0))"; x(0).n(1).q(2); x(0).m.r; x(0).n(0).q(2); LEN(x(0))
PRINT "5. LBOUND(x(0).n), UBOUND(x(0).n), UBOUND(x(0).n(1).q):"; LBOUND(x(0).n); UBOUND(x(0).n); UBOUND(x(0).n(1).q)
DIM a(1) AS t1
a(0).v(1) = 5
PRINT "6. each element has its own: a(0).v(1), a(1).v(1)"; a(0).v(1); a(1).v(1)
DIM u AS t4
u.g(2, 3) = 100: u.g(1, 0) = -1: u.s(1) = "abcdef": u.d(1) = 1# / 3
PRINT "7. two dimensions, STRING * 3, DOUBLE: "; u.g(2, 3); u.g(1, 0); u.g(2, 0); "["; u.s(1); "]"; ASC(u.s(2)); u.d(1); LEN(u)
PRINT "   bounds of g:"; LBOUND(u.g, 1); UBOUND(u.g, 1); LBOUND(u.g, 2); UBOUND(u.g, 2)
PRINT "8. t.v(4) = 1:";
t.v(4) = 1
PRINT " t.v(-1):";
PRINT t.v(-1);
PRINT " UBOUND(t.v, 2):";
PRINT UBOUND(t.v, 2)
PRINT "9. a local variable, twice:";
local: local
PRINT
PRINT "10. a float index and a float stored: "; : t.v(1.5) = 2.5: PRINT t.v(2)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB local
    DIM l AS t1
    PRINT l.v(2);
    l.v(2) = 5
END SUB
