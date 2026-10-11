$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): OPTION BASE inside a SUB, and a main-module DIM below the SUB in
' the file.
SUB s
    OPTION BASE 1
    DIM l(3)
    PRINT "in the SUB:"; LBOUND(l)
END SUB
DIM a(3)
PRINT "main below the SUB:"; LBOUND(a)
s
SYSTEM
