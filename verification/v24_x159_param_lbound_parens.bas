$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): LBOUND(a()) of an array parameter.
DIM x(3) AS LONG
s x()
SYSTEM
SUB s (a() AS LONG)
    PRINT LBOUND(a())
END SUB
