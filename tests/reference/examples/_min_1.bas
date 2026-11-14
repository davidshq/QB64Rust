$CONSOLE:ONLY
PRINT _MIN(3, 7); _MAX(3, 7)
PRINT _MIN(-2.5, 1); _MAX(-2.5, 1)
best = 0
FOR i = 1 TO 4
    READ v
    best = _MAX(best, v)
NEXT i
PRINT "The highest is"; best
DATA 12, 47, 5, 30
END
