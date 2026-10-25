$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): STATIC v(n) with a shared n, read by the start code: when the program
' starts (n is 0), when a handled error enters it with the array allocated, and after ERASE v with n changed.
ON ERROR GOTO h
DIM SHARED n AS LONG
n = 4
PRINT "1. first call:";: s 0
n = 7
PRINT "2. n = 7, error:";: ERROR 5: s 0
PRINT "3. ERASE, error:";: s 1: ERROR 5: s 0
PRINT "4. ERASE, n = 9, error:";: s 1: n = 9: ERROR 5: s 0
PRINT "5. ERASE, no error:";: s 1: s 0
PRINT "6. again:";: s 0
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s (op AS INTEGER)
    STATIC v(n) AS LONG
    IF op = 1 THEN ERASE v: EXIT SUB
    v(0) = v(0) + 1
    PRINT UBOUND(v); v(0)
END SUB
