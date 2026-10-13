$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (35_put_bit_array)
ON ERROR GOTO h
OPEN "v30_x35_put_bit_array.tmp" FOR BINARY AS #1
DIM ba(7) AS _BIT
PUT #1, , ba()
CLOSE
KILL "v30_x35_put_bit_array.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
