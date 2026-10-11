$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a SUB that uses its array parameter with one index, given an
' array of two dimensions.
ON ERROR GOTO h
DIM x(3, 3) AS LONG
one x()
PRINT x(1, 0)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB one (m() AS LONG)
    m(1) = 5
END SUB
