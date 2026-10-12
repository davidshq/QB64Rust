$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 6.1): STATIC v(2 + 3), a bound with an operator whose value is known.
ON ERROR GOTO h
s: s

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    STATIC v(2 + 3) AS LONG
    v(1) = v(1) + 1
    PRINT v(1); UBOUND(v)
END SUB
