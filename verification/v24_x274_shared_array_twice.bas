$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 6.1): SHARED a() twice in one SUB.
DIM a(3) AS LONG
a(1) = 7
s
SYSTEM
SUB s
    SHARED a() AS LONG
    SHARED a() AS LONG
    PRINT a(1)
END SUB
