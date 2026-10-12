$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7): two array parameters of one name and type.
DIM x(3) AS LONG
both x(), x()
SYSTEM
SUB both (a() AS LONG, a() AS LONG)
    PRINT a(1)
END SUB
