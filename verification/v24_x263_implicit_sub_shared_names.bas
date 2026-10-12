$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 5.1): in a SUB, name(...) where the main module has an array of that
' name that is not shared, a DIM SHARED scalar of that name typed with AS, and a parameter of that name.
ON ERROR GOTO h
DIM a(5)
DIM SHARED g AS LONG
a(1) = 7: g = 3
p 4
PRINT a(1); g

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB p (n)
    PRINT "a(1) in the SUB:";
    PRINT a(1)
    PRINT "g(1) beside DIM SHARED g AS LONG:";
    g(1) = 2.5
    PRINT g(1); LEN(g(1))
    PRINT "n(1) beside the parameter n:";
    n(1) = 1
    PRINT n(1); n
END SUB
