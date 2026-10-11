$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 6.1): SHARED m() in a SUB for a main-module array that was used
' without DIM above the SUB, and for a DIM SHARED one.
ON ERROR GOTO h
DIM SHARED g(3)
m(2) = 4: g(2) = 5
s
PRINT "main:"; m(2); g(2)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    SHARED m(), g()
    PRINT "SUB:"; m(2); g(2); UBOUND(m)
    m(2) = 6: g(2) = 7
END SUB
