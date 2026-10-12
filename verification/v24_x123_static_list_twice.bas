$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): STATIC a() twice in one SUB.
s
SYSTEM
SUB s
    STATIC a() AS LONG
    STATIC a() AS LONG
    PRINT 1
END SUB
