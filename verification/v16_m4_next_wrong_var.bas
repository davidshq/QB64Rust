$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4): NEXT with the variable of an outer FOR.
FOR i = 1 TO 2
    FOR j = 1 TO 2
        PRINT i; j
    NEXT i
NEXT j
SYSTEM
