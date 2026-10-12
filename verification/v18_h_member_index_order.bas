$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): with a member store into an element of a TYPE array, which part is
' evaluated first: the index or the value? Both call a FUNCTION that prints.
TYPE t
    m AS LONG
END TYPE
DIM a(3) AS t
DIM x(3) AS LONG
a(ix&(1)).m = va&(5)
PRINT a(1).m
x(ix&(2)) = va&(6)
PRINT x(2)
y& = x(ix&(2)) + va&(0)
PRINT y&
SYSTEM

FUNCTION ix& (n AS LONG)
    PRINT "index"; n
    ix& = n
END FUNCTION

FUNCTION va& (n AS LONG)
    PRINT "value"; n
    va& = n
END FUNCTION
