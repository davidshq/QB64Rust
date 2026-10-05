$CONSOLE:ONLY
' Verification (m2-parser-breadth, M2): an apostrophe inside an unquoted DATA item.
DATA a'b, c
DATA "END"
DO
    READ s$
    IF s$ = "END" THEN EXIT DO
    PRINT "["; s$; "]"
LOOP
SYSTEM
