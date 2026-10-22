$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, review): does the old compiler accept this, and what does it do? (41_number_25_digits)
ON ERROR GOTO handler
GOTO 1234567890123456789012345
PRINT "skipped"
1234567890123456789012345 PRINT "long": ERROR 5
PRINT "end"
SYSTEM
handler:
PRINT "  ERL"; ERL
RESUME NEXT
