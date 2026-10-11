$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): the SUB above the call in the file, never indexing, the array
' having two dimensions.
SUB show (m() AS LONG)
    PRINT UBOUND(m, 2)
END SUB
DIM y(2, 4) AS LONG
show y()
SYSTEM
