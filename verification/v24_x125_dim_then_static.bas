$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): DIM a(3) then STATIC a(3) in one SUB.
s
SYSTEM
SUB s
    DIM a(3) AS LONG
    STATIC a(3) AS LONG
    PRINT UBOUND(a)
END SUB
