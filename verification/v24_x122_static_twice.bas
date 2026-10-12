$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): STATIC a(3) twice in one SUB.
s
SYSTEM
SUB s
    STATIC a(3) AS LONG
    STATIC a(3) AS LONG
    PRINT UBOUND(a)
END SUB
