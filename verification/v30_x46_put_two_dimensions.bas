$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (46_put_two_dimensions)
ON ERROR GOTO h
OPEN "v30_x46_put_two_dimensions.tmp" FOR BINARY AS #1
DIM a2(1 TO 2, 1 TO 3) AS INTEGER
PUT #1, 1, a2()
PRINT LOF(1)
CLOSE
KILL "v30_x46_put_two_dimensions.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
