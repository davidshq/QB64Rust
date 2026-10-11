$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): STATIC v(n) with a bound that is not constant.
ON ERROR GOTO h
s 3: s 5

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s (n AS LONG)
    STATIC v(n) AS LONG
    v(1) = v(1) + 1
    PRINT v(1); UBOUND(v)
END SUB
