$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): ERASE and LBOUND of a dynamic _BIT array, and a _BIT array used
' without DIM.
ON ERROR GOTO h
REDIM r(4) AS _BIT * 5
r(1) = 3
ERASE r
PRINT "UBOUND after ERASE:";
PRINT UBOUND(r)
im`(3) = 1
PRINT im`(3); UBOUND(im`)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
