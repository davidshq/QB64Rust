$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (04_leading_zero)
ON ERROR GOTO handler
GOTO 010
10 PRINT "at 10": ERROR 5
010 PRINT "at 010": ERROR 5
0 PRINT "at 0": ERROR 5
PRINT "end"
SYSTEM
handler:
PRINT "  ERL"; ERL
RESUME NEXT
