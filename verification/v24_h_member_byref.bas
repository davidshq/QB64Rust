$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an element of a member array passed to a SUB: by reference for
' the exact type, a copy otherwise, and with an index outside.
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

ON ERROR GOTO h
DIM t AS t1, a(1) AS t1
t.v(2) = 1: a(1).v(2) = 1
bump t.v(2)
bump a(1).v(2)
PRINT "by reference: t.v(2), a(1).v(2)"; t.v(2); a(1).v(2)
bump (t.v(2))
bumpi t.v(2)
PRINT "in parentheses and to an INTEGER parameter: t.v(2)"; t.v(2)
PRINT "an index outside, bump t.v(9):";
bump t.v(9)
PRINT " t.v(0)"; t.v(0)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB bump (n AS LONG)
    n = n + 1
END SUB
SUB bumpi (n AS INTEGER)
    n = n + 1
END SUB
