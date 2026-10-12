$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7): an array parameter and a scalar parameter of one name.
DIM x(3) AS LONG
x(1) = 4
both x(), 9
SYSTEM
SUB both (a() AS LONG, a AS LONG)
    PRINT a(1); a
END SUB
