$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 8): a whole value assigned between two elements whose indexes
' call a FUNCTION, for a type without STRING members: which index is evaluated first, and how often.
TYPE t1
    x AS LONG
    y AS LONG
    z AS INTEGER
END TYPE
DIM a(3) AS t1, b(3) AS t1, q AS t1
b(1).x = 7
PRINT "a(one(1)) = b(one(2)):";
a(one(1)) = b(one(2))
PRINT a(1).x
PRINT "q = b(one(3)):";
q = b(one(3))
PRINT q.x
PRINT "a(one(4)) = q:";
a(one(4)) = q
PRINT a(1).x
SYSTEM

FUNCTION one (n AS LONG)
    PRINT n;
    one = 1
END FUNCTION
