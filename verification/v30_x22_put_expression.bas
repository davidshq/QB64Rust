$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (22_put_expression)
ON ERROR GOTO h
OPEN "v30_x22_put_expression.tmp" FOR BINARY AS #1
DIM i AS INTEGER
PUT #1, , i + 1
CLOSE
KILL "v30_x22_put_expression.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
