$CONSOLE:ONLY
' Verification (m2-parser-breadth, M2): REM inside an unquoted DATA item.
DATA a REM b, c
DATA "END"
DO
    READ s$
    IF s$ = "END" THEN EXIT DO
    PRINT "["; s$; "]"
LOOP
SYSTEM
