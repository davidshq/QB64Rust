$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (03_ten_and_ten_point_zero)
ON ERROR GOTO handler
GOTO 10.0
10 PRINT "at 10": ERROR 5
10.0 PRINT "at 10.0": ERROR 5
PRINT "end"
SYSTEM
handler:
PRINT "  ERL"; ERL
RESUME NEXT
