$CONSOLE:ONLY
' Verification (m2-parser-breadth, M2): a colon inside a quoted DATA item, then a colon outside quotes.
DATA "a:b", c: PRINT "after colon"
DATA "END"
DO
    READ s$
    IF s$ = "END" THEN EXIT DO
    PRINT "["; s$; "]"
LOOP
SYSTEM
