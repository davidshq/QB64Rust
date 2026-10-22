$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (24_zero_line)
ON ERROR GOTO handler
0 PRINT "at 0": ERROR 5
IF n = 0 THEN n = 1: GOTO 0
PRINT "end"
SYSTEM
handler:
PRINT "  ERL"; ERL
RESUME NEXT
