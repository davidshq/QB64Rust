$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): STATIC a() then STATIC a(3).
s
SYSTEM
SUB s
    STATIC a() AS LONG
    STATIC a(3) AS LONG
    PRINT UBOUND(a)
END SUB
