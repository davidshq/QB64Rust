$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4): NEXT i, j in the wrong order.
FOR i = 1 TO 2
    FOR j = 1 TO 2
        PRINT i; j
NEXT i, j
SYSTEM
