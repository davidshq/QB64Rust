$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): the call above the SUB in the file, the SUB using two indexes,
' the array having two.
DIM y(2, 4) AS LONG
two y()
PRINT y(1, 2)
SYSTEM
SUB two (m() AS LONG)
    m(1, 2) = 5
END SUB
