$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): SHARED a(3) with bounds.
DIM a(3) AS LONG
s
SYSTEM
SUB s
    SHARED a(3) AS LONG
    PRINT UBOUND(a)
END SUB
