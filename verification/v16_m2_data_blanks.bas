$CONSOLE:ONLY
' Verification (m2-parser-breadth, M2): DATA items with leading and trailing blanks, quoted blanks, empty items.
DATA   x  ,  y y  ,,"  q  ",
DATA ,
DATA "END"
DO
    READ s$
    IF s$ = "END" THEN EXIT DO
    PRINT "["; s$; "]"; LEN(s$)
LOOP
SYSTEM
