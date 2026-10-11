$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM and ERASE of a member array of an array element, and what
' the other element shows.
ON ERROR GOTO h
TYPE t1
    v(0 TO 3) AS LONG
END TYPE
DIM a(1) AS t1
a(0).v(1) = 5: a(1).v(1) = 6
PRINT "REDIM a(0).v(0 TO 3):";
REDIM a(0).v(0 TO 3)
PRINT a(0).v(1); a(1).v(1)
a(0).v(1) = 5
PRINT "ERASE a(1).v:";
ERASE a(1).v
PRINT a(0).v(1); a(1).v(1)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
