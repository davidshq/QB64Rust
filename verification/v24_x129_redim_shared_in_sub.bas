$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM SHARED inside a SUB.
s
SYSTEM
SUB s
    REDIM SHARED a(3) AS LONG
    PRINT UBOUND(a)
END SUB
