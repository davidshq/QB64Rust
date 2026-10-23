$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): STATIC c() AS DOUBLE then REDIM c(2) AS DOUBLE in a SUB: does the plain c name the DOUBLE array.
ON ERROR GOTO h
s
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    STATIC c() AS DOUBLE
    REDIM c(2) AS DOUBLE
    c(1) = 1 / 3
    PRINT c(1); LEN(c(1)); UBOUND(c)
END SUB
