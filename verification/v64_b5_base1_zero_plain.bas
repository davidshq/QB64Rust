$CONSOLE:ONLY
' Verification (beside v64_b4 and verification\qb45\q45_base1_zero): the number 0 as the upper bound of a plain
' array under OPTION BASE 1: a static DIM, a REDIM with the 0 in a variable and in an expression, under a handler.
OPTION BASE 1
ON ERROR GOTO h
DIM n AS LONG
DIM s(0) AS LONG
PRINT "1. DIM s(0):"; LBOUND(s); UBOUND(s)
REDIM x(5) AS LONG
PRINT "2. REDIM x(n), n = 0:";
REDIM x(n) AS LONG
PRINT ":";
PRINT LBOUND(x);
PRINT UBOUND(x)
PRINT "3. REDIM x(0 + 0):";
REDIM x(0 + 0) AS LONG
PRINT ":";
PRINT LBOUND(x);
PRINT UBOUND(x)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
