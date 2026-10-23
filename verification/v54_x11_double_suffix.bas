$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (11_double_suffix)
ON ERROR GOTO handler
GOTO 30#
30 PRINT "at 30": ERROR 5
30# PRINT "at 30#": ERROR 5
40! PRINT "at 40!": ERROR 5
PRINT "end"
SYSTEM
handler:
PRINT "  ERL"; ERL
RESUME NEXT
