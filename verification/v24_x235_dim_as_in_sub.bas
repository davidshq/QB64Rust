$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1, for task 3.3): DIM AS type inside a SUB, a scalar and an array.
s: s
SYSTEM
SUB s
    DIM AS LONG k, c(3)
    k = k + 1: c(1) = c(1) + 2
    PRINT k; c(1)
END SUB
