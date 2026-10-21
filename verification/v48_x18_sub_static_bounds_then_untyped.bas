$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): STATIC c(2) AS DOUBLE then DIM c(3) in a SUB: which array the plain c names.
ON ERROR GOTO h
s
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    STATIC c(2) AS DOUBLE
    c(1) = 1 / 3
    DIM c(3)
    c(1) = c(1) + 1
    PRINT c(1); LEN(c(1)); UBOUND(c); UBOUND(c!); UBOUND(c#)
END SUB
