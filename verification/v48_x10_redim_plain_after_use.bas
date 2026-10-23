$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): REDIM i without AS in a SUB after a use of i.
s
SYSTEM

SUB s
    i = 2.5
    PRINT i;
    REDIM i
    i = i + 1
    PRINT i; LEN(i)
END SUB
