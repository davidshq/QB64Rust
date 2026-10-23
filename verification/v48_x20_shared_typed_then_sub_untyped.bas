$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): DIM SHARED c(2) AS DOUBLE, then DIM c(3) in a SUB: which array the plain c names in the SUB.
ON ERROR GOTO h
DIM SHARED c(2) AS DOUBLE
c(1) = 1 / 3
s
PRINT c(1); UBOUND(c)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    DIM c(3)
    c(1) = c(1) + 1
    PRINT c(1); LEN(c(1)); UBOUND(c); UBOUND(c!); UBOUND(c#)
END SUB
