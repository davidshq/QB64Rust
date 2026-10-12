$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): under OPTION BASE 1, STATIC a(0) and a local DIM a(0) in a SUB.
OPTION BASE 1
s
SYSTEM
SUB s
    STATIC a(0)
    DIM b(0)
    PRINT LBOUND(a); UBOUND(a); LBOUND(b); UBOUND(b)
END SUB
