$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): DIM c AS LONG, then DIM c(2), then DIM c(3) AS DOUBLE in a SUB.
ON ERROR GOTO h
s
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    DIM c AS LONG
    c = 7
    DIM c(2)
    c(1) = 0.5
    DIM c(3) AS DOUBLE
    c(1) = c(1) + 1 / 3
    PRINT c; c(1); c!(1); UBOUND(c); UBOUND(c!)
END SUB
