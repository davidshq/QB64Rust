$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (12_point_forms)
ON ERROR GOTO handler
.5 PRINT "at .5": ERROR 5
50. PRINT "at 50.": ERROR 5
1.10 PRINT "at 1.10": ERROR 5
1.1 PRINT "at 1.1": ERROR 5
IF n = 0 THEN n = 1: GOTO 1.10
PRINT "end"
SYSTEM
handler:
PRINT "  ERL"; ERL
RESUME NEXT
