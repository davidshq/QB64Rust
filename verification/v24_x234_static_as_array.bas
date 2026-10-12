$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1, for task 3.3): STATIC AS type with an array in the list.
s: s
SYSTEM
SUB s
    STATIC AS LONG k, c(3)
    k = k + 1: c(1) = c(1) + 2
    PRINT k; c(1)
END SUB
