$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): COMMON SHARED a() without a type, then DIM a(3) AS LONG: shared
' or not.
ON ERROR GOTO h
COMMON SHARED a()
DIM a(3) AS LONG
a(2) = 6
s

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    PRINT "SUB a(2):";
    PRINT a(2)
END SUB
