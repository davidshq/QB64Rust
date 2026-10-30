$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (SOMEDAY, member arrays): REDIM t.d(0) AS LONG under OPTION BASE 1 of a _DYNAMIC member, beside
' the same REDIM of a plain dynamic array, under a handler.
TYPE dy
    _DYNAMIC d(1 TO 3) AS LONG
END TYPE
ON ERROR GOTO h
DIM t AS dy
REDIM x(5) AS LONG
OPTION BASE 1
PRINT "1. REDIM x(0):";
REDIM x(0) AS LONG
PRINT ":"; LBOUND(x); UBOUND(x)
PRINT "2. REDIM t.d(0):";
REDIM t.d(0) AS LONG
PRINT ":"; LBOUND(t.d); UBOUND(t.d)
PRINT "3. REDIM _PRESERVE t.d(0):";
REDIM t.d(1 TO 3) AS LONG
REDIM _PRESERVE t.d(0) AS LONG
PRINT ":"; LBOUND(t.d); UBOUND(t.d)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
