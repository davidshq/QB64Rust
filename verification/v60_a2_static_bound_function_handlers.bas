$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): STATIC v(f) with a FUNCTION that prints and counts its calls: how often
' it runs when the program starts and when handled errors enter the start code, with v allocated and after ERASE v,
' and whether what it prints then is seen.
ON ERROR GOTO h
DIM SHARED c AS LONG
PRINT "1. calls at start:"; c
PRINT "2. two calls:";: s 0: s 0
PRINT "3. error:";: ERROR 5: PRINT " calls:"; c
PRINT "4. error again:";: ERROR 5: PRINT " calls:"; c
PRINT "5. ERASE, error:";: s 1: ERROR 5: PRINT " calls:"; c
PRINT "6. after:";: s 0
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

FUNCTION f
    c = c + 1
    PRINT "[f]";
    f = 6
END FUNCTION

SUB s (op AS INTEGER)
    STATIC v(f) AS LONG
    IF op = 1 THEN ERASE v: EXIT SUB
    v(1) = v(1) + 1
    PRINT UBOUND(v); v(1)
END SUB
