$CONSOLE:ONLY
' Verification (OpenSpec change eval-order-left-to-right, task 2.1): the order of an assignment's target and value.
' A store into an element evaluates the index first, as a C++ statement of its own. A store through a member with an
' index on the way evaluates a number before the index, in one C++ assignment, and a string after it. Each FUNCTION
' prints its name and argument.
TYPE rec
    n AS LONG
    s AS STRING
END TYPE
DIM SHARED a(5) AS LONG, s(5) AS STRING, r(5) AS rec
PRINT "a(f(1)) = f(2):";: a(f(1)) = f(2): PRINT a(1)
PRINT "s(f(1)) = t$(2):";: s(f(1)) = t$(2): PRINT " "; s(1)
PRINT "r(f(1)).n = f(2):";: r(f(1)).n = f(2): PRINT r(1).n
PRINT "r(f(1)).s = t$(2):";: r(f(1)).s = t$(2): PRINT " "; r(1).s
PRINT "a(f(1)) = f(2) + f(3):";: a(f(1)) = f(2) + f(3): PRINT a(1)
SYSTEM

FUNCTION f& (k AS LONG)
    PRINT " f"; LTRIM$(STR$(k));
    f& = k
END FUNCTION

FUNCTION t$ (k AS LONG)
    PRINT " t"; LTRIM$(STR$(k));
    t$ = "v" + LTRIM$(STR$(k))
END FUNCTION
