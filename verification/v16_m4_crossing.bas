$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4): DO closed inside a FOR it encloses (crossing blocks).
DO
    FOR i = 1 TO 2
        PRINT i
LOOP UNTIL 1
NEXT
SYSTEM
