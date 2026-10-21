$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): STATIC c() AS DOUBLE in a SUB, then c(1) used with no other declaration.
ON ERROR GOTO h
s
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    STATIC c() AS DOUBLE
    c(1) = 1 / 3
    PRINT c(1); LEN(c(1))
END SUB
