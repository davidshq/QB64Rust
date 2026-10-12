$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 9): a whole value assigned between two elements whose indexes
' call a FUNCTION, for types with member arrays: how often the indexes are evaluated, and what is copied.
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
    tail AS INTEGER
END TYPE
TYPE t2
    id AS LONG
    s(0 TO 2) AS STRING
    v(0 TO 3) AS LONG
END TYPE
TYPE t3
    n(0 TO 1) AS t2
    k AS LONG
END TYPE
TYPE t4
    s AS STRING
    v(0 TO 3) AS LONG
    f(0 TO 1) AS STRING * 2
    w(0 TO 1, 0 TO 1) AS INTEGER
END TYPE
TYPE t5
    n(0 TO 1) AS t1
    s AS STRING
END TYPE
DIM a1(3) AS t1, b1(3) AS t1
DIM a2(3) AS t2, b2(3) AS t2
DIM a3(3) AS t3, b3(3) AS t3
DIM a4(3) AS t4, b4(3) AS t4
DIM a5(3) AS t5, b5(3) AS t5
b1(1).id = 1: b1(1).v(0) = 10: b1(1).v(3) = 13: b1(1).tail = 9
PRINT "t1, no string:";
a1(one(1)) = b1(one(2))
PRINT ":"; a1(1).id; a1(1).v(0); a1(1).v(3); a1(1).tail
b2(1).id = 2: b2(1).s(0) = "s0": b2(1).s(2) = "s2": b2(1).v(3) = 23
PRINT "t2, a STRING array of 3 and a LONG array:";
a2(one(1)) = b2(one(2))
PRINT ":"; a2(1).id; "["; a2(1).s(0); "]["; a2(1).s(1); "]["; a2(1).s(2); "]"; a2(1).v(3)
b3(1).n(0).s(1) = "n0s1": b3(1).n(1).s(2) = "n1s2": b3(1).n(1).v(3) = 33: b3(1).k = 3
PRINT "t3, an array of 2 of t2:";
a3(one(1)) = b3(one(2))
PRINT ":"; "["; a3(1).n(0).s(1); "]["; a3(1).n(1).s(2); "]"; a3(1).n(1).v(3); a3(1).k
b4(1).s = "s": b4(1).v(3) = 43: b4(1).f(1) = "ff": b4(1).w(1, 1) = 44
PRINT "t4, a STRING and arrays without one:";
a4(one(1)) = b4(one(2))
PRINT ":"; "["; a4(1).s; "]"; a4(1).v(3); "["; a4(1).f(1); "]"; a4(1).w(1, 1)
b5(1).n(1).v(3) = 53: b5(1).s = "t5"
PRINT "t5, an array of t1 and a STRING:";
a5(one(1)) = b5(one(2))
PRINT ":"; a5(1).n(1).v(3); "["; a5(1).s; "]"
PRINT "an element of a member array of t2, a3(one(1)).n(one(2)) = b3(one(3)).n(one(4)):";
a3(one(1)).n(one(2)) = b3(one(3)).n(one(4))
PRINT ":"; "["; a3(1).n(1).s(2); "]"
SYSTEM

FUNCTION one (n AS LONG)
    PRINT n;
    one = 1
END FUNCTION
