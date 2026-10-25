$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): STATIC v(n) AS STRING with a shared n, kept between calls, after ERASE
' and a handled error with n changed.
ON ERROR GOTO h
DIM SHARED n AS LONG
PRINT "1. two calls:";: s 0: s 0
n = 3
PRINT "2. ERASE, n = 3, error:";: s 1: ERROR 5: s 0: s 0
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s (op AS INTEGER)
    STATIC v(n) AS STRING
    IF op = 1 THEN ERASE v: EXIT SUB
    v(0) = v(0) + "x"
    PRINT UBOUND(v); v(0);
END SUB
