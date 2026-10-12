$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7): an array parameter passed on to the procedure itself, and two parameters given one array.
ON ERROR GOTO h
DIM x(1 TO 4) AS LONG
REDIM d(1 TO 4) AS LONG
down x(), 4
PRINT x(1); x(2); x(3); x(4)
pair x(), x()
PRINT x(1); x(2); UBOUND(x)
pair d(), d()
PRINT d(1); UBOUND(d)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
SUB down (a() AS LONG, n AS LONG)
    IF n = 0 THEN EXIT SUB
    a(n) = n * 10
    down a(), n - 1
END SUB
SUB pair (p() AS LONG, q() AS LONG)
    p(1) = 1: q(1) = q(1) + 1
    REDIM q(1 TO 2) AS LONG
    PRINT UBOUND(p);
END SUB
