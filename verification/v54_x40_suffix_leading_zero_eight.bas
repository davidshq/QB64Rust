$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, review): does the old compiler accept this, and what does it do? (40_suffix_leading_zero_eight)
ON ERROR GOTO handler
GOTO 08&
PRINT "skipped"
08& PRINT "at 08&": ERROR 5
PRINT "end"
SYSTEM
handler:
PRINT "  ERL"; ERL
RESUME NEXT
