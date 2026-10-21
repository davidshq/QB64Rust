$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): REDIM SHARED i AS LONG in a SUB.
s
PRINT i
SYSTEM

SUB s
    REDIM SHARED i AS LONG
    i = 5
END SUB
