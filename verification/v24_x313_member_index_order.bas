$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 9): the order of the indexes of a member-array element in a read
' and in a store, with two dimensions, through a chain, and after an index outside; a STRING element stored
' with an index outside.
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
    g(1 TO 2, 0 TO 3) AS INTEGER
    s(0 TO 1) AS STRING
    f(0 TO 1) AS STRING * 3
END TYPE
TYPE t2
    AS INTEGER q(2), r
END TYPE
TYPE t3
    AS t2 n(1), m
END TYPE
ON ERROR GOTO h
DIM a(3) AS t1, x(1) AS t3, t AS t1, k AS LONG
a(1).v(1) = 11
PRINT "1. a read, a(one(1)).v(one(2)):";
k = a(one(1)).v(one(2))
PRINT ":"; k
PRINT "2. two dimensions, t.g(one(1), one(2)) = one(3):";
t.g(one(1), one(2)) = one(3)
PRINT ":"; t.g(1, 1)
PRINT "3. a chain, x(one(1)).n(one(2)).q(one(3)) = one(4):";
x(one(1)).n(one(2)).q(one(3)) = one(4)
PRINT ":"; x(1).n(1).q(1)
PRINT "4. the outer index outside, a(nine(1)).v(one(2)) = one(3):";
a(0).v(1) = 100
a(nine(1)).v(one(2)) = one(3)
PRINT ":"; a(0).v(1)
PRINT "5. the first of two dimensions outside, t.g(nine(1), one(2)) = 7:";
t.g(nine(1), one(2)) = 7
PRINT ":"; t.g(1, 1)
PRINT "6. the second of two dimensions outside, t.g(2, 9) = 8:";
t.g(2, 0) = 1: t.g(1, 0) = 1
t.g(2, 9) = 8
PRINT ":"; t.g(2, 0); t.g(1, 0)
PRINT "7. a STRING element with its index outside, t.s(9) = new:";
t.s(0) = "old"
t.s(9) = "new"
PRINT ":["; t.s(0); "]"
PRINT "8. a STRING * 3 element with its index outside, t.f(9) = new:";
t.f(0) = "old"
t.f(9) = "new"
PRINT ":["; t.f(0); "]"
PRINT "9. reads with an index outside: t.s(9), t.f(9), t.g(9, 0), t.g(2, 9):";
t.g(1, 0) = 5: t.g(2, 0) = 6
PRINT "["; t.s(9); "]";
PRINT "["; t.f(9); "]";
PRINT t.g(9, 0);
PRINT t.g(2, 9)
PRINT "10. a string value that reads what the index changes, t.s(setz(1)) = z$:";
z$ = "before"
t.s(setz(1)) = z$
PRINT ":["; t.s(1); "]"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

FUNCTION one (n AS LONG)
    PRINT n;
    one = 1
END FUNCTION

FUNCTION nine (n AS LONG)
    PRINT n;
    nine = 9
END FUNCTION

FUNCTION setz (n AS LONG)
    SHARED z$
    z$ = "after"
    setz = n
END FUNCTION
