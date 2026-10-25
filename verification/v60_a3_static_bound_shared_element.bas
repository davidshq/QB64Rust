$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): STATIC v(sa(1)) with an element of a shared static array as the bound,
' at the start and after ERASE v and a handled error with sa(1) changed.
ON ERROR GOTO h
DIM SHARED sa(3) AS LONG
sa(1) = 5
PRINT "1. first call:";: s 0
PRINT "2. ERASE, sa(1) = 8, error:";: s 1: sa(1) = 8: ERROR 5: s 0
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s (op AS INTEGER)
    STATIC v(sa(1)) AS LONG
    IF op = 1 THEN ERASE v: EXIT SUB
    v(0) = v(0) + 1
    PRINT UBOUND(v); v(0)
END SUB
