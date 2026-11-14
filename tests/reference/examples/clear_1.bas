$CONSOLE:ONLY
DIM SHARED total AS LONG
DIM a(1 TO 3) AS INTEGER
total = 5: a(2) = 7: s$ = "text"
CLEAR
PRINT "After CLEAR:"; total; a(2); "["; s$; "]"
END
