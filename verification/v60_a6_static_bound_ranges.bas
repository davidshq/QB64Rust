$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): STATIC with two dimensions and ranges whose bounds read shared
' variables and a CONST of the main module, at the start and after ERASE and a handled error.
ON ERROR GOTO h
CONST k = 2
DIM SHARED lo AS LONG, hi AS LONG
lo = 1: hi = 5
PRINT "1. first call:";: s 0
PRINT "2. ERASE, error:";: s 1: ERROR 5: s 0
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s (op AS INTEGER)
    STATIC v(lo TO hi + k, k + hi) AS LONG
    IF op = 1 THEN ERASE v: EXIT SUB
    PRINT LBOUND(v, 1); UBOUND(v, 1); LBOUND(v, 2); UBOUND(v, 2)
END SUB
