$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a SUB that uses its array parameter with two indexes, given an
' array of one dimension.
ON ERROR GOTO h
DIM x(3) AS LONG
two x()
PRINT x(1)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB two (m() AS LONG)
    m(1, 2) = 5
END SUB
