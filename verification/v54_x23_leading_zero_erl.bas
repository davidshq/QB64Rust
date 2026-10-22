$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (23_leading_zero_erl)
ON ERROR GOTO handler
GOTO 010
PRINT "skipped"
010 PRINT "at 010": ERROR 5
PRINT "end"
SYSTEM
handler:
PRINT "  ERL"; ERL
RESUME NEXT
