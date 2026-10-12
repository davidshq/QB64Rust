$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): the store rule of a member-array element: the value against the
' index, the outer against the inner index, what is written when one is outside, and a read with an index
' outside.
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
DIM t AS t1, a(2) AS t1, k AS LONG
t.v(0) = 100: a(0).v(0) = 200: a(0).v(1) = 201: a(1).v(0) = 210: a(1).v(1) = 211
PRINT "1. t.v(9) = 5:";
t.v(9) = 5
PRINT " t.v(0)"; t.v(0)
PRINT "2. t.v(9) = ASC(empty):";
t.v(9) = ASC("")
PRINT " t.v(0)"; t.v(0)
PRINT "3. t.v(1) = ASC(empty):";
t.v(1) = 55
t.v(1) = ASC("")
PRINT " t.v(1)"; t.v(1)
PRINT "4. a(9).v(1) = 5 (outer index outside):";
a(9).v(1) = 5
PRINT " a(0).v(1), a(1).v(1)"; a(0).v(1); a(1).v(1)
PRINT "5. a(1).v(9) = 6 (inner index outside):";
a(1).v(9) = 6
PRINT " a(1).v(0), a(1).v(1), a(0).v(0)"; a(1).v(0); a(1).v(1); a(0).v(0)
PRINT "6. a(9).v(9) = 7 (both outside):";
a(9).v(9) = 7
PRINT " a(0).v(0), a(0).v(1)"; a(0).v(0); a(0).v(1)
PRINT "7. a(9).v(9) = ASC(empty):";
a(9).v(9) = ASC("")
PRINT " a(0).v(0)"; a(0).v(0)
PRINT "8. reads: t.v(9), a(9).v(1), a(1).v(9):";
k = t.v(9): PRINT k;
k = a(9).v(1): PRINT k;
k = a(1).v(9): PRINT k
PRINT "9. order of the indexes, a(one(1)).v(one(2)) = one(3):";
a(one(1)).v(one(2)) = one(3)
PRINT

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

FUNCTION one (n AS LONG)
    PRINT n;
    one = 1
END FUNCTION
