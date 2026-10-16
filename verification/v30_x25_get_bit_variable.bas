$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (25_get_bit_variable)
ON ERROR GOTO h
OPEN "v30_x25_get_bit_variable.tmp" FOR BINARY AS #1
DIM bt AS _BIT
GET #1, , bt
CLOSE
KILL "v30_x25_get_bit_variable.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
