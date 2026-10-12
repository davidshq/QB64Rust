$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 8): as v24_x306 for a type with a STRING member between two
' other members, and for one with a nested type that has one: how often the indexes are evaluated.
TYPE t1
    x AS LONG
    n AS STRING
    f AS STRING * 2
END TYPE
TYPE t2
    i AS t1
    k AS LONG
    m AS STRING
END TYPE
DIM a(3) AS t1, b(3) AS t1, q AS t1
DIM c(3) AS t2, d(3) AS t2
b(1).x = 7: b(1).n = "seven"
PRINT "a(one(1)) = b(one(2)):";
a(one(1)) = b(one(2))
PRINT a(1).x; a(1).n
PRINT "q = b(one(3)):";
q = b(one(3))
PRINT q.x; q.n
PRINT "a(one(4)) = q:";
a(one(4)) = q
PRINT a(1).x; a(1).n
d(1).i.n = "in": d(1).m = "out": d(1).k = 5
PRINT "c(one(5)) = d(one(6)):";
c(one(5)) = d(one(6))
PRINT c(1).k; c(1).i.n; " "; c(1).m
PRINT "c(one(7)).i = d(one(8)).i:";
d(1).i.n = "again"
c(one(7)).i = d(one(8)).i
PRINT " "; c(1).i.n
SYSTEM

FUNCTION one (n AS LONG)
    PRINT n;
    one = 1
END FUNCTION
