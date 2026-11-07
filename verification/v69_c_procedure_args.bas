$CONSOLE:ONLY
' Verification (OpenSpec change eval-order-left-to-right, task 2.1): the order in which the arguments of a FUNCTION,
' a SUB and a library function are evaluated, by value and by reference. Each FUNCTION prints its name and argument.
DECLARE LIBRARY
    FUNCTION labs& ALIAS abs (BYVAL v AS LONG)
    FUNCTION pw# ALIAS pow (BYVAL a AS DOUBLE, BYVAL b AS DOUBLE)
END DECLARE
DIM SHARED a(5) AS LONG
a(1) = 10: a(2) = 20
PRINT "FUNCTION by value:";: PRINT h(f(1), f(2))
PRINT "FUNCTION by reference:";: PRINT h(a(f(1)), a(f(2)))
PRINT "SUB by value:";: two f(1), f(2)
PRINT "SUB by reference:";: two a(f(1)), a(f(2))
PRINT "nested:";: PRINT h(h(f(1), f(2)), h(f(3), f(4)))
PRINT "library by value:";: PRINT pw(f(2), f(3))
PRINT "library calls:";: PRINT labs(f(1)) - labs(f(2))
PRINT "library and FUNCTION:";: PRINT labs(f(1)) - f(2)
SYSTEM

FUNCTION f& (k AS LONG)
    PRINT " f"; LTRIM$(STR$(k));
    f& = k
END FUNCTION

FUNCTION h& (p AS LONG, q AS LONG)
    PRINT " h"; LTRIM$(STR$(p)); ","; LTRIM$(STR$(q));
    h& = p * 10 + q
END FUNCTION

SUB two (p AS LONG, q AS LONG)
    PRINT " two"; p; q
END SUB
