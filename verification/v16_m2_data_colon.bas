$CONSOLE:ONLY
' Verification (m2-parser-breadth, M2): a colon inside an unquoted DATA item. Does it end the statement?
DATA a: PRINT "after colon"
DATA "END"
DO
    READ s$
    IF s$ = "END" THEN EXIT DO
    PRINT "["; s$; "]"
LOOP
SYSTEM
