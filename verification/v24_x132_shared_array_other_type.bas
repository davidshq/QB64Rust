$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): SHARED g() without AS when the main array is LONG.
ON ERROR GOTO h
DIM g(3) AS LONG
g(2) = 8
s

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    SHARED g()
    PRINT g(2); LEN(g(2))
END SUB
