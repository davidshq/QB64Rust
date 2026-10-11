$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): DIM SHARED of an array inside a SUB.
s
SYSTEM
SUB s
    DIM SHARED a(3) AS LONG
    PRINT UBOUND(a)
END SUB
