$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a whole value assigned from an element of another array of the
' type, both indexes raising.
ON ERROR GOTO h
TYPE t1
    x AS LONG
END TYPE
DIM a(2) AS t1, b(2) AS t1
a(0).x = 1: b(0).x = 2: b(1).x = 3
PRINT "a(8) = b(9):";
a(8) = b(9)
PRINT a(0).x
PRINT "a(1) = b(ASC(empty) + 1):";
a(1) = b(ASC("") + 1)
PRINT a(1).x

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
