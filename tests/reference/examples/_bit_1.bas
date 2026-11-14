$CONSOLE:ONLY
DIM flag AS _BIT
flag = 1
PRINT "A _BIT set to 1 holds"; flag
DIM small AS _BIT * 3
small = 3: PRINT "3 in _BIT * 3:"; small
small = 5: PRINT "5 in _BIT * 3:"; small
DIM u AS _UNSIGNED _BIT * 3
u = 13: PRINT "13 in _UNSIGNED _BIT * 3:"; u
END
